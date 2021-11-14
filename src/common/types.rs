pub struct ScaledValue<V, R> {
    pub value: V,
    pub scale: std::ops::Range<R>,
}
