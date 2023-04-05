pub(crate) fn wall(wall: &ycresources::formats::pro::object::wall::Instance) {
    println!("Prototype is Wall");
    println!();

    println!("Light: {:?}", wall.light);
    println!("Script: {:?}", wall.script);
    println!("Actions: {:?}", wall.actions);
    println!("Material: {:?}", wall.material);
}