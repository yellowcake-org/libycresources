#[derive(Debug, Hash, Eq, PartialEq)]
pub struct ScaledValue<V, S> {
    pub value: V,
    pub scale: S,
}
