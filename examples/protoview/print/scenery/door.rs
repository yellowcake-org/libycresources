pub(crate) fn door(door: &ycresources::formats::pro::object::scenery::door::Instance) {
    println!("Scenery is Door");
    println!();

    println!("Flags: {:?}", door.flags);
    println!("Unknown: {:?}", door._unknown);
}