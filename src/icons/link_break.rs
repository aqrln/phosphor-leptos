use crate::IconWeight;
/// GENERATED FILE
use leptos::*;

#[component]
pub fn LinkBreak(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M148.48,67.93l-3.41,3.93a12,12,0,1,1-18.14-15.72l3.72-4.29c.19-.21.38-.42.58-.62a52,52,0,0,1,73.54,73.54c-.2.2-.41.39-.62.58l-4.29,3.72a12,12,0,1,1-15.72-18.14l3.93-3.41a28,28,0,0,0-39.59-39.59Zm-20.62,115a12,12,0,0,0-16.93,1.21l-3.41,3.93a28,28,0,0,1-39.59-39.59l3.93-3.41a12,12,0,0,0-15.72-18.14l-4.29,3.72c-.21.19-.42.38-.62.58a52,52,0,0,0,73.54,73.54c.2-.2.39-.41.58-.62l3.72-4.29A12,12,0,0,0,127.86,182.93ZM208,148H188a12,12,0,0,0,0,24h20a12,12,0,0,0,0-24ZM48,108H68a12,12,0,0,0,0-24H48a12,12,0,0,0,0,24Zm112,68a12,12,0,0,0-12,12v20a12,12,0,0,0,24,0V188A12,12,0,0,0,160,176ZM96,80a12,12,0,0,0,12-12V48a12,12,0,0,0-24,0V68A12,12,0,0,0,96,80Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M196.28,116.28l-80,80a40,40,0,0,1-56.56-56.56l80-80a40,40,0,0,1,56.56,56.56Z" opacity="0.2"/><path d="M190.63,65.37a32,32,0,0,0-45.19-.06L133.79,77.52a8,8,0,0,1-11.58-11l11.72-12.29a1.59,1.59,0,0,1,.13-.13,48,48,0,0,1,67.88,67.88,1.59,1.59,0,0,1-.13.13l-12.29,11.72a8,8,0,0,1-11-11.58l12.21-11.65A32,32,0,0,0,190.63,65.37ZM122.21,178.48l-11.65,12.21a32,32,0,0,1-45.25-45.25l12.21-11.65a8,8,0,0,0-11-11.58L54.19,133.93a1.59,1.59,0,0,0-.13.13,48,48,0,0,0,67.88,67.88,1.59,1.59,0,0,0,.13-.13l11.72-12.29a8,8,0,1,0-11.58-11ZM208,152H184a8,8,0,0,0,0,16h24a8,8,0,0,0,0-16ZM48,104H72a8,8,0,0,0,0-16H48a8,8,0,0,0,0,16Zm112,72a8,8,0,0,0-8,8v24a8,8,0,0,0,16,0V184A8,8,0,0,0,160,176ZM96,80a8,8,0,0,0,8-8V48a8,8,0,0,0-16,0V72A8,8,0,0,0,96,80Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M208,32H48A16,16,0,0,0,32,48V208a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V48A16,16,0,0,0,208,32ZM96,64a8,8,0,0,1,16,0V80a8,8,0,0,1-16,0ZM64,96H80a8,8,0,0,1,0,16H64a8,8,0,0,1,0-16Zm64.08,85.66-7.21,7.21a38,38,0,1,1-53.74-53.74l7.21-7.21a8,8,0,1,1,11.32,11.31l-7.22,7.21a22,22,0,0,0,31.12,31.12l7.21-7.22a8,8,0,1,1,11.31,11.32ZM160,192a8,8,0,0,1-16,0V176a8,8,0,0,1,16,0Zm32-32H176a8,8,0,0,1,0-16h16a8,8,0,0,1,0,16Zm-3.13-39.13-7.21,7.21a8,8,0,1,1-11.32-11.31l7.22-7.21a22,22,0,0,0-31.12-31.12l-7.21,7.22a8,8,0,1,1-11.31-11.32l7.21-7.21a38,38,0,1,1,53.74,53.74Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M192,64a34,34,0,0,0-48-.05L132.34,76.14a6,6,0,1,1-8.68-8.28l11.71-12.28.1-.11a46,46,0,0,1,65.06,65.06l-.11.1-12.28,11.71a6,6,0,0,1-8.28-8.68L192.09,112A34,34,0,0,0,192,64Zm-68.38,115.9L112,192.09A34,34,0,0,1,63.91,144l12.23-11.67a6,6,0,1,0-8.28-8.68L55.58,135.37l-.11.1a46,46,0,0,0,65.06,65.06l.1-.11,11.71-12.28a6,6,0,1,0-8.68-8.28ZM208,154H184a6,6,0,0,0,0,12h24a6,6,0,0,0,0-12ZM48,102H72a6,6,0,0,0,0-12H48a6,6,0,0,0,0,12Zm112,76a6,6,0,0,0-6,6v24a6,6,0,0,0,12,0V184A6,6,0,0,0,160,178ZM96,78a6,6,0,0,0,6-6V48a6,6,0,0,0-12,0V72A6,6,0,0,0,96,78Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M190.63,65.37a32,32,0,0,0-45.19-.06L133.79,77.52a8,8,0,0,1-11.58-11l11.72-12.29a1.59,1.59,0,0,1,.13-.13,48,48,0,0,1,67.88,67.88,1.59,1.59,0,0,1-.13.13l-12.29,11.72a8,8,0,0,1-11-11.58l12.21-11.65A32,32,0,0,0,190.63,65.37ZM122.21,178.48l-11.65,12.21a32,32,0,0,1-45.25-45.25l12.21-11.65a8,8,0,0,0-11-11.58L54.19,133.93a1.59,1.59,0,0,0-.13.13,48,48,0,0,0,67.88,67.88,1.59,1.59,0,0,0,.13-.13l11.72-12.29a8,8,0,1,0-11.58-11ZM208,152H184a8,8,0,0,0,0,16h24a8,8,0,0,0,0-16ZM48,104H72a8,8,0,0,0,0-16H48a8,8,0,0,0,0,16Zm112,72a8,8,0,0,0-8,8v24a8,8,0,0,0,16,0V184A8,8,0,0,0,160,176ZM96,80a8,8,0,0,0,8-8V48a8,8,0,0,0-16,0V72A8,8,0,0,0,96,80Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M193.46,62.54a36.06,36.06,0,0,0-50.92,0L130.89,74.76a4,4,0,0,1-5.78-5.52L136.82,57a44,44,0,1,1,62.29,62.15l-12.35,11.78a4,4,0,1,1-5.52-5.78l12.28-11.72A36,36,0,0,0,193.46,62.54Zm-68.35,118.7-11.65,12.22a36,36,0,0,1-51-50.85l12.28-11.72a4,4,0,0,0-5.52-5.78L56.89,136.89A44,44,0,1,0,119.18,199l11.71-12.28a4,4,0,1,0-5.78-5.52ZM208,156H184a4,4,0,0,0,0,8h24a4,4,0,0,0,0-8ZM48,100H72a4,4,0,0,0,0-8H48a4,4,0,0,0,0,8Zm112,80a4,4,0,0,0-4,4v24a4,4,0,0,0,8,0V184A4,4,0,0,0,160,180ZM96,76a4,4,0,0,0,4-4V48a4,4,0,0,0-8,0V72A4,4,0,0,0,96,76Z"/> }.into_view()
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
