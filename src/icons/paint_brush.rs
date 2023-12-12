use crate::IconWeight;
/// GENERATED FILE
use leptos::*;

#[component]
pub fn PaintBrush(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M236,32a12,12,0,0,0-12-12c-44.78,0-90,48.54-115.9,82a64,64,0,0,0-80,62c0,12-3.1,22.71-9.23,31.76A43,43,0,0,1,9.4,206.05a11.88,11.88,0,0,0-4.91,13.38A12.07,12.07,0,0,0,16.11,228h76A64,64,0,0,0,154,148C187.49,122.05,236,76.8,236,32ZM209.62,46.39c-4,12.92-13.15,27.49-26.92,42.91-3,3.39-6.16,6.7-9.35,9.89a104.31,104.31,0,0,0-16.5-16.51c3.19-3.19,6.49-6.32,9.88-9.35C182.15,59.55,196.71,50.43,209.62,46.39ZM92.07,204H42a80.17,80.17,0,0,0,10.14-40,40,40,0,1,1,40,40Zm38.18-91.32c3.12-3.93,6.55-8.09,10.23-12.35a80.52,80.52,0,0,1,15.23,15.24c-4.26,3.68-8.42,7.11-12.35,10.23A64.43,64.43,0,0,0,130.25,112.68Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M224,32c0,32.81-31.64,67.43-58.64,91.05A84.39,84.39,0,0,0,133,90.64C156.57,63.64,191.19,32,224,32Z" opacity="0.2"/><path d="M232,32a8,8,0,0,0-8-8c-44.08,0-89.31,49.71-114.43,82.63A60,60,0,0,0,32,164c0,30.88-19.54,44.73-20.47,45.37A8,8,0,0,0,16,224H92a60,60,0,0,0,57.37-77.57C182.3,121.31,232,76.08,232,32ZM92,208H34.63C41.38,198.41,48,183.92,48,164a44,44,0,1,1,44,44Zm32.42-94.45q5.14-6.66,10.09-12.55A76.23,76.23,0,0,1,155,121.49q-5.9,4.94-12.55,10.09A60.54,60.54,0,0,0,124.42,113.55Zm42.7-2.68a92.57,92.57,0,0,0-22-22c31.78-34.53,55.75-45,69.9-47.91C212.17,55.12,201.65,79.09,167.12,110.87Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M232,32a8,8,0,0,0-8-8c-44.08,0-89.31,49.71-114.43,82.63A60,60,0,0,0,32,164c0,30.88-19.54,44.73-20.47,45.37A8,8,0,0,0,16,224H92a60,60,0,0,0,57.37-77.57C182.3,121.31,232,76.08,232,32ZM124.42,113.55q5.14-6.66,10.09-12.55A76.23,76.23,0,0,1,155,121.49q-5.9,4.94-12.55,10.09A60.54,60.54,0,0,0,124.42,113.55Zm42.7-2.68a92.57,92.57,0,0,0-22-22c31.78-34.53,55.75-45,69.9-47.91C212.17,55.12,201.65,79.09,167.12,110.87Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M224,26c-20.8,0-44.11,11.41-69.3,33.9C136.62,76.06,121,94.9,110.3,109A58,58,0,0,0,34,164c0,32.07-20.43,46.39-21.35,47A6,6,0,0,0,16,222H92a58,58,0,0,0,55-76.3c14.08-10.67,32.92-26.32,49.08-44.4C218.59,76.11,230,52.8,230,32A6,6,0,0,0,224,26ZM92,210H30.65C37.92,200.85,46,185.78,46,164a46,46,0,1,1,46,46Zm29.49-95.91c3.6-4.67,7.88-10,12.71-15.69a78.17,78.17,0,0,1,23.4,23.4c-5.67,4.83-11,9.11-15.69,12.71A58.38,58.38,0,0,0,121.49,114.09Zm45.2-.3a90.24,90.24,0,0,0-24.48-24.48C163.05,66.46,191,42,217.56,38.44,214,65,189.54,93,166.69,113.79Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M232,32a8,8,0,0,0-8-8c-44.08,0-89.31,49.71-114.43,82.63A60,60,0,0,0,32,164c0,30.88-19.54,44.73-20.47,45.37A8,8,0,0,0,16,224H92a60,60,0,0,0,57.37-77.57C182.3,121.31,232,76.08,232,32ZM92,208H34.63C41.38,198.41,48,183.92,48,164a44,44,0,1,1,44,44Zm32.42-94.45q5.14-6.66,10.09-12.55A76.23,76.23,0,0,1,155,121.49q-5.9,4.94-12.55,10.09A60.54,60.54,0,0,0,124.42,113.55Zm42.7-2.68a92.57,92.57,0,0,0-22-22c31.78-34.53,55.75-45,69.9-47.91C212.17,55.12,201.65,79.09,167.12,110.87Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M224,28c-20.29,0-43.16,11.24-68,33.4-18.47,16.49-34.39,35.83-45,49.93A56,56,0,0,0,36,164c0,33.22-21.26,48-22.22,48.68A4,4,0,0,0,16,220H92a56,56,0,0,0,52.67-75c14.11-10.63,33.44-26.55,49.93-45C216.76,75.16,228,52.29,228,32A4,4,0,0,0,224,28ZM92,212H26.35C33.91,203.69,44,188.08,44,164a48,48,0,1,1,48,48Zm26.52-97.31c4.13-5.44,9.32-12,15.29-18.9a80.08,80.08,0,0,1,26.4,26.4c-6.94,6-13.46,11.16-18.9,15.29A56.32,56.32,0,0,0,118.52,114.69Zm47.77,2.14a88.17,88.17,0,0,0-27.12-27.12C161,65.43,191.26,38.63,219.82,36.18,217.37,64.74,190.57,95,166.29,116.83Z"/> }.into_view()
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
