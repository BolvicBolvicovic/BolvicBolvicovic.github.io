use leptos::*;
use leptos_router::*;

#[component]
fn App() -> impl IntoView {

    view! {
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=Page>
                        <Route path=""          view=HomePage       />
                        <Route path="Projects"  view=ProjectsPage   />
                        <Route path="More"      view=MorePage       />
                        <Route path="*any"      view=NotFound       />
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Page() -> impl IntoView {
    view! {
        <div class="

            "
        >
            <NavBar/>
            <Outlet/>
            <Bottom/>
        </div>
    }
}

#[component]
fn NavBar() -> impl IntoView {
    view! {
        <div
            class="
                flex flex-row
                bg-slate-200
                justify-between
                items-center
                text-slate-600
                px-5 py-4
                rounded-br-full
                shadow-xl
                font-serif 
            "
        >
            <div class="text-4xl">Victor Cornille</div>
            <div class="flex items-start mr-4">
                <ButtonPage name="About me" path=""/>
                <ButtonPage name="Projects" path="Projects"/>
                <ButtonPage name="More..."  path="More"/>
            </div>
        </div>
    }
}

#[component]
fn Bottom() -> impl IntoView {
    view! {
        <div
            class="
                bg-slate-200
                px-5 py-4
                grid grid-cols-2 grid-rows-2 grid-flow-col
                place-content-evenly
            "
        >
            <div>"1"</div>
            <div>"2"</div>
            <div class="row-span-2">"3"</div>
        </div>
    }
}

#[component]
fn ButtonPage(
    name: &'static str,
    path: &'static str,
) -> impl IntoView {

    view! {
        <A href=path class="mr-4">
            <button 
                class="
                    transition-all
                    duration-300
                    ease-in-out
                    hover:bg-slate-400 hover:ring hover:ring-slate-300 
                    active:bg-slate-500
                    rounded-full
                    px-8 py-2
                    size-full
                    whitespace-nowrap
                "
            >
                {name}
            </button>
        </A>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <PageTitle title="Welcome to Home page"/>
    }
}

#[component]
fn ProjectsPage() -> impl IntoView {
    view! {
        <PageTitle title="Welcome to Projects page"/>
        <div class="flex flex-col overflow-auto min-h-screen">
            <ProjectCanvas text="This is a project"/>
            <div class="flex justify-end">
                <ProjectCanvas text="
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam scelerisque ipsum ut arcu hendrerit feugiat. Quisque in arcu laoreet, eleifend dui sed, vehicula erat. In quis iaculis ligula, sit amet vestibulum eros. Pellentesque fermentum pharetra diam ut porttitor. Maecenas sed dignissim eros. Maecenas tempor libero vel purus porttitor, sed tristique sapien scelerisque. Ut consequat libero vitae consequat maximus. Proin tortor tortor, porta et placerat quis, viverra eget justo. Sed malesuada porttitor nisl a venenatis. Phasellus varius tellus vitae tellus cursus, at ultrices tellus sollicitudin. Sed at semper ipsum, tristique pharetra libero. Praesent blandit ex nec fermentum malesuada. Nulla quis ipsum sagittis orci aliquam lacinia sit amet sit amet lectus. Maecenas ultricies orci sem, ac tempor dolor laoreet a. Donec quis sollicitudin nibh, quis efficitur nulla. Mauris volutpat ex quis fringilla placerat.
                "/>
            </div>
        </div>
    }
}

#[component]
fn ProjectCanvas(
    text: &'static str,
) -> impl IntoView {
    view! {
        <div 
            class="
                transition-all
                duration-300
                ease-in-out
                transform
                hover:grow
                bg-white
                shrink
                rounded-lg
                w-2/3
                my-4
            "
        >
            <div class="z-10 text-lg">{text}</div>
        </div>
    }
}

#[component]
fn MorePage() -> impl IntoView {
    view! {
        <PageTitle title="Here is More...of Me!"/>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <PageTitle title="Error 404: Not Found"/>
    }
}

#[component]
fn PageTitle(
    title: &'static str,
) -> impl IntoView {
    view! {
        <div
            class="
                text-center
                text-4xl
                text-slate-600
                font-serif
                pt-4
                pb-8
            "
        >
                {title}
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App)
}
