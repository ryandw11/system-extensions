use std::ffi::CString;
use std::os::raw::c_char;
use std::path::{Path, PathBuf};
use crate::dialogues::filebox::{FileBox, Filter};

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
fn filter_to_str(filter: Vec<Filter>) -> String {
    let mut string: String = String::new();
    for fil in filter {
        string.push_str(&fil.title);
        string.push_str("\0");
        string.push_str(&fil.file_ending);
        string.push_str("\0");
    }

    return string;
}

// https://docs.microsoft.com/en-us/windows/win32/api/commdlg/nf-commdlg-getopenfilenamea
pub fn open_file_dialogue(file_box: FileBox) -> Option<PathBuf> {
    use core::mem;
    use winapi::um::commdlg::{GetOpenFileNameA, OPENFILENAMEA, OFN_PATHMUSTEXIST, OFN_FILEMUSTEXIST};

    let mut my_str: [i8; 100] = [0; 100];
    my_str[0] = '\0' as i8;

    let filt: String = filter_to_str(file_box.filters);

    // https://docs.microsoft.com/en-us/windows/win32/api/commdlg/ns-commdlg-openfilenamea
    let mut open_file: OPENFILENAMEA = OPENFILENAMEA::default();
    open_file.lStructSize = mem::size_of::<OPENFILENAMEA>() as u32;
    open_file.hwndOwner = core::ptr::null_mut();
    open_file.lpstrFile = my_str.as_ptr() as *mut i8;
    open_file.nMaxFile = my_str.len() as u32;
    open_file.lpstrFilter = str_to_c_str(filt.as_str());
    open_file.nFilterIndex = 1;
    open_file.lpstrFileTitle = core::ptr::null_mut();
    open_file.nMaxFileTitle = 0;
    open_file.lpstrInitialDir = match file_box.directory.is_some() {
        true => str_to_c_str(file_box.directory.unwrap().to_str().unwrap()),
        false => core::ptr::null_mut(),
    };
    open_file.Flags = OFN_PATHMUSTEXIST | OFN_FILEMUSTEXIST;

    let open_file_ptr: *mut OPENFILENAMEA = &mut open_file;

    unsafe {
        if GetOpenFileNameA(open_file_ptr) == 0 {
            return None;
        }
    }
    let slice = unsafe { std::slice::from_raw_parts(open_file.lpstrFile, 100) };

    return Some(PathBuf::from(slice_to_string(slice)));
}
// https://docs.microsoft.com/en-us/windows/win32/api/commdlg/nf-commdlg-getsavefilenamea
pub fn save_file_dialogue_filter(file_box: FileBox, suggested_name: &str) -> Option<PathBuf> {
    use core::mem;
    use winapi::um::commdlg::{GetSaveFileNameA, OPENFILENAMEA, OFN_PATHMUSTEXIST, OFN_FILEMUSTEXIST};

    let mut my_str: [i8; 100] = [0; 100];
    my_str[0] = '\0' as i8;

    let filt: String = filter_to_str(file_box.filters);

    // https://docs.microsoft.com/en-us/windows/win32/api/commdlg/ns-commdlg-openfilenamea
    let mut open_file: OPENFILENAMEA = OPENFILENAMEA::default();
    open_file.lStructSize = mem::size_of::<OPENFILENAMEA>() as u32;
    open_file.hwndOwner = core::ptr::null_mut();
    open_file.lpstrFile = str_to_c_str(format!("{}\0", suggested_name).as_str()) as *mut i8;
    open_file.nMaxFile = my_str.len() as u32;
    open_file.lpstrFilter = str_to_c_str(filt.as_str());
    open_file.nFilterIndex = 1;
    open_file.lpstrFileTitle = core::ptr::null_mut();
    open_file.nMaxFileTitle = 0;
    open_file.lpstrInitialDir = match file_box.directory.is_some() {
        true => str_to_c_str(file_box.directory.unwrap().to_str().unwrap()),
        false => core::ptr::null_mut(),
    };
    open_file.Flags = OFN_PATHMUSTEXIST | OFN_FILEMUSTEXIST;

    let open_file_ptr: *mut OPENFILENAMEA = &mut open_file;

    unsafe {
        if GetSaveFileNameA(open_file_ptr) == 0 {
            return None;
        }
    }
    let slice = unsafe { std::slice::from_raw_parts(open_file.lpstrFile, 100) };

    return Some(PathBuf::from(slice_to_string(slice)));
}