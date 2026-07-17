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

use reactive_stores::Store;

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
}
