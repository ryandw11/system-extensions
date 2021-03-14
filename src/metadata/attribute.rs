#[cfg(windows)]
extern crate winapi;

use std::ffi::CString;
use std::path::Path;



bitflags! {
    /**
       Attributes are bitwise flags that tell the system
       what attributes to apply to a file.

**Note:** Attributes are very OS specific. Each attribute will have a
       different value depending on the OS. Some attributes might have no use
       on certain operating systems.
    */
    pub struct Attributes: u32 {
        /**
            Represents a normal file.
        */
        const NORMAL = 128;

        /**
            Represents a hidden file.
            (Windows Only. Put a '.' in front of a file on Linux.)
        */
        const HIDDEN = 2;

        /**
            Represents a file set to Read Only.
        */
        const READ_ONLY = 1;
    }
}

/**
    A vector of all valid attributes.
*/
// This exists so you can loop through attributes.
const ATTRIBUTES: [Attributes; 3] = [Attributes::NORMAL, Attributes::HIDDEN, Attributes::READ_ONLY];

/**
    Set the attributes for a file.

    This is very OS dependent and some attributes might do nothing on some operating systems.

   # Params
   file: Path -> The path to the file.<br>
   attrib: [`Attributes`] -> The attributes to add. (Bit-wise OR can be used to set multiple attributes.)
   # Returns
   bool -> If the attributes were set successfully. (If not check to make sure the file path exists.)

   # Example
   ```rust
   use std::path::Path;
   use system_extensions::metadata::attribute::{set_attribute, Attributes};

   set_attribute(Path::new("/test.txt"), Attributes::HIDDEN);
   ```
   Using more than one attribute:
   ```rust
   use std::path::Path;
   use system_extensions::metadata::attribute::{set_attribute, Attributes};

   set_attribute(Path::new("/test.txt"), Attributes::HIDDEN | Attributes::READ_ONLY);
   ```
*/
#[cfg(windows)]
pub fn set_attribute(file: &Path, attrib: Attributes) -> bool {
    use self::winapi::um::fileapi::SetFileAttributesA;

    unsafe {
        let file_string: CString = CString::new(file.to_str().unwrap()).unwrap();
        let success = SetFileAttributesA(file_string.as_ptr(), attrib.bits);

        return success != 0;
    }
}

/**
   Check if a file has a certain attribute.

   ## Params
   file: &Path -> The path to the file. <br>
   attrib: [`Attributes`] -> The attribute to check for.

   ## Returns
   If the file has the specified attribute.

   ## Examples
   ```rust
   use std::path::Path;
   use system_extensions::metadata::attribute::{has_attribute, Attributes};

   has_attribute(Path::new("/test.txt"), Attributes::HIDDEN);
   ```
*/
#[cfg(windows)]
pub fn has_attribute(file: &Path, attrib: Attributes) -> bool {
    use self::winapi::um::fileapi::GetFileAttributesA;
    use self::winapi::um::fileapi::INVALID_FILE_ATTRIBUTES;

    unsafe {
        let file_string: CString = CString::new(file.to_str().unwrap()).unwrap();
        let bits = GetFileAttributesA(file_string.as_ptr());

        if bits == INVALID_FILE_ATTRIBUTES {
            return false;
        }

        return bits & attrib.bits == attrib.bits;
    }
}

/**
   Get a list of attributes a file has.

   ## Params
   file: &Path -> The path to the file.

   ## Returns
   The result of a vector of the attributes.

   ## Examples
   ```rust
   use std::path::Path;
   use system_extensions::metadata::attribute::{get_attributes, Attributes};

   attribs: Vec<Attributes> = get_attributes(Path::new("/test.txt")).unwrap();
   ```
*/
#[cfg(windows)]
pub fn get_attributes(file: &Path) -> Result<Vec<Attributes>, String> {
    use self::winapi::um::fileapi::GetFileAttributesA;
    use self::winapi::um::fileapi::INVALID_FILE_ATTRIBUTES;

    let mut output: Vec<Attributes> = Vec::new();

    unsafe {
        let file_string: CString = CString::new(file.to_str().unwrap()).unwrap();
        let bits = GetFileAttributesA(file_string.as_ptr());

        if bits == INVALID_FILE_ATTRIBUTES {
            return Err(String::from("Windows.h Error: Invalid file attributes. Use obtain_errors() for a detailed error."));
        }

        for att in ATTRIBUTES.iter() {
            if bits & att.bits == att.bits {
                output.push(*att);
            }
        }

        return Ok(output);
    }
}

