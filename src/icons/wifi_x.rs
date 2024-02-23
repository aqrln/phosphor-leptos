//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "system"))]
#[component]
pub fn WifiX(
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
                <path d="M144,204a16,16,0,1,1-16-16A16,16,0,0,1,144,204ZM217,80l15.52-15.51a12,12,0,0,0-17-17L200,63,184.49,47.51a12,12,0,0,0-17,17L183,80,167.51,95.51a12,12,0,0,0,17,17L200,97l15.51,15.52a12,12,0,0,0,17-17Zm-41.9,75.3a80,80,0,0,0-94.13,0,12,12,0,1,0,14.13,19.4,56,56,0,0,1,65.87,0,12,12,0,0,0,14.13-19.4ZM131.71,68h.3a12,12,0,0,0,.28-24c-1.43,0-2.86,0-4.29,0A176.27,176.27,0,0,0,16.39,83.91a12,12,0,1,0,15.23,18.55A152.24,152.24,0,0,1,128,68C129.24,68,130.48,68,131.71,68Zm-.12,48a12,12,0,0,0,.82-24C131,92,129.47,92,128,92a126.66,126.66,0,0,0-79.45,27.64,12,12,0,0,0,14.9,18.81A102.89,102.89,0,0,1,128,116C129.2,116,130.41,116,131.59,116.06Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M224.39,104.34,134.15,213.12a8,8,0,0,1-12.3,0L17.8,87.69a7.79,7.79,0,0,1,1.31-11.21A179.58,179.58,0,0,1,128,40a181.82,181.82,0,0,1,33.06,3,7.94,7.94,0,0,1,4.17,2.21L224,104Z"
        opacity="0.2"
    ></path>
    <path d="M229.66,98.34a8,8,0,0,1-11.32,11.32L200,91.31l-18.34,18.35a8,8,0,0,1-11.32-11.32L188.69,80,170.34,61.66a8,8,0,0,1,11.32-11.32L200,68.69l18.34-18.35a8,8,0,0,1,11.32,11.32L211.31,80Zm-26.43,31.5a8,8,0,0,0-11.26,1L128,208,24.09,82.74A170.76,170.76,0,0,1,128,48c5.11,0,10.25.22,15.3.67a8,8,0,1,0,1.4-15.94c-5.51-.48-11.13-.73-16.7-.73A186.67,186.67,0,0,0,14.28,70.1,15.93,15.93,0,0,0,8.11,80.91,15.65,15.65,0,0,0,11.65,92.8l104,125.43A15.93,15.93,0,0,0,128,224h0a15.93,15.93,0,0,0,12.31-5.77l64-77.12A8,8,0,0,0,203.23,129.84Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M229.51,98.13a8.21,8.21,0,0,1,.61,11.1,8,8,0,0,1-11.72.43L200.05,91.31l-18.34,18.35a8,8,0,0,1-11.72-.43,8.21,8.21,0,0,1,.61-11.1L188.74,80,170.63,61.89a8.21,8.21,0,0,1-.41-11.37,8,8,0,0,1,11.49-.18l18.34,18.35L218.4,50.34a8,8,0,0,1,11.49.18,8.21,8.21,0,0,1-.41,11.37L211.37,80ZM212,124.71a23.89,23.89,0,0,1-4.86-3.74l-4.21-4.2a4,4,0,0,0-5.65,0L193,121a24,24,0,0,1-34.21-.26c-9.19-9.48-8.69-24.73.65-34.06l3.81-3.82a4,4,0,0,0,0-5.66L159.06,73a24,24,0,0,1,.06-34h0a2.78,2.78,0,0,0-1.52-4.71A191.86,191.86,0,0,0,128,32,186.77,186.77,0,0,0,14.28,70.1,15.93,15.93,0,0,0,8.11,80.91,15.65,15.65,0,0,0,11.65,92.8L115.72,218.23A15.93,15.93,0,0,0,128,224h0a16,16,0,0,0,12.31-5.77l72.6-87.5A4,4,0,0,0,212,124.71Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M138,204a10,10,0,1,1-10-10A10,10,0,0,1,138,204ZM208.48,80l19.76-19.76a6,6,0,0,0-8.48-8.48L200,71.52,180.24,51.76a6,6,0,0,0-8.48,8.48L191.52,80,171.76,99.76a6,6,0,1,0,8.48,8.48L200,88.48l19.76,19.76a6,6,0,0,0,8.48-8.48Zm-36.95,80.15a74,74,0,0,0-87.06,0,6,6,0,0,0,7.06,9.7,62,62,0,0,1,72.94,0,6,6,0,0,0,8.38-1.32A6,6,0,0,0,171.53,160.15ZM143.42,62.74a6,6,0,1,0,1.16-11.94c-5.47-.53-11.05-.8-16.58-.8A170.32,170.32,0,0,0,20.19,88.55a6,6,0,1,0,7.62,9.27A158.26,158.26,0,0,1,128,62C133.14,62,138.33,62.25,143.42,62.74Zm-.24,48.3a6,6,0,0,0,1.64-11.89A124,124,0,0,0,128,98a120.75,120.75,0,0,0-75.73,26.34,6,6,0,0,0,7.46,9.41A108.78,108.78,0,0,1,128,110,111.24,111.24,0,0,1,143.18,111Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M229.66,98.34a8,8,0,0,1-11.32,11.32L200,91.31l-18.34,18.35a8,8,0,0,1-11.32-11.32L188.69,80,170.34,61.66a8,8,0,0,1,11.32-11.32L200,68.69l18.34-18.35a8,8,0,0,1,11.32,11.32L211.31,80ZM128,192a12,12,0,1,0,12,12A12,12,0,0,0,128,192Zm44.71-33.47a76.05,76.05,0,0,0-89.42,0,8,8,0,0,0,9.42,12.94,60,60,0,0,1,70.58,0,8,8,0,1,0,9.42-12.94Zm-29.48-93.8a8,8,0,1,0,1.54-15.92C139.24,48.27,133.59,48,128,48A172.35,172.35,0,0,0,18.92,87,8,8,0,1,0,29.08,99.37,156.25,156.25,0,0,1,128,64C133.08,64,138.2,64.25,143.23,64.73ZM142.91,113a8,8,0,0,0,2.18-15.85A124.75,124.75,0,0,0,128,96a122.74,122.74,0,0,0-77,26.77A8,8,0,0,0,56,137a7.93,7.93,0,0,0,5-1.73A106.87,106.87,0,0,1,128,112,109,109,0,0,1,142.91,113Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M136,204a8,8,0,1,1-8-8A8,8,0,0,1,136,204ZM205.66,80l21.17-21.17a4,4,0,1,0-5.66-5.66L200,74.34,178.83,53.17a4,4,0,0,0-5.66,5.66L194.34,80l-21.17,21.17a4,4,0,0,0,5.66,5.66L200,85.66l21.17,21.17a4,4,0,1,0,5.66-5.66Zm-35.31,81.77a72,72,0,0,0-84.71,0,4,4,0,0,0,4.71,6.46,64.05,64.05,0,0,1,75.29,0,4,4,0,1,0,4.71-6.46Zm-26.74-101a4,4,0,1,0,.78-8C139,52.27,133.46,52,128,52A168.33,168.33,0,0,0,21.46,90.09a4,4,0,1,0,5.08,6.19A160.22,160.22,0,0,1,128,60C133.21,60,138.46,60.25,143.61,60.75Zm-.16,48.31a4,4,0,0,0,1.1-7.93A121.84,121.84,0,0,0,128,100a118.72,118.72,0,0,0-74.48,25.91,4,4,0,0,0,5,6.27A110.84,110.84,0,0,1,128,108,113.6,113.6,0,0,1,143.45,109.06Z"></path>
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
