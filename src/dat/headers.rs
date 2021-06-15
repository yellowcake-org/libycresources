pub mod dir {
	use std::string::String;

	pub struct Dir {
		pub names: Vec<String>,
		pub(crate) offset: usize
	}
}

pub mod file {
	use std::string::String;

	pub enum Size {
		Packed(usize), Plain(usize)
	}

	pub struct File {
		pub name: String,
		pub compressed: bool,
		pub offset: usize,
		pub size: Size
	}
}