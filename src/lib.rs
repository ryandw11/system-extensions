/**
    The main library.

*/
#[macro_use]
extern crate bitflags;

#[cfg(windows)]
extern crate winapi;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

#[cfg(target_os = "macos")]
extern crate cocoa;

/**
    The core of system_extensions.
*/
#[doc(hidden)]
pub mod core;

/**
    The internal code for system_extensions.
*/
#[doc(hidden)]
pub mod internal;

/**
    Feature that involves system processes.
*/
pub mod processes;
/**
    Feature that involves additional file metadata editing.
*/
pub mod metadata;
/**
    Experimental Feature that allows the creation of GUI dialogues.
*/
pub mod dialogues;

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
/*

    Windows tests.

 */
#[cfg(test)]
#[cfg(windows)]
mod tests {
    use std::fs::File;
    use std::ops::Add;
    use std::path::{Path, PathBuf};

    use crate::metadata::time::{FileTime, set_creation_date};
    use crate::metadata::attribute::{set_attribute, Attributes, has_attribute, get_attributes};
    use crate::processes::processes::find_process_id;
    use crate::dialogues::filebox::{Filter};
    use crate::obtain_error;
    use crate::dialogues::messagebox::{MessageBox, WindowType, IconType};
    use crate::dialogues::filebox::FileBox;

    #[test]
    fn it_works() {
        let val = find_process_id("Discord.exe").expect("An error occurred!");
        println!("{:?}", val);
        set_creation_date(Path::new("./test.txt"), &FileTime::new(5, 12, 3030));
        let out = set_attribute(Path::new("./test.txt"), Attributes::READ_ONLY | Attributes::HIDDEN);
        println!("{:?}", out);
        println!("Has attrib: {:?}", get_attributes(Path::new("./test.txt")));

        let mut r = MessageBox::new("This is a test!", "Wow.png")
            .set_icon_type(IconType::ICON_WARNING)
            .show();

        if r.unwrap() == crate::dialogues::messagebox::BoxReturn::CONTINUE {
            println!("The continue button as pressed!");
        }

        let result = FileBox::new().filter("Text", "*.txt")
            .directory(Path::new("D:\\"))
            .save("my_stuff.txt");

        println!("{}", result.expect("There should be a file!").to_str().unwrap());
        // println!("This is a tst");
    }
}

/*

    Unix tests.

 */
#[cfg(test)]
#[cfg(unix)]
mod tests {
    use std::fs::File;
    use std::path::Path;

    use crate::metadata::time::{FileTime, set_creation_date, set_accessed_date, set_changed_date, filetime_to_systime};
    use crate::metadata::attribute::{set_attribute, Attributes, get_attributes};
    use crate::processes::processes::{find_process_id, is_process_running};
    use std::io::Write;

    #[test]
    fn it_works() {
        let val = find_process_id("chrome").expect("An error occurred!");
        println!("{:?}", val);
        // Test a valid PID.
        let pid : u32 = 1818;
        println!("{:?}", is_process_running(&pid));

        let time = FileTime {
            day: 13,
            month: 3,
            year: 2022,
            hour: 2,
            minute: 46,
            second: 46,
            milliseconds: 0
        };
        let systime = filetime_to_systime(&time);
        assert_eq!(systime,"202203130246.46");
        set_creation_date(Path::new("./test.txt"), &time);
        set_changed_date( Path::new("./test.txt"), &time);
        set_accessed_date( Path::new("./test.txt"), &time);
    }

    // #[test]
    // fn attribute_tests(){
    //     let mut path = Path::new("./se.test");
    //
    //     let mut file = File::create(&path).unwrap();
    //     file.write_all(b"Howdy from System-extensions").unwrap();
    //     file.sync_all().unwrap();
    //     set_attribute(path, Attributes::HIDDEN);
    //     path = Path::new(".se.test");
    //     assert!(set_attribute(&path, Attributes::READ_ONLY),"Unable to change readonly status");
    //     let attributes = get_attributes(&path).unwrap();
    //     assert!(attributes.contains(&Attributes::HIDDEN),"Hidden Attribute missing");
    //     assert!(attributes.contains(&Attributes::READ_ONLY),"Read ONLY attribute missing");
    //
    // }
}
