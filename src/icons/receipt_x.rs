//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "commerce", feature = "finance"))]
#[component]
pub fn ReceiptX(
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
                <path d="M216,36H40A20,20,0,0,0,20,56V208a12,12,0,0,0,17.37,10.73L64,205.42l26.63,13.31a12,12,0,0,0,10.74,0L128,205.42l26.63,13.31a12,12,0,0,0,10.74,0L192,205.42l26.63,13.31A12,12,0,0,0,236,208V56A20,20,0,0,0,216,36Zm-4,152.58-14.63-7.31a12,12,0,0,0-10.74,0L160,194.58l-26.63-13.31a12,12,0,0,0-10.74,0L96,194.58,69.37,181.27a12,12,0,0,0-10.74,0L44,188.58V60H212ZM95.51,135.51,111,120,95.51,104.49a12,12,0,0,1,17-17L128,103l15.51-15.52a12,12,0,0,1,17,17L145,120l15.52,15.51a12,12,0,0,1-17,17L128,137l-15.51,15.52a12,12,0,0,1-17-17Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M224,56V208l-32-16-32,16-32-16L96,208,64,192,32,208V56a8,8,0,0,1,8-8H216A8,8,0,0,1,224,56Z"
        opacity="0.2"
    ></path>
    <path d="M216,40H40A16,16,0,0,0,24,56V208a8,8,0,0,0,11.58,7.15L64,200.94l28.42,14.21a8,8,0,0,0,7.16,0L128,200.94l28.42,14.21a8,8,0,0,0,7.16,0L192,200.94l28.42,14.21A8,8,0,0,0,232,208V56A16,16,0,0,0,216,40Zm0,155.06-20.42-10.22a8,8,0,0,0-7.16,0L160,199.06l-28.42-14.22a8,8,0,0,0-7.16,0L96,199.06,67.58,184.84a8,8,0,0,0-7.16,0L40,195.06V56H216ZM98.34,138.34,116.69,120,98.34,101.66a8,8,0,0,1,11.32-11.32L128,108.69l18.34-18.35a8,8,0,0,1,11.32,11.32L139.31,120l18.35,18.34a8,8,0,0,1-11.32,11.32L128,131.31l-18.34,18.35a8,8,0,0,1-11.32-11.32Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M216,40H40A16,16,0,0,0,24,56V208a8,8,0,0,0,11.58,7.15L64,200.94l28.42,14.21a8,8,0,0,0,7.16,0L128,200.94l28.42,14.21a8,8,0,0,0,7.16,0L192,200.94l28.42,14.21A8,8,0,0,0,232,208V56A16,16,0,0,0,216,40Zm-58.34,98.34a8,8,0,0,1-11.32,11.32L128,131.31l-18.34,18.35a8,8,0,0,1-11.32-11.32L116.69,120,98.34,101.66a8,8,0,0,1,11.32-11.32L128,108.69l18.34-18.35a8,8,0,0,1,11.32,11.32L139.31,120Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M216,42H40A14,14,0,0,0,26,56V208a6,6,0,0,0,8.68,5.37L64,198.71l29.32,14.66a6,6,0,0,0,5.36,0L128,198.71l29.32,14.66a6,6,0,0,0,5.36,0L192,198.71l29.32,14.66A6,6,0,0,0,224,214a5.93,5.93,0,0,0,3.15-.9A6,6,0,0,0,230,208V56A14,14,0,0,0,216,42Zm2,156.29-23.32-11.66a6,6,0,0,0-5.36,0L160,201.29l-29.32-14.66a6,6,0,0,0-5.36,0L96,201.29,66.68,186.63a6,6,0,0,0-5.36,0L38,198.29V56a2,2,0,0,1,2-2H216a2,2,0,0,1,2,2Zm-61.76-98L136.48,120l19.76,19.76a6,6,0,1,1-8.48,8.48L128,128.48l-19.76,19.76a6,6,0,0,1-8.48-8.48L119.52,120,99.76,100.24a6,6,0,0,1,8.48-8.48L128,111.52l19.76-19.76a6,6,0,0,1,8.48,8.48Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M216,40H40A16,16,0,0,0,24,56V208a8,8,0,0,0,11.58,7.15L64,200.94l28.42,14.21a8,8,0,0,0,7.16,0L128,200.94l28.42,14.21a8,8,0,0,0,7.16,0L192,200.94l28.42,14.21A8,8,0,0,0,232,208V56A16,16,0,0,0,216,40Zm0,155.06-20.42-10.22a8,8,0,0,0-7.16,0L160,199.06l-28.42-14.22a8,8,0,0,0-7.16,0L96,199.06,67.58,184.84a8,8,0,0,0-7.16,0L40,195.06V56H216ZM98.34,138.34,116.69,120,98.34,101.66a8,8,0,0,1,11.32-11.32L128,108.69l18.34-18.35a8,8,0,0,1,11.32,11.32L139.31,120l18.35,18.34a8,8,0,0,1-11.32,11.32L128,131.31l-18.34,18.35a8,8,0,0,1-11.32-11.32Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M216,44H40A12,12,0,0,0,28,56V208a4,4,0,0,0,5.79,3.58L64,196.47l30.21,15.11a4,4,0,0,0,3.58,0L128,196.47l30.21,15.11a4,4,0,0,0,3.58,0L192,196.47l30.21,15.11A4.05,4.05,0,0,0,224,212a4,4,0,0,0,4-4V56A12,12,0,0,0,216,44Zm4,157.53-26.21-13.11a4,4,0,0,0-3.58,0L160,203.53l-30.21-15.11a4,4,0,0,0-3.58,0L96,203.53,65.79,188.42a4,4,0,0,0-3.58,0L36,201.53V56a4,4,0,0,1,4-4H216a4,4,0,0,1,4,4ZM154.83,98.83,133.66,120l21.17,21.17a4,4,0,0,1-5.66,5.66L128,125.66l-21.17,21.17a4,4,0,0,1-5.66-5.66L122.34,120,101.17,98.83a4,4,0,0,1,5.66-5.66L128,114.34l21.17-21.17a4,4,0,1,1,5.66,5.66Z"></path>
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
