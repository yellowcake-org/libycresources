use libycresources::dat;

pub(crate) fn print(tree: &dat::Directory) {
    for (depth, is_last, directory) in tree.iter() {
        if depth > 0 {
            for _ in 0..depth - 1 {
                print!("    ");
            }

            if is_last {
                print!("└");
            } else {
                print!("├");
            }
            print!("───");
        }

        println!("{:}", directory.name);

        for (index, file) in directory.files.iter().enumerate() {
            let is_last_file = index == directory.files.iter().count() - 1
                && directory.children.iter().count() == 0;

            for _ in 0..depth {
                print!("    ");
            }

            if is_last_file {
                print!("└");
            } else {
                print!("├");
            }
            print!("───");
            println!("{:}", file.name);
        }
    }
}
