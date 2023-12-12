use crate::IconWeight;
/// GENERATED FILE
use leptos::*;

#[component]
pub fn TrafficCone(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M232,204H216.53L157.2,33.43A20,20,0,0,0,138.31,20H117.69A20,20,0,0,0,98.8,33.43L39.47,204H24a12,12,0,0,0,0,24H232a12,12,0,0,0,0-24ZM98.27,108h59.46l13.91,40H84.36Zm22.26-64h14.94l13.91,40H106.62ZM76,172H180l11.13,32H64.88Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M188.52,160h-121L89.74,96h76.52Z" opacity="0.2"/><path d="M232,208H213.69L153.42,34.75A16,16,0,0,0,138.31,24H117.69a16,16,0,0,0-15.11,10.74L42.31,208H24a8,8,0,0,0,0,16H232a8,8,0,0,0,0-16ZM117.69,40h20.62L155,88H101ZM95.43,104h65.14l16.7,48H78.73ZM59.25,208l13.92-40H182.83l13.92,40Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M232,208H213.69L153.42,34.75A16,16,0,0,0,138.31,24H117.69a16,16,0,0,0-15.11,10.74L42.31,208H24a8,8,0,0,0,0,16H232a8,8,0,0,0,0-16ZM95.43,104h65.14l16.7,48H78.73Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M232,210H212.27L151.54,35.4A14,14,0,0,0,138.31,26H117.69a14,14,0,0,0-13.23,9.4L43.73,210H24a6,6,0,0,0,0,12H232a6,6,0,0,0,0-12ZM94,102h68l18.08,52H75.92Zm21.8-62.66A2,2,0,0,1,117.69,38h20.62a2,2,0,0,1,1.89,1.34L157.82,90H98.18ZM71.74,166H184.26l15.3,44H56.44Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M232,208H213.69L153.42,34.75A16,16,0,0,0,138.31,24H117.69a16,16,0,0,0-15.11,10.74L42.31,208H24a8,8,0,0,0,0,16H232a8,8,0,0,0,0-16ZM95.43,104h65.14l16.7,48H78.73Zm22.26-64h20.62L155,88H101ZM73.17,168H182.83l13.92,40H59.25Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M232,212H210.84L149.65,36.06A12,12,0,0,0,138.31,28H117.69a12,12,0,0,0-11.34,8.06L45.16,212H24a4,4,0,0,0,0,8H232a4,4,0,0,0,0-8ZM92.58,100h70.84l19.47,56H73.11Zm21.33-61.31A4,4,0,0,1,117.69,36h20.62a4,4,0,0,1,3.78,2.69L160.63,92H95.37ZM70.32,164H185.68l16.69,48H53.63Z"/> }.into_view()
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
