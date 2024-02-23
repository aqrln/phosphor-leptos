//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "arrows"))]
#[component]
pub fn ArrowsInCardinal(
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
                <path d="M87.51,72.49a12,12,0,0,1,17-17L116,67V24a12,12,0,0,1,24,0V67l11.51-11.52a12,12,0,0,1,17,17l-32,32a12,12,0,0,1-17,0Zm49,79a12,12,0,0,0-17,0l-32,32a12,12,0,0,0,17,17L116,189v43a12,12,0,0,0,24,0V189l11.51,11.52a12,12,0,0,0,17-17ZM232,116H189l11.52-11.51a12,12,0,0,0-17-17l-32,32a12,12,0,0,0,0,17l32,32a12,12,0,0,0,17-17L189,140h43a12,12,0,0,0,0-24Zm-127.51,3.51-32-32a12,12,0,0,0-17,17L67,116H24a12,12,0,0,0,0,24H67L55.51,151.51a12,12,0,0,0,17,17l32-32A12,12,0,0,0,104.49,119.51Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M128,96,96,64h64ZM96,192h64l-32-32Zm64-64,32,32V96ZM64,96v64l32-32Z"
        opacity="0.2"
    ></path>
    <path d="M122.34,101.66a8,8,0,0,0,11.32,0l32-32A8,8,0,0,0,160,56H136V24a8,8,0,0,0-16,0V56H96a8,8,0,0,0-5.66,13.66ZM140.69,72,128,84.69,115.31,72Zm-7,82.34a8,8,0,0,0-11.32,0l-32,32A8,8,0,0,0,96,200h24v32a8,8,0,0,0,16,0V200h24a8,8,0,0,0,5.66-13.66ZM115.31,184,128,171.31,140.69,184ZM232,120H200V96a8,8,0,0,0-13.66-5.66l-32,32a8,8,0,0,0,0,11.32l32,32A8,8,0,0,0,200,160V136h32a8,8,0,0,0,0-16Zm-48,20.69L171.31,128,184,115.31Zm-82.34-18.35-32-32A8,8,0,0,0,56,96v24H24a8,8,0,0,0,0,16H56v24a8,8,0,0,0,13.66,5.66l32-32A8,8,0,0,0,101.66,122.34ZM72,140.69V115.31L84.69,128Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M101.66,133.66l-32,32A8,8,0,0,1,56,160V136H24a8,8,0,0,1,0-16H56V96a8,8,0,0,1,13.66-5.66l32,32A8,8,0,0,1,101.66,133.66Zm20.68-32a8,8,0,0,0,11.32,0l32-32A8,8,0,0,0,160,56H136V24a8,8,0,0,0-16,0V56H96a8,8,0,0,0-5.66,13.66Zm11.32,52.68a8,8,0,0,0-11.32,0l-32,32A8,8,0,0,0,96,200h24v32a8,8,0,0,0,16,0V200h24a8,8,0,0,0,5.66-13.66ZM232,120H200V96a8,8,0,0,0-13.66-5.66l-32,32a8,8,0,0,0,0,11.32l32,32A8,8,0,0,0,200,160V136h32a8,8,0,0,0,0-16Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M91.76,68.24a6,6,0,0,1,8.48-8.48L122,81.51V24a6,6,0,0,1,12,0V81.51l21.76-21.75a6,6,0,0,1,8.48,8.48l-32,32a6,6,0,0,1-8.48,0Zm40.48,87.52a6,6,0,0,0-8.48,0l-32,32a6,6,0,0,0,8.48,8.48L122,174.49V232a6,6,0,0,0,12,0V174.49l21.76,21.75a6,6,0,0,0,8.48-8.48ZM232,122H174.49l21.75-21.76a6,6,0,0,0-8.48-8.48l-32,32a6,6,0,0,0,0,8.48l32,32a6,6,0,0,0,8.48-8.48L174.49,134H232a6,6,0,0,0,0-12Zm-131.76,1.76-32-32a6,6,0,0,0-8.48,8.48L81.51,122H24a6,6,0,0,0,0,12H81.51L59.76,155.76a6,6,0,1,0,8.48,8.48l32-32A6,6,0,0,0,100.24,123.76Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M90.34,69.66a8,8,0,0,1,11.32-11.32L120,76.69V24a8,8,0,0,1,16,0V76.69l18.34-18.35a8,8,0,0,1,11.32,11.32l-32,32a8,8,0,0,1-11.32,0Zm43.32,84.68a8,8,0,0,0-11.32,0l-32,32a8,8,0,0,0,11.32,11.32L120,179.31V232a8,8,0,0,0,16,0V179.31l18.34,18.35a8,8,0,0,0,11.32-11.32ZM232,120H179.31l18.35-18.34a8,8,0,0,0-11.32-11.32l-32,32a8,8,0,0,0,0,11.32l32,32a8,8,0,0,0,11.32-11.32L179.31,136H232a8,8,0,0,0,0-16Zm-130.34,2.34-32-32a8,8,0,0,0-11.32,11.32L76.69,120H24a8,8,0,0,0,0,16H76.69L58.34,154.34a8,8,0,0,0,11.32,11.32l32-32A8,8,0,0,0,101.66,122.34Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M93.17,66.83a4,4,0,0,1,5.66-5.66L124,86.34V24a4,4,0,0,1,8,0V86.34l25.17-25.17a4,4,0,1,1,5.66,5.66l-32,32a4,4,0,0,1-5.66,0Zm37.66,90.34a4,4,0,0,0-5.66,0l-32,32a4,4,0,0,0,5.66,5.66L124,169.66V232a4,4,0,0,0,8,0V169.66l25.17,25.17a4,4,0,0,0,5.66-5.66ZM232,124H169.66l25.17-25.17a4,4,0,1,0-5.66-5.66l-32,32a4,4,0,0,0,0,5.66l32,32a4,4,0,0,0,5.66-5.66L169.66,132H232a4,4,0,0,0,0-8ZM98.83,125.17l-32-32a4,4,0,0,0-5.66,5.66L86.34,124H24a4,4,0,0,0,0,8H86.34L61.17,157.17a4,4,0,0,0,5.66,5.66l32-32A4,4,0,0,0,98.83,125.17Z"></path>
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
