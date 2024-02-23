//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "office", feature = "map", feature = "objects"))]
#[component]
pub fn PushPinSlash(
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
                <path d="M56.88,31.93A12,12,0,1,0,39.12,48.07L60,71A76,76,0,0,0,35.46,84.51a20,20,0,0,0-1.6,29.73l45.46,45.47-39.8,39.8a12,12,0,0,0,17,17l39.8-39.81,45.47,45.46A20,20,0,0,0,155.9,228c.47,0,.94,0,1.41-.05A20,20,0,0,0,171.87,220a97.47,97.47,0,0,0,9.54-15.46l17.72,19.49a12,12,0,1,0,17.76-16.14Zm98.49,169.88L54.08,100.52C62,95,70.31,92.12,78.91,91.83l84.51,93A62,62,0,0,1,155.37,201.81Zm82.78-95-39,39.11a12,12,0,1,1-17-16.95l36.19-36.3-55-55L130.59,70.5a12,12,0,0,1-17-16.94l35.57-35.69a20,20,0,0,1,28.3,0l60.69,60.68A20,20,0,0,1,238.15,106.83Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M229.66,98.34,172.39,155.8c11.46,22.93-1.72,45.86-10.11,57a8,8,0,0,1-12,.83L42.34,105.76A8,8,0,0,1,43,93.85c29.65-23.92,57.4-10,57.4-10l57.27-57.46a8,8,0,0,1,11.31,0L229.66,87A8,8,0,0,1,229.66,98.34Z"
        opacity="0.2"
    ></path>
    <path d="M53.92,34.62A8,8,0,1,0,42.08,45.38L67.37,73.2A69.82,69.82,0,0,0,38,87.63a16,16,0,0,0-1.29,23.78L85,159.71,42.34,202.34a8,8,0,0,0,11.32,11.32L96.29,171l48.29,48.29A16,16,0,0,0,155.9,224c.38,0,.75,0,1.13,0a15.93,15.93,0,0,0,11.64-6.33,89.75,89.75,0,0,0,11.58-20.27l21.84,24a8,8,0,1,0,11.84-10.76ZM155.9,208,48,100.08C58.23,91.83,69.2,87.72,80.66,87.81l87.16,95.88C165.59,193.56,160.24,202.23,155.9,208Zm79.42-104-44.64,44.79a8,8,0,1,1-11.33-11.3L224,92.7,163.32,32,122.1,73.35a8,8,0,0,1-11.33-11.29L152,20.7a16,16,0,0,1,22.63,0l60.69,60.68A16,16,0,0,1,235.32,104Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M213.93,210.62a8,8,0,1,1-11.84,10.76l-21.84-24a89.75,89.75,0,0,1-11.58,20.27A15.93,15.93,0,0,1,157,224c-.38,0-.75,0-1.13,0a16,16,0,0,1-11.32-4.69L96.29,171,53.66,213.66a8,8,0,0,1-11.32-11.32L85,159.71l-48.3-48.3A16,16,0,0,1,38,87.63,69.82,69.82,0,0,1,67.37,73.2L42.08,45.38A8,8,0,1,1,53.92,34.62L182.3,175.83l0,0ZM235.32,81.37,174.63,20.69a16,16,0,0,0-22.63,0L110.77,62.06a8,8,0,0,0-.25,11l68.57,75.43a8,8,0,0,0,5.74,2.62H185a8,8,0,0,0,5.66-2.35L235.32,104A16,16,0,0,0,235.32,81.37Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M52.44,36A6,6,0,0,0,43.56,44L71.27,74.51C61.78,76,50.6,80,39.22,89.18A14,14,0,0,0,38.1,110l49.71,49.71-44.05,44a6,6,0,1,0,8.48,8.48l44.05-44.05L146,217.89a14,14,0,0,0,9.9,4.11q.49,0,1,0a14,14,0,0,0,10.19-5.54,85.51,85.51,0,0,0,12.44-22.84l24,26.45a6,6,0,1,0,8.87-8.08ZM157.49,209.21a2,2,0,0,1-3,.2L46.58,101.51a2,2,0,0,1,.18-3c13.18-10.64,25.84-12.9,34.79-12.7L170,183.11C167.83,193.74,162.11,203.07,157.49,209.21Zm76.42-106.62-44.65,44.78a6,6,0,1,1-8.5-8.47l44.65-44.79a2,2,0,0,0,0-2.84L164.73,30.59a2,2,0,0,0-2.83,0L120.68,71.94a6,6,0,0,1-8.5-8.47l41.23-41.36a14,14,0,0,1,19.81,0l60.69,60.69A14,14,0,0,1,233.91,102.59Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M53.92,34.62A8,8,0,1,0,42.08,45.38L67.37,73.2A69.82,69.82,0,0,0,38,87.63a16,16,0,0,0-1.29,23.78L85,159.71,42.34,202.34a8,8,0,0,0,11.32,11.32L96.29,171l48.29,48.29A16,16,0,0,0,155.9,224c.38,0,.75,0,1.13,0a15.93,15.93,0,0,0,11.64-6.33,89.75,89.75,0,0,0,11.58-20.27l21.84,24a8,8,0,1,0,11.84-10.76ZM155.9,208,48,100.08C58.23,91.83,69.2,87.72,80.66,87.81l87.16,95.88C165.59,193.56,160.24,202.23,155.9,208Zm79.42-104-44.64,44.79a8,8,0,1,1-11.33-11.3L224,92.7,163.32,32,122.1,73.35a8,8,0,0,1-11.33-11.29L152,20.7a16,16,0,0,1,22.63,0l60.69,60.68A16,16,0,0,1,235.32,104Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M51,37.31A4,4,0,0,0,45,42.69L75.35,76c-9.93.88-22.26,4.54-34.87,14.71a12,12,0,0,0-1,17.84l51.13,51.13L45.17,205.17a4,4,0,0,0,5.66,5.66l45.46-45.47,51.12,51.12A12,12,0,0,0,155.9,220c.28,0,.57,0,.85,0a12,12,0,0,0,8.73-4.74,81.06,81.06,0,0,0,13.19-25.55l26.37,29a4,4,0,1,0,5.92-5.38ZM159.08,210.42a4,4,0,0,1-6,.4L45.17,102.93a4,4,0,0,1,.33-6c14.08-11.36,27.62-13.5,37-13.12l89.69,98.65C170.07,193.92,164,203.92,159.08,210.42Zm73.41-109.25L187.85,146a4,4,0,0,1-5.67-5.64l44.65-44.8a4,4,0,0,0,0-5.66L166.15,29.17a4,4,0,0,0-5.66,0L119.27,70.53a4,4,0,0,1-5.67-5.65l41.23-41.36a12,12,0,0,1,17,0L232.49,84.2A12,12,0,0,1,232.49,101.17Z"></path>
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
