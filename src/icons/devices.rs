//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "objects"))]
#[component]
pub fn Devices(
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
                <path d="M224,72H208V64a24,24,0,0,0-24-24H40A24,24,0,0,0,16,64v96a24,24,0,0,0,24,24H152v8a24,24,0,0,0,24,24h48a24,24,0,0,0,24-24V96A24,24,0,0,0,224,72Zm8,120a8,8,0,0,1-8,8H176a8,8,0,0,1-8-8V96a8,8,0,0,1,8-8h48a8,8,0,0,1,8,8Zm-96,16a8,8,0,0,1-8,8H88a8,8,0,0,1,0-16h40A8,8,0,0,1,136,208Zm80-96a8,8,0,0,1-8,8H192a8,8,0,0,1,0-16h16A8,8,0,0,1,216,112Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M200,64V80H176a16,16,0,0,0-16,16v80H40a16,16,0,0,1-16-16V64A16,16,0,0,1,40,48H184A16,16,0,0,1,200,64Z"
        opacity="0.2"
    ></path>
    <path d="M224,72H208V64a24,24,0,0,0-24-24H40A24,24,0,0,0,16,64v96a24,24,0,0,0,24,24H152v8a24,24,0,0,0,24,24h48a24,24,0,0,0,24-24V96A24,24,0,0,0,224,72ZM40,168a8,8,0,0,1-8-8V64a8,8,0,0,1,8-8H184a8,8,0,0,1,8,8v8H176a24,24,0,0,0-24,24v72Zm192,24a8,8,0,0,1-8,8H176a8,8,0,0,1-8-8V96a8,8,0,0,1,8-8h48a8,8,0,0,1,8,8Zm-96,16a8,8,0,0,1-8,8H88a8,8,0,0,1,0-16h40A8,8,0,0,1,136,208Zm80-96a8,8,0,0,1-8,8H192a8,8,0,0,1,0-16h16A8,8,0,0,1,216,112Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M224,76H204V64a20,20,0,0,0-20-20H40A20,20,0,0,0,20,64v96a20,20,0,0,0,20,20H156v12a20,20,0,0,0,20,20h48a20,20,0,0,0,20-20V96A20,20,0,0,0,224,76ZM40,172a12,12,0,0,1-12-12V64A12,12,0,0,1,40,52H184a12,12,0,0,1,12,12V76H176a20,20,0,0,0-20,20v76Zm196,20a12,12,0,0,1-12,12H176a12,12,0,0,1-12-12V96a12,12,0,0,1,12-12h48a12,12,0,0,1,12,12ZM132,208a4,4,0,0,1-4,4H88a4,4,0,0,1,0-8h40A4,4,0,0,1,132,208Zm80-96a4,4,0,0,1-4,4H192a4,4,0,0,1,0-8h16A4,4,0,0,1,212,112Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M224,72H212V64a28,28,0,0,0-28-28H40A28,28,0,0,0,12,64v88a28,28,0,0,0,28,28h96v12a28,28,0,0,0,28,28h60a28,28,0,0,0,28-28V100A28,28,0,0,0,224,72ZM40,156a4,4,0,0,1-4-4V64a4,4,0,0,1,4-4H184a4,4,0,0,1,4,4v8H164a28,28,0,0,0-28,28v56Zm188,36a4,4,0,0,1-4,4H164a4,4,0,0,1-4-4V100a4,4,0,0,1,4-4h60a4,4,0,0,1,4,4ZM124,208a12,12,0,0,1-12,12H88a12,12,0,0,1,0-24h24A12,12,0,0,1,124,208Zm88-84a12,12,0,0,1-12,12H188a12,12,0,0,1,0-24h12A12,12,0,0,1,212,124Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M224,74H206V64a22,22,0,0,0-22-22H40A22,22,0,0,0,18,64v96a22,22,0,0,0,22,22H154v10a22,22,0,0,0,22,22h48a22,22,0,0,0,22-22V96A22,22,0,0,0,224,74ZM40,170a10,10,0,0,1-10-10V64A10,10,0,0,1,40,54H184a10,10,0,0,1,10,10V74H176a22,22,0,0,0-22,22v74Zm194,22a10,10,0,0,1-10,10H176a10,10,0,0,1-10-10V96a10,10,0,0,1,10-10h48a10,10,0,0,1,10,10ZM134,208a6,6,0,0,1-6,6H88a6,6,0,0,1,0-12h40A6,6,0,0,1,134,208Zm80-96a6,6,0,0,1-6,6H192a6,6,0,0,1,0-12h16A6,6,0,0,1,214,112Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M224,72H208V64a24,24,0,0,0-24-24H40A24,24,0,0,0,16,64v96a24,24,0,0,0,24,24H152v8a24,24,0,0,0,24,24h48a24,24,0,0,0,24-24V96A24,24,0,0,0,224,72ZM40,168a8,8,0,0,1-8-8V64a8,8,0,0,1,8-8H184a8,8,0,0,1,8,8v8H176a24,24,0,0,0-24,24v72Zm192,24a8,8,0,0,1-8,8H176a8,8,0,0,1-8-8V96a8,8,0,0,1,8-8h48a8,8,0,0,1,8,8Zm-96,16a8,8,0,0,1-8,8H88a8,8,0,0,1,0-16h40A8,8,0,0,1,136,208Zm80-96a8,8,0,0,1-8,8H192a8,8,0,0,1,0-16h16A8,8,0,0,1,216,112Z"></path>
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
