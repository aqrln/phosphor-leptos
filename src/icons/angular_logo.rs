//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "brand", feature = "development"))]
#[component]
pub fn AngularLogo(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
    #[prop(into, optional)] id: MaybeProp<TextProp>,
    #[prop(into, optional)] class: MaybeProp<TextProp>,
) -> impl IntoView {
    let body = Signal::derive(move || {
        match weight.get() {
            IconWeight::Bold => view! {
                <path d="M228.61,60.92l-96-40a12,12,0,0,0-9.24,0l-96,40a12,12,0,0,0-7.28,12.67l16,120a12,12,0,0,0,6.52,9.14l80,40a12,12,0,0,0,10.74,0l80-40a12,12,0,0,0,6.52-9.14l16-120A12,12,0,0,0,228.61,60.92ZM197,184.11,128,218.58,59.05,184.11,45.11,79.54,128,45l82.89,34.54ZM117.51,82.17l-40,72a12,12,0,1,0,21,11.66L106.17,152h43.66l7.68,13.83a12,12,0,1,0,21-11.66l-40-72a12,12,0,0,0-21,0Zm2,45.83L128,112.71,136.49,128Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M224,72,208,192l-80,40L48,192,32,72l96-40Z" opacity="0.2"></path>
    <path d="M227.08,64.62l-96-40a7.93,7.93,0,0,0-6.16,0l-96,40a8,8,0,0,0-4.85,8.44l16,120a8,8,0,0,0,4.35,6.1l80,40a8,8,0,0,0,7.16,0l80-40a8,8,0,0,0,4.35-6.1l16-120A8,8,0,0,0,227.08,64.62ZM200.63,186.74,128,223.06,55.37,186.74,40.74,77,128,40.67,215.26,77ZM121,84.12l-40,72a8,8,0,1,0,14,7.76L106,144H150l11,19.88a8,8,0,1,0,14-7.76l-40-72a8,8,0,0,0-14,0ZM141.07,128H114.93L128,104.47Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M128,104.47,141.07,128H114.93ZM231.93,73.06l-16,120a8,8,0,0,1-4.35,6.1l-80,40a8,8,0,0,1-7.16,0l-80-40a8,8,0,0,1-4.35-6.1l-16-120a8,8,0,0,1,4.85-8.44l96-40a7.93,7.93,0,0,1,6.16,0l96,40A8,8,0,0,1,231.93,73.06ZM175,156.12l-40-72a8,8,0,0,0-14,0l-40,72a8,8,0,1,0,14,7.76L106,144H150l11,19.88a8,8,0,1,0,14-7.76Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M226.31,66.46l-96-40a6.06,6.06,0,0,0-4.62,0l-96,40a6,6,0,0,0-3.64,6.33l16,120a6,6,0,0,0,3.27,4.58l80,40a6,6,0,0,0,5.36,0l80-40a6,6,0,0,0,3.27-4.58l16-120A6,6,0,0,0,226.31,66.46Zm-23.84,121.6L128,225.29,53.53,188.06l-15-112.29L128,38.5l89.44,37.27Zm-79.72-103-40,72a6,6,0,0,0,10.5,5.82L104.86,142h46.28l11.61,20.91A6,6,0,0,0,168,166a5.88,5.88,0,0,0,2.9-.76,6,6,0,0,0,2.34-8.15l-40-72a6,6,0,0,0-10.5,0ZM144.47,130H111.53L128,100.35Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M227.08,64.62l-96-40a7.93,7.93,0,0,0-6.16,0l-96,40a8,8,0,0,0-4.85,8.44l16,120a8,8,0,0,0,4.35,6.1l80,40a8,8,0,0,0,7.16,0l80-40a8,8,0,0,0,4.35-6.1l16-120A8,8,0,0,0,227.08,64.62ZM200.63,186.74,128,223.06,55.37,186.74,40.74,77,128,40.67,215.26,77ZM121,84.12l-40,72a8,8,0,1,0,14,7.76L106,144H150l11,19.88a8,8,0,1,0,14-7.76l-40-72a8,8,0,0,0-14,0ZM141.07,128H114.93L128,104.47Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M225.54,68.31l-96-40a4,4,0,0,0-3.08,0l-96,40A4,4,0,0,0,28,72.53l16,120a4,4,0,0,0,2.17,3.05l80,40a4,4,0,0,0,3.58,0l80-40a4,4,0,0,0,2.17-3.05l16-120A4,4,0,0,0,225.54,68.31ZM204.32,189.37,128,227.53,51.68,189.37,36.37,74.51,128,36.33l91.63,38.18ZM124.5,86.06l-40,72a4,4,0,1,0,7,3.88L103.69,140h48.62l12.19,21.94a4,4,0,1,0,7-3.88l-40-72a4,4,0,0,0-7,0ZM147.87,132H108.13L128,96.24Z"></path>
}.into_view()
        }
    });

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };
    let height = size.clone();

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=move || size.get()
            height=move || height.get()
            fill=color
            transform=transform
            viewBox="0 0 256 256"
            id=move || id.get().unwrap_or(TextProp::from(""))
            class=move || class.get().unwrap_or(TextProp::from(""))
        >
            {body}
        </svg>
    }
}
