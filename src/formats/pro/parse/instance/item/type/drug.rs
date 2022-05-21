use std::collections::HashMap;
use std::time::Duration;

use object::common::critter::{Perk, Statistic};
use object::item::drug::{Effect, Instance};

use crate::common::traits::TryFromOptional;
use crate::common::types::geometry::Scaled;

use super::super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<Instance, errors::Error> {
    let stat0_raw = source.read_i32::<BigEndian>()?;
    let stat1_raw = source.read_i32::<BigEndian>()?;
    let stat2_raw = source.read_i32::<BigEndian>()?;

    let effect0_amount0_raw = source.read_i32::<BigEndian>()?;
    let effect0_amount1_raw = source.read_i32::<BigEndian>()?;
    let effect0_amount2_raw = source.read_i32::<BigEndian>()?;

    let effect1_duration_raw = source.read_u32::<BigEndian>()?;

    let effect1_amount0_raw = source.read_i32::<BigEndian>()?;
    let effect1_amount1_raw = source.read_i32::<BigEndian>()?;
    let effect1_amount2_raw = source.read_i32::<BigEndian>()?;

    let effect2_duration_raw = source.read_u32::<BigEndian>()?;

    let effect2_amount0_raw = source.read_i32::<BigEndian>()?;
    let effect2_amount1_raw = source.read_i32::<BigEndian>()?;
    let effect2_amount2_raw = source.read_i32::<BigEndian>()?;

    let mut effects = HashMap::new();

    fn effect(duration: u32, amount: i32, prev_amount: Option<i32>) -> Effect {
        Effect {
            delay: if duration > 0 { Some(Duration::new(duration as u64 * 60, 0)) } else { None },
            impact: if let Some(prev_amount) = prev_amount {
                object::item::drug::Amount::Random(prev_amount..=amount)
            } else {
                object::item::drug::Amount::Fixed(amount)
            },
        }
    }

    match stat0_raw {
        -1 | -2 => {}
        value => {
            if value >= 0 {
                let statistic0 = Statistic::try_from(value)?;

                let effect0 = effect(0, effect0_amount0_raw, None);
                let effect1 = effect(effect1_duration_raw, effect1_amount0_raw, None);
                let effect2 = effect(effect2_duration_raw, effect2_amount0_raw, None);

                effects.insert(statistic0, [effect0, effect1, effect2]);
            } else {
                return Err(errors::Error::Format);
            }
        }
    }

    match stat1_raw {
        -1 | -2 => {}
        value => {
            if value >= 0 {
                let prev = stat0_raw;
                let statistic1 = match Statistic::try_from(value) {
                    Ok(value) => value,
                    Err(error) => return Err(error)
                };

                let effect0 = effect(0, effect0_amount1_raw, if prev == -2 {
                    Some(effect0_amount0_raw)
                } else { None });

                let effect1 = effect(effect1_duration_raw, effect1_amount1_raw, if prev == -2 {
                    Some(effect1_amount0_raw)
                } else { None });

                let effect2 = effect(effect2_duration_raw, effect2_amount1_raw, if prev == -2 {
                    Some(effect2_amount0_raw)
                } else { None });

                effects.insert(statistic1, [effect0, effect1, effect2]);
            } else {
                return Err(errors::Error::Format);
            }
        }
    }

    match stat2_raw {
        -1 | -2 => {}
        value => {
            if value >= 0 {
                let prev = stat1_raw;
                let statistic2 = Statistic::try_from(value)?;

                let effect0 = effect(0, effect0_amount2_raw, if prev == -2 {
                    Some(effect0_amount1_raw)
                } else { None });

                let effect1 = effect(effect1_duration_raw, effect1_amount2_raw, if prev == -2 {
                    Some(effect1_amount1_raw)
                } else { None });

                let effect2 = effect(effect2_duration_raw, effect2_amount2_raw, if prev == -2 {
                    Some(effect2_amount1_raw)
                } else { None });

                effects.insert(statistic2, [effect0, effect1, effect2]);
            } else {
                return Err(errors::Error::Format);
            }
        }
    }

    let addiction_rate_raw = source.read_u32::<BigEndian>()?;

    let addiction_perk = Perk::try_from_optional(source.read_i32::<BigEndian>()?, -1)?;
    let addiction_delay_raw = source.read_u32::<BigEndian>()?;

    Ok(Instance {
        effects,
        addiction: match addiction_perk {
            None => None,
            Some(perk) => Some(object::item::drug::Addiction {
                perk,
                delay: Duration::new(addiction_delay_raw as u64 * 60, 0),
                chance: Scaled {
                    value: addiction_rate_raw as u8,
                    scale: 0u8..=100u8,
                },
            })
        },
    })
}