use std::collections::HashMap;
use std::time::Duration;

use crate::formats::pro::traits::TryFromOptional;

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

    let effect0_amount0_raw = i32::from_be_bytes(effect0_amount0_bytes);

    let mut effect0_amount1_bytes = [0u8; 4];
    match source.read_exact(&mut effect0_amount1_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let effect0_amount1_raw = i32::from_be_bytes(effect0_amount1_bytes);

    let mut effect0_amount2_bytes = [0u8; 4];
    match source.read_exact(&mut effect0_amount2_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let effect0_amount2_raw = i32::from_be_bytes(effect0_amount2_bytes);

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

    let effect1_amount0_raw = i32::from_be_bytes(effect1_amount0_bytes);

    let mut effect1_amount1_bytes = [0u8; 4];
    match source.read_exact(&mut effect1_amount1_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let effect1_amount1_raw = i32::from_be_bytes(effect1_amount1_bytes);

    let mut effect1_amount2_bytes = [0u8; 4];
    match source.read_exact(&mut effect1_amount2_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let effect1_amount2_raw = i32::from_be_bytes(effect1_amount2_bytes);

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

    let effect2_amount0_raw = i32::from_be_bytes(effect2_amount0_bytes);

    let mut effect2_amount1_bytes = [0u8; 4];
    match source.read_exact(&mut effect2_amount1_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let effect2_amount1_raw = i32::from_be_bytes(effect2_amount1_bytes);

    let mut effect2_amount2_bytes = [0u8; 4];
    match source.read_exact(&mut effect2_amount2_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let effect2_amount2_raw = i32::from_be_bytes(effect2_amount2_bytes);

    let mut effects = HashMap::new();

    match stat0_raw {
        -1 | -2 => {}
        value => {
            if value >= 0 {
                let statistic0 = match object::common::critter::Statistic::try_from(value) {
                    Ok(value) => value,
                    Err(error) => return Err(error)
                };

                let effect0 = object::item::drug::Effect {
                    delay: None,
                    impact: object::item::drug::Amount::Fixed(effect0_amount0_raw),
                };

                let effect1 = object::item::drug::Effect {
                    delay: if effect1_duration_raw > 0 {
                        Some(Duration::new(effect1_duration_raw as u64 * 60, 0))
                    } else { None },
                    impact: object::item::drug::Amount::Fixed(effect1_amount0_raw),
                };

                let effect2 = object::item::drug::Effect {
                    delay: if effect2_duration_raw > 0 {
                        Some(Duration::new(effect2_duration_raw as u64 * 60, 0))
                    } else { None },
                    impact: object::item::drug::Amount::Fixed(effect2_amount0_raw),
                };

                effects.insert(statistic0, [effect0, effect1, effect2]);
            } else {
                return Err(errors::Error::Format(errors::Format::Consistency));
            }
        }
    }

    match stat1_raw {
        -1 | -2 => {}
        value => {
            if value >= 0 {
                let prev = stat0_raw;
                let statistic1 = match object::common::critter::Statistic::try_from(value) {
                    Ok(value) => value,
                    Err(error) => return Err(error)
                };

                let effect0 = object::item::drug::Effect {
                    delay: None,
                    impact: if prev == -2 {
                        object::item::drug::Amount::Random(effect0_amount0_raw..=effect0_amount1_raw)
                    } else {
                        object::item::drug::Amount::Fixed(effect0_amount1_raw)
                    },
                };

                let effect1 = object::item::drug::Effect {
                    delay: if effect1_duration_raw > 0 {
                        Some(Duration::new(effect1_duration_raw as u64 * 60, 0))
                    } else { None },
                    impact: if prev == -2 {
                        object::item::drug::Amount::Random(effect1_amount0_raw..=effect1_amount1_raw)
                    } else {
                        object::item::drug::Amount::Fixed(effect1_amount1_raw)
                    },
                };

                let effect2 = object::item::drug::Effect {
                    delay: if effect2_duration_raw > 0 {
                        Some(Duration::new(effect2_duration_raw as u64 * 60, 0))
                    } else { None },
                    impact: if prev == -2 {
                        object::item::drug::Amount::Random(effect2_amount0_raw..=effect2_amount1_raw)
                    } else {
                        object::item::drug::Amount::Fixed(effect2_amount1_raw)
                    },
                };

                effects.insert(statistic1, [effect0, effect1, effect2]);
            } else {
                return Err(errors::Error::Format(errors::Format::Consistency));
            }
        }
    }

    match stat2_raw {
        -1 | -2 => {}
        value => {
            if value >= 0 {
                let prev = stat1_raw;
                let statistic2 = match object::common::critter::Statistic::try_from(value) {
                    Ok(value) => value,
                    Err(error) => return Err(error)
                };

                let effect0 = object::item::drug::Effect {
                    delay: None,
                    impact: if prev == -2 {
                        object::item::drug::Amount::Random(effect0_amount1_raw..=effect0_amount2_raw)
                    } else {
                        object::item::drug::Amount::Fixed(effect0_amount2_raw)
                    },
                };

                let effect1 = object::item::drug::Effect {
                    delay: if effect1_duration_raw > 0 {
                        Some(Duration::new(effect1_duration_raw as u64 * 60, 0))
                    } else { None },
                    impact: if prev == -2 {
                        object::item::drug::Amount::Random(effect1_amount1_raw..=effect1_amount2_raw)
                    } else {
                        object::item::drug::Amount::Fixed(effect1_amount2_raw)
                    },
                };

                let effect2 = object::item::drug::Effect {
                    delay: if effect2_duration_raw > 0 {
                        Some(Duration::new(effect2_duration_raw as u64 * 60, 0))
                    } else { None },
                    impact: if prev == -2 {
                        object::item::drug::Amount::Random(effect2_amount1_raw..=effect2_amount2_raw)
                    } else {
                        object::item::drug::Amount::Fixed(effect2_amount2_raw)
                    },
                };

                effects.insert(statistic2, [effect0, effect1, effect2]);
            } else {
                return Err(errors::Error::Format(errors::Format::Consistency));
            }
        }
    }

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

    let addiction_perk = match object::common::critter::Perk::
    try_from_optional(addiction_perk_raw, -1) {
        Ok(value) => value,
        Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
    };

    Ok(object::item::drug::Instance {
        effects,
        addiction: match addiction_perk {
            None => None,
            Some(perk) => Some(object::item::drug::Addiction {
                perk,
                delay: Duration::new(
                    addiction_delay_raw as u64 * 60, 0,
                ),
                chance: ScaledValue {
                    value: addiction_rate_raw as u8,
                    scale: 0u8..=100u8,
                },
            })
        },
    })
}