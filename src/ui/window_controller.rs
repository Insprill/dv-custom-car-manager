use crate::util::utils;

pub fn select_install_dir() {
    let (selected, path) = utils::open_folder_chooser();
    if !selected {
        return;
    }
    println!("{path}");
}