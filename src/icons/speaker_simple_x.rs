use crate::IconWeight;
/// GENERATED FILE
use leptos::*;

#[component]
pub fn SpeakerSimpleX(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M157.27,21.22a12,12,0,0,0-12.64,1.31L75.88,76H32A20,20,0,0,0,12,96v64a20,20,0,0,0,20,20H75.88l68.75,53.47A12,12,0,0,0,164,224V32A12,12,0,0,0,157.27,21.22ZM140,199.47,87.37,158.53A12,12,0,0,0,80,156H36V100H80a12,12,0,0,0,7.37-2.53L140,56.54Zm108.49-55.95a12,12,0,0,1-17,17L216,145l-15.51,15.52a12,12,0,0,1-17-17L199,128l-15.52-15.51a12,12,0,0,1,17-17L216,111l15.51-15.51a12,12,0,0,1,17,17L233,128Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M152,32V224L80,168H32a8,8,0,0,1-8-8V96a8,8,0,0,1,8-8H80Z" opacity="0.2"/><path d="M155.51,24.81a8,8,0,0,0-8.42.88L77.25,80H32A16,16,0,0,0,16,96v64a16,16,0,0,0,16,16H77.25l69.84,54.31A8,8,0,0,0,160,224V32A8,8,0,0,0,155.51,24.81ZM144,207.64,84.91,161.69A7.94,7.94,0,0,0,80,160H32V96H80a7.94,7.94,0,0,0,4.91-1.69L144,48.36Zm101.66-61.3a8,8,0,0,1-11.32,11.32L216,139.31l-18.34,18.35a8,8,0,0,1-11.32-11.32L204.69,128l-18.35-18.34a8,8,0,0,1,11.32-11.32L216,116.69l18.34-18.35a8,8,0,0,1,11.32,11.32L227.31,128Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M155.52,24.81a8,8,0,0,0-8.43.88L77.25,80H32A16,16,0,0,0,16,96v64a16,16,0,0,0,16,16H77.25l69.84,54.31A7.94,7.94,0,0,0,152,232a8,8,0,0,0,8-8V32A8,8,0,0,0,155.52,24.81Z"/><path d="M227.31,128l18.35-18.34a8,8,0,0,0-11.32-11.32L216,116.69,197.66,98.34a8,8,0,0,0-11.32,11.32L204.69,128l-18.35,18.34a8,8,0,0,0,11.32,11.32L216,139.31l18.34,18.35a8,8,0,0,0,11.32-11.32Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M154.64,26.61a6,6,0,0,0-6.32.65L77.94,82H32A14,14,0,0,0,18,96v64a14,14,0,0,0,14,14H77.94l70.38,54.74A6,6,0,0,0,158,224V32A6,6,0,0,0,154.64,26.61ZM146,211.73,83.68,163.26A6,6,0,0,0,80,162H32a2,2,0,0,1-2-2V96a2,2,0,0,1,2-2H80a6,6,0,0,0,3.68-1.26L146,44.27Zm98.24-64a6,6,0,1,1-8.48,8.48L216,136.48l-19.76,19.76a6,6,0,0,1-8.48-8.48L207.52,128l-19.76-19.76a6,6,0,0,1,8.48-8.48L216,119.52l19.76-19.76a6,6,0,0,1,8.48,8.48L224.48,128Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M155.51,24.81a8,8,0,0,0-8.42.88L77.25,80H32A16,16,0,0,0,16,96v64a16,16,0,0,0,16,16H77.25l69.84,54.31A8,8,0,0,0,160,224V32A8,8,0,0,0,155.51,24.81ZM144,207.64,84.91,161.69A7.94,7.94,0,0,0,80,160H32V96H80a7.94,7.94,0,0,0,4.91-1.69L144,48.36Zm101.66-61.3a8,8,0,0,1-11.32,11.32L216,139.31l-18.34,18.35a8,8,0,0,1-11.32-11.32L204.69,128l-18.35-18.34a8,8,0,0,1,11.32-11.32L216,116.69l18.34-18.35a8,8,0,0,1,11.32,11.32L227.31,128Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M153.76,28.41a4,4,0,0,0-4.22.43L78.63,84H32A12,12,0,0,0,20,96v64a12,12,0,0,0,12,12H78.63l70.91,55.16A4.07,4.07,0,0,0,152,228a3.92,3.92,0,0,0,1.76-.41A4,4,0,0,0,156,224V32A4,4,0,0,0,153.76,28.41ZM148,215.82l-65.54-51A4.06,4.06,0,0,0,80,164H32a4,4,0,0,1-4-4V96a4,4,0,0,1,4-4H80a4.06,4.06,0,0,0,2.46-.84l65.54-51Zm94.83-66.65a4,4,0,0,1-5.66,5.66L216,133.66l-21.17,21.17a4,4,0,0,1-5.66-5.66L210.34,128l-21.17-21.17a4,4,0,0,1,5.66-5.66L216,122.34l21.17-21.17a4,4,0,1,1,5.66,5.66L221.66,128Z"/> }.into_view()
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
