use leptos::prelude::*;
use leptos_tutorial::{iteration, spreading::SpreadingExample};

/// Shows progress toward a goal.
#[component]
fn ProgressBar(
    /// The maximum value of the progress bar.
    #[prop(default = 100)]
    max: u16,
    /// How much progress should be displayed.
    #[prop(into)]
    progress: Signal<i32>,
) -> impl IntoView {
    view! { <progress max=max value=progress /> }
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;

    view! {
        <button
            on:click=move |_| { *set_count.write() += 1 }
            class:red=move || { count.get() % 2 == 1 }
            style:width=move || { format!("{}px", count.get() * 10 + 100) }
        >
            "Click me: "
            {count}
        </button>
        <p>"Double count: "{double_count}</p>

        <ProgressBar progress=count />
        <ProgressBar progress=Signal::derive(double_count) />

        <SpreadingExample />

        <iteration::BasicExample />
        <iteration::StaticListExample />
        <iteration::DynamicListExample initial_length=2 />
        <iteration::AccessListIndexExample initial_length=2 />
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
