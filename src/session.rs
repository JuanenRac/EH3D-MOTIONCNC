//! What a machine-control program does with a controller: show where the machine is, move it by hand, zero it, send a line, and run a
//! program one line at a time that can be paused, stopped and cut by the emergency stop. It knows nothing of the window, so it is tested
//! on the simulated controller.

use eh3d_controller::{Command, Controller, Error, State, Status, AXES};

use crate::gcode::Modal;

/// The jog steps offered, in mm.
pub const STEPS: [f64; 5] = [0.01, 0.1, 1.0, 10.0, 100.0];

pub struct Session {
    pub controller: Box<dyn Controller>,
    pub status: Status,
    pub program: Vec<String>,
    /// The next line of the program to send.
    pub next: usize,
    pub running: bool,
    pub paused: bool,
    /// What was said between the program and the controller, the oldest first.
    pub log: Vec<String>,
    pub step_mm: f64,
    pub feed_mm_min: f64,
    /// The codes in force, from the lines the controller took.
    pub modal: Modal,
    /// The zero of the operator, apart from the work zero: a position of the machine the operator chose to measure from.
    pub operator_zero: [f64; AXES],
}

impl Session {
    pub fn new(controller: Box<dyn Controller>) -> Self {
        let mut s = Session {
            controller,
            status: Status::default(),
            program: Vec::new(),
            next: 0,
            running: false,
            paused: false,
            log: Vec::new(),
            step_mm: 1.0,
            feed_mm_min: 600.0,
            modal: Modal::default(),
            operator_zero: [0.0; AXES],
        };
        s.refresh();
        s
    }

    fn say(&mut self, line: impl Into<String>) {
        self.log.push(line.into());
        if self.log.len() > 500 {
            self.log.drain(..100);
        }
    }

    /// What went wrong stops a program that is running, and is written in the log.
    fn failed(&mut self, what: &str, e: Error) {
        self.say(format!("{what}: {e}"));
        self.running = false;
        self.paused = false;
    }

    /// Asks the controller how it is.
    pub fn refresh(&mut self) {
        match self.controller.status() {
            Ok(s) => self.status = s,
            Err(e) => self.failed("status", e),
        }
    }

    pub fn alarm(&self) -> bool {
        self.status.state == State::Alarm
    }

    /// Moves an axis by one step, in the direction `dir` (1 or -1).
    pub fn jog(&mut self, axis: usize, dir: f64) {
        if self.running && !self.paused {
            self.say("jog: a program is running");
            return;
        }
        let command = Command::Jog {
            axis,
            distance_mm: dir * self.step_mm,
            feed_mm_min: self.feed_mm_min,
        };
        if let Err(e) = self.controller.command(command) {
            self.failed("jog", e);
        }
        self.refresh();
    }

    fn order(&mut self, what: &str, command: Command) {
        if let Err(e) = self.controller.command(command) {
            self.failed(what, e);
        }
        self.refresh();
    }

    /// Sends a line of the program's own making (a move to a stored position, to the zero) and keeps what it sets.
    fn own_line(&mut self, what: &str, line: &str) {
        match self.controller.send_line(line) {
            Ok(()) => self.modal.apply(line),
            Err(e) => self.failed(what, e),
        }
        self.refresh();
    }

    /// Goes to the position stored with `G28` (the first one).
    pub fn go_g28(&mut self) {
        self.own_line("G28", "G28");
    }

    /// Goes to the position stored with `G30` (the second one).
    pub fn go_g30(&mut self) {
        self.own_line("G30", "G30");
    }

    /// Goes to the work zero in X and Y.
    pub fn go_zero_xy(&mut self) {
        self.own_line("go to zero", "G90 G0 X0 Y0");
    }

    /// Goes to the work zero in Z.
    pub fn go_zero_z(&mut self) {
        self.own_line("go to zero", "G90 G0 Z0");
    }

    /// Makes the work position here zero on these axes only.
    pub fn zero_axes(&mut self, axes: &[usize]) {
        self.order("zero", Command::ZeroWork(axes.to_vec()));
    }

