//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "brand"))]
#[component]
pub fn NotionLogo(
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
                <path d="M224,48a8,8,0,0,1-8,8H200V208a8,8,0,0,1-8,8H152a8,8,0,0,1-7-4.14L72,79.15V200H88a8,8,0,0,1,0,16H40a8,8,0,0,1,0-16H56V56H40a8,8,0,0,1,0-16h64a8,8,0,0,1,7,4.14l73,132.71V56H168a8,8,0,0,1,0-16h48A8,8,0,0,1,224,48Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M192,208H152L64,48h40Z" opacity="0.2"></path>
    <path d="M216,40H168a8,8,0,0,0,0,16h16V176.85L111,44.14A8,8,0,0,0,104,40H40a8,8,0,0,0,0,16H56V200H40a8,8,0,0,0,0,16H88a8,8,0,0,0,0-16H72V79.15l73,132.71a8,8,0,0,0,7,4.14h40a8,8,0,0,0,8-8V56h16a8,8,0,0,0,0-16ZM156.73,200,77.53,56H99.27l79.2,144Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M216,44H168a4,4,0,0,0,0,8h20V192.43L107.5,46.07A4,4,0,0,0,104,44H40a4,4,0,0,0,0,8H60V204H40a4,4,0,0,0,0,8H88a4,4,0,0,0,0-8H68V63.57l80.5,146.36A4,4,0,0,0,152,212h40a4,4,0,0,0,4-4V52h20a4,4,0,0,0,0-8ZM70.77,52h30.86l83.6,152H154.37Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M216,36H176a12,12,0,0,0,0,24h8V161.28L118.51,42.22A12,12,0,0,0,108,36H40a12,12,0,0,0,0,24h8V196H40a12,12,0,0,0,0,24H80a12,12,0,0,0,0-24H72V94.72l65.49,119.06A12,12,0,0,0,148,220h48a12,12,0,0,0,12-12V60h8a12,12,0,0,0,0-24ZM80.3,60h20.6l74.8,136H155.1Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M216,42H168a6,6,0,0,0,0,12h18V184.64L109.26,45.11A6,6,0,0,0,104,42H40a6,6,0,0,0,0,12H58V202H40a6,6,0,0,0,0,12H88a6,6,0,0,0,0-12H70V71.36l76.74,139.53A6,6,0,0,0,152,214h40a6,6,0,0,0,6-6V54h18a6,6,0,0,0,0-12ZM74.15,54h26.3l81.4,148h-26.3Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M216,40H168a8,8,0,0,0,0,16h16V176.85L111,44.14A8,8,0,0,0,104,40H40a8,8,0,0,0,0,16H56V200H40a8,8,0,0,0,0,16H88a8,8,0,0,0,0-16H72V79.15l73,132.71a8,8,0,0,0,7,4.14h40a8,8,0,0,0,8-8V56h16a8,8,0,0,0,0-16ZM77.53,56H99.27l79.2,144H156.73Z"></path>
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
