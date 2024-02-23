//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "map"))]
#[component]
pub fn Synagogue(
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
            IconWeight::Fill => view! {
                <path d="M208,57.38V32a8,8,0,0,0-16,0V57.38A24,24,0,0,0,176,80v42.21L136,99.36V72a8,8,0,0,0-16,0V99.36L80,122.21V80A24,24,0,0,0,64,57.38V32a8,8,0,0,0-16,0V57.38A24,24,0,0,0,32,80V216a8,8,0,0,0,8,8h64a8,8,0,0,0,8-8V176a16,16,0,0,1,32,0v40a8,8,0,0,0,8,8h64a8,8,0,0,0,8-8V80A24,24,0,0,0,208,57.38ZM64,208H48V112H64Zm144,0H192V112h16Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M216,80v32H184V80a16,16,0,0,1,16-16h0A16,16,0,0,1,216,80ZM72,80A16,16,0,0,0,56,64h0A16,16,0,0,0,40,80v32H72Zm0,56v80h40V176a16,16,0,0,1,16-16h0a16,16,0,0,1,16,16v40h40V136l-56-32Z"
        opacity="0.2"
    ></path>
    <path d="M208,57.38V32a8,8,0,0,0-16,0V57.38A24,24,0,0,0,176,80v42.21L136,99.36V72a8,8,0,0,0-16,0V99.36L80,122.21V80A24,24,0,0,0,64,57.38V32a8,8,0,0,0-16,0V57.38A24,24,0,0,0,32,80V216a8,8,0,0,0,8,8h72a8,8,0,0,0,8-8V176a8,8,0,0,1,16,0v40a8,8,0,0,0,8,8h72a8,8,0,0,0,8-8V80A24,24,0,0,0,208,57.38ZM200,72a8,8,0,0,1,8,8v24H192V80A8,8,0,0,1,200,72ZM56,72a8,8,0,0,1,8,8v24H48V80A8,8,0,0,1,56,72Zm-8,48H64v88H48Zm80,32a24,24,0,0,0-24,24v32H80V140.64l48-27.43,48,27.43V208H152V176A24,24,0,0,0,128,152Zm64,56V120h16v88Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M204,60.4V32a4,4,0,0,0-8,0V60.4A20,20,0,0,0,180,80v49.11l-48-27.43V72a4,4,0,0,0-8,0v29.68L76,129.11V80A20,20,0,0,0,60,60.4V32a4,4,0,0,0-8,0V60.4A20,20,0,0,0,36,80V216a4,4,0,0,0,4,4h72a4,4,0,0,0,4-4V176a12,12,0,0,1,24,0v40a4,4,0,0,0,4,4h72a4,4,0,0,0,4-4V80A20,20,0,0,0,204,60.4ZM200,68a12,12,0,0,1,12,12v28H188V80A12,12,0,0,1,200,68ZM56,68A12,12,0,0,1,68,80v28H44V80A12,12,0,0,1,56,68ZM44,116H68v96H44Zm84,40a20,20,0,0,0-20,20v36H76V138.32l52-29.71,52,29.71V212H148V176A20,20,0,0,0,128,156Zm60,56V116h24v96Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M208,54.34V32a12,12,0,0,0-24,0V54.34A32.06,32.06,0,0,0,164,84v26.75L140,97V72a12,12,0,0,0-24,0V97L92,110.75V84A32.06,32.06,0,0,0,72,54.34V32a12,12,0,0,0-24,0V54.34A32.06,32.06,0,0,0,28,84V216a12,12,0,0,0,12,12H216a12,12,0,0,0,12-12V84A32.06,32.06,0,0,0,208,54.34ZM128,160a12,12,0,0,0-12,12v32H92V138.39l36-20.57,36,20.57V204H140V172A12,12,0,0,0,128,160ZM52,84a8,8,0,0,1,16,0v16H52Zm136,0a8,8,0,0,1,16,0v16H188ZM52,124H68v80H52Zm136,80V124h16v80Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M206,58.84V32a6,6,0,0,0-12,0V58.84A22,22,0,0,0,178,80v45.66l-44-25.14V72a6,6,0,0,0-12,0v28.52L78,125.66V80A22,22,0,0,0,62,58.84V32a6,6,0,0,0-12,0V58.84A22,22,0,0,0,34,80V216a6,6,0,0,0,6,6h72a6,6,0,0,0,6-6V176a10,10,0,0,1,20,0v40a6,6,0,0,0,6,6h72a6,6,0,0,0,6-6V80A22,22,0,0,0,206,58.84ZM200,70a10,10,0,0,1,10,10v26H190V80A10,10,0,0,1,200,70ZM56,70A10,10,0,0,1,66,80v26H46V80A10,10,0,0,1,56,70ZM46,118H66v92H46Zm82,36a22,22,0,0,0-22,22v34H78V139.48l50-28.57,50,28.57V210H150V176A22,22,0,0,0,128,154Zm62,56V118h20v92Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M208,57.38V32a8,8,0,0,0-16,0V57.38A24,24,0,0,0,176,80v42.21L136,99.36V72a8,8,0,0,0-16,0V99.36L80,122.21V80A24,24,0,0,0,64,57.38V32a8,8,0,0,0-16,0V57.38A24,24,0,0,0,32,80V216a8,8,0,0,0,8,8h72a8,8,0,0,0,8-8V176a8,8,0,0,1,16,0v40a8,8,0,0,0,8,8h72a8,8,0,0,0,8-8V80A24,24,0,0,0,208,57.38ZM200,72a8,8,0,0,1,8,8v24H192V80A8,8,0,0,1,200,72ZM56,72a8,8,0,0,1,8,8v24H48V80A8,8,0,0,1,56,72Zm-8,48H64v88H48Zm80,32a24,24,0,0,0-24,24v32H80V140.64l48-27.43,48,27.43V208H152V176A24,24,0,0,0,128,152Zm64,56V120h16v88Z"></path>
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
