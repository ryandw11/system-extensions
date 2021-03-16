# System Extensions
System Extensions is a cross-platform rust library that adds in additional functionality to manage opeartions of the operating system. 
System Extensions is split up into several modules that contain different functionality.
   
Add this to your `Cargo.toml` file to use it.
```toml
system-extensions = {version = "0.0.2", features = ["metadata", "processes"]}
```

[View the documentation here!](https://docs.rs/system-extensions/0.0.2/x86_64-pc-windows-msvc/system_extensions/all.html)

**Features**
- Processes
- Metadata

**Experimental Features**
- Dialogues

Experimental features are modules that might not support all platforms yet.
API in experimental features might change in the future.

# Features
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
This module allows you to modify the metadata of a file. 
## File Dates
You can change the creation, modified, and changed dates.
```rust
use std::path::Path;
use system_extensions::metadata::time::{FileTime, set_creation_date};

fn main() {
    set_creation_date(Path::new("./my_file.txt"), &FileTime::new(25, 12, 2021));
}
```
## File Attributes
You can also set the attributes of a file.
```rust
use std::path::Path;
use system_extensions::metadata::attribute::{Attributes, set_attribute};

fn main(){
    set_attribute(Path::new("./my_file.txt"), Attributes::HIDDEN);
}
```
Or check to see if a file has an attribute:
```rust
use std::path::Path;
use system_extensions::metadata::attribute::{Attributes, has_attribute};

fn main(){
    let value: bool = has_attribute(Path::new("./my_file.txt"), Attributes::HIDDEN);
}
```
## Dialogues
**Note: This is an Experimental Feature. Currently, this is Windows only.**  
Dialogues are GUI menus that function as user interaction.  
### MessageBox
Message boxes are GUI popup menus that displays information, warnings, or errors.
```rust
use system_extensions::dialogues::messagebox::{BoxReturn, BoxProperties, create_message_box};
fn main(){
    let result = create_message_box("Program Warning!", "Some kind of error occurred that needs a warning!", BoxProperties::ICON_WARNING | BoxProperties::OK_CANCEL);
    if result == BoxReturn::CANCEL {
        println!("The user canceled the message!");
    }
}
```

### FileBox
A FileBox is a box that allows the user to save or open files.
```rust
use system_extensions::dialogues::filebox::{Filter, open_select_file_menu_filter};
use std::path::PathBuf;

fn main(){
    let filter = vec![
        Filter::new("PNG File".to_string(), "*.png".to_string()),
        Filter::new("JPEG File".to_string(), "*.jpg".to_string())
    ];

    let result: PathBuf = open_select_file_menu_filter(filter);
}
```