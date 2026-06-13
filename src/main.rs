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
    let values = vec![0, 1, 2];
    // create a list of 5 signals
    let length = 5;
    let counters = (1..=length).map(|idx| RwSignal::new(idx));
    let counter_buttons = counters
        .map(|count| {
            view! {
                <li>
                    <button on:click=move |_| *count.write() += 1>{count}</button>
                </li>
            }
        })
        .collect_view();
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    struct Counter {
        id: usize,
        count: RwSignal<i32>,
    }
    view! {
        <button on:click=move |_| *set_count.write() += 1>"Click me"</button>
        // .into() converts `ReadSignal` to `Signal`
        <ProgressBar progress=count />
        // use `Signal::derive()` to wrap a derived signal with the `Signal` type
        <ProgressBar progress=Signal::derive(double_count) />
        // this will just render "012"
        <p>{values.clone()}</p>
        // or we can wrap them in <li>
        <ul>{values.into_iter().map(|n| view! { <li>{n}</li> }).collect::<Vec<_>>()}</ul>
        // but the list itself will never change
        <ul>{counter_buttons}</ul>

        <ForEnumerate
            // Same as <For/>
            each=move || counters.get()
            // Same as <For/>
            key=|counter| counter.id
            // let syntax
            let(idx,
            counter)
        >
            <button>{move || idx.get()} ". Value: " {move || counter.count.get()}</button>
        </ForEnumerate>
    }
}
