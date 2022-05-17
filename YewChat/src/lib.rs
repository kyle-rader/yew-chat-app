use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
use components::chat::Chat;
use components::login::Login;

mod services;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Login,

    #[at("/chat")]
    Chat,

    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(selected_route: &Route) -> Html {
    match selected_route {
        Route::Login => html! { <Login /> },
        Route::Chat => html! { <Chat /> },
        Route::NotFound => html! { <h1>{"404 Not Found"}</h1> },
    }
}

#[function_component(Main)]
fn main() -> Html {
    let ctx = use_state(|| {
        Rc::new(UserInner {
            username: RefCell::new("".into()),
        })
    });

    html! {
        <ContextProvider<User> context={(*ctx).clone()}>
        <BrowserRouter>
            <div class="flex w-screen h-screen">
                <Switch<Route> render={Switch::render(switch)}/>
            </div>
        </BrowserRouter>
        </ContextProvider<User>>
    }
}

pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Main>();
    Ok(())
}

pub type User = Rc<UserInner>;

#[derive(Debug, PartialEq)]
pub struct UserInner {
    pub username: RefCell<String>,
}
