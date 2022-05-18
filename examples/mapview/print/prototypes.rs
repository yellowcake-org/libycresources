use libycresources::formats::map;

pub(crate) fn print(map: &map::Map) {
    println!("Blueprints");
    println!();

    for prototype in &map.prototypes {
        println!("ID: {:?}", prototype.identifier);
        println!("Patch: {:?}", prototype.patch);
        println!("Inventory: {:?}", prototype.inventory);
        println!();
    }
}