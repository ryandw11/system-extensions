use std::ffi::CString;
use crate::dialogues::messagebox::{MessageBox, BoxReturn};
use crate::core::Bitflagable;
use gtk::{ButtonsType, DialogFlags, MessageType, MessageDialog, Window, GtkWindowExt, DialogExt};
pub fn create_message_box(message_box: MessageBox) -> Result<BoxReturn, String> {
    gtk::init();

    let dialog = MessageDialog::new(None::<&Window>,
                                    DialogFlags::empty(),
                                    MessageType::Info,
                                    ButtonsType::Ok,
                                    message_box.content);
    dialog.set_title(message_box.title);
    dialog.run();
    Result::Err("Test".to_string())
}