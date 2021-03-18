#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::dialogues::messagebox::{MessageBox, BoxReturn, IconType, WindowType};

use cocoa::base::{id, nil};
use cocoa::foundation::NSString;

/*
    Implementation of Apple's NSAlert
    https://developer.apple.com/documentation/appkit/nsalert

    Thanks to Jang Ryeol <ryeolj5911@gmail.com> for the implementation.
    https://github.com/bekker/msgbox-rs
*/

/**
 * NSAlert.Style
 * https://developer.apple.com/documentation/appkit/nsalert.style
 */
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSAlertStyle {
    warning = 0,
    informational = 1,
    critical = 2,
}

/**
    Hacky representation of the NSApplication.ModalResponse class.
*/
mod ModalResponse {
    #[allow(non_upper_case_globals)]
    pub const alertFirstButton: u8 = 232;
    #[allow(non_upper_case_globals)]
    pub const alertSecondButton: u8 = 233;
    #[allow(non_upper_case_globals)]
    pub const alertThirdButton: u8 = 234;
}

#[allow(non_upper_case_globals)]
pub static NSModalPannelWindowLevel: i32 = 10;

/**
   Represents the [NSAlert](https://developer.apple.com/documentation/appkit/nsalert) class
   for the Cocoa API.
*/
pub trait NSAlert: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSAlert), alloc]
    }

    unsafe fn init(self) -> id;
    unsafe fn autorelease(self) -> id;

    unsafe fn setAlertStyle(self, style: NSAlertStyle);
    unsafe fn setMessageText(self, messageText: id);
    unsafe fn setInformativeText(self, informativeText: id);
    unsafe fn addButton(self, withTitle: id);
    unsafe fn window(self) -> id;
    unsafe fn setWindowLevel(self, level: i32);
    unsafe fn runModal(self) -> id;
}

impl NSAlert for id {
    unsafe fn init(self) -> id {
        msg_send![self, init]
    }

    unsafe fn autorelease(self) -> id {
        msg_send![self, autorelease]
    }

    unsafe fn setAlertStyle(self, alertStyle: NSAlertStyle) {
        msg_send![self, setAlertStyle: alertStyle]
    }

    unsafe fn setMessageText(self, messageText: id) {
        msg_send![self, setMessageText: messageText]
    }

    unsafe fn setInformativeText(self, informativeText: id) {
        msg_send![self, setInformativeText: informativeText]
    }

    unsafe fn addButton(self, withTitle: id) {
        msg_send![self, addButtonWithTitle: withTitle]
    }

    unsafe fn window(self) -> id {
        msg_send![self, window]
    }

    unsafe fn setWindowLevel(self, level: i32) {
        msg_send![self.window(), setLevel: level]
    }

    unsafe fn runModal(self) -> id {
        msg_send![self, runModal]
    }
}



pub fn create_message_box(message_box: MessageBox) -> Result<BoxReturn, String> {
    let alert_type: NSAlertStyle = match message_box.icon_type {
        IconType::ICON_INFORMATION => NSAlertStyle::informational,
        IconType::ICON_QUESTION => NSAlertStyle::informational,
        IconType::ICON_WARNING => NSAlertStyle::warning,
        IconType::ICON_ERROR => NSAlertStyle::critical,
        _ => NSAlertStyle::informational,
    };

    unsafe {
        let alert = NSAlert::alloc(nil).init().autorelease();
        alert.setInformativeText(NSString::alloc(nil).init_str(message_box.title));
        alert.setMessageText(NSString::alloc(nil).init_str(message_box.content));
        alert.setAlertStyle(alert_type);
        alert.setWindowLevel(NSModalPannelWindowLevel);

        match message_box.window_type {
            WindowType::OK => alert.addButton(NSString::alloc(nil).init_str("Ok")),
            WindowType::OK_CANCEL => {
                alert.addButton(NSString::alloc(nil).init_str("Ok"));
                alert.addButton(NSString::alloc(nil).init_str("Cancel"));
            },
            WindowType::HELP => alert.addButton(NSString::alloc(nil).init_str("Help")),
            WindowType::ABORT_RETRY_IGNORE => {
                alert.addButton(NSString::alloc(nil).init_str("Abort"));
                alert.addButton(NSString::alloc(nil).init_str("Retry"));
                alert.addButton(NSString::alloc(nil).init_str("Ignore"));
            },
            WindowType::CANCEL_TRY_CONTINUE => {
                alert.addButton(NSString::alloc(nil).init_str("Cancel"));
                alert.addButton(NSString::alloc(nil).init_str("Try Again"));
                alert.addButton(NSString::alloc(nil).init_str("Continue"));
            },
            WindowType::RETRY_CANCEL => {
                alert.addButton(NSString::alloc(nil).init_str("Retry"));
                alert.addButton(NSString::alloc(nil).init_str("Cancel"));
            },
            WindowType::YES_NO => {
                alert.addButton(NSString::alloc(nil).init_str("Yes"));
                alert.addButton(NSString::alloc(nil).init_str("No"));
            },
            WindowType::YES_NO_CANCEL => {
                alert.addButton(NSString::alloc(nil).init_str("Yes"));
                alert.addButton(NSString::alloc(nil).init_str("No"));
                alert.addButton(NSString::alloc(nil).init_str("Cancel"));
            },
            _ => alert.addButton(NSString::alloc(nil).init_str("Ok"))
        }

        // Super hacky way of doing this.
        let val = alert.runModal() as u8;

        let response: BoxReturn = match message_box.window_type {
            WindowType::OK => if val == ModalResponse::alertFirstButton { BoxReturn::OK} else {BoxReturn::NONE}
            WindowType::OK_CANCEL => if val == ModalResponse::alertFirstButton { BoxReturn::OK }
                else if val == ModalResponse::alertSecondButton { BoxReturn::CANCEL }
                else {BoxReturn::NONE},
            WindowType::HELP => BoxReturn::NONE,
            WindowType::ABORT_RETRY_IGNORE => if val == ModalResponse::alertFirstButton { BoxReturn::ABORT }
                else if val == ModalResponse::alertSecondButton { BoxReturn::RETRY }
                else if val == ModalResponse::alertThirdButton { BoxReturn::CANCEL }
                else {BoxReturn::NONE},
            WindowType::CANCEL_TRY_CONTINUE => if val == ModalResponse::alertFirstButton { BoxReturn::CANCEL }
                else if val == ModalResponse::alertSecondButton { BoxReturn::TRY_AGAIN }
                else if val == ModalResponse::alertThirdButton { BoxReturn::CONTINUE }
                else {BoxReturn::CANCEL},
            WindowType::RETRY_CANCEL => if val == ModalResponse::alertFirstButton { BoxReturn::RETRY }
                else if val == ModalResponse::alertSecondButton { BoxReturn::CANCEL }
                else {BoxReturn::NONE},
            WindowType::YES_NO => if val == ModalResponse::alertFirstButton { BoxReturn::YES }
                else if val == ModalResponse::alertSecondButton { BoxReturn::NO }
                else {BoxReturn::NONE},
            WindowType::YES_NO_CANCEL => if val == ModalResponse::alertFirstButton { BoxReturn::YES }
                else if val == ModalResponse::alertSecondButton { BoxReturn::NO }
                else if val == ModalResponse::alertThirdButton { BoxReturn::CANCEL }
                else {BoxReturn::NONE},
            _ => BoxReturn::NONE
        };

        return Ok(response)
    }
}