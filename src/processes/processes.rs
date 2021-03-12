#[cfg(windows)]
extern crate winapi;

use core::mem;

/**
   Find a process by its name.
   (Note: The name is system dependent. Ex: Windows uses .exe at the end).

   It is also important to note that ids are assigned by the Operating System.
   Operating Systems, like Windows, may reuse process ids.

   # Params
   process_name -> The name of the process to find the id for.
   # Returns
   Result<u32> -> The result containg the id of the process.
   # Examples
   ```rust
   use system_extensions::processes::processes;
   let pid : u32 = processes::find_process_id("chrome.exe").unwrap();
   ```
*/
#[cfg(windows)]
pub fn find_process_id(process_name: &str) -> Result<u32, String> {
    use winapi::um::winnt;
    use winapi::um::tlhelp32;
    use winapi::um::winuser::WM_NULL;
    use self::winapi::um::handleapi::CloseHandle;
    use self::winapi::um::tlhelp32::{Process32Next, PROCESSENTRY32};

    unsafe {
        let mut process_info: tlhelp32::PROCESSENTRY32 = tlhelp32::PROCESSENTRY32::default();
        process_info.dwSize = mem::size_of::<tlhelp32::PROCESSENTRY32>() as u32;

        let processes_snapshot: winnt::HANDLE = tlhelp32::CreateToolhelp32Snapshot(tlhelp32::TH32CS_SNAPPROCESS, WM_NULL);

        if processes_snapshot == winapi::um::handleapi::INVALID_HANDLE_VALUE {
            return Err("Internal Error: Unable to process Windows handle.".to_string());
        }

        let process_info_ptr: *mut PROCESSENTRY32 = &mut process_info;

        tlhelp32::Process32First(processes_snapshot, process_info_ptr);
        let temp_exe_file = String::from_utf8(mem::transmute::<Vec<i8>, Vec<u8>>(remove_zeros(process_info.szExeFile.to_vec()))).unwrap();
        let exe_file = temp_exe_file.split_whitespace().next().unwrap();
        if process_name == exe_file {
            CloseHandle(processes_snapshot);
            return Ok(process_info.th32ProcessID);
        }

        while Process32Next(processes_snapshot, process_info_ptr) != 0 {
            let temp_exe_file = String::from_utf8(mem::transmute::<Vec<i8>, Vec<u8>>(remove_zeros(process_info.szExeFile.to_vec()))).unwrap();
            let exe_file = temp_exe_file.split(" ").next().unwrap();
            if process_name == exe_file {
                CloseHandle(processes_snapshot);
                return Ok(process_info.th32ProcessID);
            }
        }

        CloseHandle(processes_snapshot);
        return Err(format!("Cannot find process with name {}.", process_name));
    }

    /// Cut off unused indices in the szExeFile array.
    fn remove_zeros(vec: Vec<i8>) -> Vec<i8> {
        let mut output: Vec<i8> = Vec::new();
        for mt in vec {
            if mt != 0 {
                output.push(mt);
            } else {
                return output;
            }
        }
        return output;
    }
}

/**
*    Check if a process is running by its id.
*    # Params
*    process_id -> The process id to find. (Note: process ids can be recycled.)
*    # Returns
*    bool -> If the process is running.
*    # Examples
*    ```rust
*    use system_extensions::processes::processes;
*    let is_running : bool = processes::is_process_running(55555 as &u32);
*    ```
*/
#[cfg(windows)]
pub fn is_process_running(process_id: &u32) -> bool {
    use winapi::um::winnt;
    use winapi::um;
    use self::winapi::um::winnt::SYNCHRONIZE;
    use self::winapi::shared::minwindef::FALSE;
    use self::winapi::um::handleapi::CloseHandle;
    use self::winapi::shared::winerror::WAIT_TIMEOUT;

    unsafe {
        let process: winnt::HANDLE = um::processthreadsapi::OpenProcess(SYNCHRONIZE, FALSE, *process_id);
        let ret = um::synchapi::WaitForSingleObject(process, 0);
        CloseHandle(process);
        return ret == WAIT_TIMEOUT;
    }
}

#[cfg(unix)]
pub fn is_process_running(process_id: &u32) -> bool {
    let mut result: bool = false;
    let file = Path::new("/proc").join(process_id.to_string()).join("cmdline");
    if file.exists() {
        result = true;
    }
    result
}

#[cfg(unix)]
pub fn find_process_id(process_name: &str) -> Result<u32, String> {
    let paths = fs::read_dir("/proc/").unwrap();
    let mut result: u32 = 0;
    for path in paths {
        let entry = path.unwrap();
        let file = Path::new(entry.path().as_path()).join("cmdline");
        if file.exists() {
            let contents = fs::read_to_string(file).expect("Something went wrong reading the file");
            if contents.contains(process_name) {
                let string = entry.path().as_path().to_str().unwrap().replace("/proc/", "");
                result = string.parse().unwrap()
            }
        }
    }
    if result == 0 {
        return Err(format!("Cannot find process with name {}.", process_name));
    }

    Ok(result)
}