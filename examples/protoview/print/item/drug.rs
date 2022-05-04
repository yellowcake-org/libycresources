pub(crate) fn drug(drug: &libycresources::formats::pro::object::item::drug::Instance) {
    println!("Item is Drug");
    println!();

    for effect in &drug.effects {
        println!("{:?}:", effect.0);

        for effect in effect.1 {
            println!("{:?}", effect)
        }
        println!();
    }

    match &drug.addiction {
        None => println!("Non-addictive"),
        Some(addiction) => {
            println!("Addictive: {:?}", addiction.perk);
            println!("Chance: {:?}", addiction.chance);
            println!("Delay: {:?}", addiction.delay);
        }
    }
}