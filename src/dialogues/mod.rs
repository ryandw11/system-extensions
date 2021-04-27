/**
    Allows the creation of message boxes.
*/
#[cfg(any(feature="dialogues", test))]
pub mod messagebox;

/**
    A FileBox allows you to Open or Save files.
    (Mac currently not supported)
*/
#[cfg(any(feature="dialogues", test))]
pub mod filebox;