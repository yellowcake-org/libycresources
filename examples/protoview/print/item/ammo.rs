pub(crate) fn ammo(ammo: &libycresources::formats::pro::object::item::ammo::Instance) {
    println!("Item is Ammo");
    println!("Caliber: {:?}", ammo.caliber);
    println!("Count: {:?}", ammo.count);
    println!("Armor Class modifier: {:?}", ammo.adjustments.armor.class);
    println!("Damage Resistance modifier: {:?}", ammo.adjustments.armor.resistance);
    println!("Damage multiplier: {:?}", ammo.adjustments.damage.multiplier);
    println!("Damage divider: {:?}", ammo.adjustments.damage.divider);
}