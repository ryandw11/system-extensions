/**
    Allows the API to get the bits of bitflags.
*/
pub(crate) trait Bitflagable<T> {
    fn get_bits(self) -> T;
}