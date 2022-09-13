use crate::util::file_chooser;

pub fn select_install_dir() {
    let (selected, path) = file_chooser::open_folder_chooser();
    if !selected {
        return;
    }
    println!("{path}");
}
