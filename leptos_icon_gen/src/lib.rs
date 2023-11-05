extern crate proc_macro;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
	parenthesized,
	parse::{Parse, ParseStream},
	parse_macro_input,
	punctuated::Punctuated,
	Ident, Token, Visibility,
};

struct IconComponent {
	vis: Visibility,
	component_name: Ident,
	icon_set: Ident,
	icon_name: Ident,
}

impl Parse for IconComponent {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let vis = input.parse::<Visibility>()?;
		let component_name = input.parse()?;

		let contents;
		parenthesized!(contents in input);
		let icon_set = contents.parse()?;
		contents.parse::<Token![=>]>()?;
		let icon_name = contents.parse()?;

		Ok(IconComponent {
			vis,
			component_name,
			icon_set,
			icon_name,
		})
	}
}

fn generate_component(component: IconComponent) -> TokenStream {
	let IconComponent {
		vis,
		component_name,
		icon_set,
		icon_name,
	} = component;

	let icon_set_key = icon_set.to_string();
	let icon_name_key = icon_name.to_string();

	let icon = icon_sets::ICON_SETS
		.get(icon_set_key.as_str())
		.unwrap_or_else(|| panic!("invalid icon set {icon_set_key}"))
		.get(icon_name_key.as_str())
		.unwrap_or_else(|| panic!("failed to find icon {icon_name_key}"));

	let class = icon.class;
	let icon_view_box = icon.view_box;

	let width_comment = format!("Sets the width. Defaults to \"{}\"", icon.width);
	let width = icon.width;
	let height_comment = format!("Sets the height. Defaults to \"{}\"", icon.height);
	let height = icon.height;
	let stroke_comment = format!("Sets the stroke. Defaults to \"{}\"", icon.stroke);
	let stroke = icon.stroke;
	let fill_comment = format!("Sets the fill. Defaults to \"{}\"", icon.fill);
	let fill = icon.fill;
	let stroke_width_comment = format!(
		"Sets the stroke-width. Defaults to \"{}\"",
		icon.stroke_width
	);
	let stroke_width = icon.stroke_width;
	let stroke_linecap_comment = format!(
		"Sets the stroke-linecap. Defaults to \"{}\"",
		icon.stroke_linecap
	);
	let stroke_linecap = icon.stroke_linecap;
	let stroke_linejoin_comment = format!(
		"Sets the stroke-linejoin. Defaults to \"{}\"",
		icon.stroke_linejoin
	);
	let stroke_linejoin = icon.stroke_linejoin;

	let icon_content = icon
		.nodes
		.iter()
		.map(|icon| icon.parse().unwrap())
		.collect::<Vec<TokenStream>>();

	quote! {
		#[::leptos::component]
		#vis fn #component_name(
			/// Set the class attribute
			#[prop(into)]
			#[prop(optional)]
			class: ::leptos::MaybeSignal<String>,
			/// Set the style attribute
			#[prop(into)]
			#[prop(optional)]
			style: ::leptos::MaybeSignal<String>,
			#[doc=#width_comment]
			#[prop(into)]
			#[prop(default = #width.into())]
			width: ::leptos::MaybeSignal<String>,
			#[doc=#height_comment]
			#[prop(into)]
			#[prop(default = #height.into())]
			height: ::leptos::MaybeSignal<String>,
			#[doc=#fill_comment]
			#[prop(into)]
			#[prop(default = #fill.into())]
			fill: ::leptos::MaybeSignal<String>,
			#[doc=#stroke_comment]
			#[prop(into)]
			#[prop(default = #stroke.into())]
			stroke: ::leptos::MaybeSignal<String>,
			#[doc=#stroke_width_comment]
			#[prop(into)]
			#[prop(default = #stroke_width.into())]
			stroke_width: ::leptos::MaybeSignal<String>,
			#[doc=#stroke_linecap_comment]
			#[prop(into)]
			#[prop(default = #stroke_linecap.into())]
			stroke_linecap: ::leptos::MaybeSignal<String>,
			#[doc=#stroke_linejoin_comment]
			#[prop(into)]
			#[prop(default = #stroke_linejoin.into())]
			stroke_linejoin: ::leptos::MaybeSignal<String>,
		) -> impl ::leptos::IntoView {
			let class = move || format!("{} {}", #class, class.get());

			::leptos::view! {
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class={class}
					style={style}
					width={width}
					height={height}
					viewBox=#icon_view_box
					fill={fill}
					stroke={stroke}
					stroke-width={stroke_width}
					stroke-linecap={stroke_linecap}
					stroke-linejoin={stroke_linejoin}
				>
					#(#icon_content)*
				</svg>
			}
		}
	}
}

