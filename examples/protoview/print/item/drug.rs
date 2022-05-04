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
        None => println!("Drug is non-addictive"),
        Some(addiction) => {
            println!("Drug is addictive");
            println!();
            println!("Perk: {:?}", addiction.perk);
            println!("Delay: {:?}", addiction.delay);
            println!("Chance: {:?}", addiction.chance);
        }
    }
}