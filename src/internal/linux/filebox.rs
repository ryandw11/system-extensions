use crate::dialogues::filebox::FileBox;
use std::path::PathBuf;
use gtk::{FileChooserDialog, FileChooserAction, DialogExt, ResponseType, Window, FileChooserExt, FileFilter};

pub fn open_file_dialogue(file_box: FileBox) -> Option<PathBuf> {

gtk::init();
    let result = gtk::init();
    if result.is_err(){
        return Option::None;
    }
    let dialog = FileChooserDialog::with_buttons::<Window>(
        Some("Open File"),
        None,
        FileChooserAction::Open,
        &[("_Cancel", ResponseType::Cancel), ("_Open", ResponseType::Accept)]
    );
    if file_box.directory.is_some() {
        dialog.set_current_folder(file_box.directory.unwrap());
    }
    let filter = FileFilter::new();
    for x in file_box.filters {
        filter.add_pattern(&*x.file_ending);
    }
    dialog.set_filter(&filter);
    dialog.run();
    dialog.get_filename()
}

pub fn save_file_dialogue_filter(file_box: FileBox, suggested_name: &str) -> Option<PathBuf> {
    let result = gtk::init();
    if result.is_err(){
        return Option::None;
    }
    let dialog = FileChooserDialog::with_buttons::<Window>(
        Some("Save File"),
        None,
        FileChooserAction::Save,
        &[("_Cancel", ResponseType::Cancel), ("_Open", ResponseType::Accept)]
    );
    if file_box.directory.is_some() {
        dialog.set_current_folder(file_box.directory.unwrap());
    }
    dialog.set_filename(suggested_name);
    let filter = FileFilter::new();
    for x in file_box.filters {
        filter.add_pattern(&*x.file_ending);
    }
    dialog.set_filter(&filter);
    dialog.run();
    dialog.get_filename()
}