//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "office", feature = "editor", feature = "system"))]
#[component]
pub fn FolderSimpleStar(
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
                <path d="M120,208a12,12,0,0,1-12,12H40a20,20,0,0,1-20-20V64A20,20,0,0,1,40,44H93.33a20.12,20.12,0,0,1,12,4L132,68h84a20,20,0,0,1,20,20v20a12,12,0,0,1-24,0V92H130.67a20.12,20.12,0,0,1-12-4L92,68H44V196h64A12,12,0,0,1,120,208Zm119.64-37.13-20.58,17,6.25,25.26a12,12,0,0,1-17.73,13.22L184,212.46l-23.58,13.88a12,12,0,0,1-17.73-13.22l6.25-25.26-20.58-17a12,12,0,0,1,6.72-21.22l27.42-2.12L173,123.24a12,12,0,0,1,22,0l10.48,24.29,27.42,2.12a12,12,0,0,1,6.72,21.22Zm-38.2.42-5-.39a12,12,0,0,1-10.09-7.21l-2.33-5.4-2.33,5.4a12,12,0,0,1-10.1,7.21l-5,.39,3.48,2.87a12,12,0,0,1,4,12.13l-1.21,4.89,5.07-3a12,12,0,0,1,12.18,0l5.07,3L194,186.29a12,12,0,0,1,4-12.13Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M205.6,183.41,213.67,216,184,198.54,154.33,216l8.07-32.59L136,161.61l34.65-2.67L184,128l13.35,30.94L232,161.61Z"
        opacity="0.2"
    ></path>
    <path d="M128,208a8,8,0,0,1-8,8H40a16,16,0,0,1-16-16V64A16,16,0,0,1,40,48H93.33a16.12,16.12,0,0,1,9.6,3.2L130.67,72H216a16,16,0,0,1,16,16v32a8,8,0,0,1-16,0V88H130.67a16.12,16.12,0,0,1-9.6-3.2L93.33,64H40V200h80A8,8,0,0,1,128,208Zm109.09-40.22-22.51,18.59,6.85,27.71a8,8,0,0,1-11.82,8.81L184,207.82l-25.61,15.07a8,8,0,0,1-11.82-8.81l6.85-27.71-22.51-18.59a8,8,0,0,1,4.47-14.14l29.84-2.31,11.43-26.5a8,8,0,0,1,14.7,0l11.43,26.5,29.84,2.31a8,8,0,0,1,4.47,14.14Zm-25.47.28-14.89-1.15a8,8,0,0,1-6.73-4.8l-6-13.92-6,13.92a8,8,0,0,1-6.73,4.8l-14.89,1.15,11.11,9.18a8,8,0,0,1,2.68,8.09l-3.5,14.12,13.27-7.81a8,8,0,0,1,8.12,0l13.27,7.81-3.5-14.12a8,8,0,0,1,2.68-8.09Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M128,208a8,8,0,0,1-8,8H40a16,16,0,0,1-16-16V64A16,16,0,0,1,40,48H93.33a16.12,16.12,0,0,1,9.6,3.2L130.67,72H216a16,16,0,0,1,16,16v32a8,8,0,0,1-16,0V88H130.67a16.12,16.12,0,0,1-9.6-3.2L93.33,64H40V200h80A8,8,0,0,1,128,208Zm111.63-48.8a8,8,0,0,0-7-5.56l-29.84-2.31-11.43-26.5a8,8,0,0,0-14.7,0l-11.43,26.5-29.84,2.31a8,8,0,0,0-4.47,14.14l22.51,18.59-6.85,27.71a8,8,0,0,0,11.82,8.81L184,207.82l25.61,15.07a8,8,0,0,0,11.82-8.81l-6.85-27.71,22.51-18.59A8,8,0,0,0,239.63,159.2Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M126,208a6,6,0,0,1-6,6H40a14,14,0,0,1-14-14V64A14,14,0,0,1,40,50H93.33a14.06,14.06,0,0,1,8.4,2.8l27.74,20.8a2,2,0,0,0,1.2.4H216a14,14,0,0,1,14,14v32a6,6,0,0,1-12,0V88a2,2,0,0,0-2-2H130.67a14,14,0,0,1-8.4-2.8L94.53,62.4a2,2,0,0,0-1.2-.4H40a2,2,0,0,0-2,2V200a2,2,0,0,0,2,2h80A6,6,0,0,1,126,208Zm109.82-41.76-23.49,19.39,7.16,28.93a6,6,0,0,1-8.87,6.61L184,205.5l-26.62,15.67a6,6,0,0,1-8.87-6.61l7.16-28.93-23.49-19.39a6,6,0,0,1,3.36-10.61l31-2.4,11.91-27.61a6,6,0,0,1,11,0l11.91,27.61,31,2.4a6,6,0,0,1,3.36,10.61Zm-19.1.21-19.83-1.53a6,6,0,0,1-5-3.61L184,143.14l-7.84,18.17a6,6,0,0,1-5,3.61l-19.83,1.53,14.94,12.33a6,6,0,0,1,2,6.07l-4.63,18.74L181,193.36a6,6,0,0,1,6.08,0l17.37,10.23-4.64-18.74a6,6,0,0,1,2-6.07Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M128,208a8,8,0,0,1-8,8H40a16,16,0,0,1-16-16V64A16,16,0,0,1,40,48H93.33a16.12,16.12,0,0,1,9.6,3.2L130.67,72H216a16,16,0,0,1,16,16v32a8,8,0,0,1-16,0V88H130.67a16.12,16.12,0,0,1-9.6-3.2L93.33,64H40V200h80A8,8,0,0,1,128,208Zm109.09-40.22-22.51,18.59,6.85,27.71a8,8,0,0,1-11.82,8.81L184,207.82l-25.61,15.07a8,8,0,0,1-11.82-8.81l6.85-27.71-22.51-18.59a8,8,0,0,1,4.47-14.14l29.84-2.31,11.43-26.5a8,8,0,0,1,14.7,0l11.43,26.5,29.84,2.31a8,8,0,0,1,4.47,14.14Zm-25.47.28-14.89-1.15a8,8,0,0,1-6.73-4.8l-6-13.92-6,13.92a8,8,0,0,1-6.73,4.8l-14.89,1.15,11.11,9.18a8,8,0,0,1,2.68,8.09l-3.5,14.12,13.27-7.81a8,8,0,0,1,8.12,0l13.27,7.81-3.5-14.12a8,8,0,0,1,2.68-8.09Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M36,64V200a4,4,0,0,0,4,4h80a4,4,0,0,1,0,8H40a12,12,0,0,1-12-12V64A12,12,0,0,1,40,52H93.33a12.05,12.05,0,0,1,7.2,2.4l27.74,20.8a4,4,0,0,0,2.4.8H216a12,12,0,0,1,12,12v32a4,4,0,0,1-8,0V88a4,4,0,0,0-4-4H130.67a12.05,12.05,0,0,1-7.2-2.4L95.73,60.8a4,4,0,0,0-2.4-.8H40A4,4,0,0,0,36,64ZM234.55,164.7l-24.46,20.19L217.55,215a4,4,0,0,1-1.49,4.17,4.05,4.05,0,0,1-2.39.79,4,4,0,0,1-2-.55L184,203.18l-27.64,16.27a4,4,0,0,1-5.91-4.41l7.46-30.15L133.45,164.7a4,4,0,0,1,2.24-7.08l32.24-2.49,12.4-28.72a4,4,0,0,1,7.34,0l12.4,28.72,32.24,2.49a4,4,0,0,1,2.24,7.08Zm-12.74.14L197,162.92a4,4,0,0,1-3.36-2.4L184,138.1l-9.68,22.42a4,4,0,0,1-3.36,2.4l-24.77,1.92L165,180.32a4,4,0,0,1,1.33,4.05l-5.78,23.36L182,195.09a4,4,0,0,1,4.06,0l21.47,12.64-5.78-23.36a4,4,0,0,1,1.33-4.05Z"></path>
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
