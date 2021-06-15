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
		Packed { compressed: usize, plain: usize }, Plain(usize)
	}

	pub struct File {
		pub name: String,
		pub path: String,
		pub size: Size,

		pub(crate) offset: usize
	}
}