/// Generate a single icon component for an icon set.
///
/// For example, to generate the home icon from the Lucide icon set, call this macro with an optional visibility modifer, the desired name of the generated icon component and within parenthesis, the icon set and target icon name in pascal case:
///
/// ```
/// use leptos_icon_gen::leptos_icon;
///
/// leptos_icon!(pub HomeIcon(lucide => Home));
/// // leptos_icon!(pub(crate) HomeIcon(lucide => Home));
/// // leptos_icon!(HomeIcon(lucide => Home));
/// ```
///
/// To use the generated icon above, refer to it with the given name:
///
/// ```ignore
/// view! {
///   <HomeIcon />
/// }
/// ```
///
/// A downside of generating just the required icons is that no component documentation exists until the icon is generated. The following is a list of properties that can be set on a generated icon component:
///
/// - class (Optional): Sets the class attribute.
/// - style (Optional): Sets the style attribute.
/// - width (Optional): Sets the width attribute.
/// - height (Optional): Sets the height attribute.
/// - stroke (Optional): Sets the stroke attribute.
/// - fill (Optional): Sets the fill attribute.
/// - stroke_width (Optional): Sets the stroke-width attribute.
/// - stroke_linecap (Optional): Sets the stroke-linecap attribute.
/// - stroke_linejoin (Optional): Sets the stroke-linejoin attribute.
///
/// Except for the `class` and `style`, the properties all receive default values configured per icon variation within a set. For example, Heroicons have a mini set which has a default width and height of "20", whereas the solid and outline variations have "24".
#[proc_macro]
pub fn leptos_icon(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	generate_component(parse_macro_input!(input as IconComponent)).into()
}

struct IconComponentList(Punctuated<IconComponent, Token![,]>);

impl Parse for IconComponentList {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		Ok(Self(
			input.parse_terminated(IconComponent::parse, Token![,])?,
		))
	}
}

/// Generate a icon components from a list of icon set combinations.
///
/// It is possible to generate icons from multiple icon sets in a single invocation if desired.
///
/// For example, to generate the home icon from the Lucide icon set, call this macro with a list of configurations in the same format as [`leptos_icon!`]:
///
/// ```
/// use leptos_icon_gen::leptos_icons;
///
/// leptos_icons! {
///   pub HomeIcon(lucide => Home),
///   pub MenuIcon(lucide => Menu),
///   pub ChatBubbleMiniIcon(heroicons => ChatBubbleLeftRightMini),
/// };
/// ```
///
/// Usage of icons is identical to those generated by [`leptos_icon!`].
#[proc_macro]
pub fn leptos_icons(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let IconComponentList(icon_structs) = parse_macro_input!(input as IconComponentList);

	let structs = icon_structs.into_iter().map(generate_component);

	let tokens = quote! {
		use leptos::*;
		#(#structs)*
	};

	tokens.into()
}

struct AllIcons {
	vis: Visibility,
	icon_set: Ident,
	component_postfix: Option<Ident>,
}

impl Parse for AllIcons {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let vis = input.parse::<Visibility>()?;
		let icon_set = input.parse()?;

		let component_postfix = if input.peek(syn::token::Paren) {
			let contents;
			parenthesized!(contents in input);
			Some(contents.parse()?)
		} else {
			None
		};

		Ok(AllIcons {
			vis,
			icon_set,
			component_postfix,
		})
	}
}

/// _**Warning:** This is not recommended because it can generate thousands of components, most of which won't get used, and you may end up feeling sad._
///
/// Generate every icon available for an icon set.
///
/// For example, the following invocation will generate every icon in the Feather icon set and append `Icon` to the generated component names:
///
/// ```
/// use leptos_icon_gen::leptos_icons_all;
///
/// leptos_icons_all!(pub feather(Icon));
/// ```
///
/// To use generated icons, use the icon name in pascal case with `Icon` appended. For example, to use the menu icon from the feather set in the example above, refer to it as `MenuIcon`:
///
/// ```ignore
/// view! {
///   <MenuIcon />
/// }
/// ```
///
/// You can opt out of a postfix to the generated component name:
///
/// ```
/// use leptos_icon_gen::leptos_icons_all;
///
/// leptos_icons_all!(pub feather);
/// ```
///
/// Now refer to generated icons just by their pascal cased name:
///
/// ```ignore
/// view! {
///   <Menu />
/// }
/// ```
#[proc_macro]
pub fn leptos_icons_all(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let AllIcons {
		vis,
		component_postfix,
		icon_set,
	} = parse_macro_input!(input as AllIcons);

	let icon_set_key = icon_set.to_string();

	let icon_set = icon_sets::ICON_SETS
		.get(icon_set_key.as_str())
		.unwrap_or_else(|| panic!("invalid icon set {icon_set_key}"))
		.iter()
		.map(|(key, _value)| {
			let icon_name = format_ident!(
				"{key}{}",
				component_postfix
					.as_ref()
					.map(|postfix| postfix.to_string())
					.unwrap_or_default()
			);
			let icon_set_key = format_ident!("{icon_set_key}");
			let key = format_ident!("{key}");
			quote! {
				 ::leptos_icon_gen::leptos_icon!(#vis #icon_name(#icon_set_key => #key));
			}
		})
		.collect::<TokenStream>();

	icon_set.into()
}
