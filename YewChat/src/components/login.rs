use yew::function_component;
use yew::html;
use yew::use_context;
use yew::use_state;
use yew::Callback;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found!");

    let on_input = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let on_click = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div class="bg-gray-800 flex w-screen">
            <div class="container mx-auto flex flex-col justify-center items-center">
                <form class="m-4 flex">
                    <input
                        {on_input}
                        class="rounded-1-lg p-4 border-t mr-0 border-b border-1 text-gray-800 border-gray-200 bg-white"
                        placeholder="User Name"
                        />
                    <Link<Route> to={Route::Chat}>
                        <button {on_click} disabled={username.len()<3} class="px-8 rounded-r-lg bg-violet-600 text-white font-bold p-4 uppercase border-violet-600 border-t border-b border-r">
                            {"Start Chatting!"}
                        </button>
                    </Link<Route>>
                </form>
            </div>
        </div>
    }
}
