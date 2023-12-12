use crate::IconWeight;
/// GENERATED FILE
use leptos::*;

#[component]
pub fn PlugCharging(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M224,48H180V16a12,12,0,0,0-24,0V48H100V16a12,12,0,0,0-24,0V48H32.55C24.4,48,20,54.18,20,60A12,12,0,0,0,32,72H44v92a44.05,44.05,0,0,0,44,44h28v32a12,12,0,0,0,24,0V208h28a44.05,44.05,0,0,0,44-44V72h12a12,12,0,0,0,0-24ZM188,164a20,20,0,0,1-20,20H88a20,20,0,0,1-20-20V72H188Zm-85.86-29.17a12,12,0,0,1-1.38-11l12-32a12,12,0,1,1,22.48,8.42L129.32,116H144a12,12,0,0,1,11.24,16.21l-12,32a12,12,0,0,1-22.48-8.42L126.68,140H112A12,12,0,0,1,102.14,134.83Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M200,64v96a32,32,0,0,1-32,32H88a32,32,0,0,1-32-32V64Z" opacity="0.2"/><path d="M224,56H176V16a8,8,0,0,0-16,0V56H96V16a8,8,0,0,0-16,0V56H32.55C26.28,56,24,60.78,24,64a8,8,0,0,0,8,8H48v88a40,40,0,0,0,40,40h32v40a8,8,0,0,0,16,0V200h32a40,40,0,0,0,40-40V72h16a8,8,0,0,0,0-16ZM192,160a24,24,0,0,1-24,24H88a24,24,0,0,1-24-24V72H192Zm-86.58-27.44a8,8,0,0,1-.91-7.37l12-32a8,8,0,1,1,15,5.62l-8,21.19H144a8,8,0,0,1,7.49,10.81l-12,32a8,8,0,0,1-15-5.62l8-21.19H112A8,8,0,0,1,105.42,132.56Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M224,56H176V16a8,8,0,0,0-16,0V56H96V16a8,8,0,0,0-8-8c-3.21,0-8,2.27-8,8.54V56H32.55C26.28,56,24,60.78,24,64a8,8,0,0,0,8,8H48v88a40,40,0,0,0,40,40h32v40a8,8,0,0,0,16,0V200h32a40,40,0,0,0,40-40V72h16a8,8,0,0,0,0-16Zm-72.51,74.81-12,32a8,8,0,0,1-15-5.62l8-21.19H112a8,8,0,0,1-7.49-10.81l12-32a8,8,0,1,1,15,5.62l-8,21.19H144a8,8,0,0,1,7.49,10.81Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M224,58H174V16a6,6,0,0,0-12,0V58H94V16a6,6,0,0,0-12,0V58H32.55A6.1,6.1,0,0,0,26,64a6,6,0,0,0,6,6H50v90a38,38,0,0,0,38,38h34v42a6,6,0,0,0,12,0V198h34a38,38,0,0,0,38-38V70h18a6,6,0,0,0,0-12ZM194,160a26,26,0,0,1-26,26H88a26,26,0,0,1-26-26V70H194Zm-86.93-28.58a6,6,0,0,1-.69-5.53l12-32a6,6,0,1,1,11.24,4.22l-9,23.89H144a6,6,0,0,1,5.62,8.11l-12,32a6,6,0,0,1-11.24-4.22l9-23.89H112A6,6,0,0,1,107.07,131.42Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M224,56H176V16a8,8,0,0,0-16,0V56H96V16a8,8,0,0,0-16,0V56H32.55C26.28,56,24,60.78,24,64a8,8,0,0,0,8,8H48v88a40,40,0,0,0,40,40h32v40a8,8,0,0,0,16,0V200h32a40,40,0,0,0,40-40V72h16a8,8,0,0,0,0-16ZM168,184H88a24,24,0,0,1-24-24V72H192v88A24,24,0,0,1,168,184Zm-17.42-60.56a8,8,0,0,1,.91,7.37l-12,32a8,8,0,0,1-15-5.62l8-21.19H112a8,8,0,0,1-7.49-10.81l12-32a8,8,0,1,1,15,5.62l-8,21.19H144A8,8,0,0,1,150.58,123.44Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M224,60H172V16a4,4,0,0,0-8,0V60H92V16a4,4,0,0,0-8,0V60H32.55C28.13,60,28,63.59,28,64a4,4,0,0,0,4,4H52v92a36,36,0,0,0,36,36h36v44a4,4,0,0,0,8,0V196h36a36,36,0,0,0,36-36V68h20a4,4,0,0,0,0-8ZM196,160a28,28,0,0,1-28,28H88a28,28,0,0,1-28-28V68H196Zm-87.29-29.72a4,4,0,0,1-.46-3.68l12-32a4,4,0,0,1,7.5,2.8l-10,26.6H144a4,4,0,0,1,3.75,5.4l-12,32a4,4,0,1,1-7.5-2.8l10-26.6H112A4,4,0,0,1,108.71,130.28Z"/> }.into_view()
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
