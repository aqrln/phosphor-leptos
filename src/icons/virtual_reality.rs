use crate::IconWeight;
/// GENERATED FILE
use leptos::*;

#[component]
pub fn VirtualReality(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M123.12,100.52l-26,64a12,12,0,0,1-22.24,0l-26-64a12,12,0,0,1,22.24-9L86,128.12l14.88-36.64a12,12,0,0,1,22.24,9ZM256,128a84.09,84.09,0,0,1-84,84H84A84,84,0,0,1,84,44h88A84.09,84.09,0,0,1,256,128Zm-24,0a60.07,60.07,0,0,0-60-60H84a60,60,0,0,0,0,120h88A60.07,60.07,0,0,0,232,128Zm-42,11.24,8.46,14.81A12,12,0,1,1,177.58,166L167.32,148H160v12a12,12,0,0,1-24,0V96a12,12,0,0,1,12-12h20a32,32,0,0,1,22,55.24ZM160,124h8a8,8,0,0,0,0-16h-8Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M248,128h0a72,72,0,0,1-72,72H80A72,72,0,0,1,8,128H8A72,72,0,0,1,80,56h96A72,72,0,0,1,248,128Z" opacity="0.2"/><path d="M123.41,99l-26,64a8,8,0,0,1-14.82,0l-26-64a8,8,0,1,1,14.82-6L90,138.74,108.59,93a8,8,0,1,1,14.82,6ZM256,128a80.09,80.09,0,0,1-80,80H80A80,80,0,0,1,80,48h96A80.09,80.09,0,0,1,256,128Zm-16,0a64.07,64.07,0,0,0-64-64H80a64,64,0,0,0,0,128h96A64.07,64.07,0,0,0,240,128Zm-59.16,10.35L191,156a8,8,0,0,1-13.9,7.94l-11.44-20c-.53,0-1.07.05-1.61.05H152v16a8,8,0,0,1-16,0V96a8,8,0,0,1,8-8h20a28,28,0,0,1,16.84,50.35ZM152,128h12a12,12,0,0,0,0-24H152Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M176,116a12,12,0,0,1-12,12H152V104h12A12,12,0,0,1,176,116Zm80,12a80.09,80.09,0,0,1-80,80H80A80,80,0,0,1,80,48h96A80.09,80.09,0,0,1,256,128ZM119,88.59A8,8,0,0,0,108.59,93L90,138.74,71.41,93a8,8,0,1,0-14.82,6l26,64a8,8,0,0,0,14.82,0l26-64A8,8,0,0,0,119,88.59Zm61.83,49.76A28,28,0,0,0,164,88H144a8,8,0,0,0-8,8v64a8,8,0,0,0,16,0V144h12c.54,0,1.08,0,1.61-.05l11.44,20A8,8,0,0,0,191,156Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M121.56,98.26l-26,64a6,6,0,0,1-11.12,0l-26-64a6,6,0,0,1,11.12-4.52L90,144.06l20.44-50.32a6,6,0,0,1,11.12,4.52ZM254,128a78.09,78.09,0,0,1-78,78H80A78,78,0,0,1,80,50h96A78.09,78.09,0,0,1,254,128Zm-12,0a66.08,66.08,0,0,0-66-66H80a66,66,0,0,0,0,132h96A66.08,66.08,0,0,0,242,128Zm-63.8,9.76,11,19.26a6,6,0,0,1-10.42,6l-12.07-21.12A27.06,27.06,0,0,1,164,142H150v18a6,6,0,0,1-12,0V96a6,6,0,0,1,6-6h20a26,26,0,0,1,14.2,47.76ZM150,130h14a14,14,0,0,0,0-28H150Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M123.41,99l-26,64a8,8,0,0,1-14.82,0l-26-64a8,8,0,1,1,14.82-6L90,138.74,108.59,93a8,8,0,1,1,14.82,6ZM256,128a80.09,80.09,0,0,1-80,80H80A80,80,0,0,1,80,48h96A80.09,80.09,0,0,1,256,128Zm-16,0a64.07,64.07,0,0,0-64-64H80a64,64,0,0,0,0,128h96A64.07,64.07,0,0,0,240,128Zm-59.16,10.35L191,156a8,8,0,0,1-13.9,7.94l-11.44-20c-.53,0-1.07.05-1.61.05H152v16a8,8,0,0,1-16,0V96a8,8,0,0,1,8-8h20a28,28,0,0,1,16.84,50.35ZM152,128h12a12,12,0,0,0,0-24H152Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M119.71,97.51l-26,64a4,4,0,0,1-7.42,0l-26-64a4,4,0,0,1,7.42-3L90,149.37l22.29-54.88a4,4,0,0,1,7.42,3ZM252,128a76.08,76.08,0,0,1-76,76H80A76,76,0,0,1,80,52h96A76.08,76.08,0,0,1,252,128Zm-8,0a68.07,68.07,0,0,0-68-68H80a68,68,0,0,0,0,136h96A68.07,68.07,0,0,0,244,128Zm-68.5,9.06,12,21a4,4,0,0,1-1.49,5.45,3.92,3.92,0,0,1-2,.53,4,4,0,0,1-3.47-2L167.79,139.7a24.85,24.85,0,0,1-3.79.3H148v20a4,4,0,0,1-8,0V96a4,4,0,0,1,4-4h20a24,24,0,0,1,11.5,45.06ZM180,116a16,16,0,0,0-16-16H148v32h16A16,16,0,0,0,180,116Z"/> }.into_view()
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
