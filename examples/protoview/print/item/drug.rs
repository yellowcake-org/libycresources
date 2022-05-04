pub(crate) fn drug(drug: &libycresources::formats::pro::object::item::drug::Instance) {
    println!("Item is Drug");
    println!("Effects: {:?}", drug.effects);
    println!("Addiction: {:?}", drug.addiction);
}