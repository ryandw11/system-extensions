#[cfg(windows)]
extern crate winapi;

use std::ffi::CString;
use std::os::raw::c_char;
use std::path::{Path, PathBuf};

/**
    Represents a valid file filter.
*/
#[derive(Debug)]
pub struct Filter {
    pub title: String,
    pub file_ending: String,
}

impl Filter {
    /**
       Construct a new File Filter.

       ## Params
       title: String -> The title of the filter. (ex: Text)<br>
       file_ending: String -> The file ending to filter. (ex: *.txt)

       ## Example
       ```rust
       use system_extensions::dialogues::filebox::Filter;
       Filter::new("Text".to_string(), "*.txt".to_string());
       ```
    */
    pub fn new(title: String, file_ending: String) -> Filter {
        Filter {
            title,
            file_ending,
        }
    }
}


/**
    Change a str to a c string.
*/
fn str_to_c_str(s: &str) -> *mut c_char {
    let mut bytes: Vec<u8> = String::from(s).into_bytes();
    bytes.push(b"\0"[0]);
    let mut cchar_vec: Vec<c_char> = bytes.iter().map(|b| *b as c_char).collect();
    cchar_vec.as_mut_ptr()
}

/**
    Convert an array slice of i8 to a String.

    The array slice of i8 is expected to represent a UTF-8 String.
    The array slice is also permitted to have NULL (\0) characters.
*/
fn slice_to_string(arr: &[i8]) -> String {
    let mut output: Vec<u8> = Vec::new();
    for i in arr.iter() {
        let ui = *i as u8;
        if ui == b"\0"[0] {
            break;
        }
        output.push(ui);
    }
    String::from_utf8(output).unwrap()
}

/**
    Converts a filter to the proper format for the Windows API.
*/
#[cfg(windows)]
fn filter_to_str(filter: Vec<Filter>) -> String {
    let mut string: String = String::new();
    for fil in filter {
        string.push_str(&fil.title);
        string.push_str("\0");
        string.push_str(&fil.file_ending);
    }
    string.push_str("\0");

    return string;
}

/**
*    Open a file selection menu with a defined filter.
*
*    ## Params
*    filter: Vec<[`Filter`]> -> The vector containing the desired filters.
*    ## Returns
*    PathBuf -> A PathBuf with the location of the file. (Not validated)
*
*    ## Examples
*    ```rust
*    use system_extensions::dialogues::filebox::{Filter, open_select_file_menu_filter};
*    use std::path::PathBuf;
*
*    let filter = vec![
*        Filter::new("PNG File".to_string(), "*.png".to_string()),
*        Filter::new("JPEG File".to_string(), "*.jpg".to_string())
*    ];
*
*    let result: PathBuf = open_select_file_menu_filter(filter);
*    ```
*/
#[cfg(windows)]
pub fn open_select_file_menu_filter(filter: Vec<Filter>) -> PathBuf {
    use core::mem;
    use winapi::um::commdlg::{GetOpenFileNameA, OPENFILENAMEA, OFN_PATHMUSTEXIST, OFN_FILEMUSTEXIST};

    let mut my_str: [i8; 100] = [0; 100];
    my_str[0] = '\0' as i8;

    let filt: String = filter_to_str(filter);

    let mut open_file: OPENFILENAMEA = OPENFILENAMEA::default();
    open_file.lStructSize = mem::size_of::<OPENFILENAMEA>() as u32;
    open_file.hwndOwner = core::ptr::null_mut();
    open_file.lpstrFile = my_str.as_ptr() as *mut i8;
    open_file.nMaxFile = my_str.len() as u32;
    // open_file.lpstrFilter = str_to_c_str("All\0*.*\0Text\0*.TXT\0");
    open_file.lpstrFilter = str_to_c_str(filt.as_str());
    open_file.nFilterIndex = 1;
    open_file.lpstrFileTitle = core::ptr::null_mut();
    open_file.nMaxFileTitle = 0;
    open_file.lpstrInitialDir = core::ptr::null_mut();
    open_file.Flags = OFN_PATHMUSTEXIST | OFN_FILEMUSTEXIST;

    let open_file_ptr: *mut OPENFILENAMEA = &mut open_file;

    unsafe {
        GetOpenFileNameA(open_file_ptr);
    }
    let slice = unsafe { std::slice::from_raw_parts(open_file.lpstrFile, 100) };

    return PathBuf::from(slice_to_string(slice));
}

/**
*    Open a file selection menu with a default filter.
*
*    ## Returns
*    PathBuf -> A PathBuf with the location of the file. (Not validated)
*
*    ## Examples
*    ```rust
*    use system_extensions::dialogues::filebox::open_select_file_menu;
*    use std::path::PathBuf;
*
*    let result: PathBuf = open_select_file_menu();
*    ```
*/
#[cfg(windows)]
pub fn open_select_file_menu() -> PathBuf {
    let filter: Vec<Filter> = vec![Filter::new("All".to_string(), "*.*".to_string())];

    open_select_file_menu_filter(filter)
}

/*
    Unix Section
 */


/**
*    Open a file selection menu with a defined filter.
*    **(Currently not implemented for Unix Systems)**
*
*    ## Params
*    filter: Vec<[`Filter`]> -> The vector containing the desired filters.
*    ## Returns
*    PathBuf -> A PathBuf with the location of the file. (Not validated)
*
*    ## Examples
*    ```rust
*    use system_extensions::dialogues::filebox::{Filter, open_select_file_menu_filter};
*    use std::path::PathBuf;
*
*    let filter = vec![
*        Filter::new("PNG File".to_string(), "*.png".to_string()),
*        Filter::new("JPEG File".to_string(), "*.jpg".to_string())
*    ];
*
*    let result: PathBuf = open_select_file_menu_filter(filter);
*    ```
*/
#[cfg(unix)]
pub fn open_select_file_menu_filter(filter: Vec<Filter>) -> PathBuf {
    unimplemented!()
}

/**
*    Open a file selection menu with a default filter.
*   **(Currently not implemented for Unix Systems)**
*
*    ## Returns
*    PathBuf -> A PathBuf with the location of the file. (Not validated)
*
*    ## Examples
*    ```rust
*    use system_extensions::dialogues::filebox::open_select_file_menu;
*    use std::path::PathBuf;
*
*    let result: PathBuf = open_select_file_menu();
*    ```
*/
#[cfg(unix)]
pub fn open_select_file_menu() -> PathBuf {
    unimplemented!()
}

