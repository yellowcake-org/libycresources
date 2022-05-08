use libycresources::formats::map;

mod tiles;

pub fn map(map: &map::Map) {
    println!("Map ID: {:?}", map.id);
    println!("Version: {:?}", map.version);
    println!("Name: {:?}", map.filename);
    println!();

    println!("Ticks: {:?}", map.ticks);
    println!("Flags: {:?}", map.flags);
    println!("Darkness: {:?}", map.darkness);
    println!();

    println!("Player's Defaults");
    println!();
    println!("Position: {:?}", map.defaults.position);
    println!("Elevation: {:?}", map.defaults.elevation);
    println!("Orientation: {:?}", map.defaults.orientation);
    println!();

    println!("Variables");
    println!();

    println!("Local: {:?}", map.variables.local);
    println!("Global: {:?}", map.variables.global);
    println!();

    self::tiles::print(map);
}