    /// Makes the operator's position here zero on these axes.
    pub fn zero_operator(&mut self, axes: &[usize]) {
        for &a in axes {
            if a < AXES {
                self.operator_zero[a] = self.status.machine_mm[a];
            }
        }
    }

    /// Where the machine is measured from the operator's zero.
    pub fn operator_mm(&self) -> [f64; AXES] {
        let mut o = [0.0; AXES];
        for (i, v) in o.iter_mut().enumerate() {
            *v = self.status.machine_mm[i] - self.operator_zero[i];
        }
        o
    }

    pub fn home(&mut self) {
        self.order("home", Command::Home(Vec::new()));
    }

    /// Makes the work position here zero on X, Y and Z.
    pub fn zero(&mut self) {
        self.order("zero", Command::ZeroWork(vec![0, 1, 2]));
    }

    pub fn unlock(&mut self) {
        self.order("unlock", Command::Unlock);
    }

    /// Stops everything at once; the controller stays in alarm until it is unlocked.
    pub fn estop(&mut self) {
        self.running = false;
        self.paused = false;
        self.order("emergency stop", Command::EmergencyStop);
        self.say("EMERGENCY STOP");
    }

    /// One line typed by the operator.
    pub fn mdi(&mut self, line: &str) {
        let line = line.trim();
        if line.is_empty() {
            return;
        }
        self.say(format!("> {line}"));
        match self.controller.send_line(line) {
            Ok(()) => {
                self.modal.apply(line);
                self.say("ok")
            }
            Err(e) => self.failed("mdi", e),
        }
        self.refresh();
    }

    /// Takes a program (the lines of a G-code file); a program that was running is stopped.
    pub fn load(&mut self, text: &str) {
        self.running = false;
        self.paused = false;
        self.program = text.lines().map(|l| l.trim_end().to_string()).collect();
        self.next = 0;
    }

    /// Starts the program from its first line, or goes on if it was paused.
    pub fn start(&mut self) {
        if self.alarm() {
            self.say("start: the controller is in alarm");
            return;
        }
        if self.paused {
            self.order("resume", Command::Resume);
            self.paused = false;
            return;
        }
        if self.program.is_empty() {
            return;
        }
        self.next = 0;
        self.running = true;
    }

    pub fn pause(&mut self) {
        if self.running && !self.paused {
            self.paused = true;
            self.order("pause", Command::Pause);
        }
    }

    pub fn stop(&mut self) {
        self.running = false;
        self.paused = false;
        self.order("stop", Command::Stop);
    }

