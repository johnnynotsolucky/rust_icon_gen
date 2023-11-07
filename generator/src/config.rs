use crate::{kv_map, IconRepo};
use once_cell::sync::Lazy;

pub static ICON_REPOS: Lazy<Vec<IconRepo>> = Lazy::new(|| {
	vec![
		IconRepo {
			name: "ant-design-icons",
			icons_paths: kv_map!([
				"Outlined" => "packages/icons-svg/svg/outlined",
				"Filled" => "packages/icons-svg/svg/filled",
			]),
		},
		IconRepo::single_variant("bootstrap-icons", "icons"),
		IconRepo {
			name: "boxicons",
			icons_paths: kv_map!([
				"Logo" => "svg/logos",
				"Regular" => "svg/regular",
				"Solid" => "svg/solid",
			]),
		},
		IconRepo::single_variant("charm-icons", "icons"),
		IconRepo::single_variant("codicons", "src/icons"),
		IconRepo::single_variant("css.gg", "icons/svg"),
		IconRepo::single_variant("feather", "icons"),
		IconRepo {
			name: "font-awesome",
			icons_paths: kv_map!([
				"Brand" => "svgs/brands",
				"Regular" => "svgs/regular",
				"Solid" => "svgs/solid",
			]),
		},
		IconRepo {
			name: "heroicons",
			icons_paths: kv_map!([
				"Outline" => "optimized/24/outline",
				"Solid" => "optimized/24/solid",
				"Mini" => "optimized/20/solid",
			]),
		},
		// Ionicons actually has variants, but they're all included in the same dir with filled
		// variant being the "default" and the outlined and sharp variants having their respective
		// name postfixed.
		IconRepo::single_variant("ionicons", "src/svg"),
		IconRepo::single_variant("lucide", "icons"),
		IconRepo::single_variant("microns", "svg"),
		// Octicons have variants defined by sizes, although the only icons > 24px are the copilot icons.
		IconRepo::single_variant("octicons", "icons"),
		IconRepo {
			name: "remixicon",
			icons_paths: kv_map!([
				"Arrows" => "icons/Arrows",
				"Buildings" => "icons/Buildings",
				"Business" => "icons/Business",
				"Communication" => "icons/Communication",
				"Design" => "icons/Design",
				"Development" => "icons/Development",
				"Device" => "icons/Device",
				"Document" => "icons/Document",
				"Editor" => "icons/Editor",
				"Finance" => "icons/Finance",
				"HealthMedical" => "icons/Health & Medical",
				"Logos" => "icons/Logos",
				"Map" => "icons/Map",
				"Media" => "icons/Media",
				"Others" => "icons/Others",
				"System" => "icons/System",
				"UserFaces" => "icons/User & Faces",
				"Weather" => "icons/Weather",
			]),
		},
		IconRepo::single_variant("simple-icons", "icons"),
		IconRepo::single_variant("tabler-icons", "icons"),
	]
});
