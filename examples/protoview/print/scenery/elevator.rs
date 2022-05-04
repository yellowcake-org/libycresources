pub(crate) fn elevator(elevator: &libycresources::formats::pro::object::scenery::elevator::Instance) {
    println!("Scenery is Elevator");
    println!();

    println!("Floor: {:?}", elevator.floor);
    println!("Type: {:?}", elevator.r#type);
}