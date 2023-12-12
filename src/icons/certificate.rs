use crate::IconWeight;
/// GENERATED FILE
use leptos::*;

#[component]
pub fn Certificate(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M248,128a52,52,0,1,0-92,33.19V220A12,12,0,0,0,174,230.42L196,217.82l22.05,12.6A12,12,0,0,0,236,220V161.19A51.77,51.77,0,0,0,248,128Zm-52-28a28,28,0,1,1-28,28A28,28,0,0,1,196,100ZM202,193.58a12,12,0,0,0-11.9,0L180,199.32V177.47a51.86,51.86,0,0,0,32,0v21.85ZM140,192a12,12,0,0,1-12,12H40a20,20,0,0,1-20-20V56A20,20,0,0,1,40,36H216a20,20,0,0,1,20,20,12,12,0,0,1-23.32,4H44V180h84A12,12,0,0,1,140,192Zm-12-52a12,12,0,0,1-12,12H76a12,12,0,0,1,0-24h40A12,12,0,0,1,128,140Zm0-40a12,12,0,0,1-12,12H76a12,12,0,0,1,0-24h40A12,12,0,0,1,128,100Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M224,56V92.23a48,48,0,1,0-64,71.57h0V192H40a8,8,0,0,1-8-8V56a8,8,0,0,1,8-8H216A8,8,0,0,1,224,56Z" opacity="0.2"/><path d="M248,128a56,56,0,1,0-96,39.14V224a8,8,0,0,0,11.58,7.16L192,216.94l28.42,14.22A8,8,0,0,0,232,224V167.14A55.81,55.81,0,0,0,248,128ZM192,88a40,40,0,1,1-40,40A40,40,0,0,1,192,88Zm3.58,112.84a8,8,0,0,0-7.16,0L168,211.06V178.59a55.94,55.94,0,0,0,48,0v32.47ZM136,192a8,8,0,0,1-8,8H40a16,16,0,0,1-16-16V56A16,16,0,0,1,40,40H216a16,16,0,0,1,16,16,8,8,0,0,1-16,0H40V184h88A8,8,0,0,1,136,192Zm-16-56a8,8,0,0,1-8,8H72a8,8,0,0,1,0-16h40A8,8,0,0,1,120,136Zm0-32a8,8,0,0,1-8,8H72a8,8,0,0,1,0-16h40A8,8,0,0,1,120,104Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M232,88.86V56a16,16,0,0,0-16-16H40A16,16,0,0,0,24,56V184a16,16,0,0,0,16,16H152v24a8,8,0,0,0,11.58,7.16L192,216.94l28.42,14.22A8,8,0,0,0,232,224V167.14a55.87,55.87,0,0,0,0-78.28ZM128,144H72a8,8,0,0,1,0-16h56a8,8,0,0,1,0,16Zm0-32H72a8,8,0,0,1,0-16h56a8,8,0,0,1,0,16Zm88,99.06-20.42-10.22a8,8,0,0,0-7.16,0L168,211.06V178.59a55.94,55.94,0,0,0,48,0ZM192,168a40,40,0,1,1,40-40A40,40,0,0,1,192,168Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M246,128a54,54,0,1,0-92,38.32V224a6,6,0,0,0,8.68,5.37L192,214.71l29.32,14.66A6,6,0,0,0,224,230a5.93,5.93,0,0,0,3.15-.9A6,6,0,0,0,230,224V166.32A53.83,53.83,0,0,0,246,128Zm-96,0a42,42,0,1,1,42,42A42,42,0,0,1,150,128Zm68,86.29-23.32-11.66a6,6,0,0,0-5.36,0L166,214.29v-39a53.87,53.87,0,0,0,52,0ZM134,192a6,6,0,0,1-6,6H40a14,14,0,0,1-14-14V56A14,14,0,0,1,40,42H216a14,14,0,0,1,14,14,6,6,0,0,1-12,0,2,2,0,0,0-2-2H40a2,2,0,0,0-2,2V184a2,2,0,0,0,2,2h88A6,6,0,0,1,134,192Zm-16-56a6,6,0,0,1-6,6H72a6,6,0,0,1,0-12h40A6,6,0,0,1,118,136Zm0-32a6,6,0,0,1-6,6H72a6,6,0,0,1,0-12h40A6,6,0,0,1,118,104Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M248,128a56,56,0,1,0-96,39.14V224a8,8,0,0,0,11.58,7.16L192,216.94l28.42,14.22A8,8,0,0,0,232,224V167.14A55.81,55.81,0,0,0,248,128ZM192,88a40,40,0,1,1-40,40A40,40,0,0,1,192,88Zm3.58,112.84a8,8,0,0,0-7.16,0L168,211.06V178.59a55.94,55.94,0,0,0,48,0v32.47ZM136,192a8,8,0,0,1-8,8H40a16,16,0,0,1-16-16V56A16,16,0,0,1,40,40H216a16,16,0,0,1,16,16,8,8,0,0,1-16,0H40V184h88A8,8,0,0,1,136,192Zm-16-56a8,8,0,0,1-8,8H72a8,8,0,0,1,0-16h40A8,8,0,0,1,120,136Zm0-32a8,8,0,0,1-8,8H72a8,8,0,0,1,0-16h40A8,8,0,0,1,120,104Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M244,128a52,52,0,1,0-88,37.47V224a4,4,0,0,0,5.79,3.58L192,212.47l30.21,15.11A4.05,4.05,0,0,0,224,228a4,4,0,0,0,4-4V165.47A51.85,51.85,0,0,0,244,128Zm-96,0a44,44,0,1,1,44,44A44.05,44.05,0,0,1,148,128Zm72,89.53-26.21-13.11a4,4,0,0,0-3.58,0L164,217.53V171.78a51.8,51.8,0,0,0,56,0ZM132,192a4,4,0,0,1-4,4H40a12,12,0,0,1-12-12V56A12,12,0,0,1,40,44H216a12,12,0,0,1,12,12,4,4,0,0,1-8,0,4,4,0,0,0-4-4H40a4,4,0,0,0-4,4V184a4,4,0,0,0,4,4h88A4,4,0,0,1,132,192Zm-16-56a4,4,0,0,1-4,4H72a4,4,0,0,1,0-8h40A4,4,0,0,1,116,136Zm0-32a4,4,0,0,1-4,4H72a4,4,0,0,1,0-8h40A4,4,0,0,1,116,104Z"/> }.into_view()
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
