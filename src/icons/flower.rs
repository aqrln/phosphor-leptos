//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "nature"))]
#[component]
pub fn Flower(
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
                <path d="M215.64,128a44,44,0,0,0-43.82-75.9,44,44,0,0,0-87.64,0A44,44,0,0,0,40.36,128a44,44,0,0,0,43.82,75.89,44,44,0,0,0,87.64,0A44,44,0,0,0,215.64,128ZM108,128a20,20,0,1,1,20,20A20,20,0,0,1,108,128Zm72.35-53.32a20,20,0,1,1,20,34.64c-2.65,1.53-10.52,4.88-30.1,6.42a44.08,44.08,0,0,0-10.52-18.18C170.86,81.36,177.7,76.21,180.35,74.68ZM128,36a20,20,0,0,1,20,20c0,3.06-1,11.55-9.49,29.28a43.79,43.79,0,0,0-21,0C109,67.55,108,59.06,108,56A20,20,0,0,1,128,36ZM48.33,82a20,20,0,0,1,27.32-7.32c2.65,1.53,9.49,6.68,20.62,22.88a44.08,44.08,0,0,0-10.52,18.18c-19.58-1.54-27.45-4.89-30.1-6.42A20,20,0,0,1,48.33,82Zm27.32,99.32a20,20,0,1,1-20-34.64c2.65-1.53,10.52-4.88,30.1-6.42a44.08,44.08,0,0,0,10.52,18.18C85.14,174.64,78.3,179.79,75.65,181.32ZM128,220a20,20,0,0,1-20-20c0-3.06,1-11.55,9.49-29.28a43.79,43.79,0,0,0,21,0C147,188.45,148,196.94,148,200A20,20,0,0,1,128,220Zm79.67-46a20,20,0,0,1-27.32,7.32c-2.65-1.53-9.49-6.68-20.62-22.88a44.08,44.08,0,0,0,10.52-18.18c19.58,1.54,27.45,4.89,30.1,6.42A20,20,0,0,1,207.67,174Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M206.35,136.29c-8.87-5.13-24.46-7.38-39.4-8.29,14.94-.91,30.53-3.16,39.4-8.29a32,32,0,1,0-32-55.42c-8.87,5.12-18.61,17.48-26.87,30C154.17,80.87,160,66.25,160,56a32,32,0,0,0-64,0c0,10.25,5.83,24.87,12.52,38.26-8.26-12.49-18-24.85-26.87-30a32,32,0,1,0-32,55.42c8.87,5.13,24.46,7.38,39.4,8.29-14.94.91-30.53,3.16-39.4,8.29a32,32,0,1,0,32,55.42c8.87-5.12,18.61-17.48,26.87-30C101.83,175.13,96,189.75,96,200a32,32,0,0,0,64,0c0-10.25-5.83-24.87-12.52-38.26,8.26,12.49,18,24.85,26.87,30a32,32,0,1,0,32-55.42ZM155.71,144A32,32,0,1,1,160,128,31.74,31.74,0,0,1,155.71,144Z"
        opacity="0.2"
    ></path>
    <path d="M210.35,129.36c-.81-.47-1.7-.92-2.62-1.36.92-.44,1.81-.89,2.62-1.36a40,40,0,1,0-40-69.28c-.81.47-1.65,1-2.48,1.59.08-1,.13-2,.13-3a40,40,0,0,0-80,0c0,.94,0,1.94.13,3-.83-.57-1.67-1.12-2.48-1.59a40,40,0,1,0-40,69.28c.81.47,1.7.92,2.62,1.36-.92.44-1.81.89-2.62,1.36a40,40,0,1,0,40,69.28c.81-.47,1.65-1,2.48-1.59-.08,1-.13,2-.13,2.95a40,40,0,0,0,80,0c0-.94-.05-1.94-.13-2.95.83.57,1.67,1.12,2.48,1.59A39.79,39.79,0,0,0,190.29,204a40.43,40.43,0,0,0,10.42-1.38,40,40,0,0,0,9.64-73.28ZM104,128a24,24,0,1,1,24,24A24,24,0,0,1,104,128Zm74.35-56.79a24,24,0,1,1,24,41.57c-6.27,3.63-18.61,6.13-35.16,7.19A40,40,0,0,0,154.53,98.1C163.73,84.28,172.08,74.84,178.35,71.21ZM128,32a24,24,0,0,1,24,24c0,7.24-4,19.19-11.36,34.06a39.81,39.81,0,0,0-25.28,0C108,75.19,104,63.24,104,56A24,24,0,0,1,128,32ZM44.86,80a24,24,0,0,1,32.79-8.79c6.27,3.63,14.62,13.07,23.82,26.89A40,40,0,0,0,88.81,120c-16.55-1.06-28.89-3.56-35.16-7.18A24,24,0,0,1,44.86,80ZM77.65,184.79a24,24,0,1,1-24-41.57c6.27-3.63,18.61-6.13,35.16-7.19a40,40,0,0,0,12.66,21.87C92.27,171.72,83.92,181.16,77.65,184.79ZM128,224a24,24,0,0,1-24-24c0-7.24,4-19.19,11.36-34.06a39.81,39.81,0,0,0,25.28,0C148,180.81,152,192.76,152,200A24,24,0,0,1,128,224Zm83.14-48a24,24,0,0,1-32.79,8.79c-6.27-3.63-14.62-13.07-23.82-26.89A40,40,0,0,0,167.19,136c16.55,1.06,28.89,3.56,35.16,7.18A24,24,0,0,1,211.14,176Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M210.35,129.36c-.81-.47-1.7-.92-2.62-1.36.92-.44,1.81-.89,2.62-1.36a40,40,0,1,0-40-69.28c-.81.47-1.65,1-2.48,1.59.08-1,.13-2,.13-3a40,40,0,0,0-80,0c0,.94,0,1.94.13,3-.83-.57-1.67-1.12-2.48-1.59a40,40,0,1,0-40,69.28c.81.47,1.7.92,2.62,1.36-.92.44-1.81.89-2.62,1.36a40,40,0,1,0,40,69.28c.81-.47,1.65-1,2.48-1.59-.08,1-.13,2-.13,2.95a40,40,0,0,0,80,0c0-.94-.05-1.94-.13-2.95.83.57,1.67,1.12,2.48,1.59A39.79,39.79,0,0,0,190.29,204a40.43,40.43,0,0,0,10.42-1.38,40,40,0,0,0,9.64-73.28ZM128,156a28,28,0,1,1,28-28A28,28,0,0,1,128,156Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M209.35,131.09a42.24,42.24,0,0,0-6.82-3.09,42.24,42.24,0,0,0,6.82-3.09,38,38,0,1,0-38-65.82,43.33,43.33,0,0,0-6.08,4.36A42.94,42.94,0,0,0,166,56a38,38,0,0,0-76,0,42.94,42.94,0,0,0,.73,7.45,43.33,43.33,0,0,0-6.08-4.36,38,38,0,0,0-38,65.82A42.24,42.24,0,0,0,53.47,128a42.24,42.24,0,0,0-6.82,3.09,38,38,0,0,0,9.16,69.62,38.53,38.53,0,0,0,9.9,1.31,37.82,37.82,0,0,0,18.94-5.11,43.33,43.33,0,0,0,6.08-4.36A42.94,42.94,0,0,0,90,200a38,38,0,0,0,76,0,42.94,42.94,0,0,0-.73-7.45,43.33,43.33,0,0,0,6.08,4.36A37.82,37.82,0,0,0,190.29,202a38.53,38.53,0,0,0,9.9-1.31,38,38,0,0,0,9.16-69.62Zm-32-61.61a26,26,0,1,1,26,45c-4.77,2.75-14.92,6.15-36.4,7.47l-1.44-.08A38,38,0,0,0,152,98.58l.66-1.31C164.56,79.33,172.58,72.24,177.35,69.48ZM128,154a26,26,0,1,1,26-26A26,26,0,0,1,128,154Zm0-124a26,26,0,0,1,26,26c0,5.51-2.13,16-11.73,35.27-.26.4-.53.8-.79,1.21a37.88,37.88,0,0,0-27,0l-.79-1.22C104.13,72,102,61.51,102,56A26,26,0,0,1,128,30ZM52.65,114.52a26,26,0,0,1,26-45c4.77,2.76,12.79,9.85,24.67,27.79l.66,1.31a38,38,0,0,0-13.49,23.33l-1.44.08C67.57,120.67,57.42,117.27,52.65,114.52Zm26,72a26,26,0,0,1-26-45c4.77-2.75,14.92-6.15,36.4-7.47l1.44.08A38,38,0,0,0,104,157.42l-.66,1.31C91.44,176.67,83.42,183.76,78.65,186.52ZM128,226a26,26,0,0,1-26-26c0-5.51,2.13-16,11.73-35.27.26-.4.53-.8.79-1.21a37.88,37.88,0,0,0,27,0l.79,1.22C151.87,184,154,194.49,154,200A26,26,0,0,1,128,226Zm84.87-49a26,26,0,0,1-35.52,9.52c-4.77-2.76-12.79-9.85-24.67-27.79l-.66-1.31a38,38,0,0,0,13.49-23.33L167,134c21.48,1.32,31.63,4.72,36.4,7.47A26,26,0,0,1,212.87,177Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M210.35,129.36c-.81-.47-1.7-.92-2.62-1.36.92-.44,1.81-.89,2.62-1.36a40,40,0,1,0-40-69.28c-.81.47-1.65,1-2.48,1.59.08-1,.13-2,.13-3a40,40,0,0,0-80,0c0,.94,0,1.94.13,3-.83-.57-1.67-1.12-2.48-1.59a40,40,0,1,0-40,69.28c.81.47,1.7.92,2.62,1.36-.92.44-1.81.89-2.62,1.36a40,40,0,1,0,40,69.28c.81-.47,1.65-1,2.48-1.59-.08,1-.13,2-.13,2.95a40,40,0,0,0,80,0c0-.94-.05-1.94-.13-2.95.83.57,1.67,1.12,2.48,1.59A39.79,39.79,0,0,0,190.29,204a40.43,40.43,0,0,0,10.42-1.38,40,40,0,0,0,9.64-73.28ZM104,128a24,24,0,1,1,24,24A24,24,0,0,1,104,128Zm74.35-56.79a24,24,0,1,1,24,41.57c-6.27,3.63-18.61,6.13-35.16,7.19A40,40,0,0,0,154.53,98.1C163.73,84.28,172.08,74.84,178.35,71.21ZM128,32a24,24,0,0,1,24,24c0,7.24-4,19.19-11.36,34.06a39.81,39.81,0,0,0-25.28,0C108,75.19,104,63.24,104,56A24,24,0,0,1,128,32ZM44.86,80a24,24,0,0,1,32.79-8.79c6.27,3.63,14.62,13.07,23.82,26.89A40,40,0,0,0,88.81,120c-16.55-1.06-28.89-3.56-35.16-7.18A24,24,0,0,1,44.86,80ZM77.65,184.79a24,24,0,1,1-24-41.57c6.27-3.63,18.61-6.13,35.16-7.19a40,40,0,0,0,12.66,21.87C92.27,171.72,83.92,181.16,77.65,184.79ZM128,224a24,24,0,0,1-24-24c0-7.24,4-19.19,11.36-34.06a39.81,39.81,0,0,0,25.28,0C148,180.81,152,192.76,152,200A24,24,0,0,1,128,224Zm83.14-48a24,24,0,0,1-32.79,8.79c-6.27-3.63-14.62-13.07-23.82-26.89A40,40,0,0,0,167.19,136c16.55,1.06,28.89,3.56,35.16,7.18A24,24,0,0,1,211.14,176Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M208.35,132.82A50.92,50.92,0,0,0,195.76,128a50.92,50.92,0,0,0,12.59-4.82,36,36,0,0,0-36-62.36,51.54,51.54,0,0,0-10.47,8.5A51.27,51.27,0,0,0,164,56a36,36,0,0,0-72,0,51.27,51.27,0,0,0,2.12,13.32,51.54,51.54,0,0,0-10.47-8.5,36,36,0,1,0-36,62.36A50.92,50.92,0,0,0,60.24,128a50.92,50.92,0,0,0-12.59,4.82,36,36,0,1,0,36,62.36,51.54,51.54,0,0,0,10.47-8.5A51.27,51.27,0,0,0,92,200a36,36,0,0,0,72,0,51.27,51.27,0,0,0-2.12-13.32,51.54,51.54,0,0,0,10.47,8.5,35.85,35.85,0,0,0,18,4.84,36.24,36.24,0,0,0,9.37-1.25,36,36,0,0,0,8.68-66Zm-32-65.07a28,28,0,0,1,28,48.5c-6.95,4-19.82,6.66-37.44,7.74l-3.16-.17a36,36,0,0,0-14.26-24.68c.49-1,1-1.9,1.44-2.84C160.67,81.59,169.4,71.77,176.35,67.75ZM128,156a28,28,0,1,1,28-28A28,28,0,0,1,128,156Zm0-128a28,28,0,0,1,28,28c0,8-4.14,20.5-12,36.3-.58.87-1.15,1.75-1.73,2.65a35.94,35.94,0,0,0-28.52,0c-.58-.9-1.15-1.78-1.73-2.65C104.14,76.5,100,64,100,56A28,28,0,0,1,128,28ZM51.65,116.25a28,28,0,1,1,28-48.5c6.95,4,15.68,13.84,25.42,28.55.47.94,1,1.88,1.44,2.84a36,36,0,0,0-14.26,24.68l-3.16.17C71.47,122.91,58.6,120.26,51.65,116.25Zm28,72a28,28,0,1,1-28-48.5c7-4,19.82-6.66,37.44-7.74l3.16.17a36,36,0,0,0,14.26,24.68c-.49,1-1,1.9-1.44,2.84C95.33,174.41,86.6,184.23,79.65,188.25ZM128,228a28,28,0,0,1-28-28c0-8,4.14-20.5,12-36.3.58-.87,1.15-1.75,1.73-2.65a35.94,35.94,0,0,0,28.52,0c.58.9,1.15,1.78,1.73,2.65,7.87,15.8,12,28.27,12,36.3A28,28,0,0,1,128,228Zm86.6-50a28,28,0,0,1-38.25,10.25c-6.95-4-15.68-13.84-25.42-28.55-.47-.94-1-1.88-1.44-2.84a36,36,0,0,0,14.26-24.68l3.16-.17c17.62,1.08,30.49,3.73,37.44,7.74A28,28,0,0,1,214.6,178Z"></path>
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
