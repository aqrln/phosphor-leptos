//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "media"))]
#[component]
pub fn Gif(
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
                <path d="M148,72V184a12,12,0,0,1-24,0V72a12,12,0,0,1,24,0Zm76,12a12,12,0,0,0,0-24H176a12,12,0,0,0-12,12V184a12,12,0,0,0,24,0V140h28a12,12,0,0,0,0-24H188V84ZM96,116H72a12,12,0,0,0,0,24H84v12a20,20,0,0,1-40,0V104A20,20,0,0,1,64,84,21.8,21.8,0,0,1,82.35,94.22a12,12,0,0,0,20.53-12.44A45.67,45.67,0,0,0,64,60a44.05,44.05,0,0,0-44,44v48a44,44,0,0,0,88,0V128A12,12,0,0,0,96,116Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M232,56V200a16,16,0,0,1-16,16H40a16,16,0,0,1-16-16V56A16,16,0,0,1,40,40H216A16,16,0,0,1,232,56Z"
        opacity="0.2"
    ></path>
    <path d="M208,88a8,8,0,0,1-8,8H176v24h16a8,8,0,0,1,0,16H176v32a8,8,0,0,1-16,0V88a8,8,0,0,1,8-8h32A8,8,0,0,1,208,88Zm-72-8a8,8,0,0,0-8,8v80a8,8,0,0,0,16,0V88A8,8,0,0,0,136,80Zm-32,40H88a8,8,0,0,0,0,16h8v8a16,16,0,0,1-32,0V112a16,16,0,0,1,27.93-10.67,8,8,0,1,0,11.92-10.66A32,32,0,0,0,48,112v32a32,32,0,0,0,64,0V128A8,8,0,0,0,104,120Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M216,40H40A16,16,0,0,0,24,56V200a16,16,0,0,0,16,16H216a16,16,0,0,0,16-16V56A16,16,0,0,0,216,40ZM112,144a32,32,0,0,1-64,0V112a32,32,0,0,1,55.85-21.33,8,8,0,1,1-11.92,10.66A16,16,0,0,0,64,112v32a16,16,0,0,0,32,0v-8H88a8,8,0,0,1,0-16h16a8,8,0,0,1,8,8Zm32,24a8,8,0,0,1-16,0V88a8,8,0,0,1,16,0Zm56-72H176v24h16a8,8,0,0,1,0,16H176v32a8,8,0,0,1-16,0V88a8,8,0,0,1,8-8h32a8,8,0,0,1,0,16Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M142,72V184a6,6,0,0,1-12,0V72a6,6,0,0,1,12,0Zm82-6H176a6,6,0,0,0-6,6V184a6,6,0,0,0,12,0V134h34a6,6,0,0,0,0-12H182V78h42a6,6,0,0,0,0-12ZM96,122H72a6,6,0,0,0,0,12H90v18a26,26,0,0,1-52,0V104A26,26,0,0,1,64,78c12.07,0,23.33,8.38,26.19,19.5a6,6,0,1,0,11.62-3C97.56,78,81.66,66,64,66a38,38,0,0,0-38,38v48a38,38,0,0,0,76,0V128A6,6,0,0,0,96,122Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M144,72V184a8,8,0,0,1-16,0V72a8,8,0,0,1,16,0Zm80-8H176a8,8,0,0,0-8,8V184a8,8,0,0,0,16,0V136h32a8,8,0,0,0,0-16H184V80h40a8,8,0,0,0,0-16ZM96,120H72a8,8,0,0,0,0,16H88v16a24,24,0,0,1-48,0V104A24,24,0,0,1,64,80c11.19,0,21.61,7.74,24.25,18a8,8,0,0,0,15.5-4C99.27,76.62,82.56,64,64,64a40,40,0,0,0-40,40v48a40,40,0,0,0,80,0V128A8,8,0,0,0,96,120Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M140,72V184a4,4,0,0,1-8,0V72a4,4,0,0,1,8,0Zm84-4H176a4,4,0,0,0-4,4V184a4,4,0,0,0,8,0V132h36a4,4,0,0,0,0-8H180V76h44a4,4,0,0,0,0-8ZM96,124H72a4,4,0,0,0,0,8H92v20a28,28,0,0,1-56,0V104A28,28,0,0,1,64,76c13,0,25,9,28.13,21a4,4,0,1,0,7.74-2C95.85,79.36,80.76,68,64,68a36,36,0,0,0-36,36v48a36,36,0,0,0,72,0V128A4,4,0,0,0,96,124Z"></path>
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
