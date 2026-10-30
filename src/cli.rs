//! The command line of the window.

use std::path::PathBuf;

use eh3d_view::texts::Lang;
use eh3d_view::theme::ThemeId;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Options {
    pub lang: Option<Lang>,
    pub theme: Option<ThemeId>,
    /// The G-code file to open (the first one given without an option).
    pub file: Option<PathBuf>,
    /// A picture of the window to save and leave (`--shot FILE`, after `--wait` frames).
    pub shot: Option<PathBuf>,
    pub wait: u32,
    /// Starts the program as soon as the window is open (`--start`): for the pictures of a program in progress.
    pub start: bool,
}

impl Options {
    pub fn parse<I: IntoIterator<Item = String>>(args: I) -> Result<Options, String> {
        let mut o = Options {
            wait: 6,
            ..Default::default()
        };
        let mut it = args.into_iter();
        while let Some(a) = it.next() {
            let mut value = |name: &str| it.next().ok_or(format!("{name} needs a value"));
            match a.as_str() {
                "--lang" => {
                    let v = value("--lang")?;
                    o.lang = Some(Lang::from_code(&v).ok_or(format!("unknown language '{v}'"))?);
                }
                "--theme" => {
                    let v = value("--theme")?;
                    o.theme = Some(ThemeId::from_key(&v).ok_or(format!("unknown theme '{v}'"))?);
                }
                "--shot" => o.shot = Some(PathBuf::from(value("--shot")?)),
                "--wait" => {
                    let v = value("--wait")?;
                    o.wait = v
                        .parse()
                        .map_err(|_| format!("--wait needs a number, not '{v}'"))?;
                }
                "--start" => o.start = true,
                other if !other.starts_with("--") && o.file.is_none() => {
                    o.file = Some(PathBuf::from(other))
                }
                other => return Err(format!("unknown option '{other}'")),
            }
        }
        Ok(o)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(s: &str) -> Result<Options, String> {
        Options::parse(s.split_whitespace().map(String::from))
    }

    #[test]
    fn the_options_are_read() {
        let o = parse("--lang de --theme neon --shot a.png --wait 9 --start job.nc").unwrap();
        assert_eq!(o.lang, Some(Lang::De));
        assert_eq!(o.theme, Some(ThemeId::Neon));
        assert_eq!((o.wait, o.start), (9, true));
        assert_eq!(o.file, Some(PathBuf::from("job.nc")));
        assert_eq!(o.shot, Some(PathBuf::from("a.png")));
        assert_eq!(parse("").unwrap().wait, 6);
    }

    #[test]
    fn what_is_wrong_is_said() {
        assert!(parse("--lang xx").is_err());
        assert!(parse("--theme pink").is_err());
        assert!(parse("--wait many").is_err());
        assert!(parse("--bogus").is_err());
        assert!(parse("--shot").is_err());
    }
}
