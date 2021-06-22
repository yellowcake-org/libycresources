pub mod dir {
	use std::string::String;

	pub struct Dir {
		pub names: Vec<String>,
		pub(crate) offset: u64
	}
}

pub mod file {
	use std::string::String;

	pub enum Size {
		Packed { compressed: u32, plain: u32 }, Plain(u32)
	}

	pub struct File {
		pub name: String,
		pub path: String,
		pub size: Size,

		pub(crate) offset: u32
	}
}