pub(crate) fn misc(misc: &libycresources::formats::pro::object::item::misc::Instance) {
    println!("Item is Misc");
    println!("Power item ID: {:?}", misc.connections.power_item_id);
    println!("Caliber: {:?}", misc.caliber);
    println!("Count: {:?}", misc.count);
}