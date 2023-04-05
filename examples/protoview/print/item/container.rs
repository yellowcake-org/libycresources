pub(crate) fn container(container: &ycresources::formats::pro::object::item::container::Instance) {
    println!("Item is Container");
    println!("Size: {:?}", container.size);
    println!("Flags: {:?}", container.flags);
}