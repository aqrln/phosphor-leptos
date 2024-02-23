//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "commerce", feature = "objects", feature = "health"))]
#[component]
pub fn Boot(
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
                <path d="M192,112H112.27a8.17,8.17,0,0,1-8.25-7.47A8,8,0,0,1,112,96h44a4,4,0,0,0,4-4V84a4,4,0,0,0-4-4H112.27A8.17,8.17,0,0,1,104,72.53,8,8,0,0,1,112,64h44a4,4,0,0,0,4-4V56a16,16,0,0,0-16-16H32.22a8.23,8.23,0,0,0-5.08,1.64,8,8,0,0,0-2.61,9.22c11.06,28.84,8.76,83.71-.22,114.93A8,8,0,0,0,24,168v32a16,16,0,0,0,16,16H66.11a16,16,0,0,0,7.16-1.69L85.89,208h16.22l12.62,6.31a16,16,0,0,0,7.16,1.69h28.22a16,16,0,0,0,7.16-1.69L169.89,208h16.22l12.62,6.31a16,16,0,0,0,7.16,1.69H232a16,16,0,0,0,16-16V168A56,56,0,0,0,192,112Zm40,88H205.89l-12.62-6.31a16,16,0,0,0-7.16-1.69H169.89a16,16,0,0,0-7.16,1.69L150.11,200H121.89l-12.62-6.31a16,16,0,0,0-7.16-1.69H85.89a16,16,0,0,0-7.16,1.69L66.11,200H40V176H232Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M32,168c9.22-32.06,12-88.65,0-120H144a8,8,0,0,1,8,8v64h40a48,48,0,0,1,48,48Z"
        opacity="0.2"
    ></path>
    <path d="M192,112H160V56a16,16,0,0,0-16-16H32a8,8,0,0,0-7.47,10.86c11.06,28.84,8.76,83.71-.22,114.93A8.25,8.25,0,0,0,24,168v32a16,16,0,0,0,16,16H66.11a16,16,0,0,0,7.16-1.69L85.89,208h16.22l12.62,6.31a16,16,0,0,0,7.16,1.69h28.22a16,16,0,0,0,7.16-1.69L169.89,208h16.22l12.62,6.31a16,16,0,0,0,7.16,1.69H232a16,16,0,0,0,16-16V168A56.06,56.06,0,0,0,192,112ZM144,56V80H112a8,8,0,0,0,0,16h32v16H112a8,8,0,0,0,0,16h80a40.07,40.07,0,0,1,39.2,32H42.25c6.74-30.84,8.16-74.17.61-104Zm61.89,144-12.62-6.31a16,16,0,0,0-7.16-1.69H169.89a16,16,0,0,0-7.16,1.69L150.11,200H121.89l-12.62-6.31a16,16,0,0,0-7.16-1.69H85.89a16,16,0,0,0-7.16,1.69L66.11,200H40V176H232v24Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M192,116H156V56a12,12,0,0,0-12-12H32a4,4,0,0,0-3.73,5.43c9.29,24.23,11.46,77.22-.11,117.46A3.82,3.82,0,0,0,28,168v32a12,12,0,0,0,12,12H66.11a12.08,12.08,0,0,0,5.37-1.27l12.62-6.31a4.09,4.09,0,0,1,1.79-.42h16.22a4.09,4.09,0,0,1,1.79.42l12.62,6.31a12.08,12.08,0,0,0,5.37,1.27h28.22a12.08,12.08,0,0,0,5.37-1.27l12.62-6.31a4.09,4.09,0,0,1,1.79-.42h16.22a4.09,4.09,0,0,1,1.79.42l12.62,6.31a12.08,12.08,0,0,0,5.37,1.27H232a12,12,0,0,0,12-12V168A52.06,52.06,0,0,0,192,116ZM37.6,52H144a4,4,0,0,1,4,4V84H112a4,4,0,0,0,0,8h36v24H112a4,4,0,0,0,0,8h80a44.06,44.06,0,0,1,43.81,40H37.2C45.24,131.81,47,82.65,37.6,52ZM236,200a4,4,0,0,1-4,4H205.89a4.09,4.09,0,0,1-1.79-.42l-12.62-6.31a12.08,12.08,0,0,0-5.37-1.27H169.89a12.08,12.08,0,0,0-5.37,1.27l-12.62,6.31a4.09,4.09,0,0,1-1.79.42H121.89a4.09,4.09,0,0,1-1.79-.42l-12.62-6.31a12.08,12.08,0,0,0-5.37-1.27H85.89a12.08,12.08,0,0,0-5.37,1.27L67.9,203.58a4.09,4.09,0,0,1-1.79.42H40a4,4,0,0,1-4-4V172H236Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M192,104H164V56a20,20,0,0,0-20-20H32A12,12,0,0,0,20.8,52.3c10.88,28.39,8.3,78.36-.33,108.38A12,12,0,0,0,20,164v36a20,20,0,0,0,20,20H66.11a20.16,20.16,0,0,0,9-2.11L86.83,212h14.34l11.77,5.89a20.16,20.16,0,0,0,9,2.11h28.22a20.16,20.16,0,0,0,8.95-2.11L170.83,212h14.34l11.77,5.89a20.16,20.16,0,0,0,8.95,2.11H232a20,20,0,0,0,20-20V164A60.07,60.07,0,0,0,192,104ZM48,60h92v44H116a12,12,0,0,0,0,24h76a36.07,36.07,0,0,1,33.94,24H47.21C52.75,124.08,54.23,88.41,48,60ZM228,196H206.83l-11.77-5.89a20.16,20.16,0,0,0-8.95-2.11H169.89a20.16,20.16,0,0,0-8.95,2.11L149.17,196H122.83l-11.77-5.89a20.16,20.16,0,0,0-9-2.11H85.89a20.16,20.16,0,0,0-9,2.11L65.17,196H44V176H228Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M192,114H158V56a14,14,0,0,0-14-14H32a6,6,0,0,0-5.6,8.15c9,23.44,11.13,76.92-.17,116.19A6.21,6.21,0,0,0,26,168v32a14,14,0,0,0,14,14H66.11a14,14,0,0,0,6.26-1.48L85,206.21a2,2,0,0,1,.9-.21h16.22a2,2,0,0,1,.9.21l12.62,6.31a14,14,0,0,0,6.26,1.48h28.22a14,14,0,0,0,6.26-1.48L169,206.21a2,2,0,0,1,.9-.21h16.22a2,2,0,0,1,.9.21l12.62,6.31a14,14,0,0,0,6.26,1.48H232a14,14,0,0,0,14-14V168A54.06,54.06,0,0,0,192,114ZM40.27,54H144a2,2,0,0,1,2,2V82H112a6,6,0,0,0,0,12h34v20H112a6,6,0,0,0,0,12h80a42.05,42.05,0,0,1,41.56,36H39.75C47.11,130.44,48.71,84.31,40.27,54ZM234,200a2,2,0,0,1-2,2H205.89a2,2,0,0,1-.9-.21l-12.62-6.31a14,14,0,0,0-6.26-1.48H169.89a14,14,0,0,0-6.26,1.48L151,201.79a2,2,0,0,1-.9.21H121.89a2,2,0,0,1-.9-.21l-12.62-6.31a14,14,0,0,0-6.26-1.48H85.89a14,14,0,0,0-6.26,1.48L67,201.79a2,2,0,0,1-.9.21H40a2,2,0,0,1-2-2V174H234Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M192,112H160V56a16,16,0,0,0-16-16H32a8,8,0,0,0-7.47,10.86c11.06,28.84,8.76,83.71-.22,114.93A8.25,8.25,0,0,0,24,168v32a16,16,0,0,0,16,16H66.11a16,16,0,0,0,7.16-1.69L85.89,208h16.22l12.62,6.31a16,16,0,0,0,7.16,1.69h28.22a16,16,0,0,0,7.16-1.69L169.89,208h16.22l12.62,6.31a16,16,0,0,0,7.16,1.69H232a16,16,0,0,0,16-16V168A56.06,56.06,0,0,0,192,112ZM42.86,56H144V80H112a8,8,0,0,0,0,16h32v16H112a8,8,0,0,0,0,16h80a40.07,40.07,0,0,1,39.2,32H42.25C49,129.16,50.41,85.83,42.86,56ZM232,200H205.89l-12.62-6.31a16,16,0,0,0-7.16-1.69H169.89a16,16,0,0,0-7.16,1.69L150.11,200H121.89l-12.62-6.31a16,16,0,0,0-7.16-1.69H85.89a16,16,0,0,0-7.16,1.69L66.11,200H40V176H232Z"></path>
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
