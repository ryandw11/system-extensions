use std::ffi::CString;
use crate::dialogues::messagebox::{MessageBox, BoxReturn, WindowType};
use crate::core::Bitflagable;
use gtk::{ButtonsType, DialogFlags, MessageType, MessageDialog, Window, GtkWindowExt, DialogExt, ResponseType};
pub fn create_message_box(message_box: MessageBox) -> Result<BoxReturn, String> {
    let result = gtk::init();
    if result.is_err(){
        return Err("GTK unable to initialize".parse().unwrap());
    }


    let dialog = MessageDialog::new(None::<&Window>,
                                    DialogFlags::empty(),
                                    MessageType::Info,
                                    ButtonsType::None,
                                    message_box.content);
    //TODO track down icon types.
    dialog.set_title(message_box.title);

    if message_box.window_type == WindowType::OK{
     dialog.add_button("Ok", ResponseType::Ok);
    }else if message_box.window_type== WindowType::OK_CANCEL{
        dialog.add_button("Ok", ResponseType::Ok);
        dialog.add_button("Cancel", ResponseType::Cancel);
    }else if message_box.window_type== WindowType::HELP{
        dialog.add_button("Help", ResponseType::Help);
    }else if message_box.window_type== WindowType::ABORT_RETRY_IGNORE{
        dialog.add_button("Abort", ResponseType::Cancel);
        dialog.add_button("Retry", ResponseType::Other(1));
        dialog.add_button("Ignore", ResponseType::Other(2));
    }else if message_box.window_type== WindowType::CANCEL_TRY_CONTINUE{
        dialog.add_button("Abort", ResponseType::Cancel);
        dialog.add_button("Try", ResponseType::Other(3));
        dialog.add_button("Continue", ResponseType::Other(4));
    }else if message_box.window_type== WindowType::RETRY_CANCEL{
        dialog.add_button("Abort", ResponseType::Cancel);
        dialog.add_button("Retry", ResponseType::Other(1));
    }else if message_box.window_type== WindowType::YES_NO{
        dialog.add_button("Yes", ResponseType::Yes);
        dialog.add_button("No", ResponseType::No);
    }else if message_box.window_type== WindowType::YES_NO_CANCEL{
        dialog.add_button("Yes", ResponseType::Yes);
        dialog.add_button("No", ResponseType::No);
        dialog.add_button("Cancel", ResponseType::Cancel);

    }
    let response_type = dialog.run();
    let mut box_return = match response_type {
        ResponseType::Ok => Some(BoxReturn::OK),
        ResponseType::Cancel => Some(BoxReturn::CANCEL),
        ResponseType::Yes => Some(BoxReturn::YES),
        ResponseType::No => Some(BoxReturn::NO),
        _ => {None}
    };
    return if box_return.is_none() {
        let mut box_return = match response_type.into() {
            1 => Some(BoxReturn::RETRY),
            2 => Some(BoxReturn::IGNORE),
            3 => Some(BoxReturn::TRY_AGAIN),
            4 => Some(BoxReturn::CONTINUE),
            _ => { None }
        };
        if box_return.is_none(){
            return Err("Invalid Response".parse().unwrap());
        }
        Ok(box_return.unwrap())
    } else {
        Ok(box_return.unwrap())
    }
}
