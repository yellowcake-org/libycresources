use libycresources::formats::map;

pub fn map(map: &map::Map) {
    println!("Map ID: {:?}", map.id);
    println!("Version: {:?}", map.version);
    println!("Name: {:?}", map.filename);
    println!();

    println!("Flags: {:?}", map.flags);
    println!("Elevations: {:?}", map.elevations);
    println!();

    println!("Ticks: {:?}", map.ticks);
    println!("Darkness: {:?}", map.darkness);
    println!();

    println!("Player's Defaults");
    println!();
    println!("Position: {:?}", map.defaults.position);
    println!("Elevation: {:?}", map.defaults.elevation);
    println!("Orientation: {:?}", map.defaults.orientation);
}