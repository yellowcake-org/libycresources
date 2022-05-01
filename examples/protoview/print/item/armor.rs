pub(crate) fn armor(armor: &libycresources::formats::pro::object::item::armor::Instance) {
    println!("Item is Armor");
    println!("Class: {:?}", armor.class);
    println!("Perk: {:?}", armor.perk);
    println!();
    println!("Damage Resistance");
    println!("{:?}", armor.resistance);
    println!("Damage Threshold");
    println!("{:?}", armor.threshold);
    println!("Appearance");
    println!("{:?}", armor.appearance.sprites);
}