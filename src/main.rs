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

    view! {
            <StaticButtons counters=counters/>
            <button on:click=add_button>"Add button"</button>
            <DynamicButtons buttons=buttons set_buttons=set_buttons/>
            <br/>
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
                <li
                    style:margin="0"
                    style:margin-bottom="150px"
                >
                     <button
                         on:click=move |_| {
                             set_count.update(|n| if *n < 255 {*n += 5});
                         }
                         style="position: absolute"
                         style:background=move || format!("rgb({}, {}, {})", count() % 100, count() % 42, count() % 200)
                         style:max-width="1000px"
                         style:min-width="500px"
                         style:max-height="400px"
                         style:min-height="150px"
                         style:font-size="xx-large"
                         style:text-align="center"
                         style:border-radius="50px"
                         style=("--columns", count)
                         class:red=move || count() % 2 == 1
                         class:white=move || count() % 2 == 0
                     >
                         "Click me: "
                         {count}
                     <br/>
                     <ProgressBar progress=count/>
                     </button>
                </li>
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
