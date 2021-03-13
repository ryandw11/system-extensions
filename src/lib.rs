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
#[cfg(windows)]
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
#[cfg(test)]
#[cfg(unix)]
mod tests {
    use std::fs::File;
    use std::ops::Add;
    use std::path::Path;

    use crate::metadata::time::{FileTime, set_creation_date, set_accessed_date, set_changed_date, filetime_to_systime};
    use crate::metadata::attribute::{set_attribute, Attributes, has_attribute, get_attributes};
    use crate::processes::processes::find_process_id;

    #[test]
    fn it_works() {
        let val = find_process_id("NetworkManager").expect("An error occurred!");
        println!("{:?}", val);
        let time = FileTime {
            day: 13,
            month: 3,
            year: 2021,
            hour: 2,
            minute: 46,
            second: 46,
            milliseconds: 0
        };
        let systime = filetime_to_systime(&time);
        assert_eq!(systime,"202103130246.46");
        set_changed_date( Path::new(&std::env::var("HOME").unwrap()).join(".profile").as_path(), &time);
        set_accessed_date( Path::new(&std::env::var("HOME").unwrap()).join(".profile").as_path(), &time);

    }
}
