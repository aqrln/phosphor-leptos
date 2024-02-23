//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "communication", feature = "system"))]
#[component]
pub fn Translate(
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
                <path d="M242.73,210.63l-56-112a12,12,0,0,0-21.46,0l-20.52,41A84.2,84.2,0,0,1,106,126.22,107.48,107.48,0,0,0,131.33,68H152a12,12,0,0,0,0-24H100V32a12,12,0,0,0-24,0V44H24a12,12,0,0,0,0,24h83.13A83.69,83.69,0,0,1,88,110.35,84,84,0,0,1,75.6,91a12,12,0,1,0-21.81,10A107.55,107.55,0,0,0,70,126.24,83.54,83.54,0,0,1,24,140a12,12,0,0,0,0,24,107.47,107.47,0,0,0,64-21.07,108.4,108.4,0,0,0,45.39,19.44l-24.13,48.26a12,12,0,1,0,21.46,10.73L143.41,196h65.17l12.68,25.36a12,12,0,1,0,21.47-10.73ZM155.41,172,176,130.83,196.58,172Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M216,184H136l40-80ZM88,127.56h0A95.78,95.78,0,0,0,120,56H56A95.78,95.78,0,0,0,88,127.56Z"
        opacity="0.2"
    ></path>
    <path d="M239.15,212.42l-56-112a8,8,0,0,0-14.31,0l-21.71,43.43A88,88,0,0,1,100,126.93,103.65,103.65,0,0,0,127.69,64H152a8,8,0,0,0,0-16H96V32a8,8,0,0,0-16,0V48H24a8,8,0,0,0,0,16h87.63A87.7,87.7,0,0,1,88,116.35a87.74,87.74,0,0,1-19-31,8,8,0,1,0-15.08,5.34A103.63,103.63,0,0,0,76,127a87.55,87.55,0,0,1-52,17,8,8,0,0,0,0,16,103.46,103.46,0,0,0,64-22.08,104.18,104.18,0,0,0,51.44,21.31l-26.6,53.19a8,8,0,0,0,14.31,7.16L140.94,192h70.11l13.79,27.58A8,8,0,0,0,232,224a8,8,0,0,0,7.15-11.58ZM148.94,176,176,121.89,203.05,176Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M160,129.89,175.06,160H144.94l6.36-12.7v0ZM224,48V208a16,16,0,0,1-16,16H48a16,16,0,0,1-16-16V48A16,16,0,0,1,48,32H208A16,16,0,0,1,224,48ZM207.16,188.42l-40-80a8,8,0,0,0-14.32,0L139.66,134.8a62.31,62.31,0,0,1-23.61-10A79.61,79.61,0,0,0,135.6,80H152a8,8,0,0,0,0-16H112V56a8,8,0,0,0-16,0v8H56a8,8,0,0,0,0,16h63.48a63.73,63.73,0,0,1-15.3,34.05,65.93,65.93,0,0,1-9-13.61,8,8,0,0,0-14.32,7.12,81.75,81.75,0,0,0,11.4,17.15A63.62,63.62,0,0,1,56,136a8,8,0,0,0,0,16,79.56,79.56,0,0,0,48.11-16.13,78.33,78.33,0,0,0,28.18,13.66l-19.45,38.89a8,8,0,0,0,14.32,7.16L136.94,176h46.12l9.78,19.58a8,8,0,1,0,14.32-7.16Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M237.37,213.32l-56-112a6,6,0,0,0-10.74,0l-22.3,44.6A90,90,0,0,1,97,127.19,101.73,101.73,0,0,0,125.82,62H152a6,6,0,0,0,0-12H94V32a6,6,0,0,0-12,0V50H24a6,6,0,0,0,0,12h89.79A89.71,89.71,0,0,1,88,119.23,89.81,89.81,0,0,1,67.11,86,6,6,0,1,0,55.8,90,101.66,101.66,0,0,0,79,127.2,89.56,89.56,0,0,1,24,146a6,6,0,0,0,0,12,101.55,101.55,0,0,0,64-22.63,102.11,102.11,0,0,0,54.53,22.17l-27.89,55.78a6,6,0,0,0,10.74,5.36L139.71,190h72.58l14.34,28.68A6,6,0,0,0,232,222a5.87,5.87,0,0,0,2.68-.64A6,6,0,0,0,237.37,213.32ZM145.71,178,176,117.42,206.29,178Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M239.15,212.42l-56-112a8,8,0,0,0-14.31,0l-21.71,43.43A88,88,0,0,1,100,126.93,103.65,103.65,0,0,0,127.69,64H152a8,8,0,0,0,0-16H96V32a8,8,0,0,0-16,0V48H24a8,8,0,0,0,0,16h87.63A87.76,87.76,0,0,1,88,116.35a87.74,87.74,0,0,1-19-31,8,8,0,1,0-15.08,5.34A103.63,103.63,0,0,0,76,127a87.55,87.55,0,0,1-52,17,8,8,0,0,0,0,16,103.46,103.46,0,0,0,64-22.08,104.18,104.18,0,0,0,51.44,21.31l-26.6,53.19a8,8,0,0,0,14.31,7.16L140.94,192h70.11l13.79,27.58A8,8,0,0,0,232,224a8,8,0,0,0,7.15-11.58ZM148.94,176,176,121.89,203.05,176Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M235.58,214.21l-56-112a4,4,0,0,0-7.16,0L149.55,148A92.05,92.05,0,0,1,94,127.36,99.68,99.68,0,0,0,123.91,60H152a4,4,0,0,0,0-8H92V32a4,4,0,0,0-8,0V52H24a4,4,0,0,0,0,8h91.91A91.8,91.8,0,0,1,88,122.05,92,92,0,0,1,65.23,86.67a4,4,0,1,0-7.54,2.66,99.59,99.59,0,0,0,24.3,38A91.59,91.59,0,0,1,24,148a4,4,0,0,0,0,8,99.54,99.54,0,0,0,64-23.21,100.09,100.09,0,0,0,57.66,23l-29.22,58.43a4,4,0,0,0,7.16,3.58L138.47,188h75.06l14.89,29.79A4,4,0,0,0,232,220a4.12,4.12,0,0,0,1.79-.42A4,4,0,0,0,235.58,214.21ZM142.47,180,176,112.94,209.53,180Z"></path>
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
