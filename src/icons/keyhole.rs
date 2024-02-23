//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "objects", feature = "system"))]
#[component]
pub fn Keyhole(
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
                <path d="M128,20A108,108,0,1,0,236,128,108.12,108.12,0,0,0,128,20Zm0,192a84,84,0,1,1,84-84A84.09,84.09,0,0,1,128,212Zm0-144a44,44,0,0,0-32.94,73.16l-9.92,24.92A16,16,0,0,0,100,188h56a16,16,0,0,0,14.86-21.92l-9.92-24.92A44,44,0,0,0,128,68Zm7.59,74.38L144.2,164H111.8l8.61-21.62a12,12,0,0,0-4.11-14.16,20,20,0,1,1,23.4,0A12,12,0,0,0,135.59,142.38Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M156,176H100a4,4,0,0,1-3.71-5.48l13-32.58a32,32,0,1,1,37.48,0l13,32.58A4,4,0,0,1,156,176Z"
        opacity="0.2"
    ></path>
    <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm0,192a88,88,0,1,1,88-88A88.1,88.1,0,0,1,128,216Zm0-144a40,40,0,0,0-28.28,68.28L88.86,167.56A12,12,0,0,0,100,184h56a12,12,0,0,0,11.14-16.44l-10.86-27.28A40,40,0,0,0,128,72Zm11.31,68.9L150.1,168H105.9l10.79-27.1a8,8,0,0,0-2.74-9.44,24,24,0,1,1,28.1,0A8,8,0,0,0,139.31,140.9Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm0,192a88,88,0,1,1,88-88A88.1,88.1,0,0,1,128,216Zm22.58-80,12.56,31.55A12,12,0,0,1,152,184H104a12,12,0,0,1-11.14-16.44L105.42,136a36,36,0,1,1,45.16,0Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M128,26A102,102,0,1,0,230,128,102.12,102.12,0,0,0,128,26Zm0,192a90,90,0,1,1,90-90A90.1,90.1,0,0,1,128,218Zm38-106a38,38,0,1,0-63.93,27.78L90.71,168.3A10,10,0,0,0,100,182h56a10,10,0,0,0,9.29-13.7l-11.36-28.52A38.14,38.14,0,0,0,166,112Zm-22.78,21.08a6,6,0,0,0-2.06,7.08L153.05,170H103l11.89-29.84a6,6,0,0,0-2.06-7.08,26,26,0,1,1,30.44,0Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm0,192a88,88,0,1,1,88-88A88.1,88.1,0,0,1,128,216Zm0-144a40,40,0,0,0-28.28,68.28L88.86,167.56A12,12,0,0,0,100,184h56a12,12,0,0,0,11.14-16.44l-10.86-27.28A40,40,0,0,0,128,72Zm11.31,68.9L150.1,168H105.9l10.79-27.1a8,8,0,0,0-2.74-9.44,24,24,0,1,1,28.1,0A8,8,0,0,0,139.31,140.9Zm13,32.58Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M128,28A100,100,0,1,0,228,128,100.11,100.11,0,0,0,128,28Zm0,192a92,92,0,1,1,92-92A92.1,92.1,0,0,1,128,220Zm36-108a36,36,0,1,0-59.55,27.22L92.57,169A8,8,0,0,0,100,180h56a8,8,0,0,0,7.43-11l-11.88-29.82A36.11,36.11,0,0,0,164,112Zm-21,27.42L156,172H100l13-32.58a4,4,0,0,0-1.37-4.72,28,28,0,1,1,32.78,0A4,4,0,0,0,143,139.42Z"></path>
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
