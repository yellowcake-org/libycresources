use libycresources::formats::map;

pub(crate) fn print(map: &map::Map) {
    println!("Blueprints");
    println!();

    for blueprint in &map.scripts {
        println!("ID: {:?}", blueprint.id);
        println!("Type: {:?}", blueprint.r#type);
        println!("Variables: {:?}", blueprint.variables);
        println!("Connections: {:?}", blueprint.connections);
        println!();
    }
}