use std::env;
use std::process::exit;
use colored::Colorize;
use pathdiff::diff_paths;
use std::fs::read_to_string;
use futures_util::StreamExt;
use std::iter::repeat;
use crate::parser::diagnostics::diagnostics::{Diagnostics, DiagnosticsLog};

pub fn print_diagnostics_and_exit(diagnostics: &Diagnostics, print_warnings: bool) {
    print_diagnostics(diagnostics, print_warnings);
    exit(1);
}

pub fn print_diagnostics(diagnostics: &Diagnostics, print_warnings: bool) {
    if diagnostics.has_warnings() && print_warnings {
        for log in diagnostics.warnings() {
            print_diagnostics_log(log);
        }
    }
    if diagnostics.has_errors() {
        for log in diagnostics.errors() {
            print_diagnostics_log(log);
        }
    }
}

fn print_diagnostics_log<T>(log: T) where T: DiagnosticsLog {
    let source = log.source_path();
    let current_dir = &env::current_dir().unwrap();
    let filename = if let Some(path) = diff_paths(source, current_dir) {
        let result = path.to_str().unwrap().to_owned();
        if result.starts_with(".") {
            result
        } else {
            if cfg!(windows) {
                ".\\".to_owned() + result.as_str()
            } else {
                "./".to_owned() + result.as_str()
            }
        }
    } else {
        source.to_str().unwrap().to_owned()
    };
    let title = if log.is_warning() {
        "Warning".yellow().bold()
    } else if log.is_error() {
        "Error".red().bold()
    } else {
        "Unknown".yellow().bold()
    };
    let mut code = "".to_owned();
    let file_content = read_to_string(source).unwrap();
    let first_line_content: &str = file_content.lines().nth(log.span().start_position.0 - 1).unwrap();
    code += format!("{} {}\n", "|".blue().bold(), first_line_content).as_str();
    if log.span().start_position.0 == log.span().end_position.0 {
        let before_len = log.span().start_position.1 - 1;
        let content_len = log.span().end_position.1 - log.span().start_position.1;
        code += format!("{} {}{}\n", "|".blue().bold(), repeat(" ").take(before_len).collect::<String>(), repeat("^").take(content_len).collect::<String>().bright_blue()).as_str()
    } else {
        let before_len = log.span().start_position.1 - 1;
        let content_len = first_line_content.len() - before_len;
        code += format!("{} {}{}\n", "|".blue().bold(), repeat(" ").take(before_len).collect::<String>(), repeat("^").take(content_len).collect::<String>().bright_blue()).as_str()
    }
    if log.span().start_position.0 != log.span().end_position.0 {
        if log.span().start_position.0 + 1 != log.span().end_position.0 {
            code += format!("{} ...\n", "|".blue().bold()).as_str();
        }
        let last_line_content = file_content.lines().nth(log.span().end_position.0 - 1).unwrap();
        code += format!("{} {}\n", "|".blue().bold(), last_line_content).as_str();
        let len = log.span().end_position.1;
        code += format!("{} {}\n", "|".blue().bold(), repeat("^").take(len).collect::<String>().bright_blue()).as_str();
    }
    println!("{}: {}:{}:{} - {}:{}\n{}{}", title, filename, log.span().start_position.0, log.span().start_position.1, log.span().end_position.0, log.span().end_position.1, code, log.message());
}