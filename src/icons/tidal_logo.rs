use crate::IconWeight;
/// GENERATED FILE
use leptos::*;

#[component]
pub fn TidalLogo(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M252.49,87.51l-38-38a12,12,0,0,0-17,0L168,79,136.49,47.51a12,12,0,0,0-17,0L88,79,58.49,49.51a12,12,0,0,0-17,0l-38,38a12,12,0,0,0,0,17l38,38a12,12,0,0,0,17,0L88,113l23,23L79.51,167.51a12,12,0,0,0,0,17l40,40a12,12,0,0,0,17,0l40-40a12,12,0,0,0,0-17L145,136l23-23,29.51,29.52a12,12,0,0,0,17,0l38-38A12,12,0,0,0,252.49,87.51ZM50,117,29,96,50,75,71,96Zm78,82-23-23,23-23,23,23Zm0-80L105,96l23-23,23,23Zm78-2L185,96l21-21,21,21Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M48,56,88,96,48,136,8,96ZM88,96l40,40,40-40L128,56Zm40,120,40-40-40-40L88,176ZM208,56,168,96l40,40,40-40Z" opacity="0.2"/><path d="M253.66,90.34l-40-40a8,8,0,0,0-11.32,0L168,84.69,133.66,50.34a8,8,0,0,0-11.32,0L88,84.69,53.66,50.34a8,8,0,0,0-11.32,0l-40,40a8,8,0,0,0,0,11.32l40,40a8,8,0,0,0,11.32,0L88,107.31,116.69,136,82.34,170.34a8,8,0,0,0,0,11.32l40,40a8,8,0,0,0,11.32,0l40-40a8,8,0,0,0,0-11.32L139.31,136,168,107.31l34.34,34.35a8,8,0,0,0,11.32,0l40-40A8,8,0,0,0,253.66,90.34ZM48,124.69,19.31,96,48,67.31,76.69,96Zm80,80L99.31,176,128,147.31,156.69,176Zm0-80L99.31,96,128,67.31,156.69,96Zm80,0L179.31,96,208,67.31,236.69,96Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M253.66,101.66l-36,36a8,8,0,0,1-11.32,0l-36-36-.34-.38-.34.38L135.31,136l34.35,34.34a8,8,0,0,1,0,11.32l-36,36a8,8,0,0,1-11.32,0l-36-36a8,8,0,0,1,0-11.32L120.69,136,86.34,101.66l-.34-.38-.34.38-36,36a8,8,0,0,1-11.32,0l-36-36a8,8,0,0,1,0-11.32l36-36a8,8,0,0,1,11.32,0l36,36,.34.38.34-.38,36-36a8,8,0,0,1,11.32,0l36,36,.34.38.34-.38,36-36a8,8,0,0,1,11.32,0l36,36A8,8,0,0,1,253.66,101.66Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M252.24,91.76l-40-40a6,6,0,0,0-8.48,0L168,87.52,132.24,51.76a6,6,0,0,0-8.48,0L88,87.52,52.24,51.76a6,6,0,0,0-8.48,0l-40,40a6,6,0,0,0,0,8.48l40,40a6,6,0,0,0,8.48,0L88,104.48,119.52,136,83.76,171.76a6,6,0,0,0,0,8.48l40,40a6,6,0,0,0,8.48,0l40-40a6,6,0,0,0,0-8.48L136.48,136,168,104.48l35.76,35.76a6,6,0,0,0,8.48,0l40-40A6,6,0,0,0,252.24,91.76ZM48,127.51,16.49,96,48,64.49,79.51,96Zm80,80L96.49,176,128,144.49,159.51,176Zm0-80L96.49,96,128,64.49,159.51,96Zm80,0L176.49,96,208,64.49,239.51,96Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M253.66,90.34l-40-40a8,8,0,0,0-11.32,0L168,84.69,133.66,50.34a8,8,0,0,0-11.32,0L88,84.69,53.66,50.34a8,8,0,0,0-11.32,0l-40,40a8,8,0,0,0,0,11.32l40,40a8,8,0,0,0,11.32,0L88,107.31,116.69,136,82.34,170.34a8,8,0,0,0,0,11.32l40,40a8,8,0,0,0,11.32,0l40-40a8,8,0,0,0,0-11.32L139.31,136,168,107.31l34.34,34.35a8,8,0,0,0,11.32,0l40-40A8,8,0,0,0,253.66,90.34ZM48,124.69,19.31,96,48,67.31,76.69,96Zm80,80L99.31,176,128,147.31,156.69,176Zm0-80L99.31,96,128,67.31,156.69,96Zm80,0L179.31,96,208,67.31,236.69,96Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M250.83,93.17l-40-40a4,4,0,0,0-5.66,0L168,90.34,130.83,53.17a4,4,0,0,0-5.66,0L88,90.34,50.83,53.17a4,4,0,0,0-5.66,0l-40,40a4,4,0,0,0,0,5.66l40,40a4,4,0,0,0,5.66,0L88,101.66,122.34,136,85.17,173.17a4,4,0,0,0,0,5.66l40,40a4,4,0,0,0,5.66,0l40-40a4,4,0,0,0,0-5.66L133.66,136,168,101.66l37.17,37.17a4,4,0,0,0,5.66,0l40-40A4,4,0,0,0,250.83,93.17ZM48,130.34,13.66,96,48,61.66,82.34,96Zm80,80L93.66,176,128,141.66,162.34,176Zm0-80L93.66,96,128,61.66,162.34,96Zm80,0L173.66,96,208,61.66,242.34,96Z"/> }.into_view()
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
