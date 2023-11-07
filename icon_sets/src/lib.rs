mod icons;

pub use icons::ICON_SETS;

#[derive(Debug, Clone)]
pub struct Icon {
	pub view_box: &'static str,
	pub class: Option<&'static str>,
	pub width: Option<&'static str>,
	pub height: Option<&'static str>,
	pub stroke: Option<&'static str>,
	pub fill: Option<&'static str>,
	pub stroke_width: Option<&'static str>,
	pub stroke_linecap: Option<&'static str>,
	pub stroke_linejoin: Option<&'static str>,
	pub nodes: Vec<&'static str>,
}
