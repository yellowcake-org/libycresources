mod impls;

pub(crate) trait TryFromOptional<T>: TryFrom<T> {
    fn try_from_optional(value: T, none: T) -> Result<Option<Self>, Self::Error>;
}

impl<V, T> TryFromOptional<T> for V where V: TryFrom<T>, T: Eq {
    fn try_from_optional(value: T, none: T) -> Result<Option<Self>, Self::Error> {
        Ok(if value == none { None } else {
            Some(match Self::try_from(value) {
                Ok(value) => value,
                Err(error) => return Err(error)
            })
        })
    }
}