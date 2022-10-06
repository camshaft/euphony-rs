use env_logger::fmt::{Color, Style, StyledValue};
pub use log::*;
use std::{
    io::{BufRead, BufReader},
    process::{ChildStderr, Command, Stdio},
};

static mut IS_ALT_SCREEN: bool = false;

pub fn is_alt_screen() -> bool {
    unsafe { IS_ALT_SCREEN }
}

#[allow(dead_code)]
pub fn init_tui() {
    tui_logger::init_logger(log::LevelFilter::Info).unwrap();
    tui_logger::set_default_level(log::LevelFilter::Off);

    // TODO add other targets or make tui_logger support prefixes
    tui_logger::set_level_for_target("euphony_cli::logger", log::LevelFilter::Info);

    unsafe {
        IS_ALT_SCREEN = true;
    }
}

pub fn cmd(cmd: &mut Command) {
    if !is_alt_screen() {
        return;
    }

    cmd.stderr(Stdio::piped());
}

pub fn cmd_stderr(stderr: Option<ChildStderr>) {
    if let Some(stderr) = stderr {
        let stderr = BufReader::new(stderr);
        std::thread::spawn(move || {
            for line in stderr.lines().filter_map(Result::ok) {
                if !line.is_empty() {
                    log::info!("{}", line);
                }
            }
        });
    }
}

pub fn init() {
    env_logger::Builder::new()
        .filter(Some("euphony"), LevelFilter::Info)
        .parse_env("EUPHONY_LOG")
        .format(|f, record| {
            use std::io::Write;

            // pass INFO without modification
            if record.level() == Level::Info {
                return writeln!(f, "{}", record.args());
            }

            let mut style = f.style();
            let level = colored_level(&mut style, record.level());

            writeln!(f, " {} {}", level, record.args(),)
        })
        .init();
}

fn colored_level<'a>(style: &'a mut Style, level: Level) -> StyledValue<'a, &'static str> {
    match level {
        Level::Trace => style.set_color(Color::Magenta).value("TRACE"),
        Level::Debug => style.set_color(Color::Blue).value("DEBUG"),
        Level::Info => style.set_color(Color::Green).value("INFO "),
        Level::Warn => style.set_color(Color::Yellow).value("WARN "),
        Level::Error => style.set_color(Color::Red).value("ERROR"),
    }
}
