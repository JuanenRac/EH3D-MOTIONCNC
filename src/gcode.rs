//! The path of a G-code program, for drawing it before it runs: the straight moves (`G0` rapid and `G1` cutting), absolute or relative,
//! in millimetres or inches. Arcs and the other codes are not drawn (a controller that cannot take them refuses them when they run).

/// A point of the machine: X, Y and Z in mm.
pub type Point = (f64, f64, f64);

/// One straight move of the program.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment {
    pub from: Point,
    pub to: Point,
    /// A rapid move (`G0`), which is not cutting.
    pub rapid: bool,
    /// The line of the program it comes from (from 0).
    pub line: usize,
}

/// The straight moves of the program, in order, starting at the origin.
pub fn toolpath(lines: &[String]) -> Vec<Segment> {
    let mut out = Vec::new();
    let (mut pos, mut absolute, mut scale, mut rapid) = ((0.0, 0.0, 0.0), true, 1.0, true);
    for (n, raw) in lines.iter().enumerate() {
        let text = raw.split(';').next().unwrap_or("");
        let mut target = [None; 3];
        let mut word = String::new();
        let mut words: Vec<String> = Vec::new();
        let mut in_comment = false;
        for c in text.chars() {
            match c {
                '(' => in_comment = true,
                ')' => in_comment = false,
                c if in_comment || c.is_whitespace() => {}
                c if c.is_ascii_alphabetic() => {
                    if !word.is_empty() {
                        words.push(std::mem::take(&mut word));
                    }
                    word.push(c.to_ascii_uppercase());
                }
                c => word.push(c),
            }
        }
        if !word.is_empty() {
            words.push(word);
        }
        for w in &words {
            let (letter, num) = w.split_at(1);
            let Ok(v) = num.parse::<f64>() else { continue };
            match letter {
                "G" => match v as i32 {
                    0 => rapid = true,
                    1 => rapid = false,
                    90 => absolute = true,
                    91 => absolute = false,
                    20 => scale = 25.4,
                    21 => scale = 1.0,
                    _ => {}
                },
                "X" => target[0] = Some(v * scale),
                "Y" => target[1] = Some(v * scale),
                "Z" => target[2] = Some(v * scale),
                _ => {}
            }
        }
        if target.iter().any(Option::is_some) {
            let pick = |t: Option<f64>, now: f64| match (t, absolute) {
                (Some(v), true) => v,
                (Some(v), false) => now + v,
                (None, _) => now,
            };
            let to = (
                pick(target[0], pos.0),
                pick(target[1], pos.1),
                pick(target[2], pos.2),
            );
            if to != pos {
                out.push(Segment {
                    from: pos,
                    to,
                    rapid,
                    line: n,
                });
            }
            pos = to;
        }
    }
    out
}

/// The words of a line of G-code (a letter and its number), without the comments; what is not a word is left out.
pub fn words(line: &str) -> Vec<(char, f64)> {
    let text = line.split(';').next().unwrap_or("");
    let mut out = Vec::new();
    let mut in_comment = false;
    let mut cur: Option<(char, String)> = None;
    let flush = |cur: &mut Option<(char, String)>, out: &mut Vec<(char, f64)>| {
        if let Some((l, n)) = cur.take() {
            if let Ok(v) = n.parse::<f64>() {
                out.push((l, v));
            }
        }
    };
    for c in text.chars() {
        match c {
            '(' => in_comment = true,
            ')' => in_comment = false,
            c if in_comment || c.is_whitespace() => {}
            c if c.is_ascii_alphabetic() => {
                flush(&mut cur, &mut out);
                cur = Some((c.to_ascii_uppercase(), String::new()));
            }
            c => {
                if let Some((_, n)) = cur.as_mut() {
                    n.push(c);
                }
            }
        }
    }
    flush(&mut cur, &mut out);
    out
}

/// The codes that stay on until another one of their group changes them, as the panel of a machine-control program shows them.
#[derive(Debug, Clone, PartialEq)]
pub struct Modal {
    /// `G0` or `G1`.
    pub motion: u8,
    pub inches: bool,
    pub absolute: bool,
    /// `M3`, `M4` or `M5`.
    pub spindle: u8,
    /// `M7`, `M8` or `M9`.
    pub coolant: u8,
}

