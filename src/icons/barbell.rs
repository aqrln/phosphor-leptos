//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "health"))]
#[component]
pub fn Barbell(
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
                <path d="M244,116V88a20,20,0,0,0-20-20H208V64a20,20,0,0,0-20-20H164a20,20,0,0,0-20,20v52H112V64A20,20,0,0,0,92,44H68A20,20,0,0,0,48,64v4H32A20,20,0,0,0,12,88v28a12,12,0,0,0,0,24v28a20,20,0,0,0,20,20H48v4a20,20,0,0,0,20,20H92a20,20,0,0,0,20-20V140h32v52a20,20,0,0,0,20,20h24a20,20,0,0,0,20-20v-4h16a20,20,0,0,0,20-20V140a12,12,0,0,0,0-24ZM36,164V92H48v72Zm52,24H72V68H88Zm96,0H168V68h16Zm36-24H208V92h12Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M96,64V192a8,8,0,0,1-8,8H64a8,8,0,0,1-8-8V64a8,8,0,0,1,8-8H88A8,8,0,0,1,96,64Zm96-8H168a8,8,0,0,0-8,8V192a8,8,0,0,0,8,8h24a8,8,0,0,0,8-8V64A8,8,0,0,0,192,56Z"
        opacity="0.2"
    ></path>
    <path d="M248,120h-8V88a16,16,0,0,0-16-16H208V64a16,16,0,0,0-16-16H168a16,16,0,0,0-16,16v56H104V64A16,16,0,0,0,88,48H64A16,16,0,0,0,48,64v8H32A16,16,0,0,0,16,88v32H8a8,8,0,0,0,0,16h8v32a16,16,0,0,0,16,16H48v8a16,16,0,0,0,16,16H88a16,16,0,0,0,16-16V136h48v56a16,16,0,0,0,16,16h24a16,16,0,0,0,16-16v-8h16a16,16,0,0,0,16-16V136h8a8,8,0,0,0,0-16ZM32,168V88H48v80Zm56,24H64V64H88V192Zm104,0H168V64h24V175.83c0,.06,0,.11,0,.17s0,.12,0,.17V192Zm32-24H208V88h16Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M200,64V192a16,16,0,0,1-16,16H168a16,16,0,0,1-16-16V136H104v56a16,16,0,0,1-16,16H72a16,16,0,0,1-16-16V64A16,16,0,0,1,72,48H88a16,16,0,0,1,16,16v56h48V64a16,16,0,0,1,16-16h16A16,16,0,0,1,200,64ZM36,72H32A16,16,0,0,0,16,88v32H8.27A8.18,8.18,0,0,0,0,127.47,8,8,0,0,0,8,136h8v32a16,16,0,0,0,16,16h4a4,4,0,0,0,4-4V76A4,4,0,0,0,36,72Zm220,55.47a8.18,8.18,0,0,0-8.25-7.47H240V88a16,16,0,0,0-16-16h-4a4,4,0,0,0-4,4V180a4,4,0,0,0,4,4h4a16,16,0,0,0,16-16V136h8A8,8,0,0,0,256,127.47Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M248,122H238V88a14,14,0,0,0-14-14H206V64a14,14,0,0,0-14-14H168a14,14,0,0,0-14,14v58H102V64A14,14,0,0,0,88,50H64A14,14,0,0,0,50,64V74H32A14,14,0,0,0,18,88v34H8a6,6,0,0,0,0,12H18v34a14,14,0,0,0,14,14H50v10a14,14,0,0,0,14,14H88a14,14,0,0,0,14-14V134h52v58a14,14,0,0,0,14,14h24a14,14,0,0,0,14-14V182h18a14,14,0,0,0,14-14V134h10a6,6,0,0,0,0-12ZM32,170a2,2,0,0,1-2-2V88a2,2,0,0,1,2-2H50v84Zm58,22a2,2,0,0,1-2,2H64a2,2,0,0,1-2-2V64a2,2,0,0,1,2-2H88a2,2,0,0,1,2,2Zm104,0a2,2,0,0,1-2,2H168a2,2,0,0,1-2-2V64a2,2,0,0,1,2-2h24a2,2,0,0,1,2,2Zm32-24a2,2,0,0,1-2,2H206V86h18a2,2,0,0,1,2,2Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M248,120h-8V88a16,16,0,0,0-16-16H208V64a16,16,0,0,0-16-16H168a16,16,0,0,0-16,16v56H104V64A16,16,0,0,0,88,48H64A16,16,0,0,0,48,64v8H32A16,16,0,0,0,16,88v32H8a8,8,0,0,0,0,16h8v32a16,16,0,0,0,16,16H48v8a16,16,0,0,0,16,16H88a16,16,0,0,0,16-16V136h48v56a16,16,0,0,0,16,16h24a16,16,0,0,0,16-16v-8h16a16,16,0,0,0,16-16V136h8a8,8,0,0,0,0-16ZM32,168V88H48v80Zm56,24H64V64H88V192Zm104,0H168V64h24V175.82c0,.06,0,.12,0,.18s0,.12,0,.18V192Zm32-24H208V88h16Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M248,124H236V88a12,12,0,0,0-12-12H204V64a12,12,0,0,0-12-12H168a12,12,0,0,0-12,12v60H100V64A12,12,0,0,0,88,52H64A12,12,0,0,0,52,64V76H32A12,12,0,0,0,20,88v36H8a4,4,0,0,0,0,8H20v36a12,12,0,0,0,12,12H52v12a12,12,0,0,0,12,12H88a12,12,0,0,0,12-12V132h56v60a12,12,0,0,0,12,12h24a12,12,0,0,0,12-12V180h20a12,12,0,0,0,12-12V132h12a4,4,0,0,0,0-8ZM32,172a4,4,0,0,1-4-4V88a4,4,0,0,1,4-4H52v88Zm60,20a4,4,0,0,1-4,4H64a4,4,0,0,1-4-4V64a4,4,0,0,1,4-4H88a4,4,0,0,1,4,4Zm104,0a4,4,0,0,1-4,4H168a4,4,0,0,1-4-4V64a4,4,0,0,1,4-4h24a4,4,0,0,1,4,4Zm32-24a4,4,0,0,1-4,4H204V84h20a4,4,0,0,1,4,4Z"></path>
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
