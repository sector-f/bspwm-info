//! Library for getting information on monitors and workspaces in `bspwm`.
//! Tested with `bspwm 0.9.1`.
//!
//! # Examples
//!
//! List the names of the current monitors and the desktops that are on them:
//!
//! ````
//! extern crate bspwm_info;
//! use bspwm_info::*;
//!
//! fn main() {
//!     let current_info = status().next().unwrap();
//!     for monitor in current_info.monitors {
//!         println!("{}:", monitor.name);
//!         for desktop in monitor.desktops {
//!             println!("\t{}", desktop.name);
//!         }
//!     }
//! }
//! ````
//!
//! # To-Do
//!
//! * Use a different command (probably `bspc wm -d`) to obtain more information than
//!     `bspc subscribe report` provides.
//! * Communicate with `bspwm`'s socket directly rather than wrap the `bspc` command.
//!     Something like `fn status(path: Option<&Path>)`, where `Some(path)` specifies
//!     the socket location and `None` uses the default location as specified in
//!     `bspwm`'s man page.

use std::io::{self, BufRead, BufReader};
use std::process::{Command, Stdio, ChildStdout};

/// Creates a new `WmInfo`
pub fn status() -> WmInfo {
    WmInfo::new()
}

/// An iterator over `WmRoot`s that is created with the `status()` function
///
/// Internally, it holds a `BufReader` that collects output from `bspc subscribe report`.
/// Each call to `next()` blocks until `bspc` prints a new line. That line
/// is then parsed into a `WmRoot`
pub struct WmInfo {
    buffer: String,
    child_stdout: BufReader<ChildStdout>,
}

impl WmInfo {
    fn new() -> Self {
        let output = Command::new("bspc").args(&["subscribe", "report"]).stdout(Stdio::piped()).spawn().expect("Failed to run bspc. Is bspwm installed?");
        let stdout = output.stdout.expect("Failed to get bspc's stdout");

        WmInfo {
            buffer: String::new(),
            child_stdout: BufReader::new(stdout),
        }
    }
}

impl Iterator for WmInfo {
    type Item = io::Result<WmRoot>;

    fn next(&mut self) -> Option<io::Result<WmRoot>> {
        self.buffer.clear();

        match self.child_stdout.read_line(&mut self.buffer) {
            Ok(i) => {
                if i > 0 {
                    Some(Ok(parse_line(&self.buffer)))
                } else {
                    None
                }
            },
            Err(e) => {
                Some(Err(e))
            },
        }
    }
}

fn parse_line(line: &str) -> WmRoot {
    let mut monitors: Vec<Monitor> = Vec::new();

    for section in line[1..].split(":") {
        let input = section.chars().nth(0).unwrap();
        match input {
            'M' | 'm' => { // monitor
                monitors.push(
                    Monitor {
                        name: section[1..].to_string(),
                        desktops: Vec::new(),
                        focused: input.is_uppercase(),
                        layout: None,
                    }
                );
            },
            'O' | 'o' => { // Occupied desktop
                let desktop = {
                    Desktop {
                        name: section[1..].to_string(),
                        occupied: true,
                        focused: input.is_uppercase(),
                        urgent: false,
                    }
                };
                monitors.last_mut().unwrap().desktops.push(desktop);
            },
            'F' | 'f' => { // Free desktop
                let desktop = {
                    Desktop {
                        name: section[1..].to_string(),
                        occupied: false,
                        focused: input.is_uppercase(),
                        urgent: false,
                    }
                };
                monitors.last_mut().unwrap().desktops.push(desktop);
            },
            'U' | 'u' => { // Urgent desktop
                let desktop = {
                    Desktop {
                        name: section[1..].to_string(),
                        occupied: true,
                        focused: input.is_uppercase(),
                        urgent: true,
                    }
                };
                monitors.last_mut().unwrap().desktops.push(desktop);
            },
            'L' => { // Layout (tiling or monocle)
                let layout = {
                    match &section[1..2] {
                        "T" => { Some(Layout::Tiling) }
                        "M" => { Some(Layout::Monocle) }
                        _  => { None }
                    }
                };
                monitors.last_mut().unwrap().layout = layout;
            },
            _ => {},
        }
    }

    WmRoot { monitors: monitors }
}

/// A list of all the monitors that `bspwm` is aware of
#[derive(Debug)]
pub struct WmRoot {
    pub monitors: Vec<Monitor>
}

#[derive(Debug)]
pub struct Monitor {
    pub name: String,
    pub desktops: Vec<Desktop>,
    pub focused: bool,
    /// The Layout of the currently-focused monitor
    /// This is only an `Option` because it made parsing `bspc` easier;
    /// it should never be `None`. To-Do: Make it not an `Option`
    pub layout: Option<Layout>,
}

#[derive(Debug)]
pub struct Desktop {
    pub name: String,
    pub focused: bool,
    pub occupied: bool,
    pub urgent: bool,
}

#[derive(Debug)]
pub enum Layout {
    Tiling,
    Monocle,
}
