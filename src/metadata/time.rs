#[cfg(windows)]
extern crate winapi;

use std::ffi::OsStr;
use std::iter::once;
use std::path::Path;

/**
    Manages the imports for machines running on windows.
*/
#[cfg(windows)]
macro_rules! windows_imports {
    () => {
        use winapi::um::winnt;
        use self::winapi::um::handleapi::CloseHandle;
        use winapi::um::sysinfoapi;
        use self::winapi::um::fileapi::{OPEN_EXISTING};
        use self::winapi::shared::minwindef::{FILETIME};
        use self::winapi::_core::ptr::{null_mut};
        use self::winapi::um::winnt::{FILE_WRITE_ATTRIBUTES, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_WRITE};
        use self::winapi::um::minwinbase::SYSTEMTIME;
        use self::winapi::um::timezoneapi::SystemTimeToFileTime;
        use winapi::um::fileapi::SetFileTime;
        use std::os::windows::ffi::OsStrExt;
    };
}

/**
    Represents the time for a file.
*/
#[derive(Debug)]
pub struct FileTime {
    day: i16,
    month: i16,
    year: i16,
    hour: i16,
    minute: i16,
    second: i16,
    milliseconds: i16,
}

impl FileTime {
    /**
        Create a new file time.
    */
    pub fn new(day: i16, month: i16, year: i16) -> FileTime {
        FileTime {
            day,
            month,
            year,
            hour: -1,
            minute: -1,
            second: -1,
            milliseconds: -1,
        }
    }
}

#[cfg(windows)]
unsafe fn filetime_to_systime(system_time: *mut winapi::um::minwinbase::SYSTEMTIME, time: &FileTime) {
    if time.day != -1 {
        (*system_time).wDay = time.day as u16;
    }
    if time.month != -1 {
        (*system_time).wMonth = time.month as u16;
    }
    if time.year != -1 {
        (*system_time).wYear = time.year as u16;
    }
    if time.hour != -1 {
        (*system_time).wHour = time.hour as u16;
    }
    if time.minute != -1 {
        (*system_time).wMinute = time.minute as u16;
    }
    if time.second != -1 {
        (*system_time).wSecond = time.second as u16;
    }
    if time.milliseconds != -1 {
        (*system_time).wMilliseconds = time.milliseconds as u16;
    }
}

/**
   Set the creation date of a file.

   ## Params
   file: &Path -> The path of the file to change.
   create: &[`FileTime`] -> The new file time for a file.

   ## Returns
   bool -> True if successful, false if not.
   False means that the file could not be found or modified. Check to makesure
   the path is correct.

   ## Examples
   ```rust
   use system_extensions::metadata::time::{set_creation_date, FileTime};
   use std::path::Path;

   set_creation_date(Path::new("/test.txt"), &FileTime::new(25, 12, 2021));
   ```
*/
#[cfg(windows)]
pub fn set_creation_date(file: &Path, create: &FileTime) -> bool {
    windows_imports!();
    unsafe {
        let mut file_handle: winnt::HANDLE = std::mem::zeroed();

        let wide: Vec<u16> = OsStr::new(file.to_str().unwrap()).encode_wide().chain(once(0)).collect();
        file_handle = winapi::um::fileapi::CreateFileW(wide.as_ptr(), GENERIC_WRITE,
                                                       FILE_SHARE_READ | FILE_SHARE_WRITE, null_mut(), OPEN_EXISTING,
                                                       FILE_ATTRIBUTE_NORMAL | FILE_WRITE_ATTRIBUTES, null_mut());
        if file_handle == winapi::um::handleapi::INVALID_HANDLE_VALUE {
            return false;
        }

        let system_time: *mut SYSTEMTIME = &mut SYSTEMTIME {
            wYear: 0,
            wMonth: 0,
            wDayOfWeek: 0,
            wDay: 0,
            wHour: 0,
            wMinute: 0,
            wSecond: 0,
            wMilliseconds: 0,
        };

        sysinfoapi::GetSystemTime(system_time);

        // Convert FileTime to SystemTime
        filetime_to_systime(system_time, create);

        let file_time: *mut FILETIME = &mut FILETIME { dwLowDateTime: 0, dwHighDateTime: 0 };
        SystemTimeToFileTime(system_time, file_time);

        SetFileTime(file_handle, file_time as *const FILETIME, null_mut(), null_mut());
        CloseHandle(file_handle);
    }

    true
}


