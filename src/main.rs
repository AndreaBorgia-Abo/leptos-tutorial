use leptos::prelude::*;
fn main() {
    mount_to_body(App);
}

#[component]
fn ProgressBar(
    #[prop(default = 100)] max: u16,
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress max=max value=progress />
        <br />
    }
}

#[component]
fn StaticList(#[prop(default = 5)] itemno: u16) -> impl IntoView {
    let counters = (1..=itemno).map(|idx| RwSignal::new(idx));

    // each item manages a reactive view
    // but the list itself will never change
    let counter_buttons = counters
        .map(|count| {
            view! {
                <li>
                    <button on:click=move |_| *count.write() += 1>{count}</button>
                </li>
            }
        })
        .collect_view();
    view! { <ul>{counter_buttons}</ul> }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Counter {
    id: u16,
    count: ArcRwSignal<i32>,
}
#[component]
fn DynamicList(
    #[prop(default = 5)] initial_length: u16, // The number of counters to begin with.
) -> impl IntoView {
    // `next_counter_id` will let us generate unique IDs
    // we do this by simply incrementing the ID by one
    // each time we create a counter
    let mut next_counter_id = initial_length;

    // we generate an initial list as in <StaticList/>
    // but this time we include the ID along with the signal
    // see NOTE in add_counter below re: ArcRwSignal
    let initial_counters = (0..initial_length)
        .map(|id| Counter {
            id,
            count: ArcRwSignal::new((id + 1) as i32),
        })
        .collect::<Vec<_>>();

    // now we store that initial list in a signal
    // this way, we'll be able to modify the list over time,
    // adding and removing counters, and it will change reactively
    let (counters, set_counters) = signal(initial_counters);

    let add_counter = move |_| {
        // create a signal for the new counter
        // we use ArcRwSignal here, instead of RwSignal
        // ArcRwSignal is a reference-counted type, rather than the arena-allocated
        // signal types we've been using so far.
        // When we're creating a collection of signals like this, using ArcRwSignal
        // allows each signal to be deallocated when its row is removed.
        let sig = ArcRwSignal::new((next_counter_id + 1) as i32);
        // add this counter to the list of counters
        set_counters.update(move |counters| {
            // since `.update()` gives us `&mut T`
            // we can just use normal Vec methods like `push`
            counters.push(Counter {
                id: next_counter_id,
                count: sig,
            });
        });
        // increment the ID so it's always unique
        next_counter_id += 1;
    };

    view! {
        <div>
            <button on:click=add_counter>"Add Counter"</button>
            <ul>
                <ForEnumerate
                    each=move || counters.get()
                    key=|counter| counter.id
                    let(row_idx,
                    counter)
                >
                    <li>
                        <span>"Row position: " {row_idx} " | "</span>
                        <button on:click={
                            let count = counter.count.clone();
                            move |_| *count.write() += 1
                        }>
                            {
                                let count = counter.count.clone();
                                move || count.get()
                            }
                        </button>
                        <button on:click={
                            let id = counter.id;
                            move |_| set_counters.write().retain(|c| c.id != id)
                        }>"Remove"</button>
                    </li>
                </ForEnumerate>
            </ul>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;
    let values = vec![0, 1, 2];

    view! {
        <button on:click=move |_| *set_count.write() += 1>"Click me"</button>
        <br />
        <ProgressBar progress=count />
        <ProgressBar progress=Signal::derive(double_count) />

        <p>"this will just render the numbers: " {values.clone()}</p>
        <p>"wrapped in li:" {values.into_iter().map(|n| view! { <li>{n}</li> }).collect_view()}</p>

        <StaticList />
        <DynamicList initial_length=10 />
    }
}
