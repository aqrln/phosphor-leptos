//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "arrows"))]
#[component]
pub fn ArrowUpLeft(
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
                <path d="M197.66,197.66a8,8,0,0,1-11.32,0L116,127.31,69.66,173.66A8,8,0,0,1,56,168V64a8,8,0,0,1,8-8H168a8,8,0,0,1,5.66,13.66L127.31,116l70.35,70.34A8,8,0,0,1,197.66,197.66Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M168,64,64,168V64Z" opacity="0.2"></path>
    <path d="M197.66,186.34,127.31,116l46.35-46.34A8,8,0,0,0,168,56H64a8,8,0,0,0-8,8V168a8,8,0,0,0,13.66,5.66L116,127.31l70.34,70.35a8,8,0,0,0,11.32-11.32ZM72,72h76.69l-38.34,38.34h0L72,148.69Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M194.83,194.83a4,4,0,0,1-5.66,0L68,73.66V168a4,4,0,0,1-8,0V64a4,4,0,0,1,4-4H168a4,4,0,0,1,0,8H73.66L194.83,189.17A4,4,0,0,1,194.83,194.83Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M200.49,200.49a12,12,0,0,1-17,0L76,93v75a12,12,0,0,1-24,0V64A12,12,0,0,1,64,52H168a12,12,0,0,1,0,24H93L200.49,183.51A12,12,0,0,1,200.49,200.49Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M196.24,196.24a6,6,0,0,1-8.48,0L70,78.48V168a6,6,0,0,1-12,0V64a6,6,0,0,1,6-6H168a6,6,0,0,1,0,12H78.48L196.24,187.76A6,6,0,0,1,196.24,196.24Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M197.66,197.66a8,8,0,0,1-11.32,0L72,83.31V168a8,8,0,0,1-16,0V64a8,8,0,0,1,8-8H168a8,8,0,0,1,0,16H83.31L197.66,186.34A8,8,0,0,1,197.66,197.66Z"></path>
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
