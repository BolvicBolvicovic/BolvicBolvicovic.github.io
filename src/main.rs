use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let counters = (0..5).map(|i| create_signal(i)).collect();
    view! {
            <StaticButtons counters=counters/>
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
