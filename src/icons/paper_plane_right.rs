//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "communication", feature = "map", feature = "objects"))]
#[component]
pub fn PaperPlaneRight(
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
                <path d="M232,127.89a16,16,0,0,1-8.18,14L55.91,237.9A16.14,16.14,0,0,1,48,240a16,16,0,0,1-15.05-21.34L60.3,138.71A4,4,0,0,1,64.09,136H136a8,8,0,0,0,8-8.53,8.19,8.19,0,0,0-8.26-7.47H64.16a4,4,0,0,1-3.79-2.7l-27.44-80A16,16,0,0,1,55.85,18.07l168,95.89A16,16,0,0,1,232,127.89Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M219.91,134.86,51.93,231a8,8,0,0,1-11.44-9.67l31-90.71a7.89,7.89,0,0,0,0-5.38l-31-90.47a8,8,0,0,1,11.44-9.67l168,95.85A8,8,0,0,1,219.91,134.86Z"
        opacity="0.2"
    ></path>
    <path d="M223.87,114l-168-95.89A16,16,0,0,0,32.93,37.32l31,90.47a.42.42,0,0,0,0,.1.3.3,0,0,0,0,.1l-31,90.67A16,16,0,0,0,48,240a16.14,16.14,0,0,0,7.92-2.1l167.91-96.05a16,16,0,0,0,.05-27.89ZM48,224l0-.09L78.14,136H136a8,8,0,0,0,0-16H78.22L48.06,32.12,48,32l168,95.83Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M221.89,117.43l-168-95.88A12,12,0,0,0,36.7,36l31.05,90.48v.05a4.09,4.09,0,0,1,0,2.74L36.72,220A12,12,0,0,0,48,236a12.13,12.13,0,0,0,5.93-1.57l167.94-96.08a12,12,0,0,0,0-20.92Zm-4,14L50,227.47a4,4,0,0,1-5.7-4.88l31-90.59H136a4,4,0,0,0,0-8H75.35a.65.65,0,0,1,0-.13L44.25,33.37A4,4,0,0,1,50,28.52l168,95.87a4,4,0,0,1,0,7Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M225.86,110.48,57.8,14.58A20,20,0,0,0,29.16,38.67l30.61,89.21L29.16,217.33A20,20,0,0,0,48,244a20.1,20.1,0,0,0,9.81-2.58l.09-.06,168-96.07a20,20,0,0,0,0-34.81ZM55.24,215.23,81,140h55a12,12,0,0,0,0-24H81.07L55.25,40.76l152.69,87.13Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M222.88,115.69l-168-95.88a14,14,0,0,0-20,16.85l31,90.48,0,.07a2.11,2.11,0,0,1,0,1.42l-31,90.64A14,14,0,0,0,48,238a14.11,14.11,0,0,0,6.92-1.83L222.84,140.1a14,14,0,0,0,0-24.41Zm-5.95,14L49,225.73a1.87,1.87,0,0,1-2.27-.22,1.92,1.92,0,0,1-.56-2.28L76.7,134H136a6,6,0,0,0,0-12H76.78L46.14,32.7A2,2,0,0,1,49,30.25l168,95.89a1.93,1.93,0,0,1,1,1.74A2,2,0,0,1,216.93,129.66Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M223.87,114l-168-95.89A16,16,0,0,0,32.93,37.32l31,90.47a.42.42,0,0,0,0,.1.3.3,0,0,0,0,.1l-31,90.67A16,16,0,0,0,48,240a16.14,16.14,0,0,0,7.92-2.1l167.91-96.05a16,16,0,0,0,.05-27.89ZM48,224l0-.09L78.14,136H136a8,8,0,0,0,0-16H78.22L48.06,32.12,48,32l168,95.83Z"></path>
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
