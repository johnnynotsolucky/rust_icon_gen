use crate::Icon;
use once_cell::sync::Lazy;
use std::collections::HashMap;

mod ant_design_icons;
mod bootstrap_icons;
mod boxicons;
mod charm_icons;
mod codicons;
mod css_gg;
mod feather;
mod font_awesome;
mod heroicons;
mod ionicons;
mod lucide;
mod microns;
mod octicons;
mod remixicon;
mod simple_icons;
mod tabler_icons;

pub static ICON_SETS: Lazy<HashMap<&'static str, &Lazy<HashMap<&'static str, Icon>>>> =
	Lazy::new(|| {
		let mut sets = HashMap::new();

		sets.insert("ant_design_icons", &ant_design_icons::ICON_LIST);
		sets.insert("bootstrap_icons", &bootstrap_icons::ICON_LIST);
		sets.insert("boxicons", &boxicons::ICON_LIST);
		sets.insert("charm_icons", &charm_icons::ICON_LIST);
		sets.insert("codicons", &codicons::ICON_LIST);
		sets.insert("css_gg", &css_gg::ICON_LIST);
		sets.insert("feather", &feather::ICON_LIST);
		sets.insert("font_awesome", &font_awesome::ICON_LIST);
		sets.insert("heroicons", &heroicons::ICON_LIST);
		sets.insert("ionicons", &ionicons::ICON_LIST);
		sets.insert("lucide", &lucide::ICON_LIST);
		sets.insert("microns", &microns::ICON_LIST);
		sets.insert("octicons", &octicons::ICON_LIST);
		sets.insert("remixicon", &remixicon::ICON_LIST);
		sets.insert("simple_icons", &simple_icons::ICON_LIST);
		sets.insert("tabler_icons", &tabler_icons::ICON_LIST);

		sets
	});
