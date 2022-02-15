use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::{history::History, hooks::use_history, Routable};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Root,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Root)]
fn root() -> Html {
    let history = use_history().unwrap();
    let onclick = Callback::once(move |_| history.push(Route::NotFound));
    html! {
        <div>
            <button {onclick}>{"Go to 404"}</button>
        </div>
    }
}

enum Msg {
    AddOne,
    MinusOne,
    Twice,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            Msg::MinusOne => {
                self.value -= 1;
                true
            }
            Msg::Twice => {
                self.value *= 2;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <button onclick={link.callback(|_| Msg::MinusOne)}>{ "-1" }</button>
                <button onclick={link.callback(|_| Msg::Twice)}>{ "x2" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Root => html! {<Root/>},
        &Route::NotFound => html! {
            <div>{ "not found" }</div>
        },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}
