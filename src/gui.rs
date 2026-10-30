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
use crate::settings::{settings_path, Settings};

mod dialogs;
use crate::gcode::{self, Segment};
use crate::session::{Session, STEPS};
use dialogs::{lang_name, Win};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PosTab {
    Work,
    Machine,
    Operator,
    GCode,
}

/// The little pictures of the buttons and the rows.
#[derive(Clone, Copy)]
enum Icon {
    Cross,
    ToZero,
    Home,
    Gauge,
    Spindle,
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
    pos_tab: PosTab,
    info_tab: u8,
    settings: Settings,
    settings_path: PathBuf,
    win: Option<Win>,
    fit_program: bool,
    quit: bool,
    dialog_error: Option<&'static str>,
    mdi: String,
    path: String,
    path_toolpath: Vec<Segment>,
    frame: u32,
    start_at: Option<u32>,
    shot: Option<Shot>,
}

impl App {
    pub fn new(o: &Options) -> App {
        App::with_controller(o, Box::new(SimController::new()))
    }

    /// The window over the controller that is given (the simulator, or the one the options asked for).
    pub fn with_controller(o: &Options, controller: Box<dyn eh3d_controller::Controller>) -> App {
        let path = settings_path();
        let mut settings = Settings::load(&path);
        if let Some(l) = o.lang {
            settings.lang = l;
        }
        if let Some(th) = o.theme {
            settings.theme = th;
        }
        let mut session = Session::new(controller);
        session.step_mm = settings.step_mm;
        session.feed_mm_min = settings.feed_mm_min;
        session.set_travel(settings.travel);
        let mut app = App {
            session,
            lang: settings.lang,
            theme: Theme::for_id(settings.theme),
            visuals_for: None,
            pos_tab: PosTab::Work,
            info_tab: 0,
            settings,
            settings_path: path,
            win: None,
            fit_program: false,
            quit: false,
            dialog_error: None,
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
        if let Some(w) = &o.window {
            app.open_window(w);
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
                self.remember(path);
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
                    .selected_text(lang_name(lang))
                    .show_ui(ui, |ui| {
                        for l in Lang::ALL {
                            ui.selectable_value(&mut lang, l, lang_name(l));
                        }
                    });
                let mut id = self.theme.id;
                egui::ComboBox::from_id_salt("theme")
                    .selected_text(self.t(id.text()))
                    .show_ui(ui, |ui| {
                        for i in ThemeId::ALL {
                            ui.selectable_value(&mut id, i, self.t(i.text()));
                        }
                    });
                if lang != self.lang || id != self.theme.id {
                    self.settings.lang = lang;
                    self.settings.theme = id;
                    self.apply_settings();
                    self.save_settings();
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

    /// The left panel, made like the one of PlanetCNC: a column of buttons for the zeros and the stored positions, the position with its
    /// tabs (work, machine, operator, GCode), the speed and the spindle, the codes in force or the inputs and outputs, and the jog.
    fn left(&mut self, ui: &mut egui::Ui) {
        if self.settings.show_buttons {
            egui::Panel::left("icons").exact_size(74.0).show(ui, |ui| {
                ui.add_space(4.0);
                self.icon_column(ui);
            });
        }
        egui::Panel::left("left").exact_size(360.0).show(ui, |ui| {
            ui.add_space(4.0);
            egui::ScrollArea::vertical()
                .id_salt("leftscroll")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    self.position_block(ui);
                    ui.add_space(4.0);
                    self.speed_rows(ui);
                    ui.add_space(4.0);
                    self.info_block(ui);
                    ui.separator();
                    self.jog_block(ui);
                });
        });
    }

    /// One square button of the column: an icon drawn with lines, a short caption under it and the explanation when the pointer rests on it.
    fn icon_button(&self, ui: &mut egui::Ui, icon: Icon, caption: &str, tip: &str) -> bool {
        let (rect, resp) = ui.allocate_exact_size(Vec2::new(56.0, 52.0), egui::Sense::click());
        let t = &self.theme;
        let p = ui.painter_at(rect);
        let hot = resp.hovered();
        p.rect_filled(
            rect,
            6.0,
            if resp.is_pointer_button_down_on() {
                alpha(col(t.accent), 110)
            } else if hot {
                alpha(col(t.accent), 50)
            } else {
                col(t.panel_raised)
            },
        );
        p.rect_stroke(
            rect,
            6.0,
            Stroke::new(1.0, alpha(col(t.accent), if hot { 200 } else { 90 })),
            StrokeKind::Inside,
        );
        let c = rect.center() - Vec2::new(0.0, 8.0);
        let line = Stroke::new(1.8, col(t.text));
        match icon {
            Icon::Cross => {
                p.circle_stroke(c, 9.0, line);
                p.line_segment([c - Vec2::new(14.0, 0.0), c + Vec2::new(14.0, 0.0)], line);
                p.line_segment([c - Vec2::new(0.0, 14.0), c + Vec2::new(0.0, 14.0)], line);
            }
            Icon::ToZero => {
                p.line_segment([c - Vec2::new(14.0, 0.0), c + Vec2::new(8.0, 0.0)], line);
                p.line_segment([c + Vec2::new(8.0, 0.0), c + Vec2::new(2.0, -6.0)], line);
                p.line_segment([c + Vec2::new(8.0, 0.0), c + Vec2::new(2.0, 6.0)], line);
                p.line_segment([c + Vec2::new(13.0, -9.0), c + Vec2::new(13.0, 9.0)], line);
            }
            Icon::Home => {
                let top = c - Vec2::new(0.0, 12.0);
                p.line_segment([top, c + Vec2::new(-13.0, -1.0)], line);
                p.line_segment([top, c + Vec2::new(13.0, -1.0)], line);
                p.rect_stroke(
                    Rect::from_min_max(c + Vec2::new(-9.0, -1.0), c + Vec2::new(9.0, 11.0)),
                    0.0,
                    line,
                    StrokeKind::Inside,
                );
            }
            Icon::Gauge | Icon::Spindle => {}
        }
        p.text(
            rect.center_bottom() - Vec2::new(0.0, 3.0),
            Align2::CENTER_BOTTOM,
            caption,
            egui::FontId::proportional(11.0),
            col(t.dim),
        );
        resp.on_hover_text(tip).clicked()
    }

    fn icon_column(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 6.0;
            if self.icon_button(ui, Icon::Cross, "XYZ", self.t(Text::McZeroXyz)) {
                self.session.zero_axes(&[0, 1, 2]);
            }
            if self.icon_button(ui, Icon::Cross, "XY", self.t(Text::McZeroXy)) {
                self.session.zero_axes(&[0, 1]);
            }
            if self.icon_button(ui, Icon::Cross, "G28", self.t(Text::McGoG28)) {
                self.session.go_g28();
            }
            if self.icon_button(ui, Icon::Cross, "G30", self.t(Text::McGoG30)) {
                self.session.go_g30();
            }
            ui.separator();
            if self.icon_button(ui, Icon::ToZero, "XY", self.t(Text::McGoZeroXy)) {
                self.session.go_zero_xy();
            }
            if self.icon_button(ui, Icon::ToZero, "Z", self.t(Text::McGoZeroZ)) {
                self.session.go_zero_z();
            }
            ui.separator();
            if self.icon_button(ui, Icon::Home, "", self.t(Text::McHome)) {
                self.session.home();
            }
        });
    }

