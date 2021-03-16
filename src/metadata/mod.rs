/**
    Allows the editing of when a file was created, modified, and accessed.
*/
#[cfg(feature="metadata")]
pub mod time;
/**
    Allows the editing of file attribute data.
    This includes setting a file to hidden and read-only.
*/
#[cfg(feature="metadata")]
pub mod attribute;
#[cfg(test)]
pub mod time;
#[cfg(test)]
pub mod attribute;