/*

    Linux Section

 */
/**
    Set the attributes for a file.

    This is very OS dependent and some attributes might do nothing on some operating systems.

   # Params
   file: Path -> The path to the file. <br>
   attrib: [`Attributes`] -> The attributes to add. (Bit-wise OR can be used to set multiple attributes.)
   # Returns
   bool -> If the attributes were set successfully. (If not check to make sure the file path exists.)

   # Example
   ```rust
   use std::path::Path;
   use system_extensions::metadata::attribute::{set_attribute, Attributes};

   set_attribute(Path::new("/test.txt"), Attributes::HIDDEN);
   ```
   Using more than one attribute:
   ```rust
   use std::path::Path;
   use system_extensions::metadata::attribute::{set_attribute, Attributes};

   set_attribute(Path::new("/test.txt"), Attributes::HIDDEN | Attributes::READ_ONLY);
   ```
*/
#[cfg(unix)]
pub fn set_attribute(path: &Path, attrib: Attributes) -> bool {
    use std::fs;
    use std::fs::set_permissions;
    use std::fs::File;

    let bits = attrib.bits;

    if &bits & Attributes::HIDDEN.bits == Attributes::HIDDEN.bits {
        fs::rename(path, format!(".{}", path.to_str().unwrap())).is_ok();
    } else if &bits & Attributes::READ_ONLY.bits == Attributes::READ_ONLY.bits{
        let result = File::create(path);
        if result.is_err() {
            return false;
        }
        let file = result.unwrap();
        let result_meta = file.metadata();
        if result_meta.is_err() {
            return false;
        }
        let meta = result_meta.unwrap();
        let mut perms = meta.permissions();
        perms.set_readonly(true);
        return set_permissions(&path, perms).is_ok();
    }
    true
}

/**
   Check if a file has a certain attribute.

   ## Params
   file: &Path -> The path to the file. <br>
   attrib: [`Attributes`] -> The attribute to check for.

   ## Returns
   If the file has the specified attribute.

   ## Examples
   ```rust
   use std::path::Path;
   use system_extensions::metadata::attribute::{has_attribute, Attributes};

   has_attribute(Path::new("/test.txt"), Attributes::HIDDEN);
   ```
*/
#[cfg(unix)]
pub fn has_attribute(file: &Path, attrib: Attributes) -> bool {
    use std::fs::File;
    if attrib ==Attributes::HIDDEN {
        let option = file.file_name();
        if option.is_none(){
            return false;
        }
        let option = option.unwrap();
        let str = option.to_str();
        if str.is_none(){
            return false;
        }
        str.unwrap().starts_with(".")
    } else if attrib == Attributes::READ_ONLY{
        let result = File::open(file);
        if result.is_err() {
            return false;
        }
        let file = result.unwrap();
        let result_meta = file.metadata();
        if result_meta.is_err() {
            return false;
        }
        let meta = result_meta.unwrap();
        meta.permissions().readonly()
    }else {
        false
    }
}

/**
   Get a list of attributes a file has.

   ## Params
   file: &Path -> The path to the file.

   ## Returns
   The result of a vector of the attributes.

   ## Examples
   ```rust
   use std::path::Path;
   use system_extensions::metadata::attribute::{get_attributes, Attributes};

   attribs: Vec<Attributes> = get_attributes(Path::new("/test.txt")).unwrap();
   ```
*/
#[cfg(unix)]
pub fn get_attributes(file: &Path) -> Result<Vec<Attributes>, String> {
    let mut values: Vec<Attributes> = Vec::new();
    if has_attribute(file, Attributes::READ_ONLY){
        values.push(Attributes::READ_ONLY);
    }
    if has_attribute(file, Attributes::HIDDEN){
        values.push(Attributes::HIDDEN);
    }

    Ok(values)
}