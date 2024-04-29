use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

#[derive(Clone)]
struct Tester {
    key: String,
    value: RwSignal<i32>,
}

#[component]
fn App() -> impl IntoView {
    let counters = (0..5).map(|i| create_signal(i)).collect();
    
    let mut index = 5;
    let (buttons, set_buttons) = create_signal((0..index)
                                .map(|id| (id, create_signal(id + 1)))
                                .collect::<Vec<_>>());
    let add_button = move |_| {
        let sig = create_signal(index + 1);
        set_buttons.update(move |counters| {
            counters.push((index, sig))
        });
        index += 1;
    };

    let (tester, _) = create_signal(vec![
        Tester {
            key: "Foo".to_string(),
            value: create_rw_signal(10)
        },
        Tester {
            key: "Bar".to_string(),
            value: create_rw_signal(20)
        }
    ]);
    let (name, set_name) = create_signal("Controlled input".to_string());

    let (name_2, set_name_2) = create_signal("Uncontrolled input".to_string());
    let input_element: NodeRef<html::Input> = create_node_ref();
    let on_submit = move |ev: ev::SubmitEvent| {
        //stop reload
        ev.prevent_default();
        
        let value = input_element()
            .expect("<input> should be mounted")
            .value();
        set_name_2(value);
    };

    let (selector, set_selector) = create_signal("B".to_string());

    let (err, set_err) = create_signal(Ok(0));
    view! {
            <StaticButtons counters=counters/>
            <button on:click=add_button>"Add button"</button>
            <DynamicButtons buttons=buttons set_buttons=set_buttons/>
            <button
                on:click=move |_| {
                    tester.with(|tester| {
                        for row in tester {
                            row.value.update(|value| *value *= 2);
                        }
                    });
                }
            >
                "Update"
            </button>
            <For
                each=tester
                key=|state| state.key.clone()
                let:child
            >
                <p>{child.value}</p>
            </For>

            <input type="text"
                on:input=move |ev| {
                    set_name(event_target_value(&ev));
                }
                prop:value=name
            />
            <p>"On input: " {name}</p>

            <form on:submit=on_submit>
                <input type="text"
                    value=name_2
                    node_ref=input_element
                />
                <input type="submit" value="Submit"/>
            </form>
            <p>"On submit: " {name_2}</p>

            <textarea
                prop:value=name
                on:input=move |ev| {
                    set_name(event_target_value(&ev))
                }
            >
                {name}
            </textarea>
            <select on:change=move |ev| {
                let new_value = event_target_value(&ev);
                set_selector(new_value);
            }>
                <option
                    value="A"
                    selected= move || selector() == "A"
                >
                "A"
                </option>
                <option
                    value="B"
                    selected= move || selector() == "B"
                >
                "B"
                </option>
            </select>
            
            <Show //good for Option<_> use <ErrorBoundary/> for Result<_>
                when=move || name() == "B"
                fallback=|| view! {"A"}
            >
                "B"
            </Show><br/>
            <label>
                "Error boundry section"
                <input type="number" on:input=move |ev| set_err(event_target_value(&ev).parse::<i32>())/>
                <ErrorBoundary
                    fallback=|errors| view! {
                        <p>
                            {move || errors.get()
                                    .into_iter()
                                    .map(|(_, e)| view! {
                                        <li>{e.to_string()}</li>
                                    })
                                    .collect_view()
                            }
                        </p>
                    }
                >
                    <p>"Valid input: "{err}</p>
                </ErrorBoundary>
            </label>
    }
}

#[component]
fn DynamicButtons(
    buttons: ReadSignal<Vec<(i32, (ReadSignal<i32>, WriteSignal<i32>))>>,
    set_buttons: WriteSignal<Vec<(i32, (ReadSignal<i32>, WriteSignal<i32>))>>
) -> impl IntoView {
    view! {
        <For
            each=buttons
            key=|buttons| buttons.0
            children=move |(id, (_, _))| {
                view! {
                    <button
                        on:click=move |_| {
                            set_buttons.update(|counters| {
                                counters.retain(|(counter_id, (signal, _))| {
                                    if counter_id == &id {
                                        signal.dispose();
                                    }
                                    counter_id != &id
                                })
                            })
                        }
                    />
                }
            }
        />
    }
}

#[component]
fn StaticButtons(
    counters: Vec<(ReadSignal<i32>, WriteSignal<i32>)>,
) -> impl IntoView {
    counters
        .into_iter()
        .map(|(count, set_count)| {
            view! {
                <button
                    on:click=move |_| {
                        set_count.update(|n| if *n < 255 {*n += 5});
                    }
                    style:background=move || format!("rgb({}, {}, {})", count() % 100, count() % 42, count() % 200)
                    style=("--columns", count)
                    class:red=move || count() % 2 == 1
                    class:white=move || count() % 2 == 0
                >
                    "Click me: "
                    {count}
                    <br/>
                    <ProgressBar progress=count/>
                </button>
            }
        })
        .collect_view()
}

#[component]
fn ProgressBar<F>(
    #[prop(default = 255)]
    max: u16,
    progress: F,
) -> impl IntoView
where
    F: Fn() -> i32 + 'static,
{
    view! {
        <progress
            max=max
            style:min-width="400px"
            value=progress
        />
    }
}
