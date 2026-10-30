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
    fn lines_without_a_move_draw_nothing() {
        assert!(toolpath(&lines("M3 S1000\n; nothing\nG21\nM5")).is_empty());
        // a move to where it already is is not a segment
        assert_eq!(toolpath(&lines("G0 X0 Y0")).len(), 0);
    }
}
