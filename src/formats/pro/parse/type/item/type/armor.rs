use super::super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<object::item::armor::Instance, errors::Error> {
    let mut armor_ac_bytes = [0u8; 4];
    match source.read_exact(&mut armor_ac_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let armor_ac = u32::from_be_bytes(armor_ac_bytes);

    let mut armor_dr_normal_bytes = [0u8; 4];
    match source.read_exact(&mut armor_dr_normal_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let armor_dr_normal = u32::from_be_bytes(armor_dr_normal_bytes);

    let mut armor_dr_laser_bytes = [0u8; 4];
    match source.read_exact(&mut armor_dr_laser_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let armor_dr_laser = u32::from_be_bytes(armor_dr_laser_bytes);

    let mut armor_dr_fire_bytes = [0u8; 4];
    match source.read_exact(&mut armor_dr_fire_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let armor_dr_fire = u32::from_be_bytes(armor_dr_fire_bytes);

    let mut armor_dr_plasma_bytes = [0u8; 4];
    match source.read_exact(&mut armor_dr_plasma_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let armor_dr_plasma = u32::from_be_bytes(armor_dr_plasma_bytes);

    let mut armor_dr_electrical_bytes = [0u8; 4];
    match source.read_exact(&mut armor_dr_electrical_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let armor_dr_electrical = u32::from_be_bytes(armor_dr_electrical_bytes);

    let mut armor_dr_emp_bytes = [0u8; 4];
    match source.read_exact(&mut armor_dr_emp_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let armor_dr_emp = u32::from_be_bytes(armor_dr_emp_bytes);

    let mut armor_dr_explosive_bytes = [0u8; 4];
    match source.read_exact(&mut armor_dr_explosive_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let armor_dr_explosive = u32::from_be_bytes(armor_dr_explosive_bytes);

    let mut armor_dt_normal_bytes = [0u8; 4];
    match source.read_exact(&mut armor_dt_normal_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let armor_dt_normal = u32::from_be_bytes(armor_dt_normal_bytes);

    let mut armor_dt_laser_bytes = [0u8; 4];
    match source.read_exact(&mut armor_dt_laser_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let armor_dt_laser = u32::from_be_bytes(armor_dt_laser_bytes);

    let mut armor_dt_fire_bytes = [0u8; 4];
    match source.read_exact(&mut armor_dt_fire_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let armor_dt_fire = u32::from_be_bytes(armor_dt_fire_bytes);

    let mut armor_dt_plasma_bytes = [0u8; 4];
    match source.read_exact(&mut armor_dt_plasma_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let armor_dt_plasma = u32::from_be_bytes(armor_dt_plasma_bytes);

    let mut armor_dt_electrical_bytes = [0u8; 4];
    match source.read_exact(&mut armor_dt_electrical_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let armor_dt_electrical = u32::from_be_bytes(armor_dt_electrical_bytes);

    let mut armor_dt_emp_bytes = [0u8; 4];
    match source.read_exact(&mut armor_dt_emp_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let armor_dt_emp = u32::from_be_bytes(armor_dt_emp_bytes);

    let mut armor_dt_explosive_bytes = [0u8; 4];
    match source.read_exact(&mut armor_dt_explosive_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let armor_dt_explosive = u32::from_be_bytes(armor_dt_explosive_bytes);

    let mut armor_perk_bytes = [0u8; 4];
    match source.read_exact(&mut armor_perk_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let armor_perk = i32::from_be_bytes(armor_perk_bytes);

    let mut armor_male_fid_bytes = [0u8; 4];
    match source.read_exact(&mut armor_male_fid_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let mut armor_female_fid_bytes = [0u8; 4];
    match source.read_exact(&mut armor_female_fid_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    Ok(object::item::armor::Instance {
        class: armor_ac,
        threshold: HashMap::from([
            (object::common::combat::damage::Type::Default, armor_dt_normal),
            (object::common::combat::damage::Type::Laser, armor_dt_laser),
            (object::common::combat::damage::Type::Fire, armor_dt_fire),
            (object::common::combat::damage::Type::Plasma, armor_dt_plasma),
            (object::common::combat::damage::Type::Electrical, armor_dt_electrical),
            (object::common::combat::damage::Type::Emp, armor_dt_emp),
            (object::common::combat::damage::Type::Explosive, armor_dt_explosive),
        ]),
        resistance: HashMap::from([
            (object::common::combat::damage::Type::Default, armor_dr_normal),
            (object::common::combat::damage::Type::Laser, armor_dr_laser),
            (object::common::combat::damage::Type::Fire, armor_dr_fire),
            (object::common::combat::damage::Type::Plasma, armor_dr_plasma),
            (object::common::combat::damage::Type::Electrical, armor_dr_electrical),
            (object::common::combat::damage::Type::Emp, armor_dr_emp),
            (object::common::combat::damage::Type::Explosive, armor_dr_explosive),
        ]),
        perk: match armor_perk {
            -1 => Option::None,
            value => Option::Some(
                match object::common::critter::Perk::try_from(value) {
                    Ok(value) => value,
                    Err(_) =>
                        return Err(errors::Error::Format(errors::Format::Data))
                }
            ),
        },
        appearance: object::item::armor::Appearance {
            sprites: HashMap::from([
                (object::common::critter::Gender::Male,
                 match object::common::sprite::Reference::try_from(armor_male_fid_bytes) {
                     Ok(value) => value,
                     Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
                 }),
                (object::common::critter::Gender::Female,
                 match object::common::sprite::Reference::try_from(armor_female_fid_bytes) {
                     Ok(value) => value,
                     Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
                 })
            ])
        },
    })
}