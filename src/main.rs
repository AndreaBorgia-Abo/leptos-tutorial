use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <button // define an event listener with on:
        on:click=move |_| {
            set_count.set(3)
        }>
            // text nodes are wrapped in quotation marks
            "Click me: "
            // blocks include Rust code
            // in this case, it renders the value of the signal
            {count}
        </button>
        <p>"Double count: " {move || count.get() * 2}</p>
    }
}
