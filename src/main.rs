use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let double_count = move || count() * 2;
    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| if *n < 255 {*n += 5});
            }
            style="position: absolute"
            style:left=move || format!("rgb({}, {}, 100)", count(), 100)
            style:background-color=move || format!("rgb({}, {}, 100)", count(), 100)
            style:max-width="1000px"
            style:min-width="500px"
            style:max-height="500px"
            style:min-height="250px"
            style:font-size="xx-large"
            style:text-align="center"
            style:border-radius="50px"
            style=("--columns", count)
            class:red=move || count() % 2 == 1
        >
            "Click me: "
            {count}
        <br/>
        <progress
            max="255"
            style:min-width="400px"
            value=double_count
        />
        <br/>
            "Double Count: "
            {double_count}
        </button>
    }
}
