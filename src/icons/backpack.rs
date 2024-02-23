//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "commerce", feature = "objects"))]
#[component]
pub fn Backpack(
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
                <path d="M172,48.55V36A28,28,0,0,0,144,8H112A28,28,0,0,0,84,36V48.55A60.08,60.08,0,0,0,32,108V220a20,20,0,0,0,20,20H204a20,20,0,0,0,20-20V108A60.08,60.08,0,0,0,172,48.55ZM112,32h32a4,4,0,0,1,4,4V48H108V36A4,4,0,0,1,112,32Zm48,128H96v-8a4,4,0,0,1,4-4h56a4,4,0,0,1,4,4ZM96,184h32v4a12,12,0,0,0,24,0v-4h8v32H96Zm104,32H184V152a28,28,0,0,0-28-28H100a28,28,0,0,0-28,28v64H56V108A36,36,0,0,1,92,72h72a36,36,0,0,1,36,36ZM160,100a12,12,0,0,1-12,12H108a12,12,0,0,1,0-24h40A12,12,0,0,1,160,100Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M208,96V216a8,8,0,0,1-8,8H176V152a16,16,0,0,0-16-16H96a16,16,0,0,0-16,16v72H56a8,8,0,0,1-8-8V96A48,48,0,0,1,96,48h64A48,48,0,0,1,208,96Z"
        opacity="0.2"
    ></path>
    <path d="M168,40.58V32A24,24,0,0,0,144,8H112A24,24,0,0,0,88,32v8.58A56.09,56.09,0,0,0,40,96V216a16,16,0,0,0,16,16H200a16,16,0,0,0,16-16V96A56.09,56.09,0,0,0,168,40.58ZM112,24h32a8,8,0,0,1,8,8v8H104V32A8,8,0,0,1,112,24Zm56,136H88v-8a8,8,0,0,1,8-8h64a8,8,0,0,1,8,8ZM88,176h48v8a8,8,0,0,0,16,0v-8h16v40H88Zm112,40H184V152a24,24,0,0,0-24-24H96a24,24,0,0,0-24,24v64H56V96A40,40,0,0,1,96,56h64a40,40,0,0,1,40,40V216ZM152,88a8,8,0,0,1-8,8H112a8,8,0,0,1,0-16h32A8,8,0,0,1,152,88Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M168,40.58V32A24,24,0,0,0,144,8H112A24,24,0,0,0,88,32v8.58A56.09,56.09,0,0,0,40,96V216a16,16,0,0,0,16,16H200a16,16,0,0,0,16-16V96A56.09,56.09,0,0,0,168,40.58ZM104,32a8,8,0,0,1,8-8h32a8,8,0,0,1,8,8v8H104Zm8,40h32a8,8,0,0,1,0,16H112a8,8,0,0,1,0-16Zm64,144H80V176h56v8a8,8,0,0,0,16,0v-8h24Zm0-56H80v-8a16,16,0,0,1,16-16h64a16,16,0,0,1,16,16Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M166,42.34V32a22,22,0,0,0-22-22H112A22,22,0,0,0,90,32V42.34A54.07,54.07,0,0,0,42,96V216a14,14,0,0,0,14,14H200a14,14,0,0,0,14-14V96A54.07,54.07,0,0,0,166,42.34ZM112,22h32a10,10,0,0,1,10,10V42H102V32A10,10,0,0,1,112,22Zm58,140H86V152a10,10,0,0,1,10-10h64a10,10,0,0,1,10,10ZM86,174h52v10a6,6,0,0,0,12,0V174h20v44H86Zm116,42a2,2,0,0,1-2,2H182V152a22,22,0,0,0-22-22H96a22,22,0,0,0-22,22v66H56a2,2,0,0,1-2-2V96A42,42,0,0,1,96,54h64a42,42,0,0,1,42,42ZM150,88a6,6,0,0,1-6,6H112a6,6,0,0,1,0-12h32A6,6,0,0,1,150,88Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M168,40.58V32A24,24,0,0,0,144,8H112A24,24,0,0,0,88,32v8.58A56.09,56.09,0,0,0,40,96V216a16,16,0,0,0,16,16H200a16,16,0,0,0,16-16V96A56.09,56.09,0,0,0,168,40.58ZM112,24h32a8,8,0,0,1,8,8v8H104V32A8,8,0,0,1,112,24Zm56,136H88v-8a8,8,0,0,1,8-8h64a8,8,0,0,1,8,8ZM88,176h48v8a8,8,0,0,0,16,0v-8h16v40H88Zm112,40H184V152a24,24,0,0,0-24-24H96a24,24,0,0,0-24,24v64H56V96A40,40,0,0,1,96,56h64a40,40,0,0,1,40,40V216ZM152,88a8,8,0,0,1-8,8H112a8,8,0,0,1,0-16h32A8,8,0,0,1,152,88Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M164,44.17V32a20,20,0,0,0-20-20H112A20,20,0,0,0,92,32V44.17A52.05,52.05,0,0,0,44,96V216a12,12,0,0,0,12,12H200a12,12,0,0,0,12-12V96A52.05,52.05,0,0,0,164,44.17ZM112,20h32a12,12,0,0,1,12,12V44H100V32A12,12,0,0,1,112,20Zm60,144H84V152a12,12,0,0,1,12-12h64a12,12,0,0,1,12,12Zm-88,8h56v12a4,4,0,0,0,8,0V172h24v48H84Zm120,44a4,4,0,0,1-4,4H180V152a20,20,0,0,0-20-20H96a20,20,0,0,0-20,20v68H56a4,4,0,0,1-4-4V96A44.05,44.05,0,0,1,96,52h64a44.05,44.05,0,0,1,44,44ZM148,88a4,4,0,0,1-4,4H112a4,4,0,0,1,0-8h32A4,4,0,0,1,148,88Z"></path>
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
