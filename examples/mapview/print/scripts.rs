use ycresources::formats::map;

pub(crate) fn print(map: &map::Map) {
    println!("Blueprints");
    println!();

    for script in &map.scripts {
        println!("ID: {:?}", script.id);
        println!("Type: {:?}", script.kind);
        println!("Variables: {:?}", script.variables);
        println!("Connections: {:?}", script.connections);
        println!();
    }
}