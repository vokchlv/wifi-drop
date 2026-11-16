use std::{env, path::Path};

pub fn get_file_path_from_arguments() -> Option<String> {
    for argument in env::args() {
        if argument.starts_with("target") {
            continue;
        }
        if Path::new(&argument).exists() {
            return Some(argument);
        }
    }
    None
}
