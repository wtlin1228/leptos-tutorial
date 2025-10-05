use leptos::prelude::*;

#[component]
pub fn BasicExample() -> impl IntoView {
    let values = vec![1, 2, 3];

    view! {
        <p>{values.clone()}</p>
        <ul>{values.into_iter().map(|n| view! { <li>{n}</li> }).collect_view()}</ul>
    }
}

/// A list of counters, without the ability to add or remove any.
#[component]
pub fn StaticListExample() -> impl IntoView {
    let length = 5;
    let counters = (1..=length).map(|idx| RwSignal::new(idx));
    let counter_buttons = counters
        .map(|counter| {
            view! {
                <li>
                    <button on:click=move |_| *counter.write() += 1>{counter}</button>
                </li>
            }
        })
        .collect_view();

    view! { <ul>{counter_buttons}</ul> }
}

/// A list of counters that allows you to add or remove counters.
#[component]
pub fn DynamicListExample(
    /// The number of counters to begin with.
    initial_length: usize,
) -> impl IntoView {
    let mut next_counter_id = initial_length;
    let initial_counters = (0..initial_length)
        .map(|id| (id, ArcRwSignal::new(id + 1)))
        .collect::<Vec<_>>();

    let (counters, set_counters) = signal(initial_counters);
    let add_counter = move |_| {
        set_counters.update(move |counters| {
            counters.push((next_counter_id, ArcRwSignal::new(next_counter_id + 1)));
        });
        next_counter_id += 1;
    };

    view! {
        <div>
            <button on:click=add_counter>"Add counter"</button>
            <ul>
                <For
                    each=move || counters.get()
                    key=|counter| counter.0
                    children=move |(id, counter)| {
                        let counter = RwSignal::from(counter);
                        view! {
                            <li>
                                <button on:click=move |_| *counter.write() += 1>{counter}</button>
                                <button on:click=move |_| {
                                    set_counters
                                        .write()
                                        .retain(|(counter_id, _)| { counter_id != &id })
                                }>"Remove"</button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
pub fn AccessListIndexExample(
    /// The number of counters to begin with.
    initial_length: usize,
) -> impl IntoView {
    let mut next_counter_id = initial_length;
    let initial_counters = (0..initial_length)
        .map(|id| (id, ArcRwSignal::new(id + 1)))
        .collect::<Vec<_>>();

    let (counters, set_counters) = signal(initial_counters);
    let add_counter = move |_| {
        set_counters.update(move |counters| {
            counters.push((next_counter_id, ArcRwSignal::new(next_counter_id + 1)));
        });
        next_counter_id += 1;
    };

    view! {
        <div>
            <button on:click=add_counter>"Add counter"</button>
            <ul>
                <ForEnumerate
                    each=move || counters.get()
                    key=|counter| counter.0
                    children=move |index, (id, counter)| {
                        let counter = RwSignal::from(counter);
                        view! {
                            <li>
                                <span>"Index: "{index}", Id: "{id}" "</span>
                                <button on:click=move |_| *counter.write() += 1>{counter}</button>
                                <button on:click=move |_| {
                                    set_counters
                                        .write()
                                        .retain(|(counter_id, _)| { counter_id != &id })
                                }>"Remove"</button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}
