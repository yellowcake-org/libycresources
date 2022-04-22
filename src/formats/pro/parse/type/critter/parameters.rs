use super::super::super::*;

mod damage;
mod statistic;

pub(crate) fn instance<S: Read>(source: &mut S) ->
Result<object::critter::Parameters, errors::Error> {
    let statistics = match statistic::map(source) {
        Ok(value) => value,
        Err(error) => return Err(error)
    };

    let threshold = match damage::threshold(source) {
        Ok(value) => value,
        Err(error) => return Err(error)
    };

    let resistance = match damage::resistance(source) {
        Ok(value) => value,
        Err(error) => return Err(error)
    };

    let mut age_bytes = [0u8; 4];
    match source.read_exact(&mut age_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let age = ScaledValue { value: u32::from_be_bytes(age_bytes) as u8, scale: 1..100 };

    let mut gender_bytes = [0u8; 4];
    match source.read_exact(&mut gender_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let gender = match object::common::critter::Gender::
    try_from(u32::from_be_bytes(gender_bytes) as u8) {
        Ok(value) => value,
        Err(error) => return Err(error)
    };

    Ok(object::critter::Parameters {
        age,
        gender,
        threshold,
        resistance,
        statistics,
    })
}