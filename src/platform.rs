pub trait Reader<E> {
    fn read(self: &mut Self, range: std::ops::Range<usize>) -> Result<Vec<u8>, E>;
}

pub trait Writer<E> {
    fn append(self: &mut Self, bytes: &[u8]) -> Result<usize, E>;
}
