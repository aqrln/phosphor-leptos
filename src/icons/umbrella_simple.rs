use crate::IconWeight;
/// GENERATED FILE
use leptos::*;

#[component]
pub fn UmbrellaSimple(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M243.93,126.3A116.43,116.43,0,0,0,49,50.81a115.45,115.45,0,0,0-37,75.49A20,20,0,0,0,32,148h84v52a36,36,0,0,0,72,0,12,12,0,0,0-24,0,12,12,0,0,1-24,0V148h84a20,20,0,0,0,20-21.7ZM36.44,124A92.45,92.45,0,0,1,190.69,68.46,91.56,91.56,0,0,1,219.56,124Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M224,136H32a8,8,0,0,1-8-8.71,104.37,104.37,0,0,1,207.94,0A8,8,0,0,1,224,136Z" opacity="0.2"/><path d="M240,126.63A112.44,112.44,0,0,0,51.75,53.75a111.56,111.56,0,0,0-35.7,72.88A16,16,0,0,0,32,144h88v56a32,32,0,0,0,64,0,8,8,0,0,0-16,0,16,16,0,0,1-32,0V144h88a16,16,0,0,0,16-17.37ZM32,128l0,0A96.43,96.43,0,0,1,193.4,65.52,95.32,95.32,0,0,1,224,128Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M235.76,138.83A16,16,0,0,1,224,144H136v56a16,16,0,0,0,32,0,8,8,0,0,1,16,0,32,32,0,0,1-64,0V144H32a16,16,0,0,1-16-17.37,112.44,112.44,0,0,1,188.2-72.88A111.56,111.56,0,0,1,240,126.63,16.1,16.1,0,0,1,235.76,138.83Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M238,126.79A110.43,110.43,0,0,0,53.11,55.22a109.51,109.51,0,0,0-35.06,71.57A14,14,0,0,0,32,142h90v58a30,30,0,0,0,60,0,6,6,0,0,0-12,0,18,18,0,0,1-36,0V142h90a14,14,0,0,0,14-15.21Zm-12.49,2.56A2,2,0,0,1,224,130H32a2,2,0,0,1-1.49-.65,2,2,0,0,1-.53-1.56A98.43,98.43,0,0,1,194.76,64.05,97.5,97.5,0,0,1,226,127.79,2,2,0,0,1,225.46,129.35Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M240,126.63A112.44,112.44,0,0,0,51.75,53.75a111.56,111.56,0,0,0-35.7,72.88A16,16,0,0,0,32,144h88v56a32,32,0,0,0,64,0,8,8,0,0,0-16,0,16,16,0,0,1-32,0V144h88a16,16,0,0,0,16-17.37ZM32,128l0,0A96.43,96.43,0,0,1,193.4,65.52,95.32,95.32,0,0,1,224,128Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M236,127A108.44,108.44,0,0,0,54.46,56.69,107.53,107.53,0,0,0,20,127a12,12,0,0,0,12,13h92v60a28,28,0,0,0,56,0,4,4,0,0,0-8,0,20,20,0,0,1-40,0V140h92a12,12,0,0,0,12-13Zm-9,3.74a4,4,0,0,1-3,1.3H32a4,4,0,0,1-4-4.38,100.43,100.43,0,0,1,168.1-65,99.53,99.53,0,0,1,31.88,65A4,4,0,0,1,226.93,130.7Z"/> }.into_view()
        }
    };

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=size.get()
            height=size.get()
            fill=color
            transform=transform
            viewBox="0 0 256 256"
        >
            {body}
        </svg>
    }
}
