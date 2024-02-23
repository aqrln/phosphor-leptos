//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "health", feature = "objects"))]
#[component]
pub fn Toilet(
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
                <path d="M128,68a12,12,0,0,1-12,12H100a12,12,0,0,1,0-24h16A12,12,0,0,1,128,68Zm48.15,127.62,3.65,25.55A20,20,0,0,1,160,244H96a20,20,0,0,1-19.8-22.83l3.65-25.55A100.08,100.08,0,0,1,28,108,12,12,0,0,1,40,96H52V40A20,20,0,0,1,72,20H184a20,20,0,0,1,20,20V96h12a12,12,0,0,1,12,12A100.08,100.08,0,0,1,176.15,195.62ZM76,96H180V44H76Zm77.21,108.78a100.3,100.3,0,0,1-50.42,0L100.61,220h54.78ZM203.05,120H53a76,76,0,0,0,150.1,0Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M167.92,222.87A8,8,0,0,1,160,232H96a8,8,0,0,1-7.92-9.13l4.34-30.36h0a88.21,88.21,0,0,0,71.14,0h0ZM184,32H72a8,8,0,0,0-8,8v72H192V40A8,8,0,0,0,184,32Z"
        opacity="0.2"
    ></path>
    <path d="M120,64a8,8,0,0,1-8,8H96a8,8,0,0,1,0-16h16A8,8,0,0,1,120,64Zm52.32,133.14,3.52,24.6A16,16,0,0,1,160,240H96a16,16,0,0,1-15.84-18.26l3.52-24.6A96.09,96.09,0,0,1,32,112a8,8,0,0,1,8-8H56V40A16,16,0,0,1,72,24H184a16,16,0,0,1,16,16v64h16a8,8,0,0,1,8,8A96.09,96.09,0,0,1,172.32,197.14ZM72,104H184V40H72Zm85.07,99.5a96.15,96.15,0,0,1-58.14,0L96,224h64ZM207.6,120H48.4a80,80,0,0,0,159.2,0Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M60,88H196a4,4,0,0,0,4-4V40a16,16,0,0,0-16-16H72A16,16,0,0,0,56,40V84A4,4,0,0,0,60,88ZM88,48h15.73A8.18,8.18,0,0,1,112,55.47,8,8,0,0,1,104,64H88.27A8.18,8.18,0,0,1,80,56.53,8,8,0,0,1,88,48Zm136,64.06a8,8,0,0,0-8-8.06H40a8,8,0,0,0-8,8.06,96.1,96.1,0,0,0,51.68,85.08l-3.47,24.27a16.43,16.43,0,0,0,1.63,10A16,16,0,0,0,96,240h63.66a16.52,16.52,0,0,0,9.72-3,16,16,0,0,0,6.46-15.23l-3.52-24.6A96.1,96.1,0,0,0,224,112.06ZM96,224l2.93-20.5a96.15,96.15,0,0,0,58.14,0L160,224Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M118,64a6,6,0,0,1-6,6H96a6,6,0,0,1,0-12h16A6,6,0,0,1,118,64Zm52.14,132,3.72,26A14,14,0,0,1,160,238H96a14,14,0,0,1-13.86-16l3.72-26A94.1,94.1,0,0,1,34,112a6,6,0,0,1,6-6H58V40A14,14,0,0,1,72,26H184a14,14,0,0,1,14,14v66h18a6,6,0,0,1,6,6A94.1,94.1,0,0,1,170.14,196ZM70,106H186V40a2,2,0,0,0-2-2H72a2,2,0,0,0-2,2Zm88.71,94.84a94,94,0,0,1-61.42,0L94,223.72a2,2,0,0,0,.47,1.59A2,2,0,0,0,96,226h64a2,2,0,0,0,1.51-.69,2,2,0,0,0,.47-1.59ZM209.78,118H46.22a82,82,0,0,0,163.56,0Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M120,64a8,8,0,0,1-8,8H96a8,8,0,0,1,0-16h16A8,8,0,0,1,120,64Zm52.32,133.14,3.52,24.6A16,16,0,0,1,160,240H96a16,16,0,0,1-15.84-18.26l3.52-24.6A96.09,96.09,0,0,1,32,112a8,8,0,0,1,8-8H56V40A16,16,0,0,1,72,24H184a16,16,0,0,1,16,16v64h16a8,8,0,0,1,8,8A96.09,96.09,0,0,1,172.32,197.14ZM72,104H184V40H72Zm85.07,99.5a96.15,96.15,0,0,1-58.14,0L96,224h64ZM207.6,120H48.4a80,80,0,0,0,159.2,0Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M116,64a4,4,0,0,1-4,4H96a4,4,0,0,1,0-8h16A4,4,0,0,1,116,64Zm52,130.86,3.92,27.44A12,12,0,0,1,160,236H96a12,12,0,0,1-11.88-13.7L88,194.86A92.11,92.11,0,0,1,36,112a4,4,0,0,1,4-4H60V40A12,12,0,0,1,72,28H184a12,12,0,0,1,12,12v68h20a4,4,0,0,1,4,4A92.11,92.11,0,0,1,168,194.86ZM68,108H188V40a4,4,0,0,0-4-4H72a4,4,0,0,0-4,4Zm92.34,90.13a92,92,0,0,1-64.68,0L92,223.43a4,4,0,0,0,.94,3.19A3.93,3.93,0,0,0,96,228h64a3.93,3.93,0,0,0,3-1.38,4,4,0,0,0,.94-3.19ZM211.91,116H44.09a84,84,0,0,0,167.82,0Z"></path>
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
