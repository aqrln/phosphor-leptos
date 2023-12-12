use crate::IconWeight;
/// GENERATED FILE
use leptos::*;

#[component]
pub fn FileDoc(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M48,140H32a12,12,0,0,0-12,12v56a12,12,0,0,0,12,12H48a40,40,0,0,0,0-80Zm0,56H44V164h4a16,16,0,0,1,0,32Zm180.3-3.8a12,12,0,0,1,.37,17A34,34,0,0,1,204,220c-19.85,0-36-17.94-36-40s16.15-40,36-40a34,34,0,0,1,24.67,10.83,12,12,0,0,1-17.34,16.6A10.27,10.27,0,0,0,204,164c-6.5,0-12,7.33-12,16s5.5,16,12,16a10.27,10.27,0,0,0,7.33-3.43A12,12,0,0,1,228.3,192.2ZM128,140c-19.85,0-36,17.94-36,40s16.15,40,36,40,36-17.94,36-40S147.85,140,128,140Zm0,56c-6.5,0-12-7.33-12-16s5.5-16,12-16,12,7.33,12,16S134.5,196,128,196ZM48,120a12,12,0,0,0,12-12V44h76V92a12,12,0,0,0,12,12h48v4a12,12,0,0,0,24,0V88a12,12,0,0,0-3.51-8.48l-56-56A12,12,0,0,0,152,20H56A20,20,0,0,0,36,40v68A12,12,0,0,0,48,120ZM160,57l23,23H160Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M208,88H152V32Z" opacity="0.2"/><path d="M52,144H36a8,8,0,0,0-8,8v56a8,8,0,0,0,8,8H52a36,36,0,0,0,0-72Zm0,56H44V160h8a20,20,0,0,1,0,40Zm169.53-4.91a8,8,0,0,1,.25,11.31A30.06,30.06,0,0,1,200,216c-17.65,0-32-16.15-32-36s14.35-36,32-36a30.06,30.06,0,0,1,21.78,9.6,8,8,0,0,1-11.56,11.06A14.24,14.24,0,0,0,200,160c-8.82,0-16,9-16,20s7.18,20,16,20a14.18,14.18,0,0,0,10.22-4.66A8,8,0,0,1,221.53,195.09ZM128,144c-17.64,0-32,16.15-32,36s14.36,36,32,36,32-16.15,32-36S145.64,144,128,144Zm0,56c-8.82,0-16-9-16-20s7.18-20,16-20,16,9,16,20S136.82,200,128,200ZM48,120a8,8,0,0,0,8-8V40h88V88a8,8,0,0,0,8,8h48v16a8,8,0,0,0,16,0V88a8,8,0,0,0-2.34-5.66l-56-56A8,8,0,0,0,152,24H56A16,16,0,0,0,40,40v72A8,8,0,0,0,48,120ZM160,51.31,188.69,80H160Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M48,120H208a8,8,0,0,0,8-8V88a8,8,0,0,0-2.34-5.66l-56-56A8,8,0,0,0,152,24H56A16,16,0,0,0,40,40v72A8,8,0,0,0,48,120ZM152,44l44,44H152ZM52,144H36a8,8,0,0,0-8,8v56a8,8,0,0,0,8,8H52a36,36,0,0,0,0-72Zm0,56H44V160h8a20,20,0,0,1,0,40Zm169.53-4.91a8,8,0,0,1,.25,11.31A30.06,30.06,0,0,1,200,216c-17.65,0-32-16.15-32-36s14.35-36,32-36a30.06,30.06,0,0,1,21.78,9.6,8,8,0,0,1-11.56,11.06A14.18,14.18,0,0,0,200,160c-8.82,0-16,9-16,20s7.18,20,16,20a14.24,14.24,0,0,0,10.22-4.66A8,8,0,0,1,221.53,195.09ZM128,144c-17.64,0-32,16.15-32,36s14.36,36,32,36,32-16.15,32-36S145.64,144,128,144Zm0,56c-8.82,0-16-9-16-20s7.18-20,16-20,16,9,16,20S136.82,200,128,200Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M52,146H36a6,6,0,0,0-6,6v56a6,6,0,0,0,6,6H52a34,34,0,0,0,0-68Zm0,56H42V158H52a22,22,0,0,1,0,44Zm168.15-5.46a6,6,0,0,1,.18,8.48A28.06,28.06,0,0,1,200,214c-16.54,0-30-15.25-30-34s13.46-34,30-34a28.06,28.06,0,0,1,20.33,9,6,6,0,0,1-8.66,8.3A16.23,16.23,0,0,0,200,158c-9.93,0-18,9.87-18,22s8.07,22,18,22a16.23,16.23,0,0,0,11.67-5.28A6,6,0,0,1,220.15,196.54ZM128,146c-16.54,0-30,15.25-30,34s13.46,34,30,34,30-15.25,30-34S144.54,146,128,146Zm0,56c-9.93,0-18-9.87-18-22s8.07-22,18-22,18,9.87,18,22S137.93,202,128,202ZM48,118a6,6,0,0,0,6-6V40a2,2,0,0,1,2-2h90V88a6,6,0,0,0,6,6h50v18a6,6,0,0,0,12,0V88a6,6,0,0,0-1.76-4.24l-56-56A6,6,0,0,0,152,26H56A14,14,0,0,0,42,40v72A6,6,0,0,0,48,118ZM158,46.48,193.52,82H158Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M52,144H36a8,8,0,0,0-8,8v56a8,8,0,0,0,8,8H52a36,36,0,0,0,0-72Zm0,56H44V160h8a20,20,0,0,1,0,40Zm169.53-4.91a8,8,0,0,1,.25,11.31A30.06,30.06,0,0,1,200,216c-17.65,0-32-16.15-32-36s14.35-36,32-36a30.06,30.06,0,0,1,21.78,9.6,8,8,0,0,1-11.56,11.06A14.24,14.24,0,0,0,200,160c-8.82,0-16,9-16,20s7.18,20,16,20a14.24,14.24,0,0,0,10.22-4.66A8,8,0,0,1,221.53,195.09ZM128,144c-17.65,0-32,16.15-32,36s14.35,36,32,36,32-16.15,32-36S145.65,144,128,144Zm0,56c-8.82,0-16-9-16-20s7.18-20,16-20,16,9,16,20S136.82,200,128,200ZM48,120a8,8,0,0,0,8-8V40h88V88a8,8,0,0,0,8,8h48v16a8,8,0,0,0,16,0V88a8,8,0,0,0-2.34-5.66l-56-56A8,8,0,0,0,152,24H56A16,16,0,0,0,40,40v72A8,8,0,0,0,48,120ZM160,51.31,188.69,80H160Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M52,148H36a4,4,0,0,0-4,4v56a4,4,0,0,0,4,4H52a32,32,0,0,0,0-64Zm0,56H40V156H52a24,24,0,0,1,0,48Zm166.77-6a4,4,0,0,1,.12,5.66A26.11,26.11,0,0,1,200,212c-15.44,0-28-14.36-28-32s12.56-32,28-32a26.11,26.11,0,0,1,18.89,8.36,4,4,0,1,1-5.78,5.54A18.15,18.15,0,0,0,200,156c-11,0-20,10.77-20,24s9,24,20,24a18.15,18.15,0,0,0,13.11-5.9A4,4,0,0,1,218.77,198ZM128,148c-15.44,0-28,14.36-28,32s12.56,32,28,32,28-14.36,28-32S143.44,148,128,148Zm0,56c-11,0-20-10.77-20-24s9-24,20-24,20,10.77,20,24S139,204,128,204ZM48,116a4,4,0,0,0,4-4V40a4,4,0,0,1,4-4h92V88a4,4,0,0,0,4,4h52v20a4,4,0,0,0,8,0V88a4,4,0,0,0-1.17-2.83l-56-56A4,4,0,0,0,152,28H56A12,12,0,0,0,44,40v72A4,4,0,0,0,48,116ZM156,41.65,198.34,84H156Z"/> }.into_view()
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
