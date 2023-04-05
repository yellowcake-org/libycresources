pub(crate) fn critter(critter: &ycresources::formats::pro::object::critter::Instance) {
    println!("Prototype is Critter");
    println!();

    println!("Flags: {:?}", critter.flags);
    println!("Team: {:?}", critter.team);
    println!("Murder: {:?}", critter.murder);
    println!("Damage: {:?}", critter.damage);
    println!("Body: {:?}", critter.body);
    println!("Head: {:?}", critter.head);
    println!("Script: {:?}", critter.script);
    println!("AI packet ID: {:?}", critter.connections.ai_packet_id);
    println!();

    println!("Skills");
    println!();

    for skill in &critter.skills {
        println!("{:?} — {:?}", skill.0, skill.1);
    }

    println!();

    println!("Basic Statistics");
    println!();

    for statistic in &critter.statistics.basic {
        println!("{:?} — {:?}", statistic.0, statistic.1);
    }

    println!();

    println!("Bonuses to Statistics");
    println!();

    for statistic in &critter.statistics.bonuses {
        println!("{:?} — {:?}", statistic.0, statistic.1);
    }
}