/*

    This file is needed to test Dialogues on Mac since the binding does not
    like test cases.

 */
extern crate system_extensions;
use system_extensions::dialogues::messagebox::*;

fn main() {
    let response = MessageBox::new("Test", "This is a test messagebox!")
        .set_window_type(WindowType::ABORT_RETRY_IGNORE)
        .show();

    println!("{:?}", response.unwrap() == BoxReturn::RETRY)
}