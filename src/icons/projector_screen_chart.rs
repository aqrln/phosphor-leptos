//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "finance", feature = "office"))]
#[component]
pub fn ProjectorScreenChart(
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
                <path d="M232,64V48a16,16,0,0,0-16-16H40A16,16,0,0,0,24,48V64A16,16,0,0,0,40,80v96H32a8,8,0,0,0,0,16h88v17.38a24,24,0,1,0,16,0V192h88a8,8,0,0,0,0-16h-8V80A16,16,0,0,0,232,64ZM104,144a8,8,0,0,1-16,0V128a8,8,0,0,1,16,0Zm24,96a8,8,0,1,1,8-8A8,8,0,0,1,128,240Zm8-96a8,8,0,0,1-16,0V120a8,8,0,0,1,16,0Zm32,0a8,8,0,0,1-16,0V112a8,8,0,0,1,16,0ZM40,64V48H216V64H40Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M208,72V184H48V72Z" opacity="0.2"></path>
    <path d="M88,144V128a8,8,0,0,1,16,0v16a8,8,0,0,1-16,0Zm40,8a8,8,0,0,0,8-8V120a8,8,0,0,0-16,0v24A8,8,0,0,0,128,152Zm32,0a8,8,0,0,0,8-8V112a8,8,0,0,0-16,0v32A8,8,0,0,0,160,152Zm56-72v96h8a8,8,0,0,1,0,16H136v17.38a24,24,0,1,1-16,0V192H32a8,8,0,0,1,0-16h8V80A16,16,0,0,1,24,64V48A16,16,0,0,1,40,32H216a16,16,0,0,1,16,16V64A16,16,0,0,1,216,80ZM136,232a8,8,0,1,0-8,8A8,8,0,0,0,136,232ZM40,64H216V48H40ZM200,80H56v96H200Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M92,144V128a4,4,0,0,1,8,0v16a4,4,0,0,1-8,0Zm36,4a4,4,0,0,0,4-4V120a4,4,0,0,0-8,0v24A4,4,0,0,0,128,148Zm32,0a4,4,0,0,0,4-4V112a4,4,0,0,0-8,0v32A4,4,0,0,0,160,148Zm52-72V180h12a4,4,0,0,1,0,8H132v24.4a20,20,0,1,1-8,0V188H32a4,4,0,0,1,0-8H44V76H40A12,12,0,0,1,28,64V48A12,12,0,0,1,40,36H216a12,12,0,0,1,12,12V64a12,12,0,0,1-12,12ZM128,220a12,12,0,1,0,12,12A12,12,0,0,0,128,220ZM40,68H216a4,4,0,0,0,4-4V48a4,4,0,0,0-4-4H40a4,4,0,0,0-4,4V64A4,4,0,0,0,40,68Zm164,8H52V180H204Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M104,128v8a12,12,0,0,1-24,0v-8a12,12,0,0,1,24,0Zm24-16a12,12,0,0,0-12,12v12a12,12,0,0,0,24,0V124A12,12,0,0,0,128,112Zm36-4a12,12,0,0,0-12,12v16a12,12,0,0,0,24,0V120A12,12,0,0,0,164,108Zm56-16.4V164h4a12,12,0,0,1,0,24H140v23.22a24,24,0,1,1-24,0V188H32a12,12,0,0,1,0-24h4V91.6A20,20,0,0,1,20,72V48A20,20,0,0,1,40,28H216a20,20,0,0,1,20,20V72A20,20,0,0,1,220,91.6ZM44,68H212V52H44Zm152,96V92H60v72Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M90,144V128a6,6,0,0,1,12,0v16a6,6,0,0,1-12,0Zm38,6a6,6,0,0,0,6-6V120a6,6,0,0,0-12,0v24A6,6,0,0,0,128,150Zm32,0a6,6,0,0,0,6-6V112a6,6,0,0,0-12,0v32A6,6,0,0,0,160,150Zm54-72V178h10a6,6,0,0,1,0,12H134v20.84a22,22,0,1,1-12,0V190H32a6,6,0,0,1,0-12H42V78H40A14,14,0,0,1,26,64V48A14,14,0,0,1,40,34H216a14,14,0,0,1,14,14V64a14,14,0,0,1-14,14ZM128,222a10,10,0,1,0,10,10A10,10,0,0,0,128,222ZM40,66H216a2,2,0,0,0,2-2V48a2,2,0,0,0-2-2H40a2,2,0,0,0-2,2V64A2,2,0,0,0,40,66ZM202,78H54V178H202Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M88,144V128a8,8,0,0,1,16,0v16a8,8,0,0,1-16,0Zm40,8a8,8,0,0,0,8-8V120a8,8,0,0,0-16,0v24A8,8,0,0,0,128,152Zm32,0a8,8,0,0,0,8-8V112a8,8,0,0,0-16,0v32A8,8,0,0,0,160,152Zm56-72v96h8a8,8,0,0,1,0,16H136v17.38a24,24,0,1,1-16,0V192H32a8,8,0,0,1,0-16h8V80A16,16,0,0,1,24,64V48A16,16,0,0,1,40,32H216a16,16,0,0,1,16,16V64A16,16,0,0,1,216,80ZM136,232a8,8,0,1,0-8,8A8,8,0,0,0,136,232ZM40,64H216V48H40ZM200,80H56v96H200Z"></path>
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
