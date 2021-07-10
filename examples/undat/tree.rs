use libycresources::dat;

pub(crate) fn print(tree: &dat::Directory) {
    let mut flag_path = Vec::new();

    for (depth, is_last, directory) in tree.iter() {
        if depth > flag_path.iter().count() {
            flag_path.push((is_last, &directory.name));
        } else {
            for _ in 0..flag_path.iter().count() - depth {
                flag_path.pop();
            }

            if flag_path.last() != Some(&(is_last, &directory.name)) {
                flag_path.push((is_last, &directory.name));
            }
        }

        for (index, tuple) in flag_path.iter().enumerate() {
            if index > 0 {
                if tuple.0 {
                    if index == depth {
                        print!("└");
                    } else {
                        print!(" ");
                    }
                } else {
                    if index == depth {
                        print!("├");
                    } else {
                        print!("│");
                    }
                }
                if index == depth {
                    print!("───");
                } else {
                    print!("   ");
                }
            }
        }

        println!("{:}", directory.name);

        for (index, file) in directory.files.iter().enumerate() {
            let is_last_file = index == directory.files.iter().count() - 1
                && directory.children.iter().count() == 0;

            for (index, tuple) in flag_path.iter().enumerate() {
                if index > 0 {
                    if tuple.0 {
                        print!(" ");
                    } else {
                        print!("│");
                    }
                    print!("   ");
                }
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
