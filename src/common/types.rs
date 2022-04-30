#[derive(Debug)]
pub struct ScaledValue<V, S> {
    pub value: V,
    pub scale: S,
}
