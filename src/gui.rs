//! The window of the machine-control program: the position of the machine, the jog, the program and its path, the line of commands and
//! the log, over a controller (the simulated one for now). The words are the ones of `eh3d-view` in the eight languages and the colours
//! are the ones of its themes.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use eframe::egui::{
    self, Align2, Color32, FontData, FontDefinitions, FontFamily, Pos2, Rect, RichText, Stroke,
    StrokeKind, Vec2,
};
use eh3d_controller::sim::SimController;
use eh3d_controller::{Family, State};
use eh3d_view::texts::{tr, Lang, Text};
use eh3d_view::theme::{Rgb, Theme, ThemeId};

use crate::cli::Options;
use crate::gcode::{self, Segment};
use crate::session::{Session, STEPS};

fn col(c: Rgb) -> Color32 {
    Color32::from_rgb(c.0, c.1, c.2)
}

fn alpha(c: Color32, a: u8) -> Color32 {
    Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), a)
}

/// The colours of egui made from the theme.
fn apply_visuals(ctx: &egui::Context, t: &Theme) {
    let mut v = egui::Visuals::dark();
    let (panel, raised, text, accent) = (
        col(t.panel),
        col(t.panel_raised),
        col(t.text),
        col(t.accent),
    );
    v.panel_fill = col(t.background.0);
    v.window_fill = panel;
    v.extreme_bg_color = raised;
    v.override_text_color = Some(text);
    v.selection.bg_fill = alpha(accent, 110);
    v.selection.stroke = Stroke::new(1.5, accent);
    for w in [
        &mut v.widgets.inactive,
        &mut v.widgets.hovered,
        &mut v.widgets.active,
    ] {
        w.corner_radius = 6.0.into();
        w.bg_fill = raised;
        w.weak_bg_fill = raised;
    }
    v.widgets.inactive.bg_stroke = Stroke::new(1.0, alpha(accent, 110));
    v.widgets.hovered.bg_stroke = Stroke::new(1.5, accent);
    v.widgets.hovered.weak_bg_fill = alpha(accent, 50);
    v.widgets.active.bg_stroke = Stroke::new(2.0, accent);
    v.widgets.active.weak_bg_fill = alpha(accent, 110);
    ctx.set_visuals(v);
}

fn install_fonts(ctx: &egui::Context) {
    let candidates = [
        "C:/Windows/Fonts/msyh.ttc",
        "C:/Windows/Fonts/YuGothM.ttc",
        "C:/Windows/Fonts/meiryo.ttc",
        "C:/Windows/Fonts/simsun.ttc",
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
        "/System/Library/Fonts/PingFang.ttc",
    ];
    for path in candidates {
        if let Ok(bytes) = std::fs::read(path) {
            let mut fonts = FontDefinitions::default();
            fonts
                .font_data
                .insert("cjk".into(), Arc::new(FontData::from_owned(bytes)));
            for family in [FontFamily::Monospace, FontFamily::Proportional] {
                fonts.families.entry(family).or_default().push("cjk".into());
            }
            ctx.set_fonts(fonts);
            return;
        }
    }
}

fn save_png(img: &egui::ColorImage, path: &Path) -> std::io::Result<()> {
    let file = std::io::BufWriter::new(std::fs::File::create(path)?);
    let mut enc = png::Encoder::new(file, img.size[0] as u32, img.size[1] as u32);
    enc.set_color(png::ColorType::Rgba);
    enc.set_depth(png::BitDepth::Eight);
    let mut w = enc.write_header().map_err(std::io::Error::other)?;
    let data: Vec<u8> = img.pixels.iter().flat_map(|c| c.to_array()).collect();
    w.write_image_data(&data).map_err(std::io::Error::other)
}

struct Shot {
    file: PathBuf,
    wait: u32,
    done: Arc<AtomicBool>,
}

pub struct App {
    session: Session,
    lang: Lang,
    theme: Theme,
    visuals_for: Option<ThemeId>,
    show_machine: bool,
    mdi: String,
    path: String,
    path_toolpath: Vec<Segment>,
    frame: u32,
    start_at: Option<u32>,
    shot: Option<Shot>,
}

