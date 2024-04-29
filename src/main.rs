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
        <div class="relative">
            <div class="
                absolute inset-0 
                bg-gradient-to-r from-orange-400 via-red-600 to-blue-400
                blur-2xl
            "/>
            <div
                class="
                    relative
                    z-10
                    flex flex-row
                    justify-between
                    items-center
                    bg-black
                    text-slate-300
                    px-5 py-4
                    rounded-br-full
                    font-serif 
                "
            >
                <div class="text-4xl first-letter:text-5xl first-letter:font-bold">Victor Cornille</div>
                <div class="flex items-start mr-4">
                    <ButtonPage name="About me" path=""/>
                    <ButtonPage name="Projects" path="Projects"/>
                    <ButtonPage name="More..."  path="More"/>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Bottom() -> impl IntoView {
    view! {
        <div
            class="
                relative
            "
        >
            <div class="
                absolute inset-0 
                bg-gradient-to-r from-orange-400 via-red-600 to-blue-400
                blur-2xl
            "/>
            <div class="
                text-slate-200
                bg-black
                px-5 py-4
                grid grid-cols-2 grid-rows-2 grid-flow-col
                place-content-evenly
                rounded-t-lg
                relative
            ">
                <div>"1"</div>
                <div>"2"</div>
                <div class="row-span-2">"3"</div>
            </div>
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
                    text-lg
                    transition-all
                    duration-500
                    ease-in-out
                    hover:bg-gradient-to-r to-orange-400 via-red-600 from-blue-400
                    active:bg-gradient-to-l to-orange-400 via-red-600 from-blue-400
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
        <div class="min-h-screen"/>
    }
}

#[component]
fn ProjectsPage() -> impl IntoView {
    let lorem = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam scelerisque ipsum ut arcu hendrerit feugiat. Quisque in arcu laoreet, eleifend dui sed, vehicula erat. In quis iaculis ligula, sit amet vestibulum eros. Pellentesque fermentum pharetra diam ut porttitor. Maecenas sed dignissim eros. Maecenas tempor libero vel purus porttitor, sed tristique sapien scelerisque. Ut consequat libero vitae consequat maximus. Proin tortor tortor, porta et placerat quis, viverra eget justo. Sed malesuada porttitor nisl a venenatis. Phasellus varius tellus vitae tellus cursus, at ultrices tellus sollicitudin. Sed at semper ipsum, tristique pharetra libero. Praesent blandit ex nec fermentum malesuada. Nulla quis ipsum sagittis orci aliquam lacinia sit amet sit amet lectus. Maecenas ultricies orci sem, ac tempor dolor laoreet a. Donec quis sollicitudin nibh, quis efficitur nulla. Mauris volutpat ex quis fringilla placerat.";
    let title = "Title";
    view! {
        <PageTitle title="Welcome to Projects page"/>
        <div class="flex flex-col overflow-auto min-h-screen">
            <div class="flex justify-start">
                <ProjectCanvas text=lorem title/>
            </div>
            <div class="flex justify-end">
                <ProjectCanvas text=lorem title/>
            </div>
        </div>
    }
}

#[component]
fn ProjectCanvas(
    #[prop(optional)]
    text: &'static str,
    #[prop(optional)]
    title: &'static str,
) -> impl IntoView {
    view! {
        <div 
            class="
                transition-all
                duration-500
                delay-300
                ease-in-out
                transform
                bg-slate-900
                text-slate-200 text-justify text-lg 
                rounded-lg
                w-2/3
                my-4
            "
        >
            <div class="ml-4 mt-3 first-letter:font-bold first-letter:text-3xl">
                {title}
            </div>
            <div 
                class="
                    m-2 
                    overflow-hidden
                    transition-all
                    duration-300 
                    ease-in-out 
                    max-h-14 
                    hover:max-h-96
                    "
                >
                    {text}
            </div>
        </div>
    }
}

#[component]
fn MorePage() -> impl IntoView {
    view! {
        <PageTitle title="Here is More...of Me!"/>
        <div class="min-h-screen"/>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <PageTitle title="Error 404: Not Found"/>
        <div class="min-h-screen"/>
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
                text-slate-200
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
