use std::path::PathBuf;

use native_dialog::{FileDialog};

pub fn open_file_chooser(filter: (&str, &[&str])) -> (bool, String) {
    process_chooser(
        FileDialog::new()
            .add_filter(filter.0, filter.1)
            .show_open_single_file()
            .unwrap()
    )
}

pub fn open_folder_chooser() -> (bool, String) {
    process_chooser(
        FileDialog::new().show_open_single_dir().unwrap()
    )
}

fn process_chooser(chooser: Option<PathBuf>) -> (bool, String) {
    return match chooser {
        None => { (false, String::new()) }
        Some(path) => { (true, path.to_string_lossy().to_string()) }
    };
}


