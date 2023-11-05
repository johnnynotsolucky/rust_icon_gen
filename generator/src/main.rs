mod config;
mod types;

use heck::{AsPascalCase, AsSnakeCase};
use once_cell::sync::Lazy;
use roxmltree::Document;
use std::{collections::HashMap, fmt::Write, fs, path::PathBuf};
use types::*;

#[macro_export]
macro_rules! into_kv_map {
    ([$($key:expr => $value:expr),* $(,)?]) => {{
		let mut map = ::std::collections::HashMap::new();
		$(map.insert($key, $value);)*
		map
	}};
}

const ICON_LIST_TEMPLATE: &str = include_str!("../templates/icon_list.rs.tpl");
const ICON_MOD_TEMPLATE: &str = include_str!("../templates/icon_set_mod.rs.tpl");

static CARGO_TOML_TEMPLATES: Lazy<HashMap<&'static str, (bool, &'static str)>> = Lazy::new(|| {
	into_kv_map!([
		"icon_sets" => (false, include_str!("../templates/icon_sets_Cargo.toml.tpl")),
		"leptos_icon_gen" => (true, include_str!("../templates/leptos_icon_gen_Cargo.toml.tpl")),
	])
});

struct Icon {
	pub name: IconName,
	pub variant: Variant,
	pub class: Class,
	pub view_box: ViewBox,
	pub width: Width,
	pub height: Height,
	pub stroke: Stroke,
	pub fill: Fill,
	pub stroke_width: StrokeWidth,
	pub stroke_linecap: StrokeLinecap,
	pub stroke_linejoin: StrokeLinejoin,
	pub nodes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct IconRepo {
	name: &'static str,
	icons_paths: HashMap<&'static str, &'static str>,
}

#[derive(Debug, Clone)]
struct IconConfig {
	pub class: Option<Class>,
	pub view_box: Option<ViewBox>,
	pub width: Option<Width>,
	pub height: Option<Height>,
	pub stroke: Option<Stroke>,
	pub fill: Option<Fill>,
	pub stroke_width: Option<StrokeWidth>,
	pub stroke_linecap: Option<StrokeLinecap>,
	pub stroke_linejoin: Option<StrokeLinejoin>,
}

impl IconRepo {
	fn single_variant(name: &'static str, path: &'static str) -> Self {
		Self {
			name,
			icons_paths: into_kv_map!(["" => path]),
		}
	}

	fn load_repo_icons(&self) -> impl Iterator<Item = Icon> + '_ {
		self.icons_paths
			.iter()
			.flat_map(|(variant, path)| {
				let dir = PathBuf::from(format!("generator/repos/{}", self.name,)).join(path);
				fs::read_dir(&dir)
					.unwrap_or_else(|_| panic!("unable to list entries in {}", dir.display()))
					.map(|entry| (variant.to_string(), entry))
			})
			.filter_map(|(variant, entry)| match entry {
				Err(error) => {
					println!("failed to read entry: {error:?}");
					None
				}
				Ok(entry) => entry.path().extension().and_then(|ext| {
					if ext == "svg" {
						let path = entry.path();
						let (icon_config, nodes) = generate_icon_nodes(&path);
						let icon_name: String = path.file_stem().unwrap().to_str().unwrap().into();

						Some(Icon {
							name: icon_name.into(),
							variant: variant.into(),
							class: icon_config.class.unwrap_or_default(),
							view_box: icon_config.view_box.unwrap_or_default(),
							width: icon_config.width.unwrap_or_default(),
							height: icon_config.height.unwrap_or_default(),
							stroke: icon_config.stroke.unwrap_or_default(),
							fill: icon_config.fill.unwrap_or_default(),
							stroke_width: icon_config.stroke_width.unwrap_or_default(),
							stroke_linecap: icon_config.stroke_linecap.unwrap_or_default(),
							stroke_linejoin: icon_config.stroke_linejoin.unwrap_or_default(),
							nodes,
						})
					} else {
						None
					}
				}),
			})
	}
}

