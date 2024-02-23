//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "nature", feature = "weather"))]
#[component]
pub fn FireSimple(
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
                <path d="M143.38,17.85a8,8,0,0,0-12.63,3.41l-22,60.41L84.59,58.26a8,8,0,0,0-11.93.89C51,87.53,40,116.08,40,144a88,88,0,0,0,176,0C216,84.55,165.21,36,143.38,17.85Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M208,144a80,80,0,0,1-160,0c0-30.57,14.42-58.26,31-80l33,32,26.27-72C159.86,41.92,208,88.15,208,144Z"
        opacity="0.2"
    ></path>
    <path d="M143.38,17.85a8,8,0,0,0-12.63,3.41l-22,60.41L84.59,58.26a8,8,0,0,0-11.93.89C51,87.53,40,116.08,40,144a88,88,0,0,0,176,0C216,84.55,165.21,36,143.38,17.85ZM128,216a72.08,72.08,0,0,1-72-72c0-22,8.09-44.79,24.06-67.84l26.37,25.58a8,8,0,0,0,13.09-3l22.27-61.07C164.21,58.08,200,97.91,200,144A72.08,72.08,0,0,1,128,216Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M140.82,20.92a4,4,0,0,0-6.31,1.71l-24.15,66.2L81.81,61.13a4,4,0,0,0-6,.44C54.71,89.25,44,117,44,144a84,84,0,0,0,168,0C212,86.24,162.22,38.69,140.82,20.92ZM128,220a76.08,76.08,0,0,1-76-76c0-24.07,9.25-48.93,27.5-74l29.71,28.83a4,4,0,0,0,6.55-1.5l24.3-66.6C162.27,50.09,204,93.18,204,144A76.08,76.08,0,0,1,128,220Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M145.93,14.77A12,12,0,0,0,127,19.89L107.07,74.5,87.38,55.39a12,12,0,0,0-17.9,1.33C47.27,85.82,36,115.19,36,144a92,92,0,0,0,184,0C220,82.85,168.2,33.25,145.93,14.77ZM128,212a68.07,68.07,0,0,1-68-68c0-19.94,7-40.62,20.71-61.64l22.93,22.25a12,12,0,0,0,19.63-4.5l20.21-55.4C165.39,65.61,196,102.29,196,144A68.07,68.07,0,0,1,128,212Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M142.1,19.38a6,6,0,0,0-9.47,2.56L109.54,85.25,83.2,59.69a6,6,0,0,0-9,.67C52.85,88.39,42,116.53,42,144a86,86,0,0,0,172,0C214,85.4,163.72,37.33,142.1,19.38ZM128,218a74.09,74.09,0,0,1-74-74c0-23,8.67-46.87,25.77-70.91l28.05,27.22a6,6,0,0,0,9.82-2.25L140.93,34.2C163.35,54.16,202,95.6,202,144A74.09,74.09,0,0,1,128,218Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M143.38,17.85a8,8,0,0,0-12.63,3.41l-22,60.41L84.59,58.26a8,8,0,0,0-11.93.89C51,87.53,40,116.08,40,144a88,88,0,0,0,176,0C216,84.55,165.21,36,143.38,17.85ZM128,216a72.08,72.08,0,0,1-72-72c0-22,8.09-44.79,24.06-67.84l26.37,25.58a8,8,0,0,0,13.09-3l22.27-61.07C164.21,58.08,200,97.91,200,144A72.08,72.08,0,0,1,128,216Z"></path>
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
