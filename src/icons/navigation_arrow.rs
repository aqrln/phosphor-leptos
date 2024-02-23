//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "map"))]
#[component]
pub fn NavigationArrow(
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
                <path d="M240,113.58a15.76,15.76,0,0,1-11.29,15l-76.56,23.56-23.56,76.56a15.77,15.77,0,0,1-15,11.29h-.3a15.77,15.77,0,0,1-15.07-10.67L33,53.41a1,1,0,0,1-.05-.16A16,16,0,0,1,53.25,32.9l.16.05L229.33,98.21A15.78,15.78,0,0,1,240,113.58Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M226.35,121,149.8,144.5a8,8,0,0,0-5.3,5.3L121,226.35a8,8,0,0,1-15.21.27l-65.28-176A8,8,0,0,1,50.63,40.46l176,65.28A8,8,0,0,1,226.35,121Z"
        opacity="0.2"
    ></path>
    <path d="M229.33,98.21,53.41,33l-.16-.05A16,16,0,0,0,32.9,53.25a1,1,0,0,0,.05.16L98.21,229.33A15.77,15.77,0,0,0,113.28,240h.3a15.77,15.77,0,0,0,15-11.29l23.56-76.56,76.56-23.56a16,16,0,0,0,.62-30.38ZM224,113.3l-76.56,23.56a16,16,0,0,0-10.58,10.58L113.3,224h0l-.06-.17L48,48l175.82,65.22.16.06Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M228,102,51.93,36.67A12,12,0,0,0,36.69,52L102,228a11.81,11.81,0,0,0,11.31,8h.22a11.82,11.82,0,0,0,11.26-8.47L148.32,151a4,4,0,0,1,2.65-2.65l76.56-23.55A12,12,0,0,0,228,102Zm-2.83,15.13-76.57,23.56a12,12,0,0,0-7.94,7.94l-23.55,76.56a3.89,3.89,0,0,1-3.76,2.82,3.93,3.93,0,0,1-3.85-2.69l0-.08L44.22,49.32a3.93,3.93,0,0,1,1-4.14A4,4,0,0,1,48,44a3.86,3.86,0,0,1,1.25.21l176.08,65.32a4,4,0,0,1-.13,7.6Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M230.7,94.46,54.81,29.21l-.25-.09A20,20,0,0,0,29.12,54.56l.09.25L94.46,230.7A20,20,0,0,0,113.3,244h.35a20,20,0,0,0,18.77-14.12l22.93-74.53,74.53-22.93a20,20,0,0,0,.82-38ZM146.27,133A20,20,0,0,0,133,146.27L113,211.55,54.8,54.8,211.55,113Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M228.65,100.1,52.72,34.83l-.13,0A14,14,0,0,0,34.78,52.59s0,.09,0,.13L100.1,228.65A13.77,13.77,0,0,0,113.28,238h.26a13.8,13.8,0,0,0,13.14-9.88l23.56-76.56a2,2,0,0,1,1.32-1.32l76.56-23.56a14,14,0,0,0,.53-26.58Zm-4.06,15.11L148,138.77a14,14,0,0,0-9.26,9.26l-23.56,76.56a1.86,1.86,0,0,1-1.88,1.41,1.82,1.82,0,0,1-1.92-1.35.61.61,0,0,0,0-.12L46.11,48.62a2,2,0,0,1,2.51-2.51l175.91,65.26.12,0a2,2,0,0,1-.06,3.8Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M229.33,98.21,53.41,33l-.16-.05A16,16,0,0,0,32.9,53.25a1,1,0,0,0,.05.16L98.21,229.33A15.77,15.77,0,0,0,113.28,240h.3a15.77,15.77,0,0,0,15-11.29l23.56-76.56,76.56-23.56a16,16,0,0,0,.62-30.38ZM224,113.3l-76.56,23.56a16,16,0,0,0-10.58,10.58L113.3,224h0l-.06-.17L48,48l175.82,65.22.16.06Z"></path>
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
