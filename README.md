# System Extensions
System Extensions is a cross-platform rust library that adds in additional functionality to manage opeartions of the operating system. 
System Extensions is split up into several modules that contain different functionality.
   
Add this to your `Cargo.toml` file to use it.
```toml
system-extensions = {version = "0.0.4", features = ["metadata", "processes", "dialogues"]}
```

[View the documentation here!](https://docs.rs/system-extensions/0.0.4/x86_64-pc-windows-msvc/system_extensions/all.html)

**Features**
- Processes
- Metadata
- Dialogues**

**Experimental Features**
- Notifications (Windows Only)

Experimental features are modules that might not support all platforms yet.
API in experimental features might change in the future.  
  
** Macos Not Supported

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
Dialogues are GUI menus that function as user interaction.  
### MessageBox
![Immage of a messagebox.](https://www.ryandw11.com/libraryContent/system-extensions/win_err.PNG)  
Message boxes are GUI popup menus that displays information, warnings, or errors.
```rust
use system_extensions::dialogues::messagebox::{MessageBox, BoxReturn, IconType, WindowType};
fn main(){
    let result = MessageBox::new("Error Message Dialogue", "This error is provided by System Extensions!")
        .set_icon_type(IconType::ICON_ERROR)
        .set_window_type(WindowType::OK_CANCEL)
        .show();

    if result.unwrap() == BoxReturn::OK {
        println!("The user acknowledge the error!");
    }
}
```

### FileBox
A FileBox is a box that allows the user to save or open files.  
(Mac is currently not supported.)
```rust
use system_extensions::dialogues::filebox::FileBox;
use std::path::Path;

fn main(){
    let result = FileBox::new()
        .filter("PNG", "*.png")
        .filter("JPG", "*.jpg")
        .filter("GIF", "*.gif")
        .directory(Path::new("D:\\"))
        .save("image.png");

    println!("{}", result.expect("The file was not saved!").to_str().unwrap());
}
```

### Notifications
** This features is experimental and only available for Windows. **
```rust
use system_extensions::notifications::notification::SimpleNotification;
fn main() {
    let notif = SimpleNotification::new("Rust Notification".to_string())
        .set_app_id("{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\\WindowsPowerShell\\v1.0\\powershell.exe".to_string())
        .add_text("This notification was sent via rust!".to_string())
        .add_text("This uses the Windows Notification Center.".to_string())
        .display();
}
```