/**
   Set the accessed date of a file.

   ## Params
   file: &Path -> The path of the file to change.
   create: &[`FileTime`] -> The new file time for a file.

   ## Returns
   bool -> True if successful, false if not.
   False means that the file could not be found or modified. Check to makesure
   the path is correct.

   ## Examples
   ```rust
   use system_extensions::metadata::time::{set_accessed_date, FileTime};
   use std::path::Path;

   set_accessed_date(Path::new("/test.txt"), &FileTime::new(25, 12, 2021));
   ```
*/
#[cfg(windows)]
pub fn set_accessed_date(file: &Path, accessed: &FileTime) -> bool {
    unsafe {
        windows_imports!();
        let mut file_handle: winnt::HANDLE = std::mem::zeroed();

        let wide: Vec<u16> = OsStr::new(file.to_str().unwrap()).encode_wide().chain(once(0)).collect();
        file_handle = winapi::um::fileapi::CreateFileW(wide.as_ptr(), GENERIC_WRITE,
                                                       FILE_SHARE_READ | FILE_SHARE_WRITE, null_mut(), OPEN_EXISTING,
                                                       FILE_ATTRIBUTE_NORMAL | FILE_WRITE_ATTRIBUTES, null_mut());

        if file_handle == winapi::um::handleapi::INVALID_HANDLE_VALUE {
            return false;
        }

        let system_time: *mut SYSTEMTIME = &mut SYSTEMTIME {
            wYear: 0,
            wMonth: 0,
            wDayOfWeek: 0,
            wDay: 0,
            wHour: 0,
            wMinute: 0,
            wSecond: 0,
            wMilliseconds: 0,
        };

        sysinfoapi::GetSystemTime(system_time);

        // Convert FileTime to SystemTime
        filetime_to_systime(system_time, accessed);

        let file_time: *mut FILETIME = &mut FILETIME { dwLowDateTime: 0, dwHighDateTime: 0 };
        SystemTimeToFileTime(system_time, file_time);

        SetFileTime(file_handle, null_mut(), file_time as *const FILETIME, null_mut());
        CloseHandle(file_handle);
    }
    true
}

/**
   Set the creation date of a file.

   ## Params
   file: &Path -> The path of the file to change.
   create: &[`FileTime`] -> The new file time for a file.

   ## Returns
   bool -> True if successful, false if not.
   False means that the file could not be found or modified. Check to makesure
   the path is correct.

   ## Examples
   ```rust
   use system_extensions::metadata::time::{set_changed_date, FileTime};
   use std::path::Path;

   set_changed_date(Path::new("/test.txt"), &FileTime::new(25, 12, 2021));
   ```
*/
#[cfg(windows)]
pub fn set_changed_date(file: &Path, changed: &FileTime) -> bool{
   unsafe {
       windows_imports!();
       let mut file_handle: winnt::HANDLE = std::mem::zeroed();

       let wide: Vec<u16> = OsStr::new(file.to_str().unwrap()).encode_wide().chain(once(0)).collect();
       file_handle = winapi::um::fileapi::CreateFileW(wide.as_ptr(), GENERIC_WRITE,
                                                      FILE_SHARE_READ | FILE_SHARE_WRITE, null_mut(), OPEN_EXISTING,
                                                      FILE_ATTRIBUTE_NORMAL | FILE_WRITE_ATTRIBUTES, null_mut());

       if file_handle == winapi::um::handleapi::INVALID_HANDLE_VALUE {
           return false;
       }

       let system_time: *mut SYSTEMTIME = &mut SYSTEMTIME {
           wYear: 0,
           wMonth: 0,
           wDayOfWeek: 0,
           wDay: 0,
           wHour: 0,
           wMinute: 0,
           wSecond: 0,
           wMilliseconds: 0,
       };

       sysinfoapi::GetSystemTime(system_time);

       // Convert FileTime to SystemTime
       filetime_to_systime(system_time, changed);

       let file_time: *mut FILETIME = &mut FILETIME { dwLowDateTime: 0, dwHighDateTime: 0 };
       SystemTimeToFileTime(system_time, file_time);

       SetFileTime(file_handle, null_mut(), null_mut(), file_time as *const FILETIME);
       CloseHandle(file_handle);
   }

    true
}

/*

    Linux Section

 */
#[cfg(unix)]
pub fn set_creation_date(file: &Path, create: &FileTime) -> bool {
    unimplemented!();
}
#[cfg(unix)]
pub fn set_accessed_date(file: &Path, create: &FileTime) -> bool {
    unimplemented!();
}
#[cfg(unix)]
pub fn set_changed_date(file: &Path, create: &FileTime) -> bool {
    unimplemented!();
}