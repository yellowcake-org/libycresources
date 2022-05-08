use libycresources::formats::map;

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

    println!("Tiles");
    println!();

    for (idx, elevation) in map.tiles.iter().enumerate() {
        match elevation {
            None => println!("Level {:?} has not presented.", idx),
            Some(e) => {
                println!("Level {:?}", idx);
                println!();

                println!("Floor");
                for h in e.floor {
                    for v in h {
                        match v {
                            None => print!("."),
                            Some(_) => print!("0"),
                        }
                    }

                    println!();
                }
                println!();

                println!("Roof");
                for h in e.roof {
                    for v in h {
                        match v {
                            None => print!("."),
                            Some(_) => print!("0"),
                        }
                    }

                    println!();
                }
                println!();
            }
        }
    }
}