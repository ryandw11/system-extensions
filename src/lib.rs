/**
    The main library.

*/
#[macro_use]
extern crate bitflags;

#[cfg(windows)]
extern crate winapi;

pub mod processes;
pub mod metadata;

/**
   If an error occurs in the Windows API, you can check it here.
   [View the error codes here.](https://docs.microsoft.com/en-us/windows/win32/debug/system-error-codes)
*/
#[cfg(windows)]
pub fn obtain_error() -> u32 {
    use winapi::um::errhandlingapi::GetLastError;
    unsafe {
        return GetLastError();
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::ops::Add;
    use std::path::Path;

    use crate::metadata::time::{FileTime, set_creation_date};
    use crate::metadata::attribute::{set_attribute, Attributes, has_attribute, get_attributes};
    use crate::processes::processes::find_process_id;

    #[test]
    fn it_works() {
        let val = find_process_id("Discord.exe").expect("An error occurred!");
        println!("{:?}", val);
        set_creation_date(Path::new("D:\\Rust\\system-extensions\\test.txt"), &FileTime::new(5, 12, 3030));
        let out = set_attribute(Path::new("D:\\Rust\\system-extensions\\test.txt"), Attributes::READ_ONLY | Attributes::HIDDEN);
        println!("{:?}", out);
        println!("Has attrib: {:?}", get_attributes(Path::new("D:\\Rust\\system-extensions\\test.txt")));
    }
}
