use std::ffi::CString;
use crate::dialogues::messagebox::{MessageBox, BoxReturn, WindowType};
use crate::core::Bitflagable;
use gtk::{ButtonsType, DialogFlags, MessageType, MessageDialog, Window, GtkWindowExt, DialogExt, ResponseType};
pub fn create_message_box(message_box: MessageBox) -> Result<BoxReturn, String> {
    let result = gtk::init();
    if result.is_err(){
        return Err("GTK unable to initialize".parse().unwrap());
    }
    let mut b_type=match message_box.window_type{
        WindowType::OK => ButtonsType::Ok,
        WindowType::OK_CANCEL => ButtonsType::OkCancel,
    _=>ButtonsType::Ok
    };

    let dialog = MessageDialog::new(None::<&Window>,
                                    DialogFlags::empty(),
                                    MessageType::Info,
                                    b_type,
                                    message_box.content);
    //TODO track down icon types.
    dialog.set_title(message_box.title);
    let response_type = dialog.run();
    let box_return = match response_type {
        ResponseType::Ok => BoxReturn::OK,
        ResponseType::Cancel => BoxReturn::CANCEL,
        _ => BoxReturn::OK
    };
    Ok(box_return)
}
