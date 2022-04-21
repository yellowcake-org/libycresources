use std::collections::HashMap;
use std::time::Duration;

use super::super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<object::item::drug::Instance, errors::Error> {
    let mut stat0_bytes = [0u8; 4];
    match source.read_exact(&mut stat0_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let stat0_raw = i32::from_be_bytes(stat0_bytes);

    let mut stat1_bytes = [0u8; 4];
    match source.read_exact(&mut stat1_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let stat1_raw = i32::from_be_bytes(stat1_bytes);

    let mut stat2_bytes = [0u8; 4];
    match source.read_exact(&mut stat2_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let stat2_raw = i32::from_be_bytes(stat2_bytes);

    let mut effect0_amount0_bytes = [0u8; 4];
    match source.read_exact(&mut effect0_amount0_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let effect0_amount0_raw = u32::from_be_bytes(effect0_amount0_bytes);

    let mut effect0_amount1_bytes = [0u8; 4];
    match source.read_exact(&mut effect0_amount1_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let effect0_amount1_raw = u32::from_be_bytes(effect0_amount1_bytes);

    let mut effect0_amount2_bytes = [0u8; 4];
    match source.read_exact(&mut effect0_amount2_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let effect0_amount2_raw = u32::from_be_bytes(effect0_amount2_bytes);

    let mut effect1_duration_bytes = [0u8; 4];
    match source.read_exact(&mut effect1_duration_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let effect1_duration_raw = u32::from_be_bytes(effect1_duration_bytes);

    let mut effect1_amount0_bytes = [0u8; 4];
    match source.read_exact(&mut effect1_amount0_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let effect1_amount0_raw = u32::from_be_bytes(effect1_amount0_bytes);

    let mut effect1_amount1_bytes = [0u8; 4];
    match source.read_exact(&mut effect1_amount1_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let effect1_amount1_raw = u32::from_be_bytes(effect1_amount1_bytes);

    let mut effect1_amount2_bytes = [0u8; 4];
    match source.read_exact(&mut effect1_amount2_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let effect1_amount2_raw = u32::from_be_bytes(effect1_amount2_bytes);

    let mut effect2_duration_bytes = [0u8; 4];
    match source.read_exact(&mut effect2_duration_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let effect2_duration_raw = u32::from_be_bytes(effect2_duration_bytes);

    let mut effect2_amount0_bytes = [0u8; 4];
    match source.read_exact(&mut effect2_amount0_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let effect2_amount0_raw = u32::from_be_bytes(effect2_amount0_bytes);

    let mut effect2_amount1_bytes = [0u8; 4];
    match source.read_exact(&mut effect2_amount1_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let effect2_amount1_raw = u32::from_be_bytes(effect2_amount1_bytes);

    let mut effect2_amount2_bytes = [0u8; 4];
    match source.read_exact(&mut effect2_amount2_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let effect2_amount2_raw = u32::from_be_bytes(effect2_amount2_bytes);

    let mut addiction_rate_bytes = [0u8; 4];
    match source.read_exact(&mut addiction_rate_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let addiction_rate_raw = u32::from_be_bytes(addiction_rate_bytes);

    let mut addiction_perk_bytes = [0u8; 4];
    match source.read_exact(&mut addiction_perk_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let addiction_perk_raw = i32::from_be_bytes(addiction_perk_bytes);

    let mut addiction_delay_bytes = [0u8; 4];
    match source.read_exact(&mut addiction_delay_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let addiction_delay_raw = u32::from_be_bytes(addiction_delay_bytes);

    let statistic0 = match stat0_raw > 0 {
        false => Option::None,
        true => Option::Some(
            match
            object::common::critter::Statistic::try_from(stat0_raw) {
                Ok(value) => value,
                Err(_) =>
                    return Err(errors::Error::Format(errors::Format::Data))
            }
        )
    };

    let statistic1 = match object::common::critter::Statistic::try_from(stat1_raw) {
        Ok(value) => value,
        Err(_) => return Err(errors::Error::Format(errors::Format::Data))
    };

    let statistic2 = match object::common::critter::Statistic::try_from(stat2_raw) {
        Ok(value) => value,
        Err(_) => return Err(errors::Error::Format(errors::Format::Data))
    };

    let mut impact0 = HashMap::from([
        (statistic1.clone(), match statistic0 {
            Some(_) => object::item::drug::Amount::Fixed(effect0_amount1_raw),
            None => object::item::drug::Amount::Random(
                effect0_amount0_raw..=effect0_amount1_raw
            ),
        }),
        (statistic2.clone(), object::item::drug::Amount::Fixed(effect0_amount2_raw))
    ]);

    if let Some(value) = statistic0.clone() {
        impact0.entry(value.clone())
            .or_insert(object::item::drug::Amount::Fixed(effect0_amount0_raw));
    }

    let effect0 = object::item::drug::Effect { delay: None, impact: impact0 };

    let mut impact1 = HashMap::from([
        (statistic1.clone(), match statistic0 {
            Some(_) => object::item::drug::Amount::Fixed(effect1_amount1_raw),
            None => object::item::drug::Amount::Random(
                effect1_amount0_raw..=effect1_amount1_raw
            ),
        }),
        (statistic2.clone(), object::item::drug::Amount::Fixed(effect1_amount2_raw))
    ]);

    if let Some(value) = statistic0.clone() {
        impact1.entry(value.clone())
            .or_insert(object::item::drug::Amount::Fixed(effect1_amount0_raw));
    }

    let effect1 = object::item::drug::Effect {
        delay: Some(Duration::new(effect1_duration_raw as u64 * 60, 0)),
        impact: impact1,
    };

    let mut impact2 = HashMap::from([
        (statistic1.clone(), match statistic0 {
            Some(_) => object::item::drug::Amount::Fixed(effect2_amount1_raw),
            None => object::item::drug::Amount::Random(
                effect2_amount0_raw..=effect2_amount1_raw
            ),
        }),
        (statistic2.clone(), object::item::drug::Amount::Fixed(effect2_amount2_raw))
    ]);

    if let Some(value) = statistic0.clone() {
        impact2.entry(value.clone())
            .or_insert(object::item::drug::Amount::Fixed(effect2_amount0_raw));
    }

    let effect2 = object::item::drug::Effect {
        delay: Some(Duration::new(effect2_duration_raw as u64 * 60, 0)),
        impact: impact2,
    };

    if effect2_duration_raw <= effect1_duration_raw {
        return Err(errors::Error::Format(errors::Format::Consistency));
    }

    let addiction_perk = match addiction_perk_raw {
        -1 => None,
        value => Some(
            match object::common::critter::Perk::
            try_from(value) {
                Ok(value) => value,
                Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
            }
        )
    };

    Ok(object::item::drug::Instance {
        effects: [effect0, effect1, effect2],
        addiction: match addiction_perk {
            None => None,
            Some(perk) => Some(object::item::drug::Addiction {
                perk,
                delay: Duration::new(
                    addiction_delay_raw as u64 * 60, 0,
                ),
                chance: ScaledValue {
                    value: addiction_rate_raw as u8,
                    scale: 0u8..101u8,
                },
            })
        },
    })
}