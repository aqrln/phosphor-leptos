use crate::IconWeight;
/// GENERATED FILE
use leptos::*;

#[component]
pub fn HandFist(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M200,76H188V64a36,36,0,0,0-60-26.8A36,36,0,0,0,69.27,54.54,36,36,0,0,0,20,88v40a108,108,0,0,0,216,0V112A36,36,0,0,0,200,76ZM140,64a12,12,0,0,1,24,0V76H140ZM92,64a12,12,0,0,1,24,0v40a12,12,0,0,1-24,0ZM44,88a12,12,0,0,1,24,0v16a12,12,0,0,1-24,0Zm168,40A84,84,0,0,1,44.61,138.15,35.93,35.93,0,0,0,80,130.8a35.89,35.89,0,0,0,43.65,3.34A36.23,36.23,0,0,0,130,140.5,51.82,51.82,0,0,0,116,176a12,12,0,0,0,24,0,28,28,0,0,1,28-28,12,12,0,0,0,0-24H152a12,12,0,0,1-12-12V100h60a12,12,0,0,1,12,12Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M224,112v16a96,96,0,0,1-192,0V88a24,24,0,0,1,48,0V64a24,24,0,0,1,48,0,24,24,0,0,1,48,0V88h24A24,24,0,0,1,224,112Z" opacity="0.2"/><path d="M200,80H184V64a32,32,0,0,0-56-21.13A32,32,0,0,0,72.21,60.42,32,32,0,0,0,24,88v40a104,104,0,0,0,208,0V112A32,32,0,0,0,200,80ZM152,48a16,16,0,0,1,16,16V80H136V64A16,16,0,0,1,152,48ZM88,64a16,16,0,0,1,32,0v40a16,16,0,0,1-32,0ZM40,88a16,16,0,0,1,32,0v16a16,16,0,0,1-32,0Zm176,40a88,88,0,0,1-175.92,3.75A31.93,31.93,0,0,0,80,125.13a31.93,31.93,0,0,0,44.58,3.35,32.21,32.21,0,0,0,11.8,11.44A47.88,47.88,0,0,0,120,176a8,8,0,0,0,16,0,32,32,0,0,1,32-32,8,8,0,0,0,0-16H152a16,16,0,0,1-16-16V96h64a16,16,0,0,1,16,16Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M232,120v8A104,104,0,0,1,127.63,232c-54-.19-98-42.06-103.12-94.78a4,4,0,0,1,5.56-4A35.94,35.94,0,0,0,72,122.59a35.92,35.92,0,0,0,53.94,2.33,40.36,40.36,0,0,0,12.87,13A47.94,47.94,0,0,0,120,176a8,8,0,0,0,8.67,8,8.21,8.21,0,0,0,7.33-8.26A32,32,0,0,1,168,144a8,8,0,0,0,8-8.53,8.18,8.18,0,0,0-8.25-7.47H160a24,24,0,0,1-24-24V88h64A32,32,0,0,1,232,120ZM44.73,120C55.57,119.6,64,110.37,64,99.52v-23C64,65.63,55.57,56.4,44.73,56A20,20,0,0,0,24,76v24A20,20,0,0,0,44.73,120Zm56,0c10.84-.39,19.27-9.62,19.27-20.47v-47c0-10.85-8.43-20.08-19.27-20.47A20,20,0,0,0,80,52v48A20,20,0,0,0,100.73,120ZM176,52a20,20,0,0,0-20.73-20C144.43,32.4,136,41.63,136,52.48V72h36a4,4,0,0,0,4-4Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M200,82H182V64a30,30,0,0,0-54-18A30,30,0,0,0,74,64v0A30,30,0,0,0,26,88v40a102,102,0,0,0,204,0V112A30,30,0,0,0,200,82ZM152,46a18,18,0,0,1,18,18V82H136a14.71,14.71,0,0,0-2,.16V64A18,18,0,0,1,152,46ZM86,64a18,18,0,0,1,36,0v40a18,18,0,0,1-36,0ZM38,88a18,18,0,0,1,36,0v16a18,18,0,0,1-36,0Zm180,40a90,90,0,0,1-180,0h0a30,30,0,0,0,42-6,30,30,0,0,0,45.12,3.3A30.18,30.18,0,0,0,140,139.51,45.92,45.92,0,0,0,122,176a6,6,0,0,0,12,0,34,34,0,0,1,34-34,6,6,0,0,0,0-12H152a18,18,0,0,1-18-18V96a2,2,0,0,1,2-2h64a18,18,0,0,1,18,18Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M200,80H184V64a32,32,0,0,0-56-21.13A32,32,0,0,0,72.21,60.42,32,32,0,0,0,24,88v40a104,104,0,0,0,208,0V112A32,32,0,0,0,200,80ZM152,48a16,16,0,0,1,16,16V80H136V64A16,16,0,0,1,152,48ZM88,64a16,16,0,0,1,32,0v40a16,16,0,0,1-32,0ZM40,88a16,16,0,0,1,32,0v16a16,16,0,0,1-32,0Zm176,40a88,88,0,0,1-175.92,3.75A31.93,31.93,0,0,0,80,125.13a31.93,31.93,0,0,0,44.58,3.35,32.21,32.21,0,0,0,11.8,11.44A47.88,47.88,0,0,0,120,176a8,8,0,0,0,16,0,32,32,0,0,1,32-32,8,8,0,0,0,0-16H152a16,16,0,0,1-16-16V96h64a16,16,0,0,1,16,16Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M200,84H180V64a28,28,0,0,0-52-14.41A28,28,0,0,0,76,64v4.43A28,28,0,0,0,28,88v40a100,100,0,0,0,200,0V112A28,28,0,0,0,200,84ZM152,44a20,20,0,0,1,20,20V84H136a11.8,11.8,0,0,0-4,.7V64A20,20,0,0,1,152,44ZM84,64a20,20,0,0,1,40,0v40a20,20,0,0,1-40,0ZM36,88a20,20,0,0,1,40,0v16a20,20,0,0,1-40,0Zm184,40a92,92,0,0,1-184,0v-4.42a28,28,0,0,0,44-5.17,28,28,0,0,0,45.73,3.23,28.11,28.11,0,0,0,18.59,17.29A44,44,0,0,0,124,176a4,4,0,0,0,8,0,36,36,0,0,1,36-36,4,4,0,0,0,0-8H152a20,20,0,0,1-20-20V96a4,4,0,0,1,4-4h64a20,20,0,0,1,20,20Z"/> }.into_view()
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
