use libycresources::formats::map;

pub(crate) fn print(map: &map::Map) {
    println!("Tiles");
    println!();

    for group in map.tiles.iter() {
        println!("Level {:?}", group.elevation.level.value);
        println!();

        println!("Floor: {:?} tiles", group.floor.len());
        println!("Ceiling: {:?} tiles", group.ceiling.len());
        println!();
    }
}