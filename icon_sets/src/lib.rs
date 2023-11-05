mod icons;

pub use icons::ICON_SETS;

pub struct Icon {
	pub class: &'static str,
	pub view_box: &'static str,
	pub width: &'static str,
	pub height: &'static str,
	pub stroke: &'static str,
	pub fill: &'static str,
	pub stroke_width: &'static str,
	pub stroke_linecap: &'static str,
	pub stroke_linejoin: &'static str,
	pub nodes: Vec<&'static str>,
}
