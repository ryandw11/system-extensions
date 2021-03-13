#[cfg(windows)]
extern crate winapi;

use std::ffi::OsStr;
use std::iter::once;
use std::path::Path;
#[cfg(unix)]
use std::process::Command;
use std::cmp::min;
use chrono::{Local, Timelike, Datelike};

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
    pub day: i16,
    pub month: i16,
    pub year: i16,
    pub hour: i16,
    pub minute: i16,
    pub second: i16,
    pub milliseconds: i16,
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
    //Creation time is not stored by Unix
    unimplemented!();
}
#[cfg(unix)]
pub fn set_accessed_date(file: &Path, create: &FileTime) -> bool {
  Command::new("touch").arg("-a").arg("-t").arg(filetime_to_systime(&create)).arg(file.to_str().unwrap()).spawn().is_ok()

}
#[cfg(unix)]
pub fn set_changed_date(file: &Path, create: &FileTime) -> bool {
    Command::new("touch").arg("-m").arg("-t").arg(filetime_to_systime(&create)).arg(file.to_str().unwrap()).spawn().is_ok()
}

#[cfg(unix)]
pub fn filetime_to_systime(time: &FileTime) -> String{
    let now = Local::now();

    let mut year:String;
    if time.year!=-1{
        year = time.year.to_string();
    }else{
        year = now.year().to_string();
    }
    if year.len()!=2{
        year = format!("{}", year);
    }

    let mut month:String;
    if time.month!=-1{
        month = time.month.to_string();
    }else{
        month = now.month().to_string();
    }
    if month.len()!=2{
        month = format!("0{}", month);
    }

    let mut day:String;
    if time.day!=-1{
        day = time.day.to_string();
    }else{
        day = (now.day()).to_string();
    }
    if day.len()!=2{
        day = format!("0{}", day);
    }

    let mut hour:String;
    if time.hour!=-1{
        hour = time.hour.to_string();
    }else{
        hour = (now.hour()+1).to_string();
    }
    if hour.len()!=2{
        hour = format!("0{}", hour);
    }

    let mut minute:String;
    if time.minute!=-1{
        minute = time.minute.to_string();
    }else{
        minute = (now.minute()+1).to_string();
    }
    if minute.len()!=2{
        minute = format!("0{}", minute);
    }

    let mut second:String;
    if time.second!=-1{
        second = time.second.to_string();
    }else{
        second = (now.second()+1).to_string();
    }
    if second.len()!=2{
        second = format!("0{}", second);
    }

    format!("{}{}{}{}{}.{}", year, month, day, hour, minute, second)
}