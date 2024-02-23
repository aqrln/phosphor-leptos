//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "nature", feature = "objects"))]
#[component]
pub fn Feather(
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
                <path d="M211.84,134.81l-59.79,60.47,0,0a15.75,15.75,0,0,1-11.2,4.68H75.32L45.66,229.66a8,8,0,0,1-11.32-11.32l22.59-22.58h0L124.7,128H209A4,4,0,0,1,211.84,134.81ZM216.7,30.57a64,64,0,0,0-85.9,4.14l-9.6,9.48A4,4,0,0,0,120,47v63l55-55a8,8,0,0,1,11.31,11.31L140.71,112h88.38a4,4,0,0,0,3.56-2.16A64.08,64.08,0,0,0,216.7,30.57ZM62.83,167.23,104,126.06V70.76a4,4,0,0,0-6.81-2.84L60.69,104A15.9,15.9,0,0,0,56,115.31V164.4A4,4,0,0,0,62.83,167.23Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M215.8,119.6l-69.26,70.06a8,8,0,0,1-5.65,2.34H64.2V115.31a8,8,0,0,1,2.34-5.65L112.2,64.52V144l24-24Z"
        opacity="0.2"
    ></path>
    <path d="M221.28,34.75a64,64,0,0,0-90.49,0L60.69,104A15.9,15.9,0,0,0,56,115.31v73.38L26.34,218.34a8,8,0,0,0,11.32,11.32L67.32,200H140.7A15.92,15.92,0,0,0,152,195.32l0,0,69.23-70A64,64,0,0,0,221.28,34.75ZM142.07,46.06A48,48,0,0,1,211.79,112H155.33l34.35-34.34a8,8,0,0,0-11.32-11.32L120,124.69V67.87ZM72,115.35l32-31.67v57l-32,32ZM140.7,184H83.32l56-56h56.74Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M236,80A60,60,0,0,0,133.59,37.56L63.52,106.83A11.9,11.9,0,0,0,60,115.31v75L29.17,221.17a4,4,0,0,0,5.66,5.66L65.66,196h75a12,12,0,0,0,8.48-3.51l0,0L218,122.83h0l.4-.4A59.63,59.63,0,0,0,236,80ZM139.23,43.23A52,52,0,0,1,213.5,116H145.66l41.17-41.17a4,4,0,1,0-5.66-5.66L116,134.34V66.19ZM68,115.31a4,4,0,0,1,1.16-2.81L108,74.1v68.24l-40,40Zm75.51,71.52a4,4,0,0,1-2.82,1.17h-67l64-64h68Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M224.11,31.92A68,68,0,0,0,128,31.87l-70.12,69.3A19.91,19.91,0,0,0,52,115.31V187L23.52,215.51a12,12,0,0,0,17,17L69,204H140.7a19.87,19.87,0,0,0,14.15-5.86l.05,0,69.21-70A68.06,68.06,0,0,0,224.11,31.92Zm-79.21,17A44,44,0,0,1,210,108H165l27.52-27.51a12,12,0,0,0-17-17L124,115V69.54ZM76,117l24-23.72V139L76,163Zm63,63H93l48-48h45.5Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M238,80A62,62,0,0,0,132.18,36.14L62.1,105.41a13.94,13.94,0,0,0-4.1,9.9v74.21L27.76,219.76a6,6,0,1,0,8.48,8.48L66.48,198h74.21a13.94,13.94,0,0,0,9.9-4.1l0,0,68.83-69.63h0l.39-.4A61.6,61.6,0,0,0,238,80ZM140.64,44.64a50,50,0,0,1,72,69.36H150.48l37.76-37.76a6,6,0,0,0-8.48-8.48l-48,48h0L118,129.52V67ZM70,115.31a2,2,0,0,1,.56-1.39l35.44-35v62.63l-36,36Zm72.09,70.11a2,2,0,0,1-1.4.58H78.48l37.76-37.75h0L138.48,126h62.35Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M221.28,34.75a64,64,0,0,0-90.49,0L60.69,104A15.9,15.9,0,0,0,56,115.31v73.38L26.34,218.34a8,8,0,0,0,11.32,11.32L67.32,200H140.7A15.92,15.92,0,0,0,152,195.32l0,0,69.23-70A64,64,0,0,0,221.28,34.75ZM142.07,46.06A48,48,0,0,1,211.79,112H155.33l34.35-34.34a8,8,0,0,0-11.32-11.32L120,124.69V67.87ZM72,115.35l32-31.67v57l-32,32ZM140.7,184H83.32l56-56h56.74Z"></path>
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
