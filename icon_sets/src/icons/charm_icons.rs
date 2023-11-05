use crate::Icon;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static ICON_LIST: Lazy<HashMap<&'static str, Icon>> = Lazy::new(|| {
	let mut icons = HashMap::new();

	icons.insert(
		"MonitorCross",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m14.2 7.75v3.5h-12.5v-9.5h6.5\"/>",
				"<path d=\"m4.75 14.2h6.5m-3.25-2.5v2.5\"/>",
				"<path d=\"m14.2 1.75-3.5 3.5m0-3.5 3.5 3.5\"/>",
			],
		},
	);

	icons.insert(
		"BluetoothSlash",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m10.75 6.25 1.5-1.25-4.5-3.25v2.5m4.5 6.75-4.5 3.25v-6l-4 3\"/>",
				"<path d=\"m1.75 3.25 12.5 9\"/>",
			],
		},
	);

	icons.insert(
		"Triangle",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<polygon points=\"8 2.75,1.75 14.25,14.25 14.25\"/>"],
		},
	);

	icons.insert(
		"Search",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m11.25 11.25 3 3\"/>",
				"<circle cx=\"7.5\" cy=\"7.5\" r=\"4.75\"/>",
			],
		},
	);

	icons.insert(
		"Notes",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"12.5\" width=\"10.5\" y=\"1.75\" x=\"2.75\"/>",
				"<path d=\"m5.75 7.75h4.5m-4.5 3h2.5m-2.5-6h4.5\"/>",
			],
		},
	);

	icons.insert(
		"MediaPause",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"10.5\" width=\"3.5\" y=\"2.75\" x=\"2.75\"/>",
				"<rect height=\"10.5\" width=\"3.5\" y=\"2.75\" x=\"9.75\"/>",
			],
		},
	);

	icons.insert(
		"Eraser",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polyline points=\"14.25 13.25,4.75 13.25,1.75 10.25,9.25 2.75,14.25 7.75,8.75 13.25\"/>",
				"<line x1=\"5.25\" y1=\"6.75\" x2=\"10.25\" y2=\"11.75\"/>",
			],
		},
	);

	icons.insert(
		"ChartLine",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m4.75 11.25 2.5-4.5 2.5 2.5 3.5-6m-11.5-1.5v12.5h12.5\"/>"],
		},
	);

	icons.insert(
		"ArrowDown",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m3.25 8.75 4.5 4.5 4.5-4.5m-4.5-6v10.5\"/>"],
		},
	);

	icons.insert(
		"ChevronsRight",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m3.75 12.25 4.5-4.25-4.5-4.25m5 8.5l4.5-4.25-4.5-4.25\"/>"],
		},
	);

	icons.insert(
		"StackPush",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m3.25 7.25-1.5.75 6.25 3.25 6.25-3.25-1.5-.75m-11 3.75 6.25 3.25 6.25-3.25\"/>",
				"<path d=\"m8 8.25v-6.5m-2.25 4.5 2.25 2 2.25-2\"/>",
			],
		},
	);

	icons.insert(
		"WifiFair",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m1.75 4.75 6.25 8.5 6.25-8.5c-3.25-2.75-9.25-2.75-12.5 0z\"/>",
				"<path d=\"m4.25 8c2-1.75 5.5-1.75 7.5 0\"/>",
			],
		},
	);

	icons.insert(
		"SoundUp",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"1.75 5.75,1.75 10.25,4.25 10.25,8.25 13.25,8.25 2.75,4.25 5.75\"/>",
				"<path d=\"m10.75 6.25s1 .5 1 1.75-1 1.75-1 1.75m1-6.5c2 1 3 2.5 3 4.75s-1 3.75-3 4.75\"/>",
			],
		},
	);

	icons.insert(
		"ChevronsDown",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m3.75 3.75 4.25 4.5 4.25-4.5m-8.5 5 4.25 4.5 4.25-4.5\"/>"],
		},
	);

	icons.insert(
		"Cog",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cx=\"8\" cy=\"8\" r=\"1.75\"/>",
				"<path d=\"m6.75 1.75-.5 1.5-1.5 1-2-.5-1 2 1.5 1.5v1.5l-1.5 1.5 1 2 2-.5 1.5 1 .5 1.5h2.5l.5-1.5 1.5-1 2 .5 1-2-1.5-1.5v-1.5l1.5-1.5-1-2-2 .5-1.5-1-.5-1.5z\"/>",
			],
		},
	);

	icons.insert(
		"Crosshair",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m8 5.25v-3m0 11.5v-3m2.75-2.75h3m-11.5 0h3\"/>",
				"<circle cx=\"8\" cy=\"8\" r=\"6.25\"/>",
			],
		},
	);

	icons.insert(
		"Hourglass",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m11.75 13.75c0-5-2-4-2-5.75s2-0.75 2-5.75m-7.5 11.5c0-5 2-4 2-5.75s-2-.75-2-5.75m-1.5-.5h10.5m-10.5 12.5h10.5\"/>",
			],
		},
	);

	icons.insert(
		"ClockAlarm",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m11.75 1.75 2.5 2m-10-2-2.5 2m10.5 9.5 1 1m-9.5-1-1 1m5.5-7.5v2.5l-1.5 1\"/>",
				"<circle cx=\"8\" cy=\"9\" r=\"5.25\"/>",
			],
		},
	);

	icons.insert(
		"WifiSlash",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m5.25 3.25c-1.5 0-3.5 1.5-3.5 1.5l6.25 8.5 2.25-3m-1.5-7.5s2.97688-.134944 5.5 2l-2 2.5\"/>",
				"<line x1=\"4.25\" y1=\"1.75\" x2=\"12.25\" y2=\"12.25\"/>",
			],
		},
	);

	icons.insert(
		"Reply",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m14.25 13.25c-.5-6-5.5-7.5-8-7v-3.5l-4.5 5.25 4.5 5.25v-3.5c2.50001-0.5 6.5 0.5 8 3.5z\"/>",
			],
		},
	);

	icons.insert(
		"ShieldWarning",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m8 1.75 5.25 2v5c0 2.25-2 4.5-5.25 5.5-3.25-1-5.25-3-5.25-5.5v-5z\"/>",
				"<path d=\"m8 10.75v0m0-5.5v3\"/>",
			],
		},
	);

	icons.insert(
		"ThumbUp",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m5.25 5.75c1.5 0 3-4 4.5-4v4h4.5s-.5 7.5-3.5 7.5h-5.5z\"/>",
				"<path d=\"m5.25 5.75h-3.5v7.5h3.5\"/>",
			],
		},
	);

	icons.insert(
		"LayoutList",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"10.5\" width=\"12.5\" y=\"2.75\" x=\"1.75\"/>",
				"<path d=\"m5.25 3.25v9.5m-3-6.5h11.5m-11.5 3.5h11.5\"/>",
			],
		},
	);

	icons.insert(
		"Umbrella",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m1.75 8.25s.5-6.5 6.25-6.5 6.25 6.5 6.25 6.5z\"/>",
				"<path d=\"m7.75 8.75v4s0 1.5 1.5 1.5 1.5-1.5 1.5-1.5\"/>",
			],
		},
	);

	icons.insert(
		"LayoutSidebar",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"10.5\" width=\"12.5\" y=\"2.75\" x=\"1.75\"/>",
				"<path d=\"m6.25 3v9.5\"/>",
			],
		},
	);

	icons.insert(
		"LayoutGrid",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"10.5\" width=\"12.5\" y=\"2.75\" x=\"1.75\"/>",
				"<path d=\"m2 8h12m-3.75-4.75v9.5m-4.5-9.5v9.5\"/>",
			],
		},
	);

	icons.insert(
		"ChevronsUpDown",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m11.25 10.75-3.25 3.5-3.25-3.5\"/>",
				"<path d=\"m11.25 5.25-3.25-3.5-3.25 3.5\"/>",
			],
		},
	);

	icons.insert(
		"Circle",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<circle cx=\"8\" cy=\"8\" r=\"6.25\"/>"],
		},
	);

	icons.insert(
		"ClipboardTick",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect x=\"5.75\" y=\"1.75\" width=\"4.5\" height=\"3.5\"/>",
				"<path d=\"m9.75 12.8 1.5 1.5 3-2.5m-9-9h-2.5v11.5h4.5m6-5v-6.5h-2.5\"/>",
			],
		},
	);

	icons.insert(
		"Person",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cx=\"8\" cy=\"6\" r=\"3.25\"/>",
				"<path d=\"m2.75 14.25c0-2.5 2-5 5.25-5s5.25 2.5 5.25 5\"/>",
			],
		},
	);

	icons.insert(
		"Home",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m3.75 5.75v7.5h8.5v-7.5m-10.5 1.5 6.25-5.5l6.25 5.5\"/>"],
		},
	);

	icons.insert(
		"Copyright",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cy=\"8\" cx=\"8\" r=\"6.25\"/>",
				"<path d=\"m10 6.75s-.75-1-2-1-2.25 1-2.25 2.25 1 2.25 2.25 2.25 2-1 2-1\"/>",
			],
		},
	);

	icons.insert(
		"Gitlab",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m8 14.25-6.25-4.5 2-8 2 5.5h4.5l2-5.5 2 8z\"/>"],
		},
	);

	icons.insert(
		"Music",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cx=\"4\" cy=\"12\" r=\"2.25\"/>",
				"<circle cx=\"12\" cy=\"11\" r=\"2.25\"/>",
				"<polyline points=\"6.25 12,6.25 2.75,14.25 1.75,14.25 11\"/>",
			],
		},
	);

	icons.insert(
		"FileSymlink",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polyline points=\"2.75 7.75,2.75 1.75,8.25 1.75,13.25 6.75,13.25 14.25,9.25 14.25\"/>",
				"<polyline points=\"7.75 2.25 7.75 7.25 12.75 7.25\"/>",
				"<path d=\"m2.75 14.25 3.5-3.5m0 3v-3h-3\"/>",
			],
		},
	);

	icons.insert(
		"ArrowUpLeft",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m10.75 4.25h-6.5v6.5m7.5 1-7.5-7.5\"/>"],
		},
	);

	icons.insert(
		"ChevronDown",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m3.75 5.75 4.25 4.5 4.25-4.5\"/>"],
		},
	);

	icons.insert(
		"Hexagon",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"1.75 4.75,8 1.25,14.25 4.75,14.25 11.25,8 14.75,1.75 11.25\"/>",
			],
		},
	);

	icons.insert(
		"LinkExternal",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polyline points=\"8.25 2.75,2.75 2.75,2.75 13.25,13.25 13.25,13.25 7.75\"/>",
				"<path d=\"m13.25 2.75-5.5 5.5m3-6.5h3.5v3.5\"/>",
			],
		},
	);

	icons.insert(
		"Signpost",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"1.75 9.25,12.25 9.25,14.25 7.00,12.25 4.75,1.75 4.75\"/>",
				"<path d=\"m7.25 9.75v4.5m0-12.5v2.5\"/>",
			],
		},
	);

	icons.insert(
		"Github",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m5.75 14.25s-.5-2 .5-3c0 0-2 0-3.5-1.5s-1-4.5 0-5.5c-.5-1.5.5-2.5.5-2.5s1.5 0 2.5 1c1-.5 3.5-.5 4.5 0 1-1 2.5-1 2.5-1s1 1 .5 2.5c1 1 1.5 4 0 5.5s-3.5 1.5-3.5 1.5c1 1 .5 3 .5 3\"/>",
				"<path d=\"m5.25 13.75c-1.5.5-3-.5-3.5-1\"/>",
			],
		},
	);

	icons.insert(
		"GrabVertical",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cy=\"2.5\" cx=\"5.5\" r=\".75\"/>",
				"<circle cy=\"8\" cx=\"5.5\" r=\".75\"/>",
				"<circle cy=\"13.5\" cx=\"5.5\" r=\".75\"/>",
				"<circle cy=\"2.5\" cx=\"10.4957\" r=\".75\"/>",
				"<circle cy=\"8\" cx=\"10.4957\" r=\".75\"/>",
				"<circle cy=\"13.5\" cx=\"10.4957\" r=\".75\"/>",
			],
		},
	);

	icons.insert(
		"Chip",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"10.5\" width=\"10.5\" y=\"2.75\" x=\"2.75\"/>",
				"<rect height=\"3.5\" width=\"3.5\" y=\"6.25\" x=\"6.25\"/>",
				"<path d=\"m2.25 10.25h-1m1-4.5h-1m13.5 4.5h-1m1-4.5h-1m-3.5 8v1m-4.5-1v1m4.5-13.5v1m-4.5-1v1\"/>",
			],
		},
	);

	icons.insert(
		"Ticket",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m1.75 3.75h12.5v3s-2 0-2 1.75 2 1.75 2 1.75v3h-12.5v-3s2 0 2-1.75-2-1.75-2-1.75z\"/>",
				"<path d=\"m8.75 11.75v1.5m0-9.5v1.5m0 2.5v1.5\"/>",
			],
		},
	);

	icons.insert(
		"ScreenMinimise",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m1.75 10.75h3.5v3.5m5.5 0v-3.5h3.5m0-5.5h-3.5v-3.5m-5.5 0v3.5h-3.5\"/>",
			],
		},
	);

	icons.insert(
		"Crop",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m4.25 1.75v10h10\"/>",
				"<path d=\"m11.8 14.2v-2.5m0-2.5v-5h-5m-2.5 0h-2.5\"/>",
			],
		},
	);

	icons.insert(
		"Tag",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"7.25 14.25,1.75 8.75,8.75 1.75,14.25 1.75,14.25 7.25\"/>",
				"<circle cx=\"11\" cy=\"5\" r=\".5\" fill=\"currentColor\"/>",
			],
		},
	);

	icons.insert(
		"Briefcase",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"9.5\" width=\"12.5\" y=\"4.75\" x=\"1.75\"/>",
				"<path d=\"m1.75 6.25s-.5 3.5 3 3.5h6.5c3.5 0 3-3.5 3-3.5m-8.5-2v-2.5h4.5v2.5\"/>",
			],
		},
	);

	icons.insert(
		"Organisation",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect x=\"6.75\" y=\"1.75\" width=\"3.5\" height=\"3.5\"/>",
				"<rect x=\"10.75\" y=\"10.75\" width=\"3.5\" height=\"3.5\"/>",
				"<rect x=\"2.75\" y=\"10.75\" width=\"3.5\" height=\"3.5\"/>",
				"<path d=\"m8.5 5.75v2m-3.75 2.5v-2h7.5v2\"/>",
			],
		},
	);

	icons.insert(
		"Map",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m10.25 5.25v8.5m-4.5-10.5v8.5m-4 2.5v-9.5l4-2 4.5 2 4-2v9.5l-4 2-4.5-2z\"/>",
			],
		},
	);

	icons.insert(
		"Inbox",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"1.75 13.25,14.25 13.25,14.25 8.25,11.75 2.75,4.25 2.75,1.75 8.25\"/>",
				"<path d=\"m2.25 8.75h3l1 1.5h3.5l1-1.5h3\"/>",
			],
		},
	);

	icons.insert(
		"Power",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m8 1.75v6.5m4.25-5s2 1.29822 2 4.75-2.79822 6.25-6.25 6.25-6.25-2.79822-6.25-6.25 2-4.75 2-4.75\"/>",
			],
		},
	);

	icons.insert(
		"Microphone",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m8 1.75c-2.25 0-2.25 2-2.25 3v1.5c0 1 0 3 2.25 3s2.25-2 2.25-3v-1.5c0-1 0-3-2.25-3z\"/>",
				"<path d=\"m8 13v1.25m-5.25-6.5s0 4.5 5.25 4.50785c5.25.0079 5.25-4.5078 5.25-4.5078\"/>",
			],
		},
	);

	icons.insert(
		"Database",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m8 1.75c-3.75 0-5.25 2-5.25 2v4.5 4s1.5 2 5.25 2 5.25-2 5.25-2v-4-4.5s-1.5-2-5.25-2z\"/>",
				"<path d=\"m2.75 8.25s1.5 2 5.25 2 5.25-2 5.25-2m-10.5-4s1.5 2 5.25 2 5.25-2 5.25-2\"/>",
			],
		},
	);

	icons.insert(
		"Swords",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m2.75 9.25 1.5 2.5 2 1.5m-4.5 0 1 1m1.5-2.5-1.5 1.5m3-1 8.5-8.5v-2h-2l-8.5 8.5\"/>",
				"<path d=\"m10.25 12.25-2.25-2.25m2-2 2.25 2.25m1-1-1.5 2.5-2 1.5m4.5 0-1 1m-1.5-2.5 1.5 1.5m-7.25-5.25-4.25-4.25v-2h2l4.25 4.25\"/>",
			],
		},
	);

	icons.insert(
		"File",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"2.75 1.75,8.25 1.75,13.25 6.75,13.25 14.25,2.75 14.25\"/>",
				"<polyline points=\"7.75 2.25,7.75 7.25,12.75 7.25\"/>",
			],
		},
	);

	icons.insert(
		"Files",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"9.25 1.75,13.25 5.75,13.25 11.25,5.75 11.25,5.75 1.75\"/>",
				"<polyline points=\"9.25 2.25,9.25 5.75,12.75 5.75\"/>",
				"<polyline points=\"10.25 11.75,10.25 14.25,2.75 14.25,2.75 4.75,5.25 4.75\"/>",
			],
		},
	);

	icons.insert(
		"RotateClockwise",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m11.25 5.25h3m0 3.5c0 2.5-2.79822 5.5-6.25 5.5s-6.25-2.7982-6.25-6.25c0-3.45178 2.79822-6.25 6.25-6.25 3.75 0 6.25 3.5 6.25 3.5v-3.5\"/>",
			],
		},
	);

	icons.insert(
		"MediaRewind",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"7.75 3.75,7.75 12.25,1.75 8\"/>",
				"<polygon points=\"14.25 3.75,14.25 12.25,8.25 8\"/>",
			],
		},
	);

	icons.insert(
		"Terminal",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"10.5\" width=\"12.5\" y=\"2.75\" x=\"1.75\"/>",
				"<path d=\"m8.75 10.25h2.5m-6.5-4.5 2.5 2.25-2.5 2.25\"/>",
			],
		},
	);

	icons.insert(
		"ShieldTick",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m8 1.75 5.25 2v5c0 2.25-2 4.5-5.25 5.5-3.25-1-5.25-3-5.25-5.5v-5z\"/>",
				"<polyline points=\"5.75 7.75,7.25 9.25,10.25 5.75\"/>",
			],
		},
	);

	icons.insert(
		"Move",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m12.25 10.25 2-2.25-2-2.25m-2-2-2.25-2-2.25 2m-2 2-2 2.25 2 2.25m2 2 2.25 2 2.25-2m-2.25-10.5v12m5.75-5.75h-12\"/>",
			],
		},
	);

	icons.insert(
		"ChevronUp",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m12.25 10.25-4.25-4.5-4.25 4.5\"/>"],
		},
	);

	icons.insert(
		"NorthStar",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m13.75 7.75h-12\"/>",
				"<path d=\"m7.75 1.75v12\"/>",
				"<path d=\"m4.25 11.25 7-7\"/>",
				"<path d=\"m11.25 11.25-7-7\"/>",
			],
		},
	);

	icons.insert(
		"FolderSymlink",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m1.75 13.25 3.5-3.5m0 3v-3h-3\"/>",
				"<polyline points=\"8.25 13.25,14.25 13.25,14.25 4.75,8.25 4.75,6.75 2.75,1.75 2.75,1.75 6.75\"/>",
			],
		},
	);

	icons.insert(
		"ArrowDownLeft",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m10.75 11.75h-6.5v-6.5m7.5-1-7.5 7.5\"/>"],
		},
	);

	icons.insert(
		"Droplet",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m2.75 9c0 2.9 2.35 5.25 5.25 5.25s5.25-2.35 5.25-5.25c0-3.25-5.25-7.25-5.25-7.25s-5.25 4-5.25 7.25z\"/>",
			],
		},
	);

	icons.insert(
		"GraduateCap",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m14.25 9.25v-3.25l-6.25-3.25-6.25 3.25 6.25 3.25 3.25-1.5v3.5c0 1-1.5 2-3.25 2s-3.25-1-3.25-2v-3.5\"/>",
			],
		},
	);

	icons.insert(
		"PhoneForward",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m1.75 1.75c0 8.5 4 12.5 12.5 12.5v-4l-3.5-1-1 1.5c-2 0-4.5-2.5-4.5-4.5l1.5-1-1-3.5z\"/>",
				"<path d=\"m9.75 4.75h4.5m-2 2 2-2-2-2\"/>",
			],
		},
	);

	icons.insert(
		"ScreenMaximise",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m5.25 14.25h-3.5v-3.5m12.5 0v3.5h-3.5m0-12.5h3.5v3.5m-12.5 0v-3.5h3.5\"/>",
			],
		},
	);

	icons.insert(
		"MediaSkip",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"2.75 13.25,11.25 8,2.75 2.75\"/>",
				"<line x1=\"14.25\" y1=\"3.75\" x2=\"14.25\" y2=\"12.25\"/>",
			],
		},
	);

	icons.insert(
		"ShoppingBag",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"9.5\" width=\"10.5\" y=\"4.75\" x=\"2.75\"/>",
				"<path d=\"m5.75 7.75c0 1.5 1 2.5 2.25 2.5s2.25-1 2.25-2.5m-7.5-3 1.5-3h7.5l1.5 3\"/>",
			],
		},
	);

	icons.insert(
		"ChevronRight",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m5.75 12.25 4.5-4.25-4.5-4.25\"/>"],
		},
	);

	icons.insert(
		"Clipboard",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"3.5\" width=\"4.5\" y=\"1.75\" x=\"5.75\"/>",
				"<path d=\"m5.25 2.75h-2.5v11.5h10.5v-11.5h-2.5\"/>",
			],
		},
	);

	icons.insert(
		"SwapVertical",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m7.75 5.75-3-3-3 3m3 7.5v-10.5m9.5 7.5-3 3-3-3m3-7.5v10.5\"/>"],
		},
	);

	icons.insert(
		"Package",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"1.75 5.75,1.75 14.25,1.75 14.25,14.25 14.25,14.25 5.75,10.75 1.75,5.25 1.75\"/>",
				"<path d=\"m8 1.75v3.5m-5.75.5h11.5\"/>",
			],
		},
	);

	icons.insert(
		"SwapHorizontal",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m5.75 8.25-3 3 3 3m7.5-3h-10.5m7.5-9.5l3 3-3 3m-7.5-3h10.5\"/>"],
		},
	);

	icons.insert(
		"Link",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m9.75 4.75c3 0 4.5 1.5 4.5 3.25s-1.5 3.25-4.5 3.25m-4-3.25h4.5m-4-3.25c-3 0-4.5 1.5-4.5 3.25s1.5 3.25 4.5 3.25\"/>",
			],
		},
	);

	icons.insert(
		"Certificate",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polyline points=\"11.25 1.75,2.75 1.75,2.75 13.25,5.25 13.25\"/>",
				"<polyline points=\"8.75 9.75,8.25 14.25,10.50 13.25,12.75 14.25,12.25 9.75\"/>",
				"<circle cx=\"10.5\" cy=\"7.5\" r=\"2.75\"/>",
			],
		},
	);

	icons.insert(
		"Key",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m10 1.75c-2.34721 0-4.25 1.90279-4.25 4.25.00023.37267.04949.74369.14648 1.10352l-4.14648 4.14648v3h3v-1.5h1.5v-1.5h1.5l1.15039-1.15039c.35839.0980.72808.1486 1.09961.1504 2.3472 0 4.25-1.90279 4.25-4.25s-1.9028-4.25-4.25-4.25z\"/>",
				"<circle cx=\"10.75\" cy=\"5.25\" r=\"0.5\" fill=\"currentColor\"/>",
			],
		},
	);

	icons.insert(
		"ArrowRight",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m8.75 3.25 4.5 4.5-4.5 4.5m-6-4.5h10.5\"/>"],
		},
	);

	icons.insert(
		"Clover",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m4.75 2.75c-.50 1.5 1.25 3.25 3.25 5.25 2-2 3.75-3.75 3.25-5.25s-2.5-1-3.25.50c-.75-1.5-2.75-2-3.25-.50zm3.25 5.25c2 2 3.75 3.75 5.25 3.25s1-2.5-.5-3.25c1.5-.75 2-2.75.5-3.25s-3.25 1.25-5.25 3.25zm0 0c-2 2-3.75 3.75-3.25 5.25s2.5 1 3.25-.5c.75 1.5 2.75 2 3.25.5s-1.25-3.25-3.25-5.25zm0 0c-2-2-3.75-3.75-5.25-3.25s-1 2.5.5 3.25c-1.5.75-2 2.75-.5 3.25s3.25-1.25 5.25-3.25z\"/>",
			],
		},
	);

	icons.insert(
		"Telescope",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m4.75 5.75 1 2.5m3.5-4.5 1.5 3.5m-9 0 1 2.5 11.5-3.5-2-4.5z\"/>",
				"<path d=\"m7.75 11.2v3m-3-0.5 2.25-2.5 1.75-0.5 2.5 3\"/>",
			],
		},
	);

	icons.insert(
		"Diff",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m3.75 13.75h8m0-7.5h-8m4-4v8\"/>"],
		},
	);

	icons.insert(
		"StackPop",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m4.25 6.75-2.5 1.25 6.25 3.25 6.25-3.25-2.5-1.25m-10 4.25 6.25 3.25 6.25-3.25\"/>",
				"<path d=\"m8 8.25v-6.5m-2.25 2 2.25-2 2.25 2\"/>",
			],
		},
	);

	icons.insert(
		"Archive",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"3.5\" width=\"12.5\" y=\"2.75\" x=\"1.75\"/>",
				"<path d=\"m6.75 9.25h2.5m-6.5-2.5v7.5h10.5v-7.5\"/>",
			],
		},
	);

	icons.insert(
		"PhoneOutgoing",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m1.75 1.75c0 8.5 4 12.5 12.5 12.5v-4l-3.5-1-1 1.5c-2 0-4.5-2.5-4.5-4.5l1.5-1-1-3.5z\"/>",
				"<path d=\"m9.75 6.25 3.5-3.5m0 3v-3h-3\"/>",
			],
		},
	);

	icons.insert(
		"Candy",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cx=\"8\" cy=\"8\" r=\"3.25\"/>",
				"<path d=\"m7.25 11.25c0 1-.5 2.5-1.5 3-.75 0-1.5-1-2-2-1-.5-2-1.5-2-2 .5-1 2-1.5 3-1.5m4-4c0-1 .5-2.5 1.5-3 .75 0 1.5 1 2 2 1 .5 2 1.5 2 2-.5 1-2 1.5-3 1.5\"/>",
			],
		},
	);

	icons.insert(
		"Mail",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"9.5\" width=\"12.5\" y=\"3.75\" x=\"1.75\"/>",
				"<path d=\"m2.25 4.25 5.75 5 5.75-5\"/>",
			],
		},
	);

	icons.insert(
		"Shield",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m8 1.75 5.25 2v5c0 2.25-2 4.5-5.25 5.5-3.25-1-5.25-3-5.25-5.5v-5z\"/>",
			],
		},
	);

	icons.insert(
		"Download",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m3.25 13.25h9m-8.5-6.5 4 3.5 4-3.5m-4-5v8.5\"/>"],
		},
	);

	icons.insert(
		"OctagonWarning",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"5.25 1.75,10.75 1.75,14.25 5.25,14.25 10.75,10.75 14.25,5.25 14.25,1.75 10.75,1.75 5.25\"/>",
				"<path d=\"m8 11.25v0m0-6.5v3.5\"/>",
			],
		},
	);

	icons.insert(
		"Pin",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m10.25 10.25 4 4m-12.5-7.5 5-5s1 2 2 3 4.5 2 4.5 2l-6.5 6.5s-1-3.5-2-4.5-3-2-3-2z\"/>",
			],
		},
	);

	icons.insert(
		"SoundMute",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"1.75 5.75 1.75 10.25 4.25 10.25 8.25 13.25 8.25 2.75 4.25 5.75\"/>",
				"<path d=\"m14.25 5.75-3.5 4.5m0-4.5 3.5 4.5\"/>",
			],
		},
	);

	icons.insert(
		"Apps",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect width=\"4.5\" height=\"4.5\" x=\"1.75\" y=\"1.75\"/>",
				"<rect width=\"4.5\" height=\"4.5\" x=\"1.75\" y=\"9.75\"/>",
				"<rect width=\"4.5\" height=\"4.5\" x=\"9.75\" y=\"9.75\"/>",
				"<rect width=\"4.5\" height=\"4.5\" x=\"9.75\" y=\"1.75\"/>",
			],
		},
	);

	icons.insert(
		"LayoutStackV",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"10.5\" width=\"12.5\" y=\"2.75\" x=\"1.75\"/>",
				"<line x1=\"8\" y1=\"3.25\" x2=\"8\" y2=\"12.75\"/>",
				"<line x1=\"8\" y1=\"8\" x2=\"14\" y2=\"8\"/>",
			],
		},
	);

	icons.insert(
		"BookOpen",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m8 3.75c-1.75-1-2.25-1-6.25-1v9.5c4 0 4.5 0 6.25 1 1.75-1 3.25-1 6.25-1v-9.5c-4 0-4.5 0-6.25 1z\"/>",
				"<path d=\"m8 4.25v8.5\"/>",
			],
		},
	);

	icons.insert(
		"ShieldCross",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m8 1.75 5.25 2v5c0 2.25-2 4.5-5.25 5.5-3.25-1-5.25-3-5.25-5.5v-5z\"/>",
				"<path d=\"m9.75 5.75-3.5 3.5m0-3.5 3.5 3.5\"/>",
			],
		},
	);

	icons.insert(
		"Server",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect x=\"1.75\" y=\"3.25\" width=\"12.5\" height=\"10\"/>",
				"<line x1=\"2.25\" y1=\"8.25\" x2=\"13.75\" y2=\"8.25\"/>",
				"<path d=\"m4.75 10.75v0m0-5v0m6.5 0h-3m3 5h-3\"/>",
			],
		},
	);

	icons.insert(
		"Folders",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"4.75 2.25,4.75 10.25,14.25 10.25,14.25 3.75,9.25 3.75,7.75 2.25\"/>",
				"<polyline points=\"4.75 5.25,1.75 5.25,1.75 13.25,11.25 13.25,11.25 10.25\"/>",
			],
		},
	);

	icons.insert(
		"LinkSlash",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m10.75 1.75-5.5 12.5m4.5-9.5c3 0 4.5 1.5 4.5 3.25s-1.5 3.25-4.5 3.25m-3.5-6.5c-3 0-4.5 1.5-4.5 3.25s1.5 3.25 4.5 3.25\"/>",
			],
		},
	);

	icons.insert(
		"Scales",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m1.75 3.75c1 1 2.5 1.5 4 0h4.5c1.5 1.5 3 1 4 0m-6.25-2v12m-3.25.5h6.5\"/>",
				"<path d=\"m12.75 4.75-2 5c.5 1 3.5 1 4 0zm-9.5 0-2 5c.5 1 3.5 1 4 0z\"/>",
			],
		},
	);

	icons.insert(
		"Book",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m11.75 11.75v2m1.5.5h-9c-.75 0-1.5-.5-1.5-1.5s.75-1.5 1.5-1.5h9v-9.5h-9c-.75 0-1.5.75-1.5 1.5v9.5\"/>",
			],
		},
	);

	icons.insert(
		"NotesTick",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polyline points=\"7.25 14.25,2.75 14.25,2.75 1.75,13.25 1.75,13.25 9.25\"/>",
				"<path d=\"m9.75 12.75 1.5 1.5 3-2.5m-8.5-4h4.5m-4.5 3h1.5m-1.5-6h4.5\"/>",
			],
		},
	);

	icons.insert(
		"Skull",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m1.75 11.25h3v3h6.5v-3h3s1-9.5-6.25-9.5-6.25 9.5-6.25 9.5z\"/>",
				"<circle cx=\"5.25\" cy=\"7.75\" r=\".5\" fill=\"currentColor\"/>",
				"<circle cx=\"10.75\" cy=\"7.75\" r=\".5\" fill=\"currentColor\"/>",
			],
		},
	);

	icons.insert(
		"AppsPlus",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect x=\"1.75\" y=\"1.75\" width=\"4.5\" height=\"4.5\"/>",
				"<rect x=\"1.75\" y=\"9.75\" width=\"4.5\" height=\"4.5\"/>",
				"<rect x=\"9.75\" y=\"9.75\" width=\"4.5\" height=\"4.5\"/>",
				"<path d=\"m14.8 3.75h-5m2.5-2.5v5\"/>",
			],
		},
	);

	icons.insert(
		"Bell",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m8 1.75c-2.46803 0-4.25 1.5-4.25 3.5v3l-2 3.5h12.5l-2-3.5v-3c0-2-1.16605-3.5-4.25-3.5z\"/>",
				"<path d=\"m5.75 12.25c0 3 4.5 3 4.5 0\"/>",
			],
		},
	);

	icons.insert(
		"GitBranch",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cx=\"4.5\" cy=\"3.5\" r=\"1.75\"/>",
				"<circle cx=\"11.5\" cy=\"3.5\" r=\"1.75\"/>",
				"<circle cx=\"4.5\" cy=\"12.5\" r=\"1.75\"/>",
				"<path d=\"m5.25 8.25c3 0 6 .5 6-2.5m-6.5 4.5v-4.5\"/>",
			],
		},
	);

	icons.insert(
		"Coffee",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m10.75 11.25c4.5 0 4.5-5.5 0-5.5h-9v5c0 5 8.5 5 8.5 0v-5\"/>",
				"<path d=\"m8.75 1.75v1.5m-3-1.5v1.5m-3-1.5v1.5\"/>",
			],
		},
	);

	icons.insert(
		"Code",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m5.25 11.25-3.5-3.25 3.5-3.25m5.5 6.5 3.5-3.25-3.5-3.25\"/>"],
		},
	);

	icons.insert(
		"Tick",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<polyline points=\"2.75 8.75,6.25 12.25,13.25 4.75\"/>"],
		},
	);

	icons.insert(
		"Cast",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m1.75 5.25v-2.5h12.5v10.5h-4.5\"/>",
				"<path d=\"m1.75 8.25c2.76142 0 5 2.23858 5 5m-5-2.5c1.38071 0 2.5 1.11929 2.5 2.5m-2.5 0v0\"/>",
			],
		},
	);

	icons.insert(
		"Flag",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m1.75 14.25v-11s2-1.5 4-1.5 2.5 1.5 4.5 1.5 4-1.5 4-1.5v7s-2 1.5-4 1.5-2.5-1.5-4.5-1.5-4 1.5-4 1.5\"/>",
			],
		},
	);

	icons.insert(
		"GitFork",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cx=\"8\" cy=\"12.5\" r=\"1.75\"/>",
				"<circle cx=\"4.5\" cy=\"3.5\" r=\"1.75\"/>",
				"<circle cx=\"11.5\" cy=\"3.5\" r=\"1.75\"/>",
				"<path d=\"m8 8.75v1.5m-3.25-4.5c0 3.5 6.5 3.5 6.5 0\"/>",
			],
		},
	);

	icons.insert(
		"Newspaper",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m11.2 14.2h0.5c1.5 0 2.5-1 2.5-2.5v-6h-3m-9.5-4h9.5v12.5h-7c-1.5 0-2.5-1-2.5-2.5v-9.44z\"/>",
				"<path d=\"m4.75 11.2h3.5\"/>",
				"<rect x=\"4.75\" y=\"4.75\" width=\"3.5\" height=\"3.5\"/>",
			],
		},
	);

	icons.insert(
		"Snowflake",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m13.75 7.75h-12\"/>",
				"<path d=\"m7.75 1.75v12\"/>",
				"<path d=\"m5.25 12.75 2.5-2.5 2.5 2.5\"/>",
				"<path d=\"m2.75 5.25 2.5 2.5-2.5 2.5\"/>",
				"<path d=\"m10.25 2.75-2.5 2.5-2.5-2.5\"/>",
				"<path d=\"m12.75 10.25-2.5-2.5 2.5-2.5\"/>",
			],
		},
	);

	icons.insert(
		"Disc",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cx=\"8\" cy=\"8\" r=\"6.25\"/>",
				"<circle cx=\"8\" cy=\"8\" r=\"1.75\"/>",
			],
		},
	);

	icons.insert(
		"Cube",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"1.75 4.75 8 1.25 14.25 4.75 14.25 11.25 8 14.75 1.75 11.25\"/>",
				"<path d=\"m8 14v-6m5.75-3-5.75 3m-6-3 6 3\"/>",
			],
		},
	);

	icons.insert(
		"MenuHamburger",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m2.75 12.25h10.5m-10.5-4h10.5m-10.5-4h10.5\"/>"],
		},
	);

	icons.insert(
		"Container",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m1.75 12.2 5.5 2 7-4.5v-6l-5.5-2-7 4.5z\"/>",
				"<path d=\"m10.8 6.25v5.5m-3.5-3.5v6m-5.5-8 5.5 2 7-4.5\"/>",
			],
		},
	);

	icons.insert(
		"Cursor",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"1.75 1.75,6.25 14.25,8.75 8.75,14.25 6.25\"/>",
				"<line x1=\"9.25\" y1=\"9.25\" x2=\"13.25\" y2=\"13.25\"/>",
			],
		},
	);

	icons.insert(
		"LayoutDashboard",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"10.5\" width=\"12.5\" y=\"2.75\" x=\"1.75\"/>",
				"<path d=\"m8.25 6.75h5.5m-11.5 2.5h5.5m.25-6v9.5\"/>",
			],
		},
	);

	icons.insert(
		"GitCherryPick",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cy=\"8\" cx=\"5\" r=\"2.25\"/>",
				"<path d=\"m5 10.75v3.5m0-12.5v3.5\"/>",
				"<path d=\"m11.75 8h1.5m-4.5-3.25h1.5l1 3.25-1 3.25h-1.5\"/>",
			],
		},
	);

	icons.insert(
		"Trophy",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"3.5\" width=\"6.5\" y=\"10.75\" x=\"4.75\"/>",
				"<path d=\"m8 8.75v2m-3.25-9c-1.5 0-3 .5-3 2.25s1.5 2.25 3 2.25m6.5-4.5c1.5 0 3 .5 3 2.25s-1.5 2.25-3 2.25m-6.5-4.5h6.5v3.5c0 1.5-1 3-3.25 3s-3.25-1.5-3.25-3z\"/>",
			],
		},
	);

	icons.insert(
		"Hash",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m2.75 10.25h9.5m-8.5-4.5h9.5m-2.5-4-1.5 12.5m-2.5-12.5-1.5 12.5\"/>",
			],
		},
	);

	icons.insert(
		"Plus",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m12.75 7.75h-10m5-5v10\"/>"],
		},
	);

	icons.insert(
		"FaceFrown",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cx=\"8\" cy=\"8\" r=\"6.25\"/>",
				"<path d=\"m9.75 6.25v-.5m-3.5.5v-.5m-.5 5s.5-1 2.25-1 2.25 1 2.25 1\"/>",
			],
		},
	);

	icons.insert(
		"Nut",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"8 1.25 14.25 4.75 14.25 11.25 8 14.75 1.75 11.25 1.75 4.75\"/>",
				"<circle cx=\"8\" cy=\"8\" r=\"2.25\"/>",
			],
		},
	);

	icons.insert(
		"Tent",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"M 5.25,14.25 8,10 l 2.75,4.25\"/>",
				"<path d=\"m9.75 1.75-8 12.5h12.5l-8-12.5\"/>",
			],
		},
	);

	icons.insert(
		"Bluetooth",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m3.75 11.25 8.5-6.25-4.5-3.25v12.5l4.5-3.25-8.5-6.25\"/>"],
		},
	);

	icons.insert(
		"FaceNeutral",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cx=\"8\" cy=\"8\" r=\"6.25\"/>",
				"<path d=\"m9.75 6.25v-.5m-3.5.5v-.5m-.5 4.5h4.5\"/>",
			],
		},
	);

	icons.insert(
		"Headphones",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m1.75 11.75c0-2.5 3.5-2 3.5-2v4.5s-3.5.5-3.5-2.5v-3.5c0-3 .5-6.5 6.25-6.5s6.25 3.5 6.25 6.5v3.5c0 3-3.5 2.5-3.5 2.5v-4.5s3.5-.5 3.5 2\"/>",
			],
		},
	);

	icons.insert(
		"Wifi",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m1.75 4.75 6.25 8.5 6.25-8.5c-3.25-2.75-9.25-2.75-12.5 0z\"/>"],
		},
	);

	icons.insert(
		"Info",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cy=\"8\" cx=\"8\" r=\"6.25\"/>",
				"<path d=\"m8 5.25v0m0 6v-3.5\"/>",
			],
		},
	);

	icons.insert(
		"GrabHorizontal",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cy=\"5.5\" cx=\"2.5\" r=\".75\"/>",
				"<circle cy=\"5.5\" cx=\"8\" r=\".75\"/>",
				"<circle cy=\"5.5\" cx=\"13.5\" r=\".75\"/>",
				"<circle cy=\"10.5\" cx=\"2.5\" r=\".75\"/>",
				"<circle cy=\"10.5\" cx=\"8\" r=\".75\"/>",
				"<circle cy=\"10.5\" cx=\"13.5\" r=\".75\"/>",
			],
		},
	);

	icons.insert(
		"People",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cx=\"5\" cy=\"9\" r=\"2.25\"/>",
				"<circle cx=\"11\" cy=\"4\" r=\"2.25\"/>",
				"<path d=\"m7.75 9.25c0-1 .75-3 3.25-3s3.25 2 3.25 3m-12.5 5c0-1 .75-3 3.25-3s3.25 2 3.25 3\"/>",
			],
		},
	);

	icons.insert(
		"GitMerge",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cx=\"4.5\" cy=\"3.5\" r=\"1.75\"/>",
				"<circle cx=\"4.5\" cy=\"12.5\" r=\"1.75\"/>",
				"<circle cx=\"12.5\" cy=\"8.5\" r=\"1.75\"/>",
				"<path d=\"m4.75 10.25v-4.5c1 2 2 3 5.5 3\"/>",
			],
		},
	);

	icons.insert(
		"GitRequestDraft",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cy=\"12.5\" cx=\"12.5\" r=\"1.75\"/>",
				"<circle cy=\"12.5\" cx=\"3.5\" r=\"1.75\"/>",
				"<circle cy=\"3.5\" cx=\"3.5\" r=\"1.75\"/>",
				"<path d=\"m7.75 2.75h.5m2.5 0h.5m1.5 2.5v-.5m0 3v.5m-9-2.5v4.5\"/>",
			],
		},
	);

	icons.insert(
		"Copy",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m11.25 4.25v-2.5h-9.5v9.5h2.5m.5-6.5v9.5h9.5v-9.5z\"/>"],
		},
	);

	icons.insert(
		"ZoomIn",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cx=\"7.5\" cy=\"7.5\" r=\"4.75\"/>",
				"<path d=\"m9.25 7.49992h-3.5m1.74992-1.74992v3.5m3.75008 2 3 3\"/>",
			],
		},
	);

	icons.insert(
		"Glasses",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cx=\"4\" cy=\"11\" r=\"2.25\"/>",
				"<circle cx=\"12\" cy=\"11\" r=\"2.25\"/>",
				"<path d=\"m14.25 10.75c-1.5-6-2-6.5-3.5-7m-9 7c1.5-6 2-6.5 3.5-7m1 7c1-1 2.5-1 3.5 0\"/>",
			],
		},
	);

	icons.insert(
		"Padlock",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"7.5\" width=\"10.5\" y=\"6.75\" x=\"2.75\"/>",
				"<path d=\"m4.75 6.25s-1-4.5 3.25-4.5 3.25 4.5 3.25 4.5\"/>",
			],
		},
	);

	icons.insert(
		"Refresh",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m4.75 10.75h-3m12.5-2c0 3-2.79822 5.5-6.25 5.5-3.75 0-6.25-3.5-6.25-3.5v3.5m9.5-9h3m-12.5 2c0-3 2.79822-5.5 6.25-5.5 3.75 0 6.25 3.5 6.25 3.5v-3.5\"/>",
			],
		},
	);

	icons.insert(
		"WifiPoor",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m1.75 4.75 6.25 8.5 6.25-8.5c-3.25-2.75-9.25-2.75-12.5 0z\"/>",
				"<path d=\"m5 9c.75-1.75 5.25-1.75 6 0\"/>",
			],
		},
	);

	icons.insert(
		"Image",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect x=\"1.75\" y=\"2.75\" width=\"12.5\" height=\"10.5\"/>",
				"<path d=\"m3.75 13.2 6.5-5.5 4 3\"/>",
				"<circle fill=\"currentColor\" cx=\"5.25\" cy=\"6.25\" r=\".5\"/>",
			],
		},
	);

	icons.insert(
		"LayoutColumns",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"10.5\" width=\"12.5\" y=\"2.75\" x=\"1.75\"/>",
				"<line x1=\"8\" y1=\"3.25\" x2=\"8\" y2=\"12.75\"/>",
			],
		},
	);

	icons.insert(
		"Lightbulb",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m6.75 14.25h2.5m-1.25-12.5c-2.75 0-4.25 2-4.25 4s2 2.5 2 4.5v1h4.5v-1c0-2 2-2.5 2-4.5s-1.5-4-4.25-4z\"/>",
			],
		},
	);

	icons.insert(
		"ThumbDown",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m5.25 10.25c1.5 0 3 4 4.5 4v-4h4.5s-.5-7.5-3.5-7.5h-5.5z\"/>",
				"<path d=\"m5.25 10.25h-3.5v-7.5h3.5\"/>",
			],
		},
	);

	icons.insert(
		"MenuKebab",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cx=\"8\" cy=\"2.5\" r=\".75\"/>",
				"<circle cx=\"8\" cy=\"8\" r=\".75\"/>",
				"<circle cx=\"8\" cy=\"13.5\" r=\".75\"/>",
			],
		},
	);

	icons.insert(
		"Paperclip",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m8.25 10.25v-7s0-1.5-1.75-1.5-1.75 1.5-1.75 1.5v8s0 3 3.25 3 3.25-3 3.25-3v-4.5\"/>",
			],
		},
	);

	icons.insert(
		"CameraVideoSlash",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m11.25 10.75 3 1.5v-7.5l-5 2.5v-2.5h-2.5m1.5 7.5h-6.5v-7.5h1.5\"/>",
				"<line x1=\"1.75\" y1=\"2.25\" x2=\"10.25\" y2=\"14.25\"/>",
			],
		},
	);

	icons.insert(
		"WifiWarning",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m14.25 4.75c-3.25-2.75-9.25-2.75-12.5 0l6.25 8.5 1-1.5\"/>",
				"<path d=\"m12.25 13.75v0m0-6v3.5\"/>",
			],
		},
	);

	icons.insert(
		"Message",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"1.75 14.25,1.75 2.75,14.25 2.75,14.25 11.25,5.75 11.25\"/>",
			],
		},
	);

	icons.insert(
		"ZoomOut",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cx=\"7.5\" cy=\"7.5\" r=\"4.75\"/>",
				"<path d=\"m9.25 7.49992h-3.5m5.5 3.75008 3 3\"/>",
			],
		},
	);

	icons.insert(
		"CameraVideo",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"7.5\" width=\"7.5\" y=\"4.75\" x=\"1.75\"/>",
				"<path d=\"m9.75 7.25 4.5-2.5v7.5l-4.5-2.5\"/>",
			],
		},
	);

	icons.insert(
		"Messages",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"14.25 14.25,14.25 5.25,4.75 5.25,4.75 11.25,10.75 11.25\"/>",
				"<path d=\"m4.75 7.25-3 3v-8.5h10v3\"/>",
			],
		},
	);

	icons.insert(
		"FaceSmile",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cx=\"8\" cy=\"8\" r=\"6.25\"/>",
				"<path d=\"m9.75 6.25v-.5m-3.5.5v-.5m-.5 4s.5 1.5 2.25 1.5 2.25-1.5 2.25-1.5\"/>",
			],
		},
	);

	icons.insert(
		"Square",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<rect height=\"10.5\" width=\"10.5\" y=\"2.75\" x=\"2.75\"/>"],
		},
	);

	icons.insert(
		"BluetoothSearching",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m1.75 11.25 8.5-6.25-4.5-3.25v12.5l4.5-3.25-8.5-6.25\"/>",
				"<path d=\"m13.25 6.25s1 .5 1 1.75-1 1.75-1 1.75m-2-1.75v0\"/>",
			],
		},
	);

	icons.insert(
		"EyeSlash",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m8.75 3.75c3.5.5 5.5 4.25 5.5 4.25s-.5 1.25-1.5 2.25m-2.5 1.5c-6 2-8.5-3.75-8.5-3.75s.5-1.75 3-3.25\"/>",
				"<path d=\"m8.625 9.08253a1.25 1.25 0 0 1 -1.64894 -.36556 1.25 1.25 0 0 1 .22046 -1.67453l.80348.95756z\" fill=\"currentColor\"/>",
				"<path d=\"m3.75 1.75 8.5 12.5\"/>",
			],
		},
	);

	icons.insert(
		"PaperPlane",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"1.75 1.75,14.25 7.75,1.75 14.25,3.25 7.75\"/>",
				"<line x1=\"3.75\" y1=\"7.75\" x2=\"7.25\" y2=\"7.75\"/>",
			],
		},
	);

	icons.insert(
		"GitCommit",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cx=\"8\" cy=\"8\" r=\"2.25\"/>",
				"<path d=\"m8 10.75v3.5m0-12.5v3.5\"/>",
			],
		},
	);

	icons.insert(
		"LayoutStackH",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"10.5\" width=\"12.5\" y=\"2.75\" x=\"1.75\"/>",
				"<line x1=\"2\" y1=\"8\" x2=\"14\" y2=\"8\"/>",
				"<line x1=\"8\" y1=\"8\" x2=\"8\" y2=\"12.75\"/>",
			],
		},
	);

	icons.insert(
		"PhoneCall",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m1.75 1.75c0 8.5 4 12.5 12.5 12.5v-4l-3.5-1-1 1.5c-2 0-4.5-2.5-4.5-4.5l1.5-1-1-3.5z\"/>",
				"<path d=\"m9.75 1.75c2.5 0 4.5 2 4.5 4.5m-4.5-2c1 0 2 1 2 2\"/>",
			],
		},
	);

	icons.insert(
		"Speaker",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"12.5\" width=\"9.5\" y=\"1.75\" x=\"3.25\"/>",
				"<path d=\"m8.25 4.25h-.5\"/>",
				"<circle cx=\"8\" cy=\"9.5\" r=\"2.25\"/>",
			],
		},
	);

	icons.insert(
		"BluetoothConnected",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m3.75 11.25 8.5-6.25-4.5-3.25v12.5l4.5-3.25-8.5-6.25\"/>",
				"<path d=\"m1.75 8h1.5m9.5 0h1.5\"/>",
			],
		},
	);

	icons.insert(
		"Camera",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m1.75 4.75v8.5h12.5v-8.5h-3l-1.5-2h-3.5l-1.5 2z\"/>",
				"<circle cx=\"8\" cy=\"8.5\" r=\"2.25\"/>",
			],
		},
	);

	icons.insert(
		"Printer",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"4.5\" width=\"6.5\" y=\"9.75\" x=\"4.75\"/>",
				"<path d=\"m4.75 4.25v-2.5h6.5v2.5m-7 8h-2.5v-7.5h12.5v7.5h-2.5\"/>",
			],
		},
	);

	icons.insert(
		"FloppyDisk",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"2.75 2.75,2.75 13.25,13.25 13.25,13.25 5.75,10.25 2.75\"/>",
				"<polyline points=\"5.75 13.25,5.75 9.75,10.25 9.75,10.25 13.25\"/>",
			],
		},
	);

	icons.insert(
		"ArrowUpRight",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m5.25 4.25h6.5v6.5m-7.5 1 7.5-7.5\"/>"],
		},
	);

	icons.insert(
		"MediaFastForward",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"8.25 3.75,8.25 12.25,14.25 8\"/>",
				"<polygon points=\"1.75 3.75,1.75 12.25,7.75 8\"/>",
			],
		},
	);

	icons.insert(
		"ShieldKeyhole",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m8 1.75 5.25 2v5c0 2.25-2 4.5-5.25 5.5-3.25-1-5.25-3-5.25-5.5v-5z\"/>",
				"<path d=\"m8 7.25v3\"/>",
				"<circle cx=\"8\" cy=\"6.5\" r=\".75\" fill=\"currentColor\"/>",
			],
		},
	);

	icons.insert(
		"AtSign",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m10.25 8c0 3.25 4 3.25 4 0 0-3.45178-2.7982-6.25-6.25-6.25-3.45178 0-6.25 2.79822-6.25 6.25s2.79822 6.25 6.25 6.25c2.25 0 3.25-1 3.25-1\"/>",
				"<circle cx=\"8\" cy=\"8\" r=\"2.25\"/>",
			],
		},
	);

	icons.insert(
		"Infinity",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m5 5c2.5 1 3.5 5 6 6s3.25-1.25 3.25-3-.75-4-3.25-3-3.5 5-6 6-3.25-1.25-3.25-3 .75-4 3.25-3z\"/>",
			],
		},
	);

	icons.insert(
		"Gem",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<polygon points=\"4.75 2.75,11.25 2.75,14.25 6.25,8 13.25,1.75 6.25\"/>"],
		},
	);

	icons.insert(
		"GitCompare",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cx=\"12.5\" cy=\"12.5\" r=\"1.75\"/>",
				"<circle cx=\"3.5\" cy=\"3.5\" r=\"1.75\"/>",
				"<path d=\"m3.75 5.75v5c0 1 .5 1.5 1.5 1.5h2m-.5 2 1.5-2-1.5-2m5.5 0v-5c0-1-.5-1.5-1.5-1.5h-2m.5-2-1.5 2 1.5 2\"/>",
			],
		},
	);

	icons.insert(
		"ChartBar",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m1.75 1.75v12.5h12.5m-9-3v-2.5m4 2.5v-5.5m4 5.5v-8.5\"/>"],
		},
	);

	icons.insert(
		"Star",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"8 1.75,5.75 5.75,1.75 6.25,4.75 9.75,3.75 14.25,8.00 12.25,12.25 14.25,11.25 9.75,14.25 6.25,10.25 5.75\"/>",
			],
		},
	);

	icons.insert(
		"Sun",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cy=\"8\" cx=\"8\" r=\"3.25\"/>",
				"<path d=\"m2.75 13.25.5-.5m9.5 0 .5.5m-.5-10 .5-.5m-10 .5-.5-.5m-.50 5.25h-1m13.5 0h-1m-5.75 5.75v1m0-13.5v1\"/>",
			],
		},
	);

	icons.insert(
		"TreeFir",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m8 1.75-4.25 5.5h2.5l-3.5 4h4v3h2.5v-3h4l-3.5-4h2.5z\"/>"],
		},
	);

	icons.insert(
		"CircleTick",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m14.25 8.75c-.5 2.5-2.3849 4.85363-5.03069 5.37991-2.64578.5263-5.33066-.7044-6.65903-3.0523-1.32837-2.34784-1.00043-5.28307.81336-7.27989 1.81379-1.99683 4.87636-2.54771 7.37636-1.54771\"/>",
				"<polyline points=\"5.75 7.75,8.25 10.25,14.25 3.75\"/>",
			],
		},
	);

	icons.insert(
		"Anchor",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m8 5.75v8.25m-4.75-6.25h-1.5c0 4 2.5 6.5 6.25 6.5s6.25-2.5 6.25-6.5h-1.5\"/>",
				"<circle cx=\"8\" cy=\"3.5\" r=\"1.75\"/>",
			],
		},
	);

	icons.insert(
		"ChevronsUp",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m12.25 12.25-4.25-4.5-4.25 4.5m8.5-5l-4.25-4.5-4.25 4.5\"/>"],
		},
	);

	icons.insert(
		"Bug",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cx=\"8\" cy=\"10\" r=\"4.25\"/>",
				"<path d=\"m14.25 10.25h-1.5m-1 2.5 1.5 1.5m0-8.5-1.5 1.5m-10 3h1.5m1 2.5-1.5 1.5m0-8.5 1.5 1.5m1.5-1.5s-.75-3 2.25-3 2.25 3 2.25 3\"/>",
			],
		},
	);

	icons.insert(
		"Compass",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cy=\"8\" cx=\"8\" r=\"6.25\"/>",
				"<polygon points=\"6.75 6.75,5.75 10.75,9.25 9.25,10.25 5.25\"/>",
			],
		},
	);

	icons.insert(
		"ConicalFlask",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m4.75 1.75h6.5m-6.5 8h6.5m-5.5-7.5v4.5l-4 7.5h12.5l-4-7.5v-4.5\"/>",
			],
		},
	);

	icons.insert(
		"CreditCard",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"9.5\" width=\"12.5\" y=\"3.75\" x=\"1.75\"/>",
				"<path d=\"m9.75 10.25h1.5m-9-3h11.5\"/>",
			],
		},
	);

	icons.insert(
		"Copyleft",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cy=\"8\" cx=\"8\" r=\"6.25\"/>",
				"<path d=\"m6 6.75s.75-1 2-1 2.25 1 2.25 2.25-1 2.25-2.25 2.25-2-1-2-1\"/>",
			],
		},
	);

	icons.insert(
		"LightningBolt",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"9.25 1.75,2.75 9.25,7.25 9.75,6.75 14.25,13.25 6.75,8.75 6.25\"/>",
			],
		},
	);

	icons.insert(
		"Pencil",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"1.75 11.25,1.75 14.25,4.75 14.25,14.25 4.75,11.25 1.75\"/>",
				"<line x1=\"8.75\" y1=\"4.75\" x2=\"11.25\" y2=\"7.25\"/>",
			],
		},
	);

	icons.insert(
		"TickDouble",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m1.75 9.75 2.5 2.5m3.5-4 2.5-2.5m-4.5 4 2.5 2.5 6-6.5\"/>"],
		},
	);

	icons.insert(
		"MonitorArrow",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m14.2 7.75v3.5h-12.5v-9.5h6.5\"/>",
				"<path d=\"m4.75 14.2h6.5m-3.25-2.5v2.5\"/>",
				"<path d=\"m9.75 6.25 4.5-4.5m-3.5-0.5h4v4\"/>",
			],
		},
	);

	icons.insert(
		"Diamond",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<polygon points=\"1.25 8,8 14.75,14.75 8,8 1.25\"/>"],
		},
	);

	icons.insert(
		"MediaEject",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"2.75 11.25,13.25 11.25,8 2.75\"/>",
				"<line x1=\"13.25\" y1=\"14.25\" x2=\"2.75\" y2=\"14.25\"/>",
			],
		},
	);

	icons.insert(
		"Clock",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cy=\"8\" cx=\"8\" r=\"6.25\"/>",
				"<path d=\"m8.25 4.75v3.5l-2.5 2\"/>",
			],
		},
	);

	icons.insert(
		"Octagon",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"5.25 1.75,10.75 1.75,14.25 5.25,14.25 10.75,10.75 14.25,5.25 14.25,1.75 10.75,1.75 5.25\"/>",
			],
		},
	);

	icons.insert(
		"Phone",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m1.75 1.75c0 8.5 4 12.5 12.5 12.5v-4l-3.5-1-1 1.5c-2 0-4.5-2.5-4.5-4.5l1.5-1-1-3.5z\"/>",
			],
		},
	);

	icons.insert(
		"Eye",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m1.75 8s2-4.25 6.25-4.25 6.25 4.25 6.25 4.25-2 4.25-6.25 4.25-6.25-4.25-6.25-4.25z\"/>",
				"<circle cx=\"8\" cy=\"8\" r=\"1.25\" fill=\"currentColor\"/>",
			],
		},
	);

	icons.insert(
		"Mobile",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"12.5\" width=\"8.5\" y=\"1.75\" x=\"3.75\"/>",
				"<path d=\"m8.25 11.75h-.5\"/>",
			],
		},
	);

	icons.insert(
		"SignOut",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m5.25 2.25h-3.5v12h3.5m5.5-9.5 3.5 3.5-3.5 3.5m-5-3.5h8.5\"/>"],
		},
	);

	icons.insert(
		"FileBinary",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polyline points=\"2.75 7.75,2.75 1.75,8.25 1.75,13.25 6.75,13.25 14.25\"/>",
				"<rect x=\"1.75\" y=\"10.8\" width=\"3\" height=\"3.5\"/>",
				"<path d=\"m7.25 14.2h3m-3-3.5h1.5v3\"/>",
				"<polyline points=\"7.75 2.25 7.75 7.25 12.8 7.25\"/>",
			],
		},
	);

	icons.insert(
		"CircleCross",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m10.25 5.75-4.5 4.5m0-4.5 4.5 4.5\"/>",
				"<circle cx=\"8\" cy=\"8\" r=\"6.25\"/>",
			],
		},
	);

	icons.insert(
		"Bin",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m5.75 4.25v-2.5h4.5v2.5m-6.5 1v9h8.5v-9m-9.5-.5h10.5\"/>"],
		},
	);

	icons.insert(
		"Gift",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"3.5\" width=\"12.5\" y=\"4.75\" x=\"1.75\"/>",
				"<path d=\"m10.25 4.75h-2.25c0-2 .5-3 2.25-3 2 0 2 3 0 3zm-4.5 0h2.25c0-2-.5-3-2.25-3-2 0-2 3 0 3zm2.25 9v-8.75m-5.25 3.75v5.5h10.5v-5.5\"/>",
			],
		},
	);

	icons.insert(
		"Binary",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"4.5\" width=\"3\" y=\"1.75\" x=\"3.25\"/>",
				"<path d=\"m9.75 6.25h3m-3-4.5h1.5v4\"/>",
				"<rect height=\"4.5\" width=\"3\" y=\"9.75\" x=\"9.75\"/>",
				"<path d=\"m3.25 14.25h3m-3-4.5h1.5v4\"/>",
			],
		},
	);

	icons.insert(
		"Share",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cx=\"4\" cy=\"8\" r=\"2.25\"/>",
				"<circle cx=\"12\" cy=\"12\" r=\"2.25\"/>",
				"<circle cx=\"12\" cy=\"4\" r=\"2.25\"/>",
				"<path d=\"m6 9 4 2m-4-4 4-2\"/>",
			],
		},
	);

	icons.insert(
		"CircleWarning",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cx=\"8\" cy=\"8\" r=\"6.25\"/>",
				"<path d=\"m8 10.75v0m0-6v3.5\"/>",
			],
		},
	);

	icons.insert(
		"ArrowUp",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m3.75 7.25 4.5-4.5 4.5 4.5m-4.5 6v-10.5\"/>"],
		},
	);

	icons.insert(
		"SoundDown",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"1.75 5.75 1.75 10.25 4.25 10.25 8.25 13.25 8.25 2.75 4.25 5.75\"/>",
				"<path d=\"m10.75 6.25s1 .5 1 1.75-1 1.75-1 1.75\"/>",
			],
		},
	);

	icons.insert(
		"MediaPlay",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<polygon points=\"2.75 2.75,2.75 13.25,12.25 8\"/>"],
		},
	);

	icons.insert(
		"SquareTick",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polyline points=\"10.25 2.75,2.75 2.75,2.75 13.25,13.25 13.25,13.25 9.75\"/>",
				"<polyline points=\"5.75 7.75,8.25 10.25,14.25 3.75\"/>",
			],
		},
	);

	icons.insert(
		"Stack",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m1.75 11 6.25 3.25 6.25-3.25m-12.5-3 6.25 3.25 6.25-3.25m-6.25-6.25-6.25 3.25 6.25 3.25 6.25-3.25z\"/>",
			],
		},
	);

	icons.insert(
		"Robot",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"7.5\" width=\"12.5\" y=\"5.75\" x=\"1.75\"/>",
				"<path d=\"m10.75 8.75v1.5m-5.5-1.5v1.5m-.5-7.5 3.25 3 3.25-3\"/>",
			],
		},
	);

	icons.insert(
		"MapPin",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m13.25 7c0 3.75-5.25 7.25-5.25 7.25s-5.25-3.5-5.25-7.25c0-2.89949 2.35051-5.25 5.25-5.25 2.8995 0 5.25 2.35051 5.25 5.25z\"/>",
				"<circle cx=\"8\" cy=\"7\" r=\"1.25\" fill=\"currentColor\"/>",
			],
		},
	);

	icons.insert(
		"MediaBack",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"13.25 13.25,4.75 8,13.25 2.75\"/>",
				"<line x1=\"1.75\" y1=\"3.75\" x2=\"1.75\" y2=\"12.25\"/>",
			],
		},
	);

	icons.insert(
		"Filter",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"1.75 1.75,14.25 1.75,14.25 3.25,9.25 8.75,9.25 12.75,6.75 14.25,6.75 8.75,1.75 3.25\"/>",
			],
		},
	);

	icons.insert(
		"GitRequest",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cx=\"12.5\" cy=\"12.5\" r=\"1.75\"/>",
				"<circle cx=\"3.5\" cy=\"12.5\" r=\"1.75\"/>",
				"<circle cx=\"3.5\" cy=\"3.5\" r=\"1.75\"/>",
				"<path d=\"m9.25 1.75-1.5 2 1.5 2m3 4.5v-5c0-1-.5-1.5-1.5-1.5h-2m-5 2v4.5\"/>",
			],
		},
	);

	icons.insert(
		"PhoneIncoming",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m1.75 1.75c0 8.5 4 12.5 12.5 12.5v-4l-3.5-1-1 1.5c-2 0-4.5-2.5-4.5-4.5l1.5-1-1-3.5z\"/>",
				"<path d=\"m13.25 2.75-3.5 3.5m0-3v3h3\"/>",
			],
		},
	);

	icons.insert(
		"BellSlash",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m5.75 12.25c0 3 4.5 3 4.5 0\"/>",
				"<path d=\"m12.25 8.25v-3c0-2-1.16605-3.5-4.25-3.5m-3.75 2c-.530590.584957-.5.674089-.5 1.5v3l-2 3.5h8.5\"/>",
				"<path d=\"m5.75 12.25c0 3 4.5 3 4.5 0\"/>",
				"<path d=\"m2.75 1.75 10.5 12.5\"/>",
			],
		},
	);

	icons.insert(
		"ChevronsLeft",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m12.25 3.75-4.5 4.25l4.5 4.25m-5-8.5-4.5 4.25 4.5 4.25\"/>"],
		},
	);

	icons.insert(
		"Calendar",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"10.5\" width=\"12.5\" y=\"3.75\" x=\"1.75\"/>",
				"<path d=\"m11.25 1.75v1.5m-6.5-1.5v1.5m-2.5 4h11.5\"/>",
			],
		},
	);

	icons.insert(
		"Globe",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cy=\"8\" cx=\"8\" r=\"6.25\"/>",
				"<path d=\"m2 8.25h12\"/>",
				"<path d=\"m8.25 14.2c2.75-3.2 2.75-9.2 0-12.4\"/>",
				"<path d=\"m7.75 14.2c-2.75-3.2-2.75-9.2 0-12.4\"/>",
			],
		},
	);

	icons.insert(
		"ArrowLeft",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m7.25 3.75-4.5 4.5 4.5 4.5m6-4.5h-10.5\"/>"],
		},
	);

	icons.insert(
		"Cross",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m11.25 4.75-6.5 6.5m0-6.5 6.5 6.5\"/>"],
		},
	);

	icons.insert(
		"LayoutRows",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"10.5\" width=\"12.5\" y=\"2.75\" x=\"1.75\"/>",
				"<line x1=\"2\" y1=\"8\" x2=\"14\" y2=\"8\"/>",
			],
		},
	);

	icons.insert(
		"AppsMinus",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect x=\"1.75\" y=\"1.75\" width=\"4.5\" height=\"4.5\"/>",
				"<rect x=\"1.75\" y=\"9.75\" width=\"4.5\" height=\"4.5\"/>",
				"<rect x=\"9.75\" y=\"9.75\" width=\"4.5\" height=\"4.5\"/>",
				"<path d=\"m14.8 3.75h-5\"/>",
			],
		},
	);

	icons.insert(
		"ArrowDownRight",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m5.25 11.75h6.5v-6.5m-7.5-1 7.5 7.5\"/>"],
		},
	);

	icons.insert(
		"Laptop",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"7.5\" width=\"10.5\" y=\"2.75\" x=\"2.75\"/>",
				"<path d=\"m2.75 10.25-1 3h12.5l-1-3\"/>",
			],
		},
	);

	icons.insert(
		"GitRequestCross",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cy=\"12.5\" cx=\"12.5\" r=\"1.75\"/>",
				"<circle cy=\"12.5\" cx=\"3.5\" r=\"1.75\"/>",
				"<circle cy=\"3.5\" cx=\"3.5\" r=\"1.75\"/>",
				"<path d=\"m12.25 7.25v3m-8.5-4.5v4.5\"/>",
				"<path d=\"m14.25 1.75-3.5 3.5m0-3.5 3.5 3.5\"/>",
			],
		},
	);

	icons.insert(
		"FileCode",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polyline points=\"2.75 7.75,2.75 1.75,8.25 1.75,13.25 6.75,13.25 14.25,11.25 14.25\"/>",
				"<polyline points=\"7.75 2.25 7.75 7.25 12.8 7.25\"/>",
				"<path d=\"m6.75 10.8 2 1.75-2 1.75m-3-3.5-2 1.75 2 1.75\"/>",
			],
		},
	);

	icons.insert(
		"SignIn",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m5.25 2.25h-3.5v12h3.5m4-9.5-3.5 3.5 3.5 3.5m5-3.5h-8.5\"/>"],
		},
	);

	icons.insert(
		"Cloud",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m7 3.75c-1.79493 0-3.25 1.45507-3.25 3.25.00152.254757.032983.508452.09375.755859h-.00195c-1.17822.08305-2.09165 1.063-2.0918 2.24414 0 1.24264 1.00736 2.25 2.25 2.25h7.5c1.51878 0 2.75-1.23122 2.75-2.75s-1.2312-2.75-2.75-2.75c-.4352-.00022-.8643.10286-1.252.30078.0008-.01692.0015-.03385.0020-.05078 0-1.79493-1.45507-3.25-3.25-3.25z\"/>",
			],
		},
	);

	icons.insert(
		"Quote",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m6.25 3.75h-4.5v5.5c0 3.5 2.5 4.5 4.5 4-1.5-1.5-1.5-2.5-1.5-4h1.5z\"/>",
				"<path d=\"m13.25 3.75h-4.5v5.5c0 3.5 2.5 4.5 4.5 4-1.5-1.5-1.5-2.5-1.5-4h1.5z\"/>",
			],
		},
	);

	icons.insert(
		"Upload",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m3.75 2.75h9m-8.5 6.5 4-3.5 4 3.5m-4 5v-8.5\"/>"],
		},
	);

	icons.insert(
		"Forward",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m1.75 13.25c.5-6 5.5-7.5 8-7v-3.5l4.5 5.25-4.5 5.25v-3.5c-2.5-0.5-6.5 0.5-8 3.5z\"/>",
			],
		},
	);

	icons.insert(
		"CircleMinus",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cx=\"8\" cy=\"8\" r=\"6.25\"/>",
				"<line x1=\"4.75\" y1=\"8\" x2=\"11.25\" y2=\"8\"/>",
			],
		},
	);

	icons.insert(
		"Bookmark",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<polygon points=\"3.75 1.75,12.25 1.75,12.25 14.25,8 9.75,3.75 14.25\"/>"],
		},
	);

	icons.insert(
		"Help",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cy=\"8\" cx=\"8\" r=\"6.25\"/>",
				"<path d=\"m5.75 6.75c0-1 1-2 2.25-2s2.25 1.0335 2.25 2c0 1.5-1.5 1.5-2.25 2m0 2.5v0\"/>",
			],
		},
	);

	icons.insert(
		"Rocket",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m4.25 9.75-2-.5s0-1.5.5-3 4-1.5 4-1.5m-.50 7l.5 2s1.5 0 3-.5 1.5-4 1.5-4m-7 .5 2 2s5-2 6.5-4.5 1.5-5.5 1.5-5.5-3 0-5.5 1.5-4.5 6.5-4.5 6.5z\"/>",
				"<path d=\"m1.75 14.25 2-1-1-1z\" fill=\"currentColor\"/>",
				"<circle cx=\"10.25\" cy=\"5.75\" r=\".5\" fill=\"currentColor\"/>",
			],
		},
	);

	icons.insert(
		"Tablet",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"12.5\" width=\"10.5\" y=\"1.75\" x=\"2.75\"/>",
				"<path d=\"m8.25 11.75h-.5\"/>",
			],
		},
	);

	icons.insert(
		"Flame",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m8.25 7.75c2 2 2.5-2.5 3.5-2s1.5 2 1.5 3.25c0 3.25-2.35 5.25-5.25 5.25s-5.25-2.5-5.25-6 3.5-7 5.5-7c0 0-2 4.5 0 6.5z\"/>",
			],
		},
	);

	icons.insert(
		"Atom",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<ellipse transform=\"rotate(45)\" cx=\"11.3\" rx=\"8.28\" ry=\"3.17\"/>",
				"<ellipse transform=\"rotate(315)\" cy=\"11.3\" rx=\"8.28\" ry=\"3.17\"/>",
				"<path d=\"m8 8v0\"/>",
			],
		},
	);

	icons.insert(
		"Sword",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m2.75 9.25 1.5 2.5 2 1.5m-4.5 0 1 1m1.5-2.5-1.5 1.5m3-1 8.5-8.5v-2h-2l-8.5 8.5\"/>",
			],
		},
	);

	icons.insert(
		"MenuMeatball",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cx=\"2.5\" cy=\"8\" r=\".75\"/>",
				"<circle cx=\"8\" cy=\"8\" r=\".75\"/>",
				"<circle cx=\"13.5\" cy=\"8\" r=\".75\"/>",
			],
		},
	);

	icons.insert(
		"StickyNote",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"9.25 13.25,2.75 13.25,2.75 2.75,13.25 2.75,13.25 9.25\"/>",
				"<polyline points=\"8.75 13.25,8.75 8.75,13.25 8.75\"/>",
			],
		},
	);

	icons.insert(
		"Id",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"10.5\" width=\"12.5\" y=\"2.75\" x=\"1.75\"/>",
				"<circle cy=\"7.5\" cx=\"8\" r=\"2.25\"/>",
				"<path d=\"m4.75 12.75c0-1 .75-3 3.25-3s3.25 2 3.25 3\"/>",
			],
		},
	);

	icons.insert(
		"PlantPot",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m8.75 6.75c0 1.25-.75 3-.75 3m.25-2.5s.75-2-1-3.5-4.5-1-4.5-1 0 2 1.5 3.5 4 1 4 1zm.5-1s-.75-2 1-3.5 4.5-1 4.5-1 0 2-1.5 3.5-4 1-4 1z\"/>",
				"<path d=\"m4.75 9.75h6.5s.5 4.5-3.25 4.5-3.25-4.5-3.25-4.5z\"/>",
			],
		},
	);

	icons.insert(
		"Block",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<circle cy=\"8\" cx=\"8\" r=\"6.25\"/>",
				"<line x1=\"4.25\" x2=\"12.25\" y1=\"11.75\" y2=\"3.75\"/>",
			],
		},
	);

	icons.insert(
		"RotateAntiClockwise",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m4.75 5.25h-3m0 3.5c0 2.5 2.79822 5.5 6.25 5.5s6.25-2.79822 6.25-6.25-2.79822-6.25-6.25-6.25c-3.75 0-6.25 3.5-6.25 3.5v-3.5\"/>",
			],
		},
	);

	icons.insert(
		"PhoneCross",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m1.75 1.75c0 8.5 4 12.5 12.5 12.5v-4l-3.5-1-1 1.5c-2 0-4.5-2.5-4.5-4.5l1.5-1-1-3.5z\"/>",
				"<path d=\"m13.25 2.75-3.5 3.5m0-3.5 3.5 3.5\"/>",
			],
		},
	);

	icons.insert(
		"Cards",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"11.5\" width=\"8.25\" y=\"2.75\" x=\"1.75\"/>",
				"<path d=\"m10 3.75 4.25 2-4.25 7.5\"/>",
			],
		},
	);

	icons.insert(
		"Folder",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polygon points=\"1.75 2.75,1.75 13.25,14.25 13.25,14.25 4.75,8.25 4.75,6.75 2.75\"/>",
			],
		},
	);

	icons.insert(
		"Pulse",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polyline points=\"1.75 8.25, 4.25 8.25, 6.25 3.75, 9.75 12.25, 11.75 8.25, 14.25 8.25\"/>",
			],
		},
	);

	icons.insert(
		"Gamepad",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m3.25 3.75c-2 5-2 9 0 9.5s2.5-2 2.5-2h4.5s.5 2.5 2.5 2 2-4.5 0-9.5h-2l-1 1h-3.5l-1-1z\"/>",
			],
		},
	);

	icons.insert(
		"Monitor",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<rect height=\"9.5\" width=\"12.5\" y=\"1.75\" x=\"1.75\"/>",
				"<path d=\"m4.75 14.25h6.5m-3.25-2.5v2.5\"/>",
			],
		},
	);

	icons.insert(
		"NotesCross",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<polyline points=\"8.25 14.25,2.75 14.25,2.75 1.75,13.25 1.75,13.25 8.25\"/>",
				"<path d=\"m14.25 10.75-3.5 3.5m-5-6.5h4.5m-4.5 3h1.5m-1.5-6h4.5m.5 6 3.5 3.5\"/>",
			],
		},
	);

	icons.insert(
		"SquareCross",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m10.25 5.75-4.5 4.5m0-4.5 4.5 4.5\"/>",
				"<rect height=\"10.5\" width=\"10.5\" y=\"2.75\" x=\"2.75\"/>",
			],
		},
	);

	icons.insert(
		"Minus",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m13.25 7.75h-10.5\"/>"],
		},
	);

	icons.insert(
		"ChevronLeft",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec!["<path d=\"m10.25 3.75-4.5 4.25l4.5 4.25\"/>"],
		},
	);

	icons.insert(
		"Moon",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m1.75 8c0 3.45 2.8 6.25 6.25 6.25 3.41-.0027 6.25-3 6.25-6-1 .5-4 1.5-6-.5s-1-5-.5-6c-3 0-6 2.84-6 6.25z\"/>",
			],
		},
	);

	icons.insert(
		"Heart",
		Icon {
			view_box: "0 0 16 16",
			class: "",
			width: "16",
			height: "16",
			stroke: "currentColor",
			fill: "none",
			stroke_width: "1.5",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			nodes: vec![
				"<path d=\"m3.25 9.75c3 3.5 4.75 4.5 4.75 4.5s1.75-1 4.75-4.5 1-7-1.5-7-3.25 3-3.25 3-.75-3-3.25-3-4.5 3.5-1.5 7z\"/>",
			],
		},
	);

	icons
});
