//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "brand", feature = "development"))]
#[component]
pub fn StackOverflowLogo(
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
                <path d="M208,32H48A16,16,0,0,0,32,48V208a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V48A16,16,0,0,0,208,32ZM134.86,46.86a8,8,0,0,1,11.32,0l45.25,45.26a8,8,0,0,1-11.31,11.31L134.86,58.18A8,8,0,0,1,134.86,46.86ZM100.18,98.77a8,8,0,0,1,10.45-4.33l59.13,24.49a8,8,0,0,1-3.06,15.4,7.89,7.89,0,0,1-3.06-.62l-59.13-24.49A8,8,0,0,1,100.18,98.77ZM96,152h64a8,8,0,0,1,0,16H96a8,8,0,0,1,0-16Zm104,40a8,8,0,0,1-8,8H64a8,8,0,0,1-8-8V144a8,8,0,0,1,16,0v40H184V144a8,8,0,0,1,16,0Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M208,152v64H48V152Z" opacity="0.2"></path>
    <path d="M216,152.09V216a8,8,0,0,1-8,8H48a8,8,0,0,1-8-8V152.09a8,8,0,0,1,16,0V208H200V152.09a8,8,0,0,1,16,0Zm-128,32h80a8,8,0,1,0,0-16H88a8,8,0,1,0,0,16Zm4.88-53,77.27,20.68a7.89,7.89,0,0,0,2.08.28,8,8,0,0,0,2.07-15.71L97,115.61A8,8,0,1,0,92.88,131Zm18.45-49.93,69.28,40a8,8,0,0,0,10.93-2.93,8,8,0,0,0-2.93-10.91L119.33,67.27a8,8,0,1,0-8,13.84Zm87.33,13A8,8,0,1,0,210,82.84l-56.57-56.5a8,8,0,0,0-11.32,11.3Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M212,152.09V216a4,4,0,0,1-4,4H48a4,4,0,0,1-4-4V152.09a4,4,0,0,1,8,0V212H204V152.09a4,4,0,0,1,8,0Zm-124,28h80a4,4,0,1,0,0-8H88a4,4,0,1,0,0,8Zm5.92-52.86,77.27,20.67a3.72,3.72,0,0,0,1,.14,4,4,0,0,0,1-7.85L96,119.47a4,4,0,1,0-2.07,7.72Zm19.41-49.54,69.28,39.95a4,4,0,1,0,4-6.92L117.33,70.73a4,4,0,1,0-4,6.92Zm88.16,13.66a4,4,0,0,0,5.66-5.64l-56.57-56.5a4,4,0,1,0-5.66,5.65Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M220,151.67V216A12,12,0,0,1,208,228H48A12,12,0,0,1,36,216V151.67a12,12,0,1,1,24,0V203.9H196V151.67a12,12,0,1,1,24,0ZM88,183.81h80a12.06,12.06,0,0,0,0-24.11H88a12.06,12.06,0,0,0,0,24.11ZM96.2,113l75.17,27.49a12.05,12.05,0,0,0,8.21-22.66L104.41,90.35A12,12,0,0,0,96.2,113ZM128,49.29l61.29,51.64a12,12,0,0,0,16.9-1.48,12.09,12.09,0,0,0-1.48-17L143.44,30.82a12,12,0,0,0-16.91,1.49A12.1,12.1,0,0,0,128,49.29Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M214,152.09V216a6,6,0,0,1-6,6H48a6,6,0,0,1-6-6V152.09a6,6,0,0,1,12,0V210H202V152.09a6,6,0,0,1,12,0Zm-126,30h80a6,6,0,1,0,0-12H88a6,6,0,1,0,0,12Zm5.4-52.93,77.27,20.67a6,6,0,1,0,3.11-11.57L96.5,117.54a6,6,0,1,0-3.1,11.58Zm18.93-49.74,69.28,40a6.05,6.05,0,0,0,3,.8,6,6,0,0,0,3-11.18L118.33,69a6,6,0,1,0-6,10.38Zm87.75,13.35a6,6,0,0,0,8.48-8.48L152,27.76a6,6,0,1,0-8.48,8.47Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M216,152.09V216a8,8,0,0,1-8,8H48a8,8,0,0,1-8-8V152.09a8,8,0,0,1,16,0V208H200V152.09a8,8,0,0,1,16,0Zm-128,32h80a8,8,0,1,0,0-16H88a8,8,0,1,0,0,16Zm4.88-53,77.27,20.68a7.89,7.89,0,0,0,2.08.28,8,8,0,0,0,2.07-15.71L97,115.61A8,8,0,1,0,92.88,131Zm18.45-49.93,69.28,40a8,8,0,0,0,10.93-2.93,8,8,0,0,0-2.93-10.91L119.33,67.27a8,8,0,1,0-8,13.84Zm87.33,13A8,8,0,1,0,210,82.84l-56.57-56.5a8,8,0,0,0-11.32,11.3Z"></path>
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
