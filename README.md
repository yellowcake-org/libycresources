## libycresources

This repository is a part of [Yellowcake](https://github.com/yellowcake-org) project.

### What it does

The library intended to provide instruments for working with original Fallout™ resources, such as `.map`, `.pro`, `.dat` files, and many others, which were used in classic Fallout™ games. This repository also does include example tools, provided for each file format separately, using `examples` targets. Sometimes they are just print out available info, but very often they provide additional functionality, which goes beyond simple examples, like `mapview` tool.

### Why this is useful

It is useful in obvious way for reproducing original game, providing modern, safe and stable codebase to work with original games' resources. It also may serve as a documenation for those, who want to learn about original file formats. And finally, it's example tools could be a good starting point for modders of original games. In very first release it can't do much, though, but it's already better than original tools released by the publisher in what it actually allows to do. For instance, again, `mapview` can be used to create some arts from maps, or guides. In case of significant demand from the community, it might become something bigger, like full-featured map editor, so stay tuned.

### How to get started

Basically you just use Cargo and default Rust tooling to explore different modules and examples. Each file format is represented with different Rust module. It also has a couple of common modules, which provided basic data structures, needed for the game files' formats and abstractions. Examples' targets provide a view on intended way of using the library's functions.

### Where to get help

[GitHub Discussions](https://github.com/yellowcake-org/libycresources/discussions) is a good starting point. GitHub Issues are disabled at the moment, but will be enabled in case of significant activity from community. As for now, I am handling all issues internally, using additional software.

### Who maintains and contributes

Currently, only [@0xceed](https://github.com/0xceed) is working on the library. While help from the community will be _very_ needed in the future, this early in development I need to focus on creating good example of my vision, guidelines and many, many more.