/**
    Allows the creation of message boxes.
*/
#[cfg(any(feature="dialogues", test))]
pub mod messagebox;

/**
    A FileBox allows you to Open or Save files.
*/
#[cfg(any(feature="dialogues", test))]
pub mod filebox;