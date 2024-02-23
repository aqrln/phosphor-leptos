//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "system"))]
#[component]
pub fn BluetoothSlash(
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
                <path d="M213.38,221.92a8,8,0,0,1-11.3-.54l-26.45-29.1L124.8,230.4a8,8,0,0,1-8.91.46,8.32,8.32,0,0,1-3.89-7.18V144L60.8,182.4a8,8,0,0,1-11.16-1.55,8.27,8.27,0,0,1,1.8-11.43l61.48-46.11L42.08,45.38A8,8,0,1,1,53.92,34.62l160,176A8,8,0,0,1,213.38,221.92ZM144.55,110.53a8,8,0,0,0,10.72,1l33.35-25a8.31,8.31,0,0,0,3-4.08,8,8,0,0,0-2.82-8.85l-64-48a8,8,0,0,0-10,.3A8.24,8.24,0,0,0,112,32.24V71.63A8,8,0,0,0,114.08,77Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M120,128l64,48-64,48Zm0-96v96l64-48Z" opacity="0.2"></path>
    <path d="M213.92,210.62l-160-176A8,8,0,1,0,42.08,45.38l70.84,77.93L51.2,169.6a8,8,0,1,0,9.6,12.8L112,144v80a8,8,0,0,0,12.8,6.4l50.83-38.12,26.45,29.1a8,8,0,1,0,11.84-10.76ZM128,208V144l11.73,8.8,25.08,27.59ZM112,71.63V32a8,8,0,0,1,12.8-6.4l64,48a8,8,0,0,1,0,12.8l-33.53,25.15a8,8,0,0,1-9.6-12.8l25-18.75L128,48V71.63a8,8,0,0,1-16,0Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M211,213.31,51,37.31A4,4,0,0,0,45,42.69l74,81.42-.14,0a5.17,5.17,0,0,0-.51.18l-.15.06-.05,0a4,4,0,0,0-.56.35l-.05,0-64,48a4,4,0,0,0,4.8,6.4L116,136v88a4,4,0,0,0,2.21,3.58A4.05,4.05,0,0,0,120,228a4,4,0,0,0,2.4-.8l53.74-40.3L205,218.69a4,4,0,1,0,5.92-5.38ZM124,216V136l18.44,13.83L170.73,181ZM116,71.63V32a4,4,0,0,1,6.4-3.2l64,48a4,4,0,0,1,0,6.4l-33.53,25.15a4,4,0,0,1-2.4.8,4,4,0,0,1-2.4-7.2l29.26-22L124,40V71.63a4,4,0,0,1-8,0Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M216.88,207.93l-160-176A12,12,0,1,0,39.12,48.07L107,122.75,48.8,166.4a12,12,0,1,0,14.4,19.2L108,152v72a12,12,0,0,0,19.2,9.6l47.91-35.94,24,26.41a12,12,0,0,0,17.76-16.14ZM132,200V152l5,3.77,21.87,24.06ZM108,59.74V32a12,12,0,0,1,19.2-9.6l64,48a12,12,0,0,1,0,19.2l-27.1,20.33a12,12,0,0,1-14.4-19.2L164,80,132,56v3.74a12,12,0,0,1-24,0Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M212.44,212,52.44,36A6,6,0,0,0,43.56,44l72.32,79.55L52.4,171.2a6,6,0,0,0,7.2,9.6L114,140v84a6,6,0,0,0,9.6,4.8l52.28-39.21L203.56,220a6,6,0,0,0,8.88-8.08ZM126,212V140l15.09,11.31,26.68,29.36ZM114,71.63V32a6,6,0,0,1,9.6-4.8l64,48a6,6,0,0,1,0,9.6L154.07,110a6,6,0,0,1-7.2-9.6L174,80,126,44V71.63a6,6,0,0,1-12,0Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M213.92,210.62l-160-176A8,8,0,1,0,42.08,45.38l70.84,77.93L51.2,169.6a8,8,0,1,0,9.6,12.8L112,144v80a8,8,0,0,0,12.8,6.4l50.83-38.12,26.45,29.1a8,8,0,1,0,11.84-10.76ZM128,208V144l11.73,8.8,25.08,27.59ZM112,71.63V32a8,8,0,0,1,12.8-6.4l64,48a8,8,0,0,1,0,12.8l-33.53,25.15a8,8,0,0,1-9.6-12.8l25-18.75L128,48V71.63a8,8,0,0,1-16,0Z"></path>
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
