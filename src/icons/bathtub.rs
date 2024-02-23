//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "objects"))]
#[component]
pub fn Bathtub(
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
                <path d="M232,92H211.3A12,12,0,0,0,200,84H136a12,12,0,0,0-11.3,8H68V52a8,8,0,0,1,8-8,8.5,8.5,0,0,1,8.24,6.39,12,12,0,0,0,23.52-4.78A32.22,32.22,0,0,0,44,52V92H24A20,20,0,0,0,4,112v32a60.07,60.07,0,0,0,56,59.85V216a12,12,0,0,0,24,0V204h88v12a12,12,0,0,0,24,0V203.85A60.07,60.07,0,0,0,252,144V112A20,20,0,0,0,232,92Zm-84,16h40v24H148Zm80,36a36,36,0,0,1-36,36H64a36,36,0,0,1-36-36V116h96v28a12,12,0,0,0,12,12h64a12,12,0,0,0,12-12V116h16Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M240,112v32a48,48,0,0,1-48,48H64a48,48,0,0,1-48-48V112a8,8,0,0,1,8-8H136v40h64V104h32A8,8,0,0,1,240,112Z"
        opacity="0.2"
    ></path>
    <path d="M232,96H208a8,8,0,0,0-8-8H136a8,8,0,0,0-8,8H64V52A12,12,0,0,1,76,40a12.44,12.44,0,0,1,12.16,9.59,8,8,0,0,0,15.68-3.18A28.32,28.32,0,0,0,76,24,28,28,0,0,0,48,52V96H24A16,16,0,0,0,8,112v32a56.06,56.06,0,0,0,56,56v16a8,8,0,0,0,16,0V200h96v16a8,8,0,0,0,16,0V200a56.06,56.06,0,0,0,56-56V112A16,16,0,0,0,232,96Zm-40,8v32H144V104Zm40,40a40,40,0,0,1-40,40H64a40,40,0,0,1-40-40V112H128v32a8,8,0,0,0,8,8h64a8,8,0,0,0,8-8V112h24Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M232,96H216a8,8,0,0,0-8-8H136a8,8,0,0,0-8,8H64V52A12,12,0,0,1,76,40a12.44,12.44,0,0,1,12.16,9.59,8,8,0,0,0,15.68-3.18A28.32,28.32,0,0,0,76,24,28,28,0,0,0,48,52V96H24A16,16,0,0,0,8,112v32a56.06,56.06,0,0,0,56,56v16a8,8,0,0,0,16,0V200h96v16a8,8,0,0,0,16,0V200a56.06,56.06,0,0,0,56-56V112A16,16,0,0,0,232,96Zm-32,48H144V104h56Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M232,98H206V96a6,6,0,0,0-6-6H136a6,6,0,0,0-6,6v2H62V52A14,14,0,0,1,76,38,14.47,14.47,0,0,1,90.12,49.19a6,6,0,1,0,11.76-2.38A26.32,26.32,0,0,0,76,26,26,26,0,0,0,50,52V98H24a14,14,0,0,0-14,14v32a54.06,54.06,0,0,0,54,54h2v18a6,6,0,0,0,12,0V198H178v18a6,6,0,0,0,12,0V198h2a54.06,54.06,0,0,0,54-54V112A14,14,0,0,0,232,98Zm-90,4h52v36H142Zm92,42a42,42,0,0,1-42,42H64a42,42,0,0,1-42-42V112a2,2,0,0,1,2-2H130v34a6,6,0,0,0,6,6h64a6,6,0,0,0,6-6V110h26a2,2,0,0,1,2,2Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M232,96H208a8,8,0,0,0-8-8H136a8,8,0,0,0-8,8H64V52A12,12,0,0,1,76,40a12.44,12.44,0,0,1,12.16,9.59,8,8,0,0,0,15.68-3.18A28.32,28.32,0,0,0,76,24,28,28,0,0,0,48,52V96H24A16,16,0,0,0,8,112v32a56.06,56.06,0,0,0,56,56v16a8,8,0,0,0,16,0V200h96v16a8,8,0,0,0,16,0V200a56.06,56.06,0,0,0,56-56V112A16,16,0,0,0,232,96Zm-40,8v32H144V104Zm40,40a40,40,0,0,1-40,40H64a40,40,0,0,1-40-40V112H128v32a8,8,0,0,0,8,8h64a8,8,0,0,0,8-8V112h24Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M232,100H204V96a4,4,0,0,0-4-4H136a4,4,0,0,0-4,4v4H60V52A16,16,0,0,1,76,36,16.49,16.49,0,0,1,92.08,48.8a4,4,0,1,0,7.84-1.6A24.32,24.32,0,0,0,76,28,24,24,0,0,0,52,52v48H24a12,12,0,0,0-12,12v32a52.06,52.06,0,0,0,52,52h4v20a4,4,0,0,0,8,0V196H180v20a4,4,0,0,0,8,0V196h4a52.06,52.06,0,0,0,52-52V112A12,12,0,0,0,232,100Zm-92,0h56v40H140Zm96,44a44.05,44.05,0,0,1-44,44H64a44.05,44.05,0,0,1-44-44V112a4,4,0,0,1,4-4H132v36a4,4,0,0,0,4,4h64a4,4,0,0,0,4-4V108h28a4,4,0,0,1,4,4Z"></path>
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
