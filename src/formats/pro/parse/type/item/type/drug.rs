use super::super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<object::item::drug::Instance, errors::Error> {
    let mut drug_stat0_bytes = [0u8; 4];
    match source.read_exact(&mut drug_stat0_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let drug_stat0_raw = i32::from_be_bytes(drug_stat0_bytes);

    let mut drug_stat1_bytes = [0u8; 4];
    match source.read_exact(&mut drug_stat1_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let drug_stat1_raw = i32::from_be_bytes(drug_stat1_bytes);

    let mut drug_stat2_bytes = [0u8; 4];
    match source.read_exact(&mut drug_stat2_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let drug_stat2_raw = i32::from_be_bytes(drug_stat2_bytes);

    let mut drug_effect0_amount0_bytes = [0u8; 4];
    match source.read_exact(&mut drug_effect0_amount0_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let drug_effect0_amount0_raw = u32::from_be_bytes(drug_effect0_amount0_bytes);

    let mut drug_effect0_amount1_bytes = [0u8; 4];
    match source.read_exact(&mut drug_effect0_amount1_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let drug_effect0_amount1_raw = u32::from_be_bytes(drug_effect0_amount1_bytes);

    let mut drug_effect0_amount2_bytes = [0u8; 4];
    match source.read_exact(&mut drug_effect0_amount2_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let drug_effect0_amount2_raw = u32::from_be_bytes(drug_effect0_amount2_bytes);

    let mut drug_effect1_duration_bytes = [0u8; 4];
    match source.read_exact(&mut drug_effect1_duration_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let drug_effect1_duration_raw = u32::from_be_bytes(drug_effect1_duration_bytes);

    let mut drug_effect1_amount0_bytes = [0u8; 4];
    match source.read_exact(&mut drug_effect1_amount0_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let drug_effect1_amount0_raw = u32::from_be_bytes(drug_effect1_amount0_bytes);

    let mut drug_effect1_amount1_bytes = [0u8; 4];
    match source.read_exact(&mut drug_effect1_amount1_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let drug_effect1_amount1_raw = u32::from_be_bytes(drug_effect1_amount1_bytes);

    let mut drug_effect1_amount2_bytes = [0u8; 4];
    match source.read_exact(&mut drug_effect1_amount2_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let drug_effect1_amount2_raw = u32::from_be_bytes(drug_effect1_amount2_bytes);

    let mut drug_effect2_duration_bytes = [0u8; 4];
    match source.read_exact(&mut drug_effect2_duration_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let drug_effect2_duration_raw = u32::from_be_bytes(drug_effect2_duration_bytes);

    let mut drug_effect2_amount0_bytes = [0u8; 4];
    match source.read_exact(&mut drug_effect2_amount0_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let drug_effect2_amount0_raw = u32::from_be_bytes(drug_effect2_amount0_bytes);

    let mut drug_effect2_amount1_bytes = [0u8; 4];
    match source.read_exact(&mut drug_effect2_amount1_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let drug_effect2_amount1_raw = u32::from_be_bytes(drug_effect2_amount1_bytes);

    let mut drug_effect2_amount2_bytes = [0u8; 4];
    match source.read_exact(&mut drug_effect2_amount2_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let drug_effect2_amount2_raw = u32::from_be_bytes(drug_effect2_amount2_bytes);

    let mut drug_addiction_rate_bytes = [0u8; 4];
    match source.read_exact(&mut drug_addiction_rate_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let drug_addiction_rate_raw = u32::from_be_bytes(drug_addiction_rate_bytes);

    let mut drug_addiction_perk_bytes = [0u8; 4];
    match source.read_exact(&mut drug_addiction_perk_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let drug_addiction_perk_raw = i32::from_be_bytes(drug_addiction_perk_bytes);

    let mut drug_addiction_delay_bytes = [0u8; 4];
    match source.read_exact(&mut drug_addiction_delay_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let drug_addiction_delay_raw = u32::from_be_bytes(drug_addiction_delay_bytes);

    let statistic0 =
        match drug_stat0_raw > 0 {
            false => Option::None,
            true => Option::Some(
                match
                object::common::critter::Statistic::try_from(drug_stat0_raw) {
                    Ok(value) => value,
                    Err(_) =>
                        return Err(errors::Error::Format(errors::Format::Data))
                }
            )
        };

    let statistic1 =
        match object::common::critter::Statistic::try_from(drug_stat1_raw) {
            Ok(value) => value,
            Err(_) => return Err(errors::Error::Format(errors::Format::Data))
        };

    let statistic2 =
        match object::common::critter::Statistic::try_from(drug_stat2_raw) {
            Ok(value) => value,
            Err(_) => return Err(errors::Error::Format(errors::Format::Data))
        };

    let mut impact0 = HashMap::from([
        (statistic1.clone(), match statistic0 {
            Some(_) => object::item::drug::Amount::Fixed(drug_effect0_amount1_raw),
            None => object::item::drug::Amount::Random(
                drug_effect0_amount0_raw..=drug_effect0_amount1_raw
            ),
        }),
        (statistic2.clone(), object::item::drug::Amount::Fixed(drug_effect0_amount2_raw))
    ]);

    if let Some(value) = statistic0.clone() {
        impact0.entry(value.clone()).or_insert(
            object::item::drug::Amount::Fixed(
                drug_effect0_amount0_raw
            ));
    }

    let effect0 = object::item::drug::Effect { delay: None, impact: impact0 };

    let mut impact1 = HashMap::from([
        (statistic1.clone(), match statistic0 {
            Some(_) => object::item::drug::Amount::Fixed(drug_effect1_amount1_raw),
            None => object::item::drug::Amount::Random(
                drug_effect1_amount0_raw..=drug_effect1_amount1_raw
            ),
        }),
        (statistic2.clone(), object::item::drug::Amount::Fixed(drug_effect1_amount2_raw))
    ]);

    if let Some(value) = statistic0.clone() {
        impact1.entry(value.clone()).or_insert(
            object::item::drug::Amount::Fixed(drug_effect1_amount0_raw)
        );
    }

    let effect1 = object::item::drug::Effect {
        delay: Some(Duration::new(drug_effect1_duration_raw as u64 * 60, 0)),
        impact: impact1,
    };

    let mut impact2 = HashMap::from([
        (statistic1.clone(), match statistic0 {
            Some(_) => object::item::drug::Amount::Fixed(drug_effect2_amount1_raw),
            None => object::item::drug::Amount::Random(
                drug_effect2_amount0_raw..=drug_effect2_amount1_raw
            ),
        }),
        (statistic2.clone(), object::item::drug::Amount::Fixed(drug_effect2_amount2_raw))
    ]);

    if let Some(value) = statistic0.clone() {
        impact2.entry(value.clone()).or_insert(
            object::item::drug::Amount::Fixed(drug_effect2_amount0_raw)
        );
    }

    let effect2 = object::item::drug::Effect {
        delay: Some(Duration::new(drug_effect2_duration_raw as u64 * 60, 0)),
        impact: impact2,
    };

    if drug_effect2_duration_raw <= drug_effect1_duration_raw {
        return Err(errors::Error::Format(errors::Format::Consistency));
    }

    let addiction_perk = match drug_addiction_perk_raw {
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
                    drug_addiction_delay_raw as u64 * 60, 0,
                ),
                chance: ScaledValue {
                    value: drug_addiction_rate_raw as u8,
                    scale: 0u8..101u8,
                },
            })
        },
    })
}