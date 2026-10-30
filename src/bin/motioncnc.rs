//! The EH3D machine-control program.
//!
//! Usage: `motioncnc [FILE.NC] [--lang CODE] [--theme NAME]`
//! For now it drives the simulated controller: jog, zero, home, the commands of the operator and a G-code program with its path.

use eh3d_motioncnc::cli::Options;

fn main() -> eframe::Result {
    match Options::parse(std::env::args().skip(1)) {
        Ok(o) => eh3d_motioncnc::gui::run(&o),
        Err(e) => {
            eprintln!("motioncnc: {e}");
            std::process::exit(2);
        }
    }
}