impl App {
    pub fn new(o: &Options) -> App {
        let mut app = App {
            session: Session::new(Box::new(SimController::new())),
            lang: o.lang.unwrap_or(Lang::En),
            theme: Theme::for_id(o.theme.unwrap_or(ThemeId::Forge)),
            visuals_for: None,
            show_machine: false,
            mdi: String::new(),
            path: String::new(),
            path_toolpath: Vec::new(),
            frame: 0,
            start_at: o.start.then_some(4),
            shot: o.shot.clone().map(|file| Shot {
                file,
                wait: o.wait,
                done: Arc::new(AtomicBool::new(false)),
            }),
        };
        if let Some(f) = &o.file {
            app.open(f);
        }
        app
    }

    fn t(&self, text: Text) -> &'static str {
        tr(self.lang, text)
    }

    /// Takes a G-code file as the program.
    pub fn open(&mut self, path: &Path) {
        match std::fs::read(path) {
            Ok(bytes) => {
                let text: String = bytes.iter().map(|&b| b as char).collect();
                self.session.load(&text);
                self.path_toolpath = gcode::toolpath(&self.session.program);
                self.path = path.display().to_string();
            }
            Err(e) => self.session.log.push(format!("{}: {e}", path.display())),
        }
    }

    fn state_text(&self) -> &'static str {
        if self.session.running && !self.session.paused {
            return self.t(Text::McStRun);
        }
        match self.session.status.state {
            State::Idle => self.t(Text::McStIdle),
            State::Run | State::Jog => self.t(Text::McStRun),
            State::Hold => self.t(Text::McStHold),
            State::Home => self.t(Text::McStHome),
            State::Alarm => self.t(Text::McStAlarm),
            _ => self.t(Text::McStUnknown),
        }
    }

    fn top(&mut self, ui: &mut egui::Ui) {
        let accent = col(self.theme.accent);
        egui::Panel::top("top").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading(RichText::new("EH3D").color(col(self.theme.text)));
                ui.heading(RichText::new("MOTIONCNC").color(accent));
                ui.separator();
                ui.label(self.t(Text::McController));
                egui::ComboBox::from_id_salt("controller")
                    .selected_text(self.t(Text::McSimulator))
                    .show_ui(ui, |ui| {
                        let _ = ui.selectable_label(true, self.t(Text::McSimulator));
                        for f in Family::ALL {
                            if !f.is_driven() {
                                continue;
                            }
                            ui.add_enabled(
                                false,
                                egui::Button::selectable(false, format!("{} (serial)", f.name())),
                            );
                        }
                        for f in Family::ALL {
                            if f.is_driven() {
                                continue;
                            }
                            ui.add_enabled(
                                false,
                                egui::Button::selectable(
                                    false,
                                    format!("{} - {}", f.name(), self.t(Text::McNotDriven)),
                                ),
                            );
                        }
                    });
                ui.separator();
                let mut lang = self.lang;
                egui::ComboBox::from_id_salt("lang")
                    .selected_text(lang.code().to_uppercase())
                    .width(60.0)
                    .show_ui(ui, |ui| {
                        for l in Lang::ALL {
                            ui.selectable_value(&mut lang, l, l.code().to_uppercase());
                        }
                    });
                self.lang = lang;
                let mut id = self.theme.id;
                egui::ComboBox::from_id_salt("theme")
                    .selected_text(id.key())
                    .show_ui(ui, |ui| {
                        for i in ThemeId::ALL {
                            ui.selectable_value(&mut id, i, i.key());
                        }
                    });
                if id != self.theme.id {
                    self.theme = Theme::for_id(id);
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let estop = egui::Button::new(
                        RichText::new(self.t(Text::EmergencyStop).to_uppercase())
                            .strong()
                            .color(Color32::WHITE),
                    )
                    .fill(Color32::from_rgb(200, 30, 40))
                    .min_size(Vec2::new(150.0, 34.0));
                    if ui.add(estop).clicked() {
                        self.session.estop();
                    }
                    let alarm = self.session.alarm();
                    let color = if alarm {
                        col(self.theme.danger)
                    } else {
                        col(self.theme.ok)
                    };
                    ui.label(
                        RichText::new(format!("{}: {}", self.t(Text::McState), self.state_text()))
                            .strong()
                            .color(color),
                    );
                });
            });
        });
    }

    fn left(&mut self, ui: &mut egui::Ui) {
        egui::Panel::left("left").exact_size(350.0).show(ui, |ui| {
            ui.add_space(4.0);
            ui.label(
                RichText::new(self.t(Text::McPosition).to_uppercase())
                    .color(col(self.theme.accent)),
            );
            let (work, machine) = (self.t(Text::McWork), self.t(Text::McMachine));
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.show_machine, false, work);
                ui.selectable_value(&mut self.show_machine, true, machine);
            });
            let st = self.session.status.clone();
            let values = if self.show_machine {
                st.machine_mm
            } else {
                st.work_mm
            };
            for (name, v) in ["X", "Y", "Z"].iter().zip(values) {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(*name).size(26.0).color(col(self.theme.dim)));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            RichText::new(format!("{v:9.3}"))
                                .monospace()
                                .size(30.0)
                                .color(col(self.theme.accent)),
                        );
                    });
                });
            }
            ui.horizontal(|ui| {
                ui.label(format!(
                    "{}: {:.0}",
                    self.t(Text::McSpindle),
                    st.spindle_rpm
                ));
                ui.label(format!(
                    "{}: {:.0}",
                    self.t(Text::McFeedRate),
                    st.feed_mm_min
                ));
            });
            ui.separator();
            ui.label(
                RichText::new(self.t(Text::McJog).to_uppercase()).color(col(self.theme.accent)),
            );
            ui.label(self.t(Text::McStep));
            ui.horizontal_wrapped(|ui| {
                for s in STEPS {
                    if ui
                        .selectable_label(self.session.step_mm == s, format!("{s}"))
                        .clicked()
                    {
                        self.session.step_mm = s;
                    }
                }
            });
            ui.horizontal(|ui| {
                ui.label(self.t(Text::McFeedRate));
                ui.add(
                    egui::DragValue::new(&mut self.session.feed_mm_min)
                        .range(1.0..=20000.0)
                        .speed(10.0),
                );
            });
            ui.add_space(6.0);
            let size = Vec2::new(64.0, 44.0);
            let mut pad = |ui: &mut egui::Ui, label: &str, axis: usize, dir: f64| {
                if ui.add(egui::Button::new(label).min_size(size)).clicked() {
                    self.session.jog(axis, dir);
                }
            };
            egui::Grid::new("pad").spacing([8.0, 8.0]).show(ui, |ui| {
                ui.label("");
                pad(ui, "Y+", 1, 1.0);
                ui.label("");
                ui.label("      ");
                pad(ui, "Z+", 2, 1.0);
                ui.end_row();
                pad(ui, "X-", 0, -1.0);
                ui.label("");
                pad(ui, "X+", 0, 1.0);
                ui.label("      ");
                ui.label("");
                ui.end_row();
                ui.label("");
                pad(ui, "Y-", 1, -1.0);
                ui.label("");
                ui.label("      ");
                pad(ui, "Z-", 2, -1.0);
                ui.end_row();
            });
            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                if ui.button(self.t(Text::McHome)).clicked() {
                    self.session.home();
                }
                if ui.button(self.t(Text::McZero)).clicked() {
                    self.session.zero();
                }
                if ui.button(self.t(Text::McUnlock)).clicked() {
                    self.session.unlock();
                }
            });
        });
    }

    fn right(&mut self, ui: &mut egui::Ui) {
        egui::Panel::right("right")
            .exact_size(330.0)
            .show(ui, |ui| {
                ui.add_space(4.0);
                ui.label(
                    RichText::new(self.t(Text::McProgram).to_uppercase())
                        .color(col(self.theme.accent)),
                );
                ui.horizontal(|ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.path)
                            .desired_width(190.0)
                            .hint_text("job.nc"),
                    );
                    if ui.button(self.t(Text::McOpenGcode)).clicked() && !self.path.is_empty() {
                        let p = PathBuf::from(self.path.clone());
                        self.open(&p);
                    }
                });
                ui.horizontal(|ui| {
                    let running = self.session.running && !self.session.paused;
                    if ui
                        .add_enabled(!running, egui::Button::new(self.t(Text::Start)))
                        .clicked()
                    {
                        self.session.start();
                    }
                    if ui
                        .add_enabled(running, egui::Button::new(self.t(Text::Pause)))
                        .clicked()
                    {
                        self.session.pause();
                    }
                    if ui
                        .add_enabled(
                            self.session.running || self.session.paused,
                            egui::Button::new(self.t(Text::Stop)),
                        )
                        .clicked()
                    {
                        self.session.stop();
                    }
                });
                ui.label(format!(
                    "{} {} / {}",
                    self.t(Text::McLine),
                    self.session.next.min(self.session.program.len()),
                    self.session.program.len()
                ));
                ui.separator();
                let next = self.session.next;
                let running = self.session.running;
                egui::ScrollArea::vertical()
                    .id_salt("program")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for (n, line) in self.session.program.iter().enumerate() {
                            let current = running && n == next;
                            let text = RichText::new(format!("{:>4}  {line}", n + 1)).monospace();
                            let text = if n < next {
                                text.color(col(self.theme.ok))
                            } else if current {
                                text.color(col(self.theme.accent)).strong()
                            } else {
                                text.color(col(self.theme.dim))
                            };
                            let r = ui.label(text);
                            if current {
                                r.scroll_to_me(None);
                            }
                        }
                    });
            });
    }

    fn bottom(&mut self, ui: &mut egui::Ui) {
        egui::Panel::bottom("bottom")
            .exact_size(170.0)
            .show(ui, |ui| {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label(self.t(Text::McMdi));
                    let r = ui.add(
                        egui::TextEdit::singleline(&mut self.mdi)
                            .desired_width(420.0)
                            .font(egui::TextStyle::Monospace)
                            .hint_text("G0 X10 Y10"),
                    );
                    let enter = r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                    if ui.button(self.t(Text::McSend)).clicked() || enter {
                        let line = std::mem::take(&mut self.mdi);
                        self.session.mdi(&line);
                        r.request_focus();
                    }
                });
                ui.separator();
                ui.label(
                    RichText::new(self.t(Text::McLog).to_uppercase()).color(col(self.theme.dim)),
                );
                egui::ScrollArea::vertical()
                    .id_salt("log")
                    .stick_to_bottom(true)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for l in &self.session.log {
                            let c = if l.contains(':') && !l.starts_with('>') {
                                col(self.theme.warn)
                            } else {
                                col(self.theme.dim)
                            };
                            ui.label(RichText::new(l).monospace().color(c));
                        }
                    });
            });
    }

    /// The path of the program on the table, what is done, and where the tool is.
    fn center(&mut self, ui: &mut egui::Ui) {
        let t = self.theme.clone();
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label(RichText::new(self.t(Text::McToolpath).to_uppercase()).color(col(t.accent)));
            let (rect, _) = ui.allocate_exact_size(ui.available_size(), egui::Sense::hover());
            let p = ui.painter_at(rect);
            p.rect_filled(rect, 8.0, col(t.panel));
            let (tx0, tx1) = (0.0f64, 500.0f64);
            let (ty0, ty1) = (0.0f64, 300.0f64);
            // the segments decide how much of the table is shown
            let (mut maxx, mut maxy) = (tx1, ty1);
            for s in &self.path_toolpath {
                for pt in [s.from, s.to] {
                    maxx = maxx.max(pt.0);
                    maxy = maxy.max(pt.1);
                }
            }
            let inner = rect.shrink(18.0);
            let k =
                ((inner.width() as f64) / (maxx - tx0)).min((inner.height() as f64) / (maxy - ty0));
            let at = |x: f64, y: f64| {
                Pos2::new(
                    inner.left() + ((x - tx0) * k) as f32,
                    inner.bottom() - ((y - ty0) * k) as f32,
                )
            };
            // the table and its grid every 50 mm
            let table = Rect::from_two_pos(at(tx0, ty0), at(tx1, ty1));
            p.rect_filled(table, 2.0, col(t.table));
            let mut x = 0.0;
            while x <= tx1 {
                p.line_segment(
                    [at(x, ty0), at(x, ty1)],
                    Stroke::new(1.0, alpha(col(t.grid), 90)),
                );
                x += 50.0;
            }
            let mut y = 0.0;
            while y <= ty1 {
                p.line_segment(
                    [at(tx0, y), at(tx1, y)],
                    Stroke::new(1.0, alpha(col(t.grid), 90)),
                );
                y += 50.0;
            }
            p.rect_stroke(
                table,
                2.0,
                Stroke::new(1.5, col(t.line)),
                StrokeKind::Inside,
            );
            let next = self.session.next;
            for s in &self.path_toolpath {
                let done = s.line < next;
                let color = match (s.rapid, done) {
                    (true, _) => alpha(col(t.dim), 140),
                    (false, true) => col(t.ok),
                    (false, false) => col(t.accent),
                };
                p.line_segment(
                    [at(s.from.0, s.from.1), at(s.to.0, s.to.1)],
                    Stroke::new(if s.rapid { 1.0 } else { 2.0 }, color),
                );
            }
            // the tool, where the machine is
            let m = self.session.status.machine_mm;
            let c = at(m[0], m[1]);
            p.circle_stroke(c, 9.0, Stroke::new(2.0, col(t.accent2)));
            p.line_segment(
                [c - Vec2::new(14.0, 0.0), c + Vec2::new(14.0, 0.0)],
                Stroke::new(1.0, col(t.accent2)),
            );
            p.line_segment(
                [c - Vec2::new(0.0, 14.0), c + Vec2::new(0.0, 14.0)],
                Stroke::new(1.0, col(t.accent2)),
            );
            p.text(
                rect.right_bottom() - Vec2::new(10.0, 8.0),
                Align2::RIGHT_BOTTOM,
                format!("{:.0} x {:.0} mm", maxx, maxy),
                egui::FontId::proportional(12.0),
                col(t.dim),
            );
        });
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.frame += 1;
        if self.visuals_for != Some(self.theme.id) {
            apply_visuals(&ctx, &self.theme);
            self.visuals_for = Some(self.theme.id);
        }
        if self.start_at == Some(self.frame) {
            self.session.start();
        }
        let dropped: Option<PathBuf> = ctx.input(|i| {
            i.raw
                .dropped_files
                .first()
                .map(|f| f.path().to_path_buf())
                .filter(|p| !p.as_os_str().is_empty())
        });
        if let Some(p) = dropped {
            self.open(&p);
        }
        if self.session.running && !self.session.paused {
            self.session.pump(1);
            ctx.request_repaint_after(std::time::Duration::from_millis(60));
        }
        self.top(ui);
        self.bottom(ui);
        self.left(ui);
        self.right(ui);
        self.center(ui);
        let shot = self
            .shot
            .as_ref()
            .map(|s| (s.file.clone(), s.wait, s.done.clone()));
        if let Some((file, wait, done)) = shot {
            ctx.request_repaint();
            if self.frame == 3 + wait {
                let (f2, d2, c2) = (file.clone(), done.clone(), ctx.clone());
                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(
                    egui::ScreenshotCallback::new(move |img| {
                        if let Err(e) = save_png(&img, &f2) {
                            eprintln!("cannot save {}: {e}", f2.display());
                        }
                        d2.store(true, Ordering::SeqCst);
                        c2.request_repaint();
                    }),
                ));
            }
            if done.load(Ordering::SeqCst) {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }
    }
}

pub fn run(o: &Options) -> eframe::Result {
    let app = App::new(o);
    let size = if o.shot.is_some() {
        Vec2::new(1400.0, 800.0)
    } else {
        Vec2::new(1300.0, 780.0)
    };
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(size)
            .with_min_inner_size([1000.0, 600.0])
            .with_title("EH3D-MOTIONCNC"),
        ..Default::default()
    };
    eframe::run_native(
        "EH3D-MOTIONCNC",
        options,
        Box::new(move |cc| {
            install_fonts(&cc.egui_ctx);
            Ok(Box::new(app))
        }),
    )
}
