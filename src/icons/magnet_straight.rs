//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "development", feature = "objects"))]
#[component]
pub fn MagnetStraight(
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
                <path d="M200,40H160a16,16,0,0,0-16,16v88a16,16,0,0,1-32,0V56A16,16,0,0,0,96,40H56A16,16,0,0,0,40,56v88a88,88,0,0,0,88,88h.67c48.15-.36,87.33-40.29,87.33-89V56A16,16,0,0,0,200,40Zm0,16V96H160V56ZM96,56V96H56V56Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M104,56V96H48V56a8,8,0,0,1,8-8H96A8,8,0,0,1,104,56Zm96-8H160a8,8,0,0,0-8,8V96h56V56A8,8,0,0,0,200,48Z"
        opacity="0.2"
    ></path>
    <path d="M200,40H160a16,16,0,0,0-16,16v88a16,16,0,0,1-32,0V56A16,16,0,0,0,96,40H56A16,16,0,0,0,40,56v88a88,88,0,0,0,88,88h.67c48.15-.36,87.33-40.29,87.33-89V56A16,16,0,0,0,200,40Zm0,16V88H160V56ZM96,56V88H56V56Zm32.55,160A72,72,0,0,1,56,144V104H96v40a32,32,0,0,0,64,0V104h40v39C200,183,168,215.71,128.55,216Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M200,44H160a12,12,0,0,0-12,12v88a20,20,0,0,1-40,0V56A12,12,0,0,0,96,44H56A12,12,0,0,0,44,56v88a84,84,0,0,0,84,84h.64c46-.34,83.36-38.47,83.36-85V56A12,12,0,0,0,200,44Zm-40,8h40a4,4,0,0,1,4,4V92H156V56A4,4,0,0,1,160,52ZM56,52H96a4,4,0,0,1,4,4V92H52V56A4,4,0,0,1,56,52Zm72.58,168H128a76,76,0,0,1-76-76V100h48v44a28,28,0,0,0,56,0V100h48v43C204,185.15,170.17,219.69,128.58,220Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M200,36H160a20,20,0,0,0-20,20v88a12,12,0,0,1-24,0V56A20,20,0,0,0,96,36H56A20,20,0,0,0,36,56v88a92,92,0,0,0,92,92h.71c50.34-.38,91.3-42.1,91.3-93V56A20,20,0,0,0,200,36Zm-4,24V84H164V60ZM92,60V84H60V60Zm36.52,152H128a68,68,0,0,1-68-68V108H92v36a36,36,0,0,0,72,0V108h32v35C196,180.77,165.73,211.72,128.52,212Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M200,42H160a14,14,0,0,0-14,14v88a18,18,0,0,1-36,0V56A14,14,0,0,0,96,42H56A14,14,0,0,0,42,56v88a86,86,0,0,0,86,86h.65c47.06-.35,85.35-39.38,85.35-87V56A14,14,0,0,0,200,42ZM160,54h40a2,2,0,0,1,2,2V90H158V56A2,2,0,0,1,160,54ZM56,54H96a2,2,0,0,1,2,2V90H54V56A2,2,0,0,1,56,54Zm72.56,164H128a74,74,0,0,1-74-74V102H98v42a30,30,0,0,0,60,0V102h44v41C202,184.05,169.06,217.7,128.56,218Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M200,40H160a16,16,0,0,0-16,16v88a16,16,0,0,1-32,0V56A16,16,0,0,0,96,40H56A16,16,0,0,0,40,56v88a88,88,0,0,0,88,88h.67c48.15-.36,87.33-40.29,87.33-89V56A16,16,0,0,0,200,40Zm0,16V88H160V56ZM96,56V88H56V56Zm32.55,160A72,72,0,0,1,56,144V104H96v40a32,32,0,0,0,64,0V104h40v39C200,183,168,215.71,128.55,216Z"></path>
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
