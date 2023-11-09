use icon_sets::Icon;
use leptos::{html::Div, *};
use leptos_use::use_element_visibility;
use web_sys::MouseEvent;

fn main() {
	mount_to_body(|| {
		view! {
			<div class="p-12">
		<Icons />
			</div>
		}
	})
}

// TODO remove this when search is implemented
#[allow(clippy::unnecessary_filter_map)]
#[component]
pub fn Icons() -> impl IntoView {
	// TODO filter for search functionoality
	let sections = move || {
		icon_sets::ICON_SETS
			.iter()
			.filter_map(|(key, icons)| {
				let icons = Signal::derive(move || {
					icons
						.iter()
						.filter_map(|(name, icon)| Some((name.to_string(), icon.to_owned())))
						.collect::<Vec<_>>()
				});
				Some((key.to_string(), icons))
			})
			.collect::<Vec<_>>()
	};

	view! {
	<div class="flex flex-col gap-8">
	  <For
		each=sections
		key=|set| set.0.clone()
		let:set
	  >
		<Section section=set.0 map=set.1 />
	  </For>
	</div>
	}
}

#[component]
fn Section(section: String, map: Signal<Vec<(String, Icon)>>) -> impl IntoView {
	let key = {
		let section = section.clone();
		move |icon: &(String, Icon)| format!("{}_{}", section, icon.0)
	};

	view! {
	<section>
	  <h2 class="sticky top-0 text-3xl px-2">{section}</h2>
	  <div class="grid grid-cols-autofill gap-2">
		<For
		  each=move || map.get()
		  key
		  let:icon
		>
		  <Icon icon_name=icon.0 icon={icon.1.clone()} size="24px" />
		</For>
	  </div>
	</section>
	}
}

#[component]
fn Icon(
	icon_name: String,
	icon: Icon,
	#[prop(into)]
	#[prop(optional)]
	class: MaybeSignal<String>,
	#[prop(into)] size: MaybeSignal<String>,
) -> impl IntoView {
	let container = create_node_ref::<Div>();
	let is_container_visible = use_element_visibility(container);

	let render_visible = {
		let icon = icon.clone();
		let class = class.clone();
		let size = size.clone();
		let class = move || format!("{} {}", icon.class.unwrap_or_default(), class.get());
		move || {
			if is_container_visible.get() {
				let copy_name = {
					let icon_name = icon_name.clone();
					move |_: MouseEvent| {
						let clipboard = window().navigator().clipboard().unwrap();
						let _ = clipboard.write_text(&icon_name);
					}
				};

				let svg = view! {
				  <svg
				   xmlns="http://www.w3.org/2000/svg"
				   class={class.clone()}
				   width={size.clone()}
				   height={size.clone()}
				   viewbox={icon.view_box}
				   fill={icon.fill}
				   stroke={icon.stroke}
				   stroke-width={icon.stroke_width}
				   stroke-linecap={icon.stroke_linecap}
				   stroke-linejoin={icon.stroke_linejoin}
				  >

				  </svg>
				};
				let svg = svg.inner_html(icon.nodes.join("\n"));

				view! {
					 <div class="tooltip tooltip-bottom w-full h-full" data-tip={&icon_name} on:click=copy_name>
					   <button class="w-full h-full btn btn-xs btn-ghost p-2 flex items-center justify-center">
				{svg}
					   </button>
					 </div>
				   }
				.into_view()
			} else {
				view! {}.into_view()
			}
		}
	};

	view! {
		<div class="w-24 h-24" node_ref=container>
			{render_visible}
		</div>
	}
}
