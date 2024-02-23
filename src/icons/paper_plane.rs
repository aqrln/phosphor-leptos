//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "communication", feature = "map", feature = "objects"))]
#[component]
pub fn PaperPlane(
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
            IconWeight::Fill => view! {
                <path d="M236.2,218.3a15.87,15.87,0,0,1-17.54,4.76L138.71,195.7a4,4,0,0,1-2.71-3.79V120a8,8,0,0,0-8.53-8,8.19,8.19,0,0,0-7.47,8.26v71.57a4,4,0,0,1-2.7,3.79l-80,27.44a16,16,0,0,1-19.25-22.92L114,32.13a16,16,0,0,1,27.89,0L237.9,200.09A15.89,15.89,0,0,1,236.2,218.3Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M221.28,215.51l-90.71-31a7.89,7.89,0,0,0-5.38,0l-90.47,31a8,8,0,0,1-9.67-11.44l95.85-168a8,8,0,0,1,14,0l96.09,168A8,8,0,0,1,221.28,215.51Z"
        opacity="0.2"
    ></path>
    <path d="M237.9,200.09,141.85,32.18a16,16,0,0,0-27.89,0l-95.89,168a16,16,0,0,0,19.25,22.92l90.47-31,.1,0,.09,0,90.68,31a16,16,0,0,0,19.24-23Zm-14,7.84L136,177.86V120a8,8,0,0,0-16,0v57.78L32.12,207.94,32,208,127.86,40,224,208Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M234.43,202.08,138.35,34.14a12,12,0,0,0-20.92,0l-95.88,168A12,12,0,0,0,32,220a12,12,0,0,0,4-.7l90.48-31.05h.05a4.09,4.09,0,0,1,2.74,0l90.66,31a12,12,0,0,0,14.49-17.2ZM227,210.56a3.93,3.93,0,0,1-4.45,1.17l-90.59-31V120a4,4,0,0,0-8,0v60.65l-.13,0-90.5,31.06a4,4,0,0,1-4.85-5.7l95.87-168a4,4,0,0,1,7,0l96.08,168A3.89,3.89,0,0,1,227,210.56Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M241.42,198.2l-.06-.09L145.3,30.17a20,20,0,0,0-34.82,0L14.58,198.2a20,20,0,0,0,24.1,28.64l89.2-30.61,89.45,30.61a20.22,20.22,0,0,0,6.72,1.16,20,20,0,0,0,17.37-29.8ZM140,175V120a12,12,0,0,0-24,0v54.93L40.76,200.75,127.89,48.06l87.34,152.7Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M236.17,201.09,140.1,33.16a14,14,0,0,0-24.41,0l-95.88,168a14,14,0,0,0,16.85,20.05l90.48-31,.07,0a2.11,2.11,0,0,1,1.42,0l90.64,31a14,14,0,0,0,16.9-20.07Zm-10.66,8.17a1.91,1.91,0,0,1-2.28.57L134,179.3V120a6,6,0,0,0-12,0v59.21L32.7,209.86a1.87,1.87,0,0,1-2.2-.6,1.83,1.83,0,0,1-.24-2.22L126.14,39a1.93,1.93,0,0,1,1.74-1,2,2,0,0,1,1.78,1.07L225.73,207A1.86,1.86,0,0,1,225.51,209.26Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M237.9,200.09,141.85,32.18a16,16,0,0,0-27.89,0l-95.89,168a16,16,0,0,0,19.25,22.92l90.47-31,.1,0,.09,0,90.68,31a16,16,0,0,0,19.24-23Zm-14,7.84L136,177.86V120a8,8,0,0,0-16,0v57.78L32.12,207.94,32,208,127.86,40,224,208Z"></path>
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
