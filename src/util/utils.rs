use fltk::dialog;
use fltk::dialog::{FileDialogOptions, FileDialogType};

pub fn open_file_chooser(t: FileDialogType, filter: String) -> (bool, String) {
    let mut chooser = dialog::FileDialog::new(t);
    chooser.set_filter(&filter);
    chooser.set_option(FileDialogOptions::Preview);
    chooser.show();
    let path = chooser.filename().to_string_lossy().to_string();
    return (!path.is_empty(), path);
}
