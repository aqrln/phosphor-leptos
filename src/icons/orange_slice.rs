//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "map", feature = "commerce", feature = "nature"))]
#[component]
pub fn OrangeSlice(
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
                <path d="M244,76H12A12,12,0,0,0,0,88v4a128,128,0,0,0,256,0V88A12,12,0,0,0,244,76ZM64.52,100h42.69L84.67,139.05A63.87,63.87,0,0,1,64.52,100ZM128,112l22.94,39.73a63.76,63.76,0,0,1-45.88,0Zm43.33,27.05L148.79,100h42.69A63.87,63.87,0,0,1,171.33,139.05ZM128,196A104.15,104.15,0,0,1,24.3,100H40.37a88,88,0,0,0,175.26,0H231.7A104.15,104.15,0,0,1,128,196Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M216,88A88,88,0,0,1,40,88Z" opacity="0.2"></path>
    <path d="M248,80H8a8,8,0,0,0-8,8,128,128,0,0,0,256,0A8,8,0,0,0,248,80ZM77.4,149.91l42.6-42.6V167.6A79.59,79.59,0,0,1,77.4,149.91ZM66.09,138.6A79.59,79.59,0,0,1,48.4,96h60.29ZM136,107.31l42.6,42.6A79.59,79.59,0,0,1,136,167.6Zm53.91,31.29L147.31,96H207.6A79.59,79.59,0,0,1,189.91,138.6ZM128,200A112.15,112.15,0,0,1,16.28,96H32.34a96,96,0,0,0,191.32,0h16.06A112.15,112.15,0,0,1,128,200Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M51.18,145.51A95.5,95.5,0,0,1,32,88c0-1.41,0-2.81.09-4.21a4,4,0,0,1,4-3.79H107a4,4,0,0,1,2.83,6.83ZM120,183.66V99.31L61.83,157.49A95.68,95.68,0,0,0,120,183.66Zm84.82-38.15A95.5,95.5,0,0,0,224,88c0-1.41,0-2.81-.09-4.21a4,4,0,0,0-4-3.79H149a4,4,0,0,0-2.83,6.83ZM248,80h-4.08a4,4,0,0,0-4,4.14c0,1.28.07,2.57.07,3.86A112,112,0,0,1,16,88c0-1.29,0-2.58.07-3.86a4,4,0,0,0-4-4.14H8a8,8,0,0,0-8,8,128,128,0,1,0,256,0A8,8,0,0,0,248,80ZM136,99.31v84.35a95.68,95.68,0,0,0,58.17-26.17Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M248,82H8a6,6,0,0,0-6,6,126,126,0,0,0,252,0A6,6,0,0,0,248,82ZM74.46,150,122,102.48v67.28A81.66,81.66,0,0,1,74.46,150ZM66,141.54A81.66,81.66,0,0,1,46.24,94h67.28Zm68-39.06L181.54,150A81.66,81.66,0,0,1,134,169.76Zm56,39.06L142.48,94h67.28A81.66,81.66,0,0,1,190,141.54ZM128,202A114.14,114.14,0,0,1,14.16,94h20a94,94,0,0,0,187.6,0h20A114.14,114.14,0,0,1,128,202Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M248,80H8a8,8,0,0,0-8,8,128,128,0,0,0,256,0A8,8,0,0,0,248,80ZM77.4,149.91l42.6-42.6V167.6A79.59,79.59,0,0,1,77.4,149.91ZM66.09,138.6A79.59,79.59,0,0,1,48.4,96h60.29ZM136,107.31l42.6,42.6A79.59,79.59,0,0,1,136,167.6Zm53.91,31.29L147.31,96H207.6A79.59,79.59,0,0,1,189.91,138.6ZM128,200A112.15,112.15,0,0,1,16.28,96H32.34a96,96,0,0,0,191.32,0h16.06A112.15,112.15,0,0,1,128,200Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M248,84H8a4,4,0,0,0-4,4,124,124,0,0,0,248,0A4,4,0,0,0,248,84ZM71.53,150.13,124,97.66V171.9A83.67,83.67,0,0,1,71.53,150.13Zm-5.66-5.66A83.67,83.67,0,0,1,44.1,92h74.24ZM132,97.66l52.47,52.47A83.67,83.67,0,0,1,132,171.9Zm58.13,46.81L137.66,92H211.9A83.67,83.67,0,0,1,190.13,144.47ZM128,204A116.14,116.14,0,0,1,12.07,92h24a92,92,0,0,0,183.82,0h24A116.14,116.14,0,0,1,128,204Z"></path>
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
