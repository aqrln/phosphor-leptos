//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "media", feature = "system"))]
#[component]
pub fn PlayPause(
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
                <path d="M184,64V192a8,8,0,0,1-16,0V64a8,8,0,0,1,16,0Zm40-8a8,8,0,0,0-8,8V192a8,8,0,0,0,16,0V64A8,8,0,0,0,224,56Zm-87.33,58.66L48.48,58.51A15.91,15.91,0,0,0,24,71.85v112.3A15.83,15.83,0,0,0,32.23,198a15.95,15.95,0,0,0,16.25-.53l88.19-56.15a15.8,15.8,0,0,0,0-26.68Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M132.37,134.59,44.18,190.74A7.91,7.91,0,0,1,32,184.15V71.85a7.91,7.91,0,0,1,12.18-6.59l88.19,56.15A7.8,7.8,0,0,1,132.37,134.59Z"
        opacity="0.2"
    ></path>
    <path d="M184,64V192a8,8,0,0,1-16,0V64a8,8,0,0,1,16,0Zm40-8a8,8,0,0,0-8,8V192a8,8,0,0,0,16,0V64A8,8,0,0,0,224,56Zm-80,72a15.76,15.76,0,0,1-7.33,13.34L48.48,197.49A15.91,15.91,0,0,1,24,184.15V71.85A15.91,15.91,0,0,1,48.48,58.51l88.19,56.15A15.76,15.76,0,0,1,144,128Zm-16.18,0L40,72.08V183.93Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M180,64V192a4,4,0,0,1-8,0V64a4,4,0,0,1,8,0Zm44-4a4,4,0,0,0-4,4V192a4,4,0,0,0,8,0V64A4,4,0,0,0,224,60Zm-84,68a11.76,11.76,0,0,1-5.48,10L46.33,194.12a12,12,0,0,1-12.18.39A11.66,11.66,0,0,1,28,184.15V71.85a11.66,11.66,0,0,1,6.15-10.36,12,12,0,0,1,12.18.39L134.52,118A11.76,11.76,0,0,1,140,128Zm-8,0a3.77,3.77,0,0,0-1.78-3.22L42,68.63A3.94,3.94,0,0,0,39.91,68a4,4,0,0,0-1.91.5,3.76,3.76,0,0,0-2,3.35v112.3a3.76,3.76,0,0,0,2,3.35,3.91,3.91,0,0,0,4-.13l88.18-56.15A3.77,3.77,0,0,0,132,128Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M188,64V192a12,12,0,0,1-24,0V64a12,12,0,0,1,24,0Zm36-12a12,12,0,0,0-12,12V192a12,12,0,0,0,24,0V64A12,12,0,0,0,224,52Zm-76,76a19.71,19.71,0,0,1-9.19,16.71L50.63,200.87A19.91,19.91,0,0,1,20,184.15V71.85A19.91,19.91,0,0,1,50.63,55.13l88.18,56.16A19.71,19.71,0,0,1,148,128Zm-27.62,0L44,79.37v97.26Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M182,64V192a6,6,0,0,1-12,0V64a6,6,0,0,1,12,0Zm42-6a6,6,0,0,0-6,6V192a6,6,0,0,0,12,0V64A6,6,0,0,0,224,58Zm-82,70a13.77,13.77,0,0,1-6.41,11.65L47.41,195.8A13.91,13.91,0,0,1,26,184.15V71.85A13.91,13.91,0,0,1,47.41,60.2l88.18,56.15A13.77,13.77,0,0,1,142,128Zm-12,0a1.77,1.77,0,0,0-.85-1.53L41,70.32a1.87,1.87,0,0,0-1-.32,2.13,2.13,0,0,0-1,.25,1.76,1.76,0,0,0-1,1.6v112.3a1.76,1.76,0,0,0,1,1.6,1.9,1.9,0,0,0,2-.07l88.19-56.15A1.77,1.77,0,0,0,130,128Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M184,64V192a8,8,0,0,1-16,0V64a8,8,0,0,1,16,0Zm40-8a8,8,0,0,0-8,8V192a8,8,0,0,0,16,0V64A8,8,0,0,0,224,56Zm-80,72a15.76,15.76,0,0,1-7.33,13.34L48.48,197.49A15.91,15.91,0,0,1,24,184.15V71.85A15.91,15.91,0,0,1,48.48,58.51l88.19,56.15A15.76,15.76,0,0,1,144,128Zm-16.18,0L40,72.08V183.93Z"></path>
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
