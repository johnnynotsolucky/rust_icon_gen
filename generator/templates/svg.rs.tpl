use leptos::*;

#[component]
pub fn __icon_name(
    #[prop(into)]
    #[prop(optional)]
    class: MaybeSignal<String>,
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = "24".into())]
    size: MaybeSignal<String>,
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = "none".into())]
    fill: MaybeSignal<String>,
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = "currentColor".into())]
    color: MaybeSignal<String>,
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = "2px".into())]
    stroke_width: MaybeSignal<String>,
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = "round".into())]
    stroke_linecap: MaybeSignal<String>,
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = "round".into())]
    stroke_linejoin: MaybeSignal<String>,
) -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            class={class}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            fill={fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            __icon_nodes
        </svg>
    }
}