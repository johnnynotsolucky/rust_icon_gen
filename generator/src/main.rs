mod types;

use heck::{AsPascalCase, AsSnakeCase};
use once_cell::sync::Lazy;
use roxmltree::Document;
use serde::{Deserialize, Serialize};
use std::{
	collections::HashMap,
	fmt::Write,
	fs,
	path::{Path, PathBuf},
	process::Command,
};
use types::*;

#[macro_export]
macro_rules! kv_map {
    ([$($key:expr => $value:expr),* $(,)?]) => {{
		let mut map = ::std::collections::HashMap::new();
		$(map.insert($key, $value);)*
		map
	}};
}

const ICON_LIST_TEMPLATE: &str = include_str!("../templates/icon_list.rs");
const ICON_MOD_TEMPLATE: &str = include_str!("../templates/icon_set_mod.rs");

static CARGO_TOML_TEMPLATES: Lazy<HashMap<&'static str, (bool, &'static str)>> = Lazy::new(|| {
	kv_map!([
		"icon_sets" => (false, include_str!("../templates/icon_sets_Cargo.toml")),
		"leptos_icon_gen" => (true, include_str!("../templates/leptos_icon_gen_Cargo.toml")),
	])
});

#[derive(Debug)]
struct Icon {
	pub name: IconName,
	pub variant: Variant,
	pub class: Option<Class>,
	pub view_box: ViewBox,
	pub width: Option<Width>,
	pub height: Option<Height>,
	pub stroke: Option<Stroke>,
	pub fill: Option<Fill>,
	pub stroke_width: Option<StrokeWidth>,
	pub stroke_linecap: Option<StrokeLinecap>,
	pub stroke_linejoin: Option<StrokeLinejoin>,
	pub nodes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Repos {
	repos: Vec<IconRepo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconRepo {
	name: String,
	url: String,
	git_ref: String,
	paths: HashMap<String, String>,
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
	fn update_repo(&self) {
		let path = format!("generator/repos/{}", self.name);
		let path = Path::new(&path);

		if !path.exists() {
			println!("{}: git clone", self.name);
			Command::new("git")
				.args(["clone", &self.url, &self.name])
				.current_dir("generator/repos")
				.output()
				.unwrap_or_else(|_| panic!("failed to clone {}", self.name));

			println!("{}: git checkout {}", self.name, self.git_ref);
			Command::new("git")
				.args(["checkout", &self.git_ref])
				.current_dir(format!("generator/repos/{}", self.name))
				.output()
				.unwrap_or_else(|_| {
					panic!("failed to checkout {} for repo {}", self.git_ref, self.name)
				});
		} else {
			let git_ref = Command::new("git")
				.args(["show-ref", "-s", "--verify", "HEAD"])
				.current_dir(format!("generator/repos/{}", self.name))
				.output()
				.unwrap_or_else(|_| panic!("failed to check refs {}", self.name));
			let git_ref = String::from_utf8(git_ref.stdout).unwrap();
			let git_ref = git_ref.lines().collect::<Vec<_>>();
			let git_ref = git_ref.first().unwrap();

			println!(
				"{}: checking refs, {git_ref} == {}",
				self.name, self.git_ref
			);
			if **git_ref != self.git_ref {
				println!("{}: git fetch", self.name);
				Command::new("git")
					.args(["fetch"])
					.current_dir(format!("generator/repos/{}", self.name))
					.output()
					.unwrap_or_else(|_| panic!("failed to update repo {}", self.name));

				println!("{}: git checkout {}", self.name, self.git_ref);
				Command::new("git")
					.args(["checkout", &self.git_ref])
					.current_dir(format!("generator/repos/{}", self.name))
					.output()
					.unwrap_or_else(|_| {
						panic!("failed to checkout {} for repo {}", self.git_ref, self.name)
					});
			}
		}
	}

	fn load_repo_icons(&self) -> impl Iterator<Item = Icon> + '_ {
		self.paths
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
							variant: if variant == "_" {
								"".into()
							} else {
								variant.into()
							},
							class: icon_config.class,
							view_box: icon_config.view_box.unwrap_or_default(),
							width: icon_config.width,
							height: icon_config.height,
							stroke: icon_config.stroke,
							fill: icon_config.fill,
							stroke_width: icon_config.stroke_width,
							stroke_linecap: icon_config.stroke_linecap,
							stroke_linejoin: icon_config.stroke_linejoin,
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
	let icon_repos_config =
		fs::read_to_string("icon_repos.toml").expect("failed to read icon_repos.toml");
	let icon_repos_config =
		toml::from_str::<Repos>(&icon_repos_config).expect("failed to parse icon_repos.toml");

	let mut cargo_features = vec![];
	let mut icons_mod = String::new();
	let mut icon_sets_insert = String::new();

	for repo in icon_repos_config.repos.iter() {
		repo.update_repo();

		let repo_name = AsSnakeCase(&repo.name);
		let mut generated_list = String::new();

		let mut icons = repo.load_repo_icons().collect::<Vec<_>>();
		icons.sort_by_key(|icon| format!("{}_{}", icon.variant, icon.name));

		for icon in icons {
			generated_list.push_str(&map_insert_str(&icon));
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

fn map_insert_str(icon: &Icon) -> String {
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

	let class = class
		.as_ref()
		.map_or("None".into(), |v| format!("Some(\"{v}\")"));
	let width = width
		.as_ref()
		.map_or("None".into(), |v| format!("Some(\"{v}\")"));
	let height = height
		.as_ref()
		.map_or("None".into(), |v| format!("Some(\"{v}\")"));
	let stroke = stroke
		.as_ref()
		.map_or("None".into(), |v| format!("Some(\"{v}\")"));
	let fill = fill
		.as_ref()
		.map_or("None".into(), |v| format!("Some(\"{v}\")"));
	let stroke_width = stroke_width
		.as_ref()
		.map_or("None".into(), |v| format!("Some(\"{v}\")"));
	let stroke_linecap = stroke_linecap
		.as_ref()
		.map_or("None".into(), |v| format!("Some(\"{v}\")"));
	let stroke_linejoin = stroke_linejoin
		.as_ref()
		.map_or("None".into(), |v| format!("Some(\"{v}\")"));

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
			class: {class},
			width: {width},
			height: {height},
			stroke: {stroke},
			fill: {fill},
			stroke_width: {stroke_width},
			stroke_linecap: {stroke_linecap},
			stroke_linejoin: {stroke_linejoin},
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
