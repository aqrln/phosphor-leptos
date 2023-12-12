use crate::IconWeight;
/// GENERATED FILE
use leptos::*;

#[component]
pub fn Vault(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M216,36H40A20,20,0,0,0,20,56V192a20,20,0,0,0,20,20H52v12a12,12,0,0,0,24,0V212H180v12a12,12,0,0,0,24,0V212h12a20,20,0,0,0,20-20V56A20,20,0,0,0,216,36ZM44,188V60H212v56H190.32a44,44,0,1,0,0,24H212v48Zm124-60a20,20,0,1,1-20-20A20,20,0,0,1,168,128Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M216,48H40a8,8,0,0,0-8,8V192a8,8,0,0,0,8,8H216a8,8,0,0,0,8-8V56A8,8,0,0,0,216,48ZM152,168a40,40,0,1,1,40-40A40,40,0,0,1,152,168Z" opacity="0.2"/><path d="M216,40H40A16,16,0,0,0,24,56V192a16,16,0,0,0,16,16H56v16a8,8,0,0,0,16,0V208H184v16a8,8,0,0,0,16,0V208h16a16,16,0,0,0,16-16V56A16,16,0,0,0,216,40Zm0,152H40V56H216v64H199.32a48,48,0,1,0,0,16H216v56Zm-50.16-72a16,16,0,1,0,0,16H183a32,32,0,1,1,0-16Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M216,40H40A16,16,0,0,0,24,56V192a16,16,0,0,0,16,16H56v16a8,8,0,0,0,16,0V208H184v16a8,8,0,0,0,16,0V208h16a16,16,0,0,0,16-16V56A16,16,0,0,0,216,40Zm-8,96H179.09a36,36,0,1,1,0-16H208a8,8,0,0,1,0,16Zm-44-8a20,20,0,1,1-20-20A20,20,0,0,1,164,128Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M216,42H40A14,14,0,0,0,26,56V192a14,14,0,0,0,14,14H58v18a6,6,0,0,0,12,0V206H186v18a6,6,0,0,0,12,0V206h18a14,14,0,0,0,14-14V56A14,14,0,0,0,216,42Zm0,152H40a2,2,0,0,1-2-2V56a2,2,0,0,1,2-2H216a2,2,0,0,1,2,2v66H197.6a46,46,0,1,0,0,12H218v58A2,2,0,0,1,216,194Zm-51.37-72a14,14,0,1,0,0,12h20.83a34,34,0,1,1,0-12Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M216,40H40A16,16,0,0,0,24,56V192a16,16,0,0,0,16,16H56v16a8,8,0,0,0,16,0V208H184v16a8,8,0,0,0,16,0V208h16a16,16,0,0,0,16-16V56A16,16,0,0,0,216,40Zm0,152H40V56H216v64H199.32a48,48,0,1,0,0,16H216v56Zm-50.16-72a16,16,0,1,0,0,16H183a32,32,0,1,1,0-16Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M216,44H40A12,12,0,0,0,28,56V192a12,12,0,0,0,12,12H60v20a4,4,0,0,0,8,0V204H188v20a4,4,0,0,0,8,0V204h20a12,12,0,0,0,12-12V56A12,12,0,0,0,216,44Zm0,152H40a4,4,0,0,1-4-4V56a4,4,0,0,1,4-4H216a4,4,0,0,1,4,4v68H195.81a44,44,0,1,0,0,8H220v60A4,4,0,0,1,216,196Zm-52.7-72a12,12,0,1,0,0,8h24.47a36,36,0,1,1,0-8Z"/> }.into_view()
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
