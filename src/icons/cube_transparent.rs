use crate::IconWeight;
/// GENERATED FILE
use leptos::*;

#[component]
pub fn CubeTransparent(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M224.5,95.53v0l-64-64A12,12,0,0,0,152,28H40A12,12,0,0,0,28,40V152a11.94,11.94,0,0,0,3,7.93c.15.18.31.36.5.56l64,64h0A12,12,0,0,0,104,228H216a12,12,0,0,0,12-12V104A12,12,0,0,0,224.5,95.53ZM164,69l23,23H164ZM92,187,69,164H92Zm0-47H52V69l40,40ZM69,52h71V92H109Zm71,64v24H116V116Zm-24,88V164h31l40,40Zm88-17-40-40V116h40Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M216,96V216H96L40,160V40H160Z" opacity="0.2"/><path d="M221.66,90.34h0l-56-56A8,8,0,0,0,160,32H40a8,8,0,0,0-8,8V160a8,8,0,0,0,2.3,5.61l56,56h0A8,8,0,0,0,96,224H216a8,8,0,0,0,8-8V96A8,8,0,0,0,221.66,90.34ZM168,59.31,196.69,88H168ZM88,196.69,59.31,168H88ZM88,152H48V59.31l40,40ZM59.31,48H152V88H99.31ZM152,104v48H104V104ZM104,208V168h52.69l40,40Zm104-11.31-40-40V104h40Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M104,152V104h48v48ZM32,53v95a4,4,0,0,0,4,4H88V99.31L38.83,50.14A4,4,0,0,0,32,53Zm188,51H168v52.69l49.17,49.17A4,4,0,0,0,224,203V108A4,4,0,0,0,220,104ZM152,36a4,4,0,0,0-4-4H53a4,4,0,0,0-2.83,6.83L99.31,88H152Zm60.49,45.17L174.83,43.51A4,4,0,0,0,168,46.34V88h41.66A4,4,0,0,0,212.49,81.17ZM156.69,168H104v52a4,4,0,0,0,4,4h95a4,4,0,0,0,2.83-6.83ZM43.51,174.83l37.66,37.66A4,4,0,0,0,88,209.66V168H46.34A4,4,0,0,0,43.51,174.83Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M220.24,91.75,164,35.56A5.93,5.93,0,0,0,160,34H40a6,6,0,0,0-6,6V160a6,6,0,0,0,1.76,4.25l56,56A6,6,0,0,0,96,222H216a6,6,0,0,0,6-6V96A6,6,0,0,0,220.24,91.75ZM166,54.48,201.52,90H166Zm-76,147L54.48,166H90ZM90,154H46V54.48l44,44ZM54.48,46H154V90H98.48ZM154,102v52H102V102ZM102,210V166h55.52l44,44Zm108-8.48-44-44V102h44Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M221.66,90.34h0l-56-56A8,8,0,0,0,160,32H40a8,8,0,0,0-8,8V160a8,8,0,0,0,2.3,5.61l56,56h0A8,8,0,0,0,96,224H216a8,8,0,0,0,8-8V96A8,8,0,0,0,221.66,90.34ZM168,59.31,196.69,88H168ZM88,196.69,59.31,168H88ZM88,152H48V59.31l40,40ZM59.31,48H152V88H99.31ZM152,104v48H104V104ZM104,208V168h52.69l40,40Zm104-11.31-40-40V104h40Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M219.92,95.22a1.8,1.8,0,0,0-.1-.33,3.38,3.38,0,0,0-.13-.42,2.58,2.58,0,0,0-.19-.37c-.06-.11-.11-.22-.17-.32a3.62,3.62,0,0,0-.5-.6h0l-56-56h0a2.32,2.32,0,0,0-.33-.27A4,4,0,0,0,160,36H40a4,4,0,0,0-4,4V160a4,4,0,0,0,.9,2.5,2.32,2.32,0,0,0,.27.33l56,56A4,4,0,0,0,96,220H216a4,4,0,0,0,4-4V96A4.13,4.13,0,0,0,219.92,95.22ZM164,49.66,206.34,92H164ZM92,206.34,49.66,164H92ZM92,156H44V49.66l48,48ZM49.66,44H156V92H97.66ZM156,100v56H100V100ZM100,212V164h58.34l48,48Zm112-5.66-48-48V100h48Z"/> }.into_view()
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