impl Default for Modal {
    fn default() -> Self {
        Modal {
            motion: 0,
            inches: false,
            absolute: true,
            spindle: 5,
            coolant: 9,
        }
    }
}

impl Modal {
    /// Takes a line that the controller accepted.
    pub fn apply(&mut self, line: &str) {
        for (l, v) in words(line) {
            match (l, v as i32) {
                ('G', 0) => self.motion = 0,
                ('G', 1) => self.motion = 1,
                ('G', 20) => self.inches = true,
                ('G', 21) => self.inches = false,
                ('G', 90) => self.absolute = true,
                ('G', 91) => self.absolute = false,
                ('M', n @ (3..=5)) => self.spindle = n as u8,
                ('M', n @ (7..=9)) => self.coolant = n as u8,
                ('M', 2 | 30) => {
                    self.spindle = 5;
                    self.coolant = 9;
                }
                _ => {}
            }
        }
    }

    /// The G-codes in force, in the order of the panel.
    pub fn g_codes(&self) -> Vec<String> {
        let g = |n: &str| n.to_string();
        vec![
            format!("G{}", self.motion),
            g("G17"),
            g(if self.inches { "G20" } else { "G21" }),
            g("G40"),
            g("G49"),
            g("G54"),
            g("G64"),
            g("G80"),
            g(if self.absolute { "G90" } else { "G91" }),
            g("G94"),
            g("G97"),
            g("G99"),
        ]
    }

    /// The M-codes in force.
    pub fn m_codes(&self) -> Vec<String> {
        vec![
            format!("M{}", self.spindle),
            format!("M{}", self.coolant),
            "M48".to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines(s: &str) -> Vec<String> {
        s.lines().map(str::to_string).collect()
    }

    #[test]
    fn moves_are_drawn_in_order_with_rapids_told_from_cuts() {
        let p = toolpath(&lines(
            "G0 X10 Y10\nG1 Z-2 F100\nG1 X20\n(a comment) M3 S1000\nG0 Z5",
        ));
        assert_eq!(p.len(), 4);
        assert!(p[0].rapid && !p[1].rapid && !p[2].rapid && p[3].rapid);
        assert_eq!(p[0].to, (10.0, 10.0, 0.0));
        assert_eq!(p[2].from, (10.0, 10.0, -2.0));
        assert_eq!(p[2].line, 2);
    }

    #[test]
    fn relative_moves_and_inches_are_followed() {
        let p = toolpath(&lines("G91\nG1 X5\nG1 X5\nG90 G20\nG0 X1"));
        assert_eq!(p[1].to.0, 10.0);
        assert!((p[2].to.0 - 25.4).abs() < 1e-9);
    }

    #[test]
    fn the_codes_in_force_follow_the_lines_taken() {
        let mut m = Modal::default();
        assert!(
            m.g_codes().contains(&"G90".to_string()) && m.g_codes().contains(&"G21".to_string())
        );
        assert_eq!(m.m_codes()[..2], ["M5", "M9"]);
        m.apply("G91 G20 G1 X1 F10");
        m.apply("M3 S1000 (on) M8");
        let g = m.g_codes();
        assert_eq!(
            (g[0].as_str(), g[2].as_str(), g[8].as_str()),
            ("G1", "G20", "G91")
        );
        assert_eq!(m.m_codes()[..2], ["M3", "M8"]);
        m.apply("M30");
        assert_eq!(m.m_codes()[..2], ["M5", "M9"]);
    }

    #[test]
    fn the_words_of_a_line_leave_the_comments_out() {
        assert_eq!(
            words("g1 x10.5 (skip x9) Y-2 ; z3"),
            vec![('G', 1.0), ('X', 10.5), ('Y', -2.0)]
        );
        assert!(words("; nothing").is_empty());
    }

    #[test]
    fn lines_without_a_move_draw_nothing() {
        assert!(toolpath(&lines("M3 S1000\n; nothing\nG21\nM5")).is_empty());
        // a move to where it already is is not a segment
        assert_eq!(toolpath(&lines("G0 X0 Y0")).len(), 0);
    }
}
