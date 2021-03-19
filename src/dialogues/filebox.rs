use std::path::{Path, PathBuf};

/**
    This stores information for the filters.
*/
#[derive(Clone, Debug)]
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
   Represents a Save or Open File Dialog.<br><br>
   This is a builder struct, all functions return self, except the open or save method.

   # Examples
   Generic open file:
   ```rust
   use system_extensions::dialogues::filebox::FileBox;
   use std::path::Path;

   let result = FileBox::new().open();

   println!("{}", result.expect("The file was not opened!").to_str().unwrap());
   ```
   Generic save file:
   ```rust
   use system_extensions::dialogues::filebox::FileBox;
   use std::path::Path;

   let result = FileBox::new().save("example.txt");

   println!("{}", result.expect("The file was not saved!").to_str().unwrap());
   ```
   Filters:
   ```rust
   use system_extensions::dialogues::filebox::FileBox;
   use std::path::Path;

   let result = FileBox::new()
            .filter("PNG", "*.png")
            .filter("JPG", "*.jpg")
            .filter("GIF", "*.gif")
            .open();

   println!("{}", result.expect("The file was not opened!").to_str().unwrap());
   ```
   Save a file with a default directory:
   ```rust
   use system_extensions::dialogues::filebox::FileBox;
   use std::path::Path;

   let result = FileBox::new()
            .filter("Text", "*.txt")
            .directory(Path::new("D:\\"))
            .save("example.txt");

   println!("{}", result.expect("The file was not saved!").to_str().unwrap());
   ```
*/
#[derive(Clone, Debug)]
pub struct FileBox {
    pub(crate) filters: Vec<Filter>,
    pub(crate) directory: Option<&'static Path>,
}

// TODO:: This is probably implemented pretty poorly.
// TODO::       fix this.
impl FileBox {
    /**
        Create a new FileBox to open or save files.

        ## Default Values
        By default the Filter is set to a Vector with the all filter inside. ('All', '*.*')
        By default there is not default directory.
    */
    pub fn new() -> FileBox {
        FileBox {
            filters: vec![Filter::new("All".to_string(), "*.*".to_string())],
            directory: None,
        }
    }

    /**
        Clear the current filters. This is useful if you don't want the 'All' filter.
    */
    pub fn clear_filters(mut self) -> Self {
        self.filters.clear();
        self
    }

    /**
        Set the vector of filters.

        # Params
        filters: Vec<[`Filter`]> -> The vector of filters to be used. (Replaces any existing filters).
    */
    pub fn set_filters(mut self, filters: Vec<Filter>) -> Self {
        self.filters = filters;
        self
    }

    /**
        Add a filter to the file box. You may want to clear the filters first if
        you don't want the Any filter.

        # Params
        name: &str -> The name of the filter.<br>
        ending: &str -> The ending of the filter.<br>

        # Example
        ```rust
        use system_extensions::dialogues::filebox::FileBox;
        FileBox::new().filter("Text", "*.*").open();
        ```
    */
    pub fn filter(mut self, name: &str, ending: &str) -> Self {
        self.filters.push(Filter::new(name.to_string(), ending.to_string()));
        self
    }

    /**
        Set the default directory for the save or open dialog to display.
        <br>
        Not setting this causes the dialog to open the last displayed directory or
        the documents folder.
        # Params
        path: &`static Path -> The path of the directory to display.
    */
    pub fn directory(mut self, path: &'static Path) -> Self {
        self.directory = Some(path);
        self
    }

    /**
        Display the open file dialog.

        # Returns
        `Option<PathBuf>` -> The Path to the opened file. An empty Option means that the window was closed
        without opening anything.
    */
    pub fn open(self) -> Option<PathBuf> {
        use crate::internal::filebox::open_file_dialogue;
        open_file_dialogue(self)
    }

    /**
        Display the save file dialog.

        # Params
        suggested_name: &str -> The default name that is given when the dialog is displayed.

        # Returns
        `Option<PathBuf>` -> The path to the saved file. An empty Option means that the window was
        closed without saving anything.
    */
    pub fn save(self, suggested_name: &str) -> Option<PathBuf> {
        use crate::internal::filebox::save_file_dialogue_filter;
        save_file_dialogue_filter(self, suggested_name)
    }
}