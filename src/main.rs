use std::vec;

use leptos::prelude::*;
fn main() {
    mount_to_body(App);
}

// Initial version, deliberately not working
/*
#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: i32,
}

#[component]
pub fn App() -> impl IntoView {
    // start with a set of three rows
    let (data, set_data) = signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: 10,
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: 20,
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: 15,
        },
    ]);
    view! {
        // when we click, update each row,
        // doubling its value
        <button on:click=move |_| {
            set_data
                .update(|data| {
                    for row in data {
                        row.value *= 2;
                    }
                });
            leptos::logging::log!("{:?}", data.get());
        }>"Update Values"</button>
        // iterate over the rows and display each value
        <For each=move || data.get() key=|state| state.key.clone() let(child)>
            <p>{child.value}</p>
        </For>
    }
}
*/

// Option 1 - rekey
/*
#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: i32,
}

#[component]
pub fn App() -> impl IntoView {
    // start with a set of three rows
    let (data, set_data) = signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: 10,
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: 20,
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: 15,
        },
    ]);
    view! {
        // when we click, update each row,
        // doubling its value
        <button on:click=move |_| {
            set_data
                .update(|data| {
                    for row in data {
                        row.value *= 2;
                    }
                });
            leptos::logging::log!("{:?}", data.get());
        }>"Update Values"</button>
        // iterate over the rows and display each value
        <For each=move || data.get() key=|state| (state.key.clone(), state.value) let(child)>
            <p>{child.value}</p>
        </For>
    }
}
 */

// Option 2 - nested signal
/* #[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: RwSignal<i32>,
}

#[component]
pub fn App() -> impl IntoView {
    // start with a set of three rows
    let (data, _set_data) = signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: RwSignal::new(10),
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: RwSignal::new(20),
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: RwSignal::new(15),
        },
    ]);
    view! {
        // when we click, update each row,
        // doubling its value
        <button on:click=move |_| {
            for row in &*data.read() {
                row.value.update(|value| *value *= 2);
            }
            leptos::logging::log!("{:?}", data.get());
        }>"Update Values"</button>
        // iterate over the rows and display each value
        <For each=move || data.get() key=|state| state.key.clone() let(child)>
            <p>{child.value}</p>
        </For>
    }
} */

// Option 3 - using Memo
/* #[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: i32,
}

#[component]
pub fn App() -> impl IntoView {
    // start with a set of three rows
    let (data, set_data) = signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: 10,
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: 20,
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: 15,
        },
    ]);
    view! {
        // when we click, update each row,
        // doubling its value
        <button on:click=move |_| {
            set_data
                .update(|data| {
                    for row in data {
                        row.value *= 2;
                    }
                });
            leptos::logging::log!("{:?}", data.get());
        }>"Update Values"</button>
        // iterate over the rows and display each value
        <ForEnumerate
            each=move || data.get()
            key=|state| state.key.clone()
            children=move |index, _| {
                let value = Memo::new(move |_| {
                    data.with(|data| data.get(index.get()).map(|d| d.value).unwrap_or(0))
                });
                view! { <p>{value}</p> }
            }
        />
    }
}
*/

// Option 4 - with Stores
/* use reactive_stores::Store;

#[derive(Store, Debug, Clone)]
pub struct Data {
    #[store(key: String = |row| row.key.clone())]
    rows: Vec<DatabaseEntry>,
}

#[derive(Store, Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: i32,
}

#[component]
pub fn App() -> impl IntoView {
    // instead of a signal with the rows, we create a store for Data
    let data = Store::new(Data {
        rows: vec![
            DatabaseEntry {
                key: "foo".to_string(),
                value: 10,
            },
            DatabaseEntry {
                key: "bar".to_string(),
                value: 20,
            },
            DatabaseEntry {
                key: "baz".to_string(),
                value: 15,
            },
        ],
    });

    view! {
        // when we click, update each row,
        // doubling its value
        <button on:click=move |_| {
            use reactive_stores::StoreFieldIterator;
            for row in data.rows().iter_unkeyed() {
                *row.value().write() *= 2;
            }
            leptos::logging::log!("{:?}", data.get());
        }>"Update Values"</button>
        // iterate over the rows and display each value
        <For
            each=move || data.rows()
            key=|row| row.read().key.clone()
            children=|child| {
                let value = child.value();
                view! { <p>{move || value.get()}</p> }
            }
        />
    }
} */

// Variant of option 4 with Stores for a Dynamic List
use reactive_stores::Store;

#[derive(Store, Debug, Clone, PartialEq, Eq)]
struct Counter {
    key: u16,
    value: u16,
}

#[derive(Default, Store, Debug, Clone)]
pub struct Counters {
    #[store(key: u16 = |row| row.key)]
    rows: Vec<Counter>,
}

#[component]
fn DynamicList(
    #[prop(default = 5)] initial_length: u16, // The number of counters to begin with.
) -> impl IntoView {
    // `next_counter_id` will let us generate unique IDs
    // we do this by simply incrementing the ID by one
    // each time we create a counter
    let next_counter_id = RwSignal::new(0);

    // instead of a signal with the rows, we create a store for Data
    let counters: Store<Counters> = Store::new(Counters::default());

    let add_counter = move |target_pos: Option<usize>| {
        let new_id = next_counter_id.get();
        next_counter_id.update(|id| *id += 1);

        let new_counter = Counter {
            key: new_id,
            value: new_id + 10,
        };

        // We mutate the store field directly using the standard reactive .update() method.
        // Specifying the type parameter on the closure block forces the trait bounds to resolve.
        counters
            .rows()
            .update(|list: &mut Vec<Counter>| match target_pos {
                Some(pos) => {
                    if pos <= list.len() {
                        list.insert(pos, new_counter);
                    } else {
                        list.push(new_counter);
                    }
                }
                None => list.push(new_counter),
            });

        // Notify the store's key tracker that the collection structure changed
        counters.rows().update_keys();
    };

    for _ in 0..initial_length {
        add_counter(None);
    }

    view! {
        <div>
            <button on:click=move |_| {
                for row in counters.rows() {
                    *row.value().write() *= 2;
                }
                leptos::logging::log!("{:?}", counters.get());
            }>"Double Values"</button>

            <button on:click=move |_: leptos::ev::MouseEvent| add_counter(
                None,
            )>"Append Counter"</button>

            <ul>
                <ForEnumerate
                    each=move || counters.rows()
                    key=|row| row.key()
                    let:row_idx
                    let:child
                >
                    <li>
                        <span>
                            "Row position: " {row_idx} " | ID: " {move || child.key()} " | "
                        </span>

                        <button on:click=move |_: leptos::ev::MouseEvent| {
                            *child.value().write() += 1;
                        }>{move || child.value().get()}</button>

                        <button on:click=move |_: leptos::ev::MouseEvent| {
                            let id_to_remove = child.key().get();
                            counters.rows().update(|list| list.retain(|c| c.key != id_to_remove));
                            counters.rows().update_keys();
                        }>"Remove"</button>

                        <button on:click=move |_: leptos::ev::MouseEvent| {
                            let current_pos = row_idx.get();
                            add_counter(Some(current_pos + 1));
                        }>"Insert Counter Below"</button>
                    </li>
                </ForEnumerate>
            </ul>
        </div>
    }
}
#[component]
pub fn App() -> impl IntoView {
    view! { <DynamicList initial_length=3 /> }
}
