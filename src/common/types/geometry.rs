#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Scaled<V, S> {
    pub value: V,
    pub scale: S,
}