    /// The position: the tabs and the four axes, big. The letter of an axis sets its zero.
    fn position_block(&mut self, ui: &mut egui::Ui) {
        let (work, machine, operator, gcode) = (
            self.t(Text::McWork),
            self.t(Text::McMachine),
            self.t(Text::McOperator),
            self.t(Text::McGcodeTab),
        );
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.pos_tab, PosTab::Work, work);
            ui.selectable_value(&mut self.pos_tab, PosTab::Machine, machine);
            ui.selectable_value(&mut self.pos_tab, PosTab::Operator, operator);
            ui.selectable_value(&mut self.pos_tab, PosTab::GCode, gcode);
        });
        let st = self.session.status.clone();
        let mut values = match self.pos_tab {
            PosTab::Work | PosTab::GCode => st.work_mm,
            PosTab::Machine => st.machine_mm,
            PosTab::Operator => self.session.operator_mm(),
        };
        // the machine and the program go on in mm; only what is shown changes
        let (decimals, unit) = if self.settings.inches {
            for v in &mut values {
                *v /= 25.4;
            }
            (4, "in")
        } else {
            (3, "mm")
        };
        let tip = self.t(Text::McZeroAxis);
        for (i, name) in ["X", "Y", "Z", "A"].iter().enumerate() {
            egui::Frame::new()
                .fill(col(self.theme.panel_raised))
                .corner_radius(6.0)
                .inner_margin(egui::Margin::symmetric(8, 3))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal(|ui| {
                        let letter = egui::Button::new(
                            RichText::new(*name).size(26.0).color(col(self.theme.dim)),
                        )
                        .frame(false)
                        .min_size(Vec2::new(30.0, 34.0));
                        if ui.add(letter).on_hover_text(tip).clicked() {
                            match self.pos_tab {
                                PosTab::Machine => {}
                                PosTab::Operator => self.session.zero_operator(&[i]),
                                _ => self.session.zero_axes(&[i]),
                            }
                        }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                RichText::new(format!("{:9.1$} {unit}", values[i], decimals))
                                    .monospace()
                                    .size(30.0)
                                    .color(col(self.theme.accent)),
                            );
                        });
                    });
                });
        }
    }

    /// The speed of the move and the speed of the spindle, each with its little picture.
    fn speed_rows(&mut self, ui: &mut egui::Ui) {
        let st = self.session.status.clone();
        let rows = [
            (Icon::Gauge, self.t(Text::McSpeed), st.feed_mm_min),
            (Icon::Spindle, self.t(Text::McSpindle), st.spindle_rpm),
        ];
        for (icon, tip, v) in rows {
            egui::Frame::new()
                .fill(col(self.theme.panel_raised))
                .corner_radius(6.0)
                .inner_margin(egui::Margin::symmetric(8, 3))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal(|ui| {
                        let (rect, resp) =
                            ui.allocate_exact_size(Vec2::new(34.0, 30.0), egui::Sense::hover());
                        resp.on_hover_text(tip);
                        let p = ui.painter_at(rect);
                        let line = Stroke::new(1.8, col(self.theme.dim));
                        let c = rect.center();
                        match icon {
                            Icon::Gauge => {
                                p.circle_stroke(c + Vec2::new(0.0, 2.0), 12.0, line);
                                p.line_segment(
                                    [c + Vec2::new(0.0, 2.0), c + Vec2::new(7.0, -6.0)],
                                    line,
                                );
                            }
                            _ => {
                                p.rect_stroke(
                                    Rect::from_center_size(
                                        c - Vec2::new(0.0, 5.0),
                                        Vec2::new(14.0, 10.0),
                                    ),
                                    2.0,
                                    line,
                                    StrokeKind::Inside,
                                );
                                p.line_segment([c, c + Vec2::new(0.0, 11.0)], line);
                            }
                        }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                RichText::new(format!("{v:.2}"))
                                    .monospace()
                                    .size(24.0)
                                    .color(col(self.theme.text)),
                            );
                        });
                    });
                });
        }
    }

    /// The codes in force (G and M), or the inputs and the outputs.
    fn info_block(&mut self, ui: &mut egui::Ui) {
        let (gc, io) = (self.t(Text::McGcodeTab), self.t(Text::McIoTab));
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.info_tab, 0, gc);
            ui.selectable_value(&mut self.info_tab, 1, io);
        });
        let dim = col(self.theme.dim);
        if self.info_tab == 0 {
            ui.label(RichText::new(self.t(Text::McActiveG)).color(dim));
            ui.label(
                RichText::new(self.session.modal.g_codes().join("  "))
                    .monospace()
                    .strong()
                    .size(15.0),
            );
            ui.label(RichText::new(self.t(Text::McActiveM)).color(dim));
            ui.label(
                RichText::new(self.session.modal.m_codes().join("  "))
                    .monospace()
                    .strong()
                    .size(15.0),
            );
        } else {
            let st = self.session.status.clone();
            let pin = |c: char| st.pins.contains(c);
            let led = |ui: &mut egui::Ui, on: bool, label: &str, color: Color32| {
                ui.horizontal(|ui| {
                    let (rect, _) =
                        ui.allocate_exact_size(Vec2::new(16.0, 16.0), egui::Sense::hover());
                    let c = rect.center();
                    ui.painter()
                        .circle_filled(c, 6.0, if on { color } else { alpha(color, 40) });
                    ui.painter()
                        .circle_stroke(c, 6.0, Stroke::new(1.0, alpha(color, 160)));
                    ui.label(label);
                });
            };
            let (ok, danger) = (col(self.theme.ok), col(self.theme.danger));
            ui.label(RichText::new(self.t(Text::McInputs)).color(dim));
            ui.horizontal_wrapped(|ui| {
                led(ui, pin('X'), "X", danger);
                led(ui, pin('Y'), "Y", danger);
                led(ui, pin('Z'), "Z", danger);
                led(ui, pin('P'), "Probe", danger);
                led(ui, pin('D'), "Door", danger);
            });
            ui.label(RichText::new(self.t(Text::McOutputs)).color(dim));
            ui.horizontal_wrapped(|ui| {
                led(ui, st.spindle_rpm > 0.0, self.t(Text::McSpindle), ok);
                led(ui, self.session.modal.coolant != 9, "Coolant", ok);
            });
        }
    }

    /// The jog: the steps, the speed, the pad and the unlock.
    fn jog_block(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new(self.t(Text::McJog).to_uppercase()).color(col(self.theme.accent)));
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
            ui.label(self.t(Text::McJogSpeed));
            ui.add(
                egui::DragValue::new(&mut self.session.feed_mm_min)
                    .range(1.0..=20000.0)
                    .speed(10.0),
            );
            if ui.button(self.t(Text::McUnlock)).clicked() {
                self.session.unlock();
            }
        });
        let size = Vec2::new(52.0, 38.0);
        let mut pad = |ui: &mut egui::Ui, label: &str, axis: usize, dir: f64| {
            if ui.add(egui::Button::new(label).min_size(size)).clicked() {
                self.session.jog(axis, dir);
            }
        };
        egui::Grid::new("pad").spacing([6.0, 6.0]).show(ui, |ui| {
            ui.label("");
            pad(ui, "Y+", 1, 1.0);
            ui.label("");
            ui.label("");
            pad(ui, "Z+", 2, 1.0);
            pad(ui, "A+", 3, 1.0);
            ui.end_row();
            pad(ui, "X-", 0, -1.0);
            ui.label("");
            pad(ui, "X+", 0, 1.0);
            ui.label("");
            ui.label("");
            ui.label("");
            ui.end_row();
            ui.label("");
            pad(ui, "Y-", 1, -1.0);
            ui.label("");
            ui.label("");
            pad(ui, "Z-", 2, -1.0);
            pad(ui, "A-", 3, -1.0);
            ui.end_row();
        });
    }

    fn right(&mut self, ui: &mut egui::Ui) {
        if !self.settings.show_program {
            return;
        }
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
        let height = if self.settings.show_log { 120.0 } else { 44.0 };
        egui::Panel::bottom("bottom")
            .exact_size(height)
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
                if !self.settings.show_log {
                    return;
                }
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
            let ((tx0, tx1), (ty0, ty1)) = (
                (self.settings.travel[0].0, self.settings.travel[0].1),
                (self.settings.travel[1].0, self.settings.travel[1].1),
            );
            // the view: the table, or the table and the program when it goes beyond it, or only the program
            let (mut minx, mut miny, mut maxx, mut maxy) = (tx0, ty0, tx1, ty1);
            if self.fit_program && !self.path_toolpath.is_empty() {
                (minx, miny, maxx, maxy) = (f64::MAX, f64::MAX, f64::MIN, f64::MIN);
            }
            for s in &self.path_toolpath {
                for pt in [s.from, s.to] {
                    minx = minx.min(pt.0);
                    miny = miny.min(pt.1);
                    maxx = maxx.max(pt.0);
                    maxy = maxy.max(pt.1);
                }
            }
            let (w, h) = ((maxx - minx).max(1.0), (maxy - miny).max(1.0));
            let inner = rect.shrink(18.0);
            let k = ((inner.width() as f64) / w).min((inner.height() as f64) / h);
            let at = |x: f64, y: f64| {
                Pos2::new(
                    inner.left() + ((x - minx) * k) as f32,
                    inner.bottom() - ((y - miny) * k) as f32,
                )
            };
            // the table and its grid every 50 mm
            let table = Rect::from_two_pos(at(tx0, ty0), at(tx1, ty1));
            p.rect_filled(table, 2.0, col(t.table));
            let mut x = (tx0 / 50.0).ceil() * 50.0;
            while x <= tx1 {
                p.line_segment(
                    [at(x, ty0), at(x, ty1)],
                    Stroke::new(1.0, alpha(col(t.grid), 90)),
                );
                x += 50.0;
            }
            let mut y = (ty0 / 50.0).ceil() * 50.0;
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
                if s.rapid && !self.settings.show_rapids {
                    continue;
                }
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
                format!("{:.0} x {:.0} mm", w, h),
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
        // the window follows its size: from 720p to 1080p and on to the large and the wide screens everything grows or shrinks with it
        let native = ctx.content_rect().size() * ctx.zoom_factor();
        let zoom = (native.x / 1400.0).min(native.y / 800.0).clamp(0.75, 3.0);
        if (zoom - ctx.zoom_factor()).abs() > 0.01 {
            ctx.set_zoom_factor(zoom);
        }
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
        self.menu_bar(ui);
        self.top(ui);
        self.bottom(ui);
        self.left(ui);
        self.right(ui);
        self.center(ui);
        self.windows(&ctx);
        if self.quit {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
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
    if o.ports {
        print!("{}", crate::link::ports_text());
        return Ok(());
    }
    let controller = match crate::link::controller_for(o) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("motioncnc: {e}");
            std::process::exit(2);
        }
    };
    let app = App::with_controller(o, controller);
    let size = if o.shot.is_some() {
        Vec2::new(1400.0, 800.0)
    } else {
        Vec2::new(1300.0, 780.0)
    };
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(size)
            .with_min_inner_size([1000.0, 600.0])
            .with_maximized(o.shot.is_none())
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
