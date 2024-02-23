//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "media", feature = "office"))]
#[component]
pub fn VinylRecord(
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
                <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24ZM72,128a8,8,0,0,1-16,0,72.08,72.08,0,0,1,72-72,8,8,0,0,1,0,16A56.06,56.06,0,0,0,72,128Zm32,0a24,24,0,1,1,24,24A24,24,0,0,1,104,128Zm24,72a8,8,0,0,1,0-16,56.06,56.06,0,0,0,56-56,8,8,0,0,1,16,0A72.08,72.08,0,0,1,128,200Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M128,32a96,96,0,1,0,96,96A96,96,0,0,0,128,32Zm0,120a24,24,0,1,1,24-24A24,24,0,0,1,128,152Z"
        opacity="0.2"
    ></path>
    <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm0,192a88,88,0,1,1,88-88A88.1,88.1,0,0,1,128,216Zm0-144a56.06,56.06,0,0,0-56,56,8,8,0,0,1-16,0,72.08,72.08,0,0,1,72-72,8,8,0,0,1,0,16Zm72,56a72.08,72.08,0,0,1-72,72,8,8,0,0,1,0-16,56.06,56.06,0,0,0,56-56,8,8,0,0,1,16,0Zm-40,0a32,32,0,1,0-32,32A32,32,0,0,0,160,128Zm-48,0a16,16,0,1,1,16,16A16,16,0,0,1,112,128Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M128,28A100,100,0,1,0,228,128,100.11,100.11,0,0,0,128,28Zm0,192a92,92,0,1,1,92-92A92.1,92.1,0,0,1,128,220Zm0-152a60.07,60.07,0,0,0-60,60,4,4,0,0,1-8,0,68.07,68.07,0,0,1,68-68,4,4,0,0,1,0,8Zm68,60a68.07,68.07,0,0,1-68,68,4,4,0,0,1,0-8,60.07,60.07,0,0,0,60-60,4,4,0,0,1,8,0Zm-40,0a28,28,0,1,0-28,28A28,28,0,0,0,156,128Zm-48,0a20,20,0,1,1,20,20A20,20,0,0,1,108,128Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M128,20A108,108,0,1,0,236,128,108.12,108.12,0,0,0,128,20Zm0,192a84,84,0,1,1,84-84A84.09,84.09,0,0,1,128,212Zm0-124a40,40,0,0,0-40,40,12,12,0,0,1-24,0,64.07,64.07,0,0,1,64-64,12,12,0,0,1,0,24Zm64,40a64.07,64.07,0,0,1-64,64,12,12,0,0,1,0-24,40,40,0,0,0,40-40,12,12,0,0,1,24,0Zm-64,20a20,20,0,1,1,20-20A20,20,0,0,1,128,148Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M128,26A102,102,0,1,0,230,128,102.12,102.12,0,0,0,128,26Zm0,192a90,90,0,1,1,90-90A90.1,90.1,0,0,1,128,218Zm0-148a58.07,58.07,0,0,0-58,58,6,6,0,0,1-12,0,70.08,70.08,0,0,1,70-70,6,6,0,0,1,0,12Zm70,58a70.08,70.08,0,0,1-70,70,6,6,0,0,1,0-12,58.07,58.07,0,0,0,58-58,6,6,0,0,1,12,0Zm-40,0a30,30,0,1,0-30,30A30,30,0,0,0,158,128Zm-48,0a18,18,0,1,1,18,18A18,18,0,0,1,110,128Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm0,192a88,88,0,1,1,88-88A88.1,88.1,0,0,1,128,216Zm0-144a56.06,56.06,0,0,0-56,56,8,8,0,0,1-16,0,72.08,72.08,0,0,1,72-72,8,8,0,0,1,0,16Zm72,56a72.08,72.08,0,0,1-72,72,8,8,0,0,1,0-16,56.06,56.06,0,0,0,56-56,8,8,0,0,1,16,0Zm-40,0a32,32,0,1,0-32,32A32,32,0,0,0,160,128Zm-48,0a16,16,0,1,1,16,16A16,16,0,0,1,112,128Z"></path>
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
