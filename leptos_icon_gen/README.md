# leptos_icon_gen

Icon generation macros for [Leptos](https://leptos.dev/).

## Leptos compatibility

| Crate version | Compatible Leptos version |
|---------------|---------------------------|
| 0.1.0         | 0.5                       |

## Usage

```rust
use leptos::*;
use leptos_lucide::{icon_component, icon_components};

icon_component!(MenuIcon(feather => Menu));
// icon_component!(pub MenuIcon(feather => Menu));
// icon_component!(pub(crate) MenuIcon(feather => Menu));

icon_components! {
	pub HomeIcon(feather => Home),
	pub AtSignIcon(lucide => AtSign),
	pub SquareAsteriskIcon(lucide => SquareAsterisk),
	pub MonitorIcon(lucide => Monitor),
	pub UserCogIcon(lucide => UserCog),
	pub UserIcon(lucide => User),
	pub LogOutIcon(lucide => LogOut),
}
```

In your `view!` macros:

```rust

view! {
  <HomeIcon class="my-class" size="24px" fill="currentColor"/>
}
```

## Icon sets

- Ant Design SVG Icons - https://github.com/ant-design/ant-design-icons
- Boxicons - https://github.com/atisawd/boxicons
- Bootstrap Icons - https://github.com/twbs/icons
- Charm Icons - https://github.com/jaynewey/charm-icons
- Codicons - https://github.com/microsoft/vscode-codicons
- css.gg - https://github.com/astrit/css.gg
- Feather - https://github.com/feathericons/feather
- Font Awesome - https://github.com/FortAwesome/Font-Awesome
- Heroicons - https://github.com/refactoringui/heroicons
- Ionicons - https://github.com/ionic-team/ionicons
- Lucide - https://github.com/lucide-icons/lucide
- Microns - https://github.com/stephenhutchings/microns
- Octicons - https://github.com/primer/octicons
- RemixIcon - https://github.com/Remix-Design/RemixIcon
- Simple Icons - https://github.com/simple-icons/simple-icons
- Tabler Icons - https://github.com/tabler/tabler-icons

## Icon Licenses

- Refer to individual icon set licenses in the list above.
