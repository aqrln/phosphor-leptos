//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "editor", feature = "objects"))]
#[component]
pub fn Funnel(
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
                <path d="M227.81,66.76l-.08.09L160,139.17v55.49A16,16,0,0,1,152.87,208l-32,21.34A16,16,0,0,1,96,216V139.17L28.27,66.85l-.08-.09A16,16,0,0,1,40,40H216a16,16,0,0,1,11.84,26.76Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M221.9,61.38l-67.74,72.31a8,8,0,0,0-2.16,5.47v55.49a8,8,0,0,1-3.56,6.66l-32,21.33A8,8,0,0,1,104,216V139.16a8,8,0,0,0-2.16-5.47L34.1,61.38A8,8,0,0,1,40,48H216A8,8,0,0,1,221.9,61.38Z"
        opacity="0.2"
    ></path>
    <path d="M230.6,49.53A15.81,15.81,0,0,0,216,40H40A16,16,0,0,0,28.19,66.76l.08.09L96,139.17V216a16,16,0,0,0,24.87,13.32l32-21.34A16,16,0,0,0,160,194.66V139.17l67.74-72.32.08-.09A15.8,15.8,0,0,0,230.6,49.53ZM40,56h0Zm108.34,72.28A15.92,15.92,0,0,0,144,139.17v55.49L112,216V139.17a15.92,15.92,0,0,0-4.32-10.94L40,56H216Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M227,51.15A11.85,11.85,0,0,0,216,44H40a12,12,0,0,0-8.88,20.07l.05.05,67.73,72.31a4,4,0,0,1,1.08,2.74V216a12,12,0,0,0,18.66,10l32-21.33a12,12,0,0,0,5.35-10V139.17a4,4,0,0,1,1.08-2.74l67.78-72.36A11.85,11.85,0,0,0,227,51.15Zm-8,7.5L151.24,131a12,12,0,0,0-3.24,8.21v55.49a4,4,0,0,1-1.78,3.33l-32,21.33A4,4,0,0,1,108,216V139.17a12,12,0,0,0-3.24-8.21L37.05,58.67A4,4,0,0,1,40,52H216a4,4,0,0,1,3,6.65Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M234.29,47.91A20,20,0,0,0,216,36H40A20,20,0,0,0,25.2,69.45l.12.14L92,140.75V216a20,20,0,0,0,31.1,16.64l32-21.33A20,20,0,0,0,164,194.66V140.75l66.67-71.16.12-.14A20,20,0,0,0,234.29,47.91Zm-88.88,77.58A19.93,19.93,0,0,0,140,139.17v53.35l-24,16V139.17a19.93,19.93,0,0,0-5.41-13.68L49.23,60H206.77Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M228.77,50.34A13.8,13.8,0,0,0,216,42H40A14,14,0,0,0,29.67,65.42l.06.07L97.46,137.8a2,2,0,0,1,.54,1.37V216a14,14,0,0,0,21.77,11.64l32-21.33A14,14,0,0,0,158,194.66V139.17a2,2,0,0,1,.54-1.37l67.79-72.38A13.82,13.82,0,0,0,228.77,50.34Zm-11.26,6.94L149.78,129.6a13.93,13.93,0,0,0-3.78,9.57v55.49a2,2,0,0,1-.89,1.67l-32,21.33A2,2,0,0,1,110,216V139.17a14,14,0,0,0-3.78-9.58L38.53,57.32A2,2,0,0,1,40,54H216a1.9,1.9,0,0,1,1.83,1.19A1.86,1.86,0,0,1,217.51,57.28Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M230.6,49.53A15.81,15.81,0,0,0,216,40H40A16,16,0,0,0,28.19,66.76l.08.09L96,139.17V216a16,16,0,0,0,24.87,13.32l32-21.34A16,16,0,0,0,160,194.66V139.17l67.74-72.32.08-.09A15.8,15.8,0,0,0,230.6,49.53ZM40,56h0Zm108.34,72.28A15.92,15.92,0,0,0,144,139.17v55.49L112,216V139.17a15.92,15.92,0,0,0-4.32-10.94L40,56H216Z"></path>
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
