#[cfg(windows)]
extern crate winapi;
use std::ffi::CString;

bitflags! {
/**
    These are the properties for a message box.
    The values of these can change depending on the operating system.
*/
    pub struct BoxProperties: u32 {
        const OK = 0x00000000;
        const OK_CANCEL = 0x00000001;
        const ABORT_RETRY_IGNORE = 0x00000002;
        const CANCEL_TRY_CONTINUE = 0x00000006;
        const HELP = 0x00004000;
        const RETRY_CANCEL = 0x00000005;
        const YES_NO = 0x00000004;
        const YES_NO_CANCEL = 0x00000003;
        /*
           Icon Properties
         */
        const ICON_WARNING = 0x00000030;
        const ICON_INFORMATION = 0x00000040;
        const ICON_QUESTION = 0x00000020;
        const ICON_ERROR = 0x00000010;
        /*
            Default Buttons
         */
        const DEFAULT_BUTTON_ONE = 0x00000000;
        const DEFAULT_BUTTON_TWO = 0x00000100;
        const DEFAULT_BUTTON_THREE = 0x00000200;
        const DEFAULT_BUTTON_FOUR = 0x00000300;

    }
}

bitflags! {
/**
    This depicts what button was pressed in the message box.
*/
    pub struct BoxReturn: i32{
        const ABORT = 3;
        const CANCEL = 2;
        const CONTINUE = 11;
        const IGNORE = 5;
        const NO = 7;
        const OK = 1;
        const RETRY = 4;
        const TRY_AGAIN = 10;
        const YES = 6;
    }
}

/**
    Creates a message box. (A popup window that displays information, a warning, or an error.)

   ## Params
   title: &str -> The title of the message box. <br>
   content: &str -> The content of the message box. <br>
   box_type: [`BoxProperties`] -> The properties of the message box. Message box properties define
        the type of message box (info, warn, etc), the buttons, and the default selected button.

   ## Returns
   [`BoxReturn`] -> The action that the user took. (The clicked button).

   ## Examples
   Standard info box with an ok button:
   ```rust
   use system_extensions::dialogues::messagebox::{BoxReturn, BoxProperties, create_message_box};
   let result = create_message_box("Test Message", "This is a test message!", BoxProperties::ICON_INFORMATION);
   if result == BoxReturn::OK {
       println!("The user selected ok!");
   }
   ```
   Warning with an OK or Cancel button:
   ```rust
   use system_extensions::dialogues::messagebox::{BoxReturn, BoxProperties, create_message_box};
   let result = create_message_box("Test Message", "This is a test message!", BoxProperties::ICON_WARNING | BoxProperties::OK_CANCEL);
   if result == BoxReturn::CANCEL {
       println!("The user canceled the message!");
   }
   ```
   Error with Abort, Retry, and Ignore buttons. The Retry button is selected by default.
   ```rust
   use system_extensions::dialogues::messagebox::{BoxReturn, BoxProperties, create_message_box};
   let result = create_message_box("Test Message", "This is a test message!", BoxProperties::ICON_ERROR | BoxProperties::ABORT_RETRY_IGNORE | BoxProperties::DEFAULT_BUTTON_TWO);
   if result == BoxReturn::RETRY {
       println!("The user retried the message!");
   }
   ```
*/
#[cfg(windows)]
pub fn create_message_box(title: &str, content: &str, box_type: BoxProperties) -> BoxReturn {
    use winapi::um::winuser::MessageBoxA;
    use core::ptr::null_mut;

    let title_cstr: CString = CString::new(title).unwrap();
    let content_ctr: CString = CString::new(content).unwrap();

    unsafe{
        return BoxReturn::from_bits(
            MessageBoxA(null_mut(), content_ctr.as_ptr(), title_cstr.as_ptr(),
                        box_type.bits)
        ).unwrap();
    }
}

/**
   This is a version of [`create_message_box`] without any of the pre-defined Structs.
   View the [Windows API documentation](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messagebox) for valid parameters.

   ## Params
   title: &str -> The title of the message box.<br>
   content: &str -> The content of the message box.<br>
   box_type: u32 -> The properties of the message box.<br>

   ## Returns
   i32 -> The user result.

   ## Example
   ```rust
   use system_extensions::dialogues::messagebox::ncreate_message_box;
   ncreate_message_box("Title", "The message of the box.", 0x00000041);
   ```
*/
#[cfg(windows)]
pub fn ncreate_message_box(title: &str, content: &str, box_type: u32) -> i32 {
    use winapi::um::winuser::MessageBoxA;
    use core::ptr::null_mut;

    let title_cstr: CString = CString::new(title).unwrap();
    let content_ctr: CString = CString::new(content).unwrap();

    unsafe{
        return MessageBoxA(null_mut(), content_ctr.as_ptr(),
                           title_cstr.as_ptr(), box_type);
    }
}