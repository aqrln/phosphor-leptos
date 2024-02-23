//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "map", feature = "objects"))]
#[component]
pub fn Van(
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
                <path d="M253.11,112.18,207.54,59.09A19.94,19.94,0,0,0,192.26,52H32A20,20,0,0,0,12,72V184a20,20,0,0,0,20,20H46.06a36,36,0,0,0,67.88,0h40.12a36,36,0,0,0,67.88,0H236a20,20,0,0,0,20-20V120A12.05,12.05,0,0,0,253.11,112.18ZM217.89,108H176V76h14.42ZM104,108V76h48v32ZM80,76v32H36V76Zm0,128a12,12,0,1,1,12-12A12,12,0,0,1,80,204Zm108,0a12,12,0,1,1,12-12A12,12,0,0,1,188,204Zm33.94-24a36,36,0,0,0-67.88,0H113.94a36,36,0,0,0-67.88,0H36V132H232v48Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M248,120v64a8,8,0,0,1-8,8H216a24,24,0,0,0-48,0H104a24,24,0,0,0-48,0H32a8,8,0,0,1-8-8V120Z"
        opacity="0.2"
    ></path>
    <path d="M254.07,114.79,208.53,61.73A16,16,0,0,0,196.26,56H32A16,16,0,0,0,16,72V184a16,16,0,0,0,16,16H49a32,32,0,0,0,62,0h50a32,32,0,0,0,62,0h17a16,16,0,0,0,16-16V120A8,8,0,0,0,254.07,114.79ZM230.59,112H176V72h20.26ZM104,112V72h56v40ZM88,72v40H32V72ZM80,208a16,16,0,1,1,16-16A16,16,0,0,1,80,208Zm112,0a16,16,0,1,1,16-16A16,16,0,0,1,192,208Zm31-24a32,32,0,0,0-62,0H111a32,32,0,0,0-62,0H32V128H240v56Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M254.07,114.79,208.53,61.73A16,16,0,0,0,196.26,56H32A16,16,0,0,0,16,72V184a16,16,0,0,0,16,16H49a32,32,0,0,0,62,0h50a32,32,0,0,0,62,0h17a16,16,0,0,0,16-16V120A8,8,0,0,0,254.07,114.79ZM32,112V72H88v40Zm48,96a16,16,0,1,1,16-16A16,16,0,0,1,80,208Zm80-96H104V72h56Zm32,96a16,16,0,1,1,16-16A16,16,0,0,1,192,208Zm-16-96V72h20.26l34.33,40Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M252.55,116.09,207,63a14,14,0,0,0-10.74-5H32A14,14,0,0,0,18,72V184a14,14,0,0,0,14,14H50.6a30,30,0,0,0,58.8,0h53.2a30,30,0,0,0,58.8,0H240a14,14,0,0,0,14-14V120A6,6,0,0,0,252.55,116.09Zm-54.7-45.32L234.94,114H174V70h22.26A2,2,0,0,1,197.85,70.77ZM102,114V70h60v44ZM32,70H90v44H30V72A2,2,0,0,1,32,70ZM80,210a18,18,0,1,1,18-18A18,18,0,0,1,80,210Zm112,0a18,18,0,1,1,18-18A18,18,0,0,1,192,210Zm48-24H221.4a30,30,0,0,0-58.8,0H109.4a30,30,0,0,0-58.8,0H32a2,2,0,0,1-2-2V126H242v58A2,2,0,0,1,240,186Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M254.07,114.79,208.53,61.73A16,16,0,0,0,196.26,56H32A16,16,0,0,0,16,72V184a16,16,0,0,0,16,16H49a32,32,0,0,0,62,0h50a32,32,0,0,0,62,0h17a16,16,0,0,0,16-16V120A8,8,0,0,0,254.07,114.79ZM230.59,112H176V72h20.26ZM104,112V72h56v40ZM88,72v40H32V72ZM80,208a16,16,0,1,1,16-16A16,16,0,0,1,80,208Zm112,0a16,16,0,1,1,16-16A16,16,0,0,1,192,208Zm31-24a32,32,0,0,0-62,0H111a32,32,0,0,0-62,0H32V128H240v56Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M251,117.4,205.47,64.3a12,12,0,0,0-9.21-4.3H32A12,12,0,0,0,20,72V184a12,12,0,0,0,12,12H52.29a28,28,0,0,0,55.42,0h56.58a28,28,0,0,0,55.42,0H240a12,12,0,0,0,12-12V120A4,4,0,0,0,251,117.4ZM199.36,69.47,239.3,116H172V68h24.26A4,4,0,0,1,199.36,69.47ZM100,116V68h64v48ZM32,68H92v48H28V72A4,4,0,0,1,32,68ZM80,212a20,20,0,1,1,20-20A20,20,0,0,1,80,212Zm112,0a20,20,0,1,1,20-20A20,20,0,0,1,192,212Zm48-24H219.71a28,28,0,0,0-55.42,0H107.71a28,28,0,0,0-55.42,0H32a4,4,0,0,1-4-4V124H244v60A4,4,0,0,1,240,188Z"></path>
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