    /// Sends the next lines of the running program (at most `max_lines`), the empty ones and the comments counting as sent. A line the
    /// controller refuses stops the program at that line.
    pub fn pump(&mut self, max_lines: usize) {
        let mut sent = 0;
        while self.running && !self.paused && sent < max_lines {
            let Some(line) = self.program.get(self.next).cloned() else {
                self.running = false;
                self.say("program done");
                break;
            };
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with(';') {
                if let Err(e) = self.controller.send_line(trimmed) {
                    let n = self.next + 1;
                    self.failed(&format!("line {n} ({trimmed})"), e);
                    break;
                }
                self.modal.apply(trimmed);
            }
            self.next += 1;
            sent += 1;
        }
        self.refresh();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eh3d_controller::sim::SimController;

    fn session() -> Session {
        Session::new(Box::new(SimController::new()))
    }

    #[test]
    fn a_jog_moves_by_the_step_and_zero_makes_the_work_position_nothing() {
        let mut s = session();
        s.step_mm = 10.0;
        s.jog(0, 1.0);
        s.jog(1, 1.0);
        assert_eq!(
            (s.status.machine_mm[0], s.status.machine_mm[1]),
            (10.0, 10.0)
        );
        s.zero();
        s.jog(0, -1.0);
        s.step_mm = 1.0;
        s.jog(0, 1.0);
        assert_eq!(s.status.work_mm[0], -9.0);
    }

    #[test]
    fn a_program_runs_line_by_line_and_ends_by_itself() {
        let mut s = session();
        s.load("G21 G90\nG0 X10 Y10\n\n; a comment\nG1 X20 F300\nM30");
        s.start();
        assert!(s.running);
        s.pump(2);
        assert_eq!((s.next, s.status.machine_mm[0]), (2, 10.0));
        s.pump(100);
        assert!(!s.running);
        assert_eq!(s.status.machine_mm[0], 20.0);
        assert!(s.log.last().unwrap().contains("program done"));
    }

    #[test]
    fn a_pause_holds_the_program_and_start_goes_on() {
        let mut s = session();
        s.load("G0 X1\nG0 X2\nG0 X3");
        s.start();
        s.pump(1);
        s.pause();
        s.pump(10);
        assert_eq!(s.next, 1, "lines went out while it was paused");
        s.start();
        s.pump(10);
        assert_eq!(s.status.machine_mm[0], 3.0);
    }

    #[test]
    fn a_refused_line_stops_the_program_where_it_is_and_says_which() {
        let mut s = session();
        s.load("G0 X1\nG2 X5 I1\nG0 X9");
        s.start();
        s.pump(10);
        assert!(!s.running);
        assert_eq!(s.next, 1);
        assert!(s.log.iter().any(|l| l.contains("line 2")), "{:?}", s.log);
        assert_eq!(s.status.machine_mm[0], 1.0);
    }

    #[test]
    fn the_emergency_stop_ends_the_run_and_blocks_it_until_unlocked() {
        let mut s = session();
        s.load("G0 X1\nG0 X2");
        s.start();
        s.pump(1);
        s.estop();
        assert!(!s.running && s.alarm());
        s.start();
        assert!(!s.running, "a program started in alarm");
        s.jog(0, 1.0);
        assert_eq!(s.status.machine_mm[0], 1.0, "it moved in alarm");
        s.unlock();
        assert!(!s.alarm());
        s.start();
        assert!(s.running);
    }

    #[test]
    fn a_jog_is_refused_while_a_program_runs_and_the_mdi_line_is_logged() {
        let mut s = session();
        s.load("G0 X1");
        s.start();
        s.jog(0, 1.0);
        assert!(s.log.last().unwrap().contains("running"));
        s.stop();
        s.mdi("G0 X7");
        assert_eq!(s.status.machine_mm[0], 7.0);
        assert!(s.log.iter().any(|l| l == "> G0 X7"));
    }

    #[test]
    fn the_stored_positions_the_axis_zero_and_the_operator_zero_work_on_their_own() {
        let mut s = session();
        s.mdi("G0 X30 Y40 Z-5");
        s.zero_axes(&[0]);
        s.mdi("G0 X50");
        assert_eq!((s.status.work_mm[0], s.status.work_mm[1]), (50.0, 40.0));
        s.zero_operator(&[1]);
        s.mdi("G0 Y60");
        assert_eq!(s.operator_mm()[1], 20.0);
        s.go_zero_xy();
        assert_eq!((s.status.work_mm[0], s.status.work_mm[1]), (0.0, 0.0));
        s.go_g28();
        assert_eq!(s.status.machine_mm[2], 50.0);
        s.go_zero_z();
        assert_eq!(s.status.work_mm[2], 0.0);
    }

    #[test]
    fn the_codes_in_force_come_from_what_the_controller_took() {
        let mut s = session();
        s.mdi("G91 M3 S500");
        assert_eq!(s.modal.m_codes()[0], "M3");
        assert!(s.modal.g_codes().contains(&"G91".to_string()));
        s.load(
            "G90
M5",
        );
        s.start();
        s.pump(5);
        assert!(s.modal.g_codes().contains(&"G90".to_string()));
        assert_eq!(s.modal.m_codes()[0], "M5");
        // a refused line changes nothing
        s.mdi("G2 X1");
        assert!(s.modal.g_codes().contains(&"G90".to_string()));
    }

    #[test]
    fn homing_and_an_alarm_from_the_travel_are_shown() {
        let mut s = session();
        s.mdi("G0 X400");
        s.home();
        assert_eq!(s.status.machine_mm[0], 0.0);
        s.mdi("G0 X9000");
        assert!(s.alarm());
    }
}
