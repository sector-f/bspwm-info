use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio, ChildStdout};

pub fn status() -> Info {
    Info::new()
}

pub struct Info {
    child_stdout: BufReader<ChildStdout>
}

impl Info {
    fn new() -> Self {
        let output = Command::new("bspc").args(&["subscribe", "report"]).stdout(Stdio::piped()).spawn().expect("Failed to run bspc. Is bspwm installed?");
        let stdout = output.stdout.expect("Failed to get bspc's stdout");

        Info {
            child_stdout: BufReader::new(stdout),
        }
    }
}

impl Iterator for Info {
    type Item = Wm;

    fn next(&mut self) -> Option<Wm> {
        let mut buffer = String::new();
        if self.child_stdout.read_line(&mut buffer).unwrap() > 0 {
            Some(parse_line(&buffer))
        } else {
            None
        }
    }
}

fn parse_line(line: &str) -> Wm {
    let mut monitors: Vec<Monitor> = Vec::new();

    for section in line[1..].split(":") {
        match &section[0..1] {
            "M" => { // Focused monitor
                monitors.push(
                    Monitor {
                        name: section[1..].to_string(),
                        desktops: Vec::new(),
                        focused: true,
                        layout: None,
                    }
                );
            },
            "m" => { // Unfocused monitor
                monitors.push(
                    Monitor {
                        name: section[1..].to_string(),
                        desktops: Vec::new(),
                        focused: false,
                        layout: None,
                    }
                );
            },
            "O" => { // Occupied focused desktop
                let desktop = {
                    Desktop {
                        name: section[1..].to_string(),
                        occupied: true,
                        focused: true,
                        urgent: false,
                    }
                };
                monitors.last_mut().unwrap().desktops.push(desktop);
            },
            "o" => { // Occupied unfocused desktop
                let desktop = {
                    Desktop {
                        name: section[1..].to_string(),
                        occupied: true,
                        focused: false,
                        urgent: false,
                    }
                };
                monitors.last_mut().unwrap().desktops.push(desktop);
            },
            "F" => { // Free focused desktop
                let desktop = {
                    Desktop {
                        name: section[1..].to_string(),
                        occupied: false,
                        focused: true,
                        urgent: false,
                    }
                };
                monitors.last_mut().unwrap().desktops.push(desktop);
            },
            "f" => { // Free unfocused desktop
                let desktop = {
                    Desktop {
                        name: section[1..].to_string(),
                        occupied: false,
                        focused: false,
                        urgent: false,
                    }
                };
                monitors.last_mut().unwrap().desktops.push(desktop);
            },
            "U" => { // Urgent focused desktop
                let desktop = {
                    Desktop {
                        name: section[1..].to_string(),
                        occupied: true,
                        focused: true,
                        urgent: true,
                    }
                };
                monitors.last_mut().unwrap().desktops.push(desktop);
            },
            "u" => { // Urgent unfocused desktop
                let desktop = {
                    Desktop {
                        name: section[1..].to_string(),
                        occupied: true,
                        focused: false,
                        urgent: true,
                    }
                };
                monitors.last_mut().unwrap().desktops.push(desktop);
            },
            "L" => { // Layout (tiling or monocle)
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

    Wm { monitors: monitors }
}

#[derive(Debug)]
pub struct Wm {
    pub monitors: Vec<Monitor>
}

#[derive(Debug)]
pub struct Monitor {
    pub name: String,
    pub desktops: Vec<Desktop>,
    pub focused: bool,
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