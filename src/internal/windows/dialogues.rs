use std::ffi::CString;
use crate::dialogues::messagebox::{MessageBox, BoxReturn};
use crate::core::Bitflagable;

pub fn create_message_box(message_box: MessageBox) -> Result<BoxReturn, String> {
    use winapi::um::winuser::MessageBoxA;
    use core::ptr::null_mut;

    let title_cstr: CString = CString::new(message_box.title).unwrap();
    let content_ctr: CString = CString::new(message_box.content).unwrap();

    let box_type = message_box.default_button.get_bits() | message_box.icon_type.get_bits() | message_box.window_type.get_bits();

    let result = unsafe { MessageBoxA(null_mut(), content_ctr.as_ptr(), title_cstr.as_ptr(), box_type) };

    if result == 0 {
        return Err("An error has occurred creating a MessageBox on Windows!".to_string());
    }

    Ok(BoxReturn::from_bits(result).unwrap())
}