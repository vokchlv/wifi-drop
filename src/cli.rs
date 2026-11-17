use std::{env, path::Path};

pub fn get_file_path_from_arguments() -> Option<String> {
    env::args()
        .skip(1)
        .find(|argument| Path::new(&argument).exists())
}
