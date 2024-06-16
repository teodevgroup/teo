use std::time::Duration;
use chrono::{DateTime, Local};
use colored::{ColoredString, Colorize};
use array_tool::vec::Join;
use crate::prelude::{Entrance, RuntimeVersion};

fn format_code_into_string(code: u16) -> ColoredString {
    match code {
        0..=199 => code.to_string().purple().bold(),
        200..=299 => code.to_string().green().bold(),
        300..=399 => code.to_string().yellow().bold(),
        _ => code.to_string().red().bold(),
    }
}

fn timestamp() -> ColoredString {
    let local: DateTime<Local> = Local::now();
    let local_formatted = format!("[{}]", local.format("%Y-%m-%d %H:%M:%S")).bright_blue();
    local_formatted
}

pub fn info_message(content: impl AsRef<str>) {
    println!("{} {}", timestamp(), content.as_ref())
}

pub fn request_message(
    time_elapsed: Duration,
    method: &str,
    path: &str,
    handler_group_path: &Vec<String>,
    action: &str,
    code: u16,
) {
    let handler_str: String = handler_group_path.join(".") + ".";
    let code_string = format_code_into_string(code);
    let ms = time_elapsed.as_millis();
    let ms_str = format!("{ms}ms").normal().clear();
    println!("{} {} {} => {}{} {} {}", timestamp(), method.bright_blue().bold(), path.bright_yellow(), handler_str.magenta(), action.purple(), code_string, ms_str)
}

pub fn unhandled_request_message(
    time_elapsed: Duration,
    method: &str,
    path: &str,
    code: u16,
) {
    let code_string = format_code_into_string(code);
    let ms = time_elapsed.as_millis();
    let ms_str = format!("{ms}ms").normal().clear();
    println!("{} {} {} {} {}", timestamp(), method.bright_blue().bold(), path.bright_yellow(), code_string, ms_str)
}

async fn server_start_message(port: u16, runtime_version: &'static RuntimeVersion, entrance: &'static Entrance, silent: bool) -> teo_result::Result<()> {
    if silent { return Ok(()) }
    // Introducing
    let teo_version = env!("CARGO_PKG_VERSION");
    let teo = format!("Teo {}", teo_version);
    info_message(format!("{} ({}, {})", teo, runtime_version.to_string(), entrance.to_str()));
    // Listening
    let port_str = format!("{port}").bold();
    info_message(format!("listening on port {}", port_str));
    Ok(())
}