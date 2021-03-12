/**
    The main library.

*/
pub mod processes;
pub mod metadata;


#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::ops::Add;
    use std::path::Path;

    use crate::metadata::time::{FileTime, set_creation_date};
    use crate::processes::processes::find_process_id;

    #[test]
    fn it_works() {
        let val = find_process_id("Discord.exe").expect("An error occurred!");
        println!("{:?}", val);
        set_creation_date(Path::new("D:\\Rust\\system-extensions\\test.txt"), &FileTime::new(5, 12, 3030));
    }
}
