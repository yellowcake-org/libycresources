use libycresources::formats::map;

pub(crate) fn print(map: &map::Map) {
    println!("Blueprints");
    println!();

    for prototype in &map.prototypes {
        println!("ID: {:?}", prototype.id);
        println!("Patch: {:?}", prototype.patch);
        println!("Location: {:?}", prototype.location);
        println!("Appearance: {:?}", prototype.appearance);
        println!("Inventory: {:?}", prototype.inventory);
        println!();
    }
}