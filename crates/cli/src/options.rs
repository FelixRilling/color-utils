use crate::color_format::ColorFormat;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Options {
	pub verbosity: u8,
	pub format: ColorFormat,
}
