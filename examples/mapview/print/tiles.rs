use libycresources::formats::map;

pub(crate) fn tiles(map: &map::Map) {
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