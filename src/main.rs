use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

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
    view! {
        <progress max=max value=progress />
        <br />
    }
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;

    view! {
        <button on:click=move |_| *set_count.write() += 1>"Click me"</button>
        // .into() converts `ReadSignal` to `Signal`
        <ProgressBar progress=count />
        // use `Signal::derive()` to wrap a derived signal with the `Signal` type
        <ProgressBar progress=Signal::derive(double_count) />
    }
}
