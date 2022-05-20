use libycresources::formats::map;

mod tiles;
mod scripts;
mod prototypes;

pub fn map(map: &map::Map) {
    println!("Map ID: {:?}", map.id);
    println!("Version: {:?}", map.version);
    println!("Name: {:?}", map.filename);
    println!();

    println!("Ticks: {:?}", map.ticks);
    println!("Flags: {:?}", map.flags);
    println!("Darkness: {:?}", map.darkness);
    println!();

    println!("Entrance");
    println!();
    println!("Position: {:?}", map.entrance.position);
    println!("Elevation: {:?}", map.entrance.elevation);
    println!("Orientation: {:?}", map.entrance.orientation);
    println!();

    println!("Variables");
    println!();

    println!("Local: {:?}", map.variables.local);
    println!("Global: {:?}", map.variables.global);
    println!();

    tiles::print(map);
    println!();

    scripts::print(map);
    println!();

    prototypes::print(map);
}