fn generate_icon_nodes(icon_path: &PathBuf) -> (IconConfig, Vec<String>) {
	let svg_str = std::fs::read_to_string(icon_path)
		.unwrap_or_else(|_| panic!("failed to read {}", icon_path.display()));
	let svg = Document::parse(&svg_str)
		.unwrap_or_else(|_| panic!("failed to parse {}", icon_path.display()));

	let root = svg.root_element();
	let icon_config = IconConfig {
		class: root.attribute("class").map(|a| a.to_string().into()),
		view_box: root.attribute("viewBox").map(|a| a.to_string().into()),
		width: root.attribute("width").map(|a| a.to_string().into()),
		height: root.attribute("height").map(|a| a.to_string().into()),
		stroke: root.attribute("stroke").map(|a| a.to_string().into()),
		fill: root.attribute("fill").map(|a| a.to_string().into()),
		stroke_width: root.attribute("stroke-width").map(|a| a.to_string().into()),
		stroke_linecap: root
			.attribute("stroke-linecap")
			.map(|a| a.to_string().into()),
		stroke_linejoin: root
			.attribute("stroke-linejoin")
			.map(|a| a.to_string().into()),
	};
	let children = root.children();

	let mut nodes = Vec::new();
	for child in children {
		if child.is_element() {
			let attributes = child
				.attributes()
				.fold(String::new(), |mut acc, attribute| {
					let _ = write!(
						&mut acc,
						" {}=\\\"{}\\\"",
						attribute.name(),
						attribute.value()
					);

					acc
				});

			nodes.push(format!("<{}{attributes}/>", child.tag_name().name()));
		}
	}
	(icon_config, nodes)
}

fn main() {
	let mut cargo_features = vec![];
	let mut icons_mod = String::new();
	let mut icon_sets_insert = String::new();
	for repo in config::ICON_REPOS.iter() {
		let repo_name = AsSnakeCase(repo.name);
		let mut generated_list = String::new();
		for icon in repo.load_repo_icons() {
			generated_list.push_str(&map_insert_str(icon));
		}
		let output = ICON_LIST_TEMPLATE.replace("__icons_insert", &generated_list);

		fs::write(format!("icon_sets/src/icons/{}.rs", repo_name), output)
			.unwrap_or_else(|_| panic!("failed to write {}.rs", repo_name));

		icons_mod.push_str(&format!("mod {};\n", repo_name));
		icon_sets_insert.push_str(&format!(
			"\t\tsets.insert(\"{0}\", &{0}::ICON_LIST);\n",
			repo_name
		));

		cargo_features.push(repo_name);
	}

	let output = ICON_MOD_TEMPLATE
		.replace("__icon_sets_mod", &icons_mod)
		.replace("__icon_sets_inserts", &icon_sets_insert);
	fs::write("icon_sets/src/icons/mod.rs", output)
		.unwrap_or_else(|_| panic!("failed to write mod.rs"));

	let features_full = cargo_features
		.iter()
		.fold(String::new(), |mut acc, feature| {
			let _ = writeln!(&mut acc, "\t\"{feature}\",");
			acc
		});

	for (package, (enable_feature, template)) in CARGO_TOML_TEMPLATES.iter() {
		let output = template.replace("__features_full", &features_full).replace(
			"__features_feature",
			&cargo_features
				.iter()
				.fold(String::new(), |mut acc, feature| {
					let _ = if *enable_feature {
						writeln!(&mut acc, "{feature} = [\"icon_sets/{feature}\"]")
					} else {
						writeln!(&mut acc, "{feature} = []")
					};

					acc
				}),
		);

		fs::write(format!("{package}/Cargo.toml"), output)
			.unwrap_or_else(|_| panic!("failed to write {package}/Cargo.toml"));
	}
}

fn map_insert_str(icon: Icon) -> String {
	let Icon {
		name,
		variant,
		class,
		view_box,
		width,
		height,
		stroke,
		fill,
		nodes,
		stroke_width,
		stroke_linecap,
		stroke_linejoin,
	} = icon;

	let nodes = nodes
		.iter()
		.map(|node| format!("\t\t\t\t\"{node}\","))
		.collect::<Vec<String>>();

	let nodes = nodes.join("\n");

	let insert_str = format!(
		r#"
	icons.insert(
		"{}{variant}",
		Icon {{
			view_box: "{view_box}",
			class: "{class}",
			width: "{width}",
			height: "{height}",
			stroke: "{stroke}",
			fill: "{fill}",
			stroke_width: "{stroke_width}",
			stroke_linecap: "{stroke_linecap}",
			stroke_linejoin: "{stroke_linejoin}",
			nodes: vec![
{nodes}
			],
		}},
	);
"#,
		AsPascalCase(name.to_string()),
	);

	insert_str
}
