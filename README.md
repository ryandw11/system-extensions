# System Extensions
System Extensions is a cross-platform rust library that adds in additional functionality to manage opeartions of the operating system. 
System Extensions is split up into several modules that contain different functionality.

**Modules**
- Processes
- Metadata

# Modules
## Processes
The processes module gives functionality to detect processes running on the operating system.
```rust
use system_extensions::processes::processes::find_process_id;
fn main() {
    let result = find_process_id("Discord.exe");
    println!("Program Id: {:?}", result.unwrap());
}
```

## Metadata
**Note:** Currently only implemented for Windows.  
This module allows you to modify the metadata of a file. 
## File Dates
You can change the creation, modified, and changed dates.
```rust
use std::path::Path;
use system_extensions::metadata::time::{FileTime, set_creation_date};

fn main() {
    set_creation_date(Path::new("C:\\my_file.txt"), &FileTime::new(25, 12, 2021));
}
```
## File Attributes
You can also set the attributes of a file.
```rust
use std::path::Path;
use system_extensions::metadata::attribute::{Attributes, set_attribute};

fn main(){
    set_attribute(Path::new("C:\\my_file.txt"), Attribute::HIDDEN);
}
```
Or check to see if a file has an attribute:
```rust
use std::path::Path;
use system_extensions::metadata::attribute::{Attributes, has_attribute};

fn main(){
    let value: bool = has_attribute(Path::new("C:\\my_file.txt"), Attribute::HIDDEN);
}
```