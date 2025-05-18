use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div
            style="
                background-color: #fefefe;
                background-image:
                    radial-gradient(circle at 15% 20%, rgba(255, 0, 85, 0.6) 0%, transparent 40%),
                    radial-gradient(circle at 85% 25%, rgba(0, 200, 255, 0.6) 0%, transparent 40%),
                    radial-gradient(circle at 40% 80%, rgba(255, 255, 0, 0.6) 0%, transparent 40%),
                    radial-gradient(circle at 70% 70%, rgba(0, 255, 150, 0.6) 0%, transparent 40%),
                    radial-gradient(circle at 30% 50%, rgba(128, 0, 255, 0.5) 0%, transparent 40%);
                background-size: cover;
                background-repeat: no-repeat;
            "
            class="w-screen min-h-screen flex items-center justify-center"
        >
            <div class="bg-gray-900 bg-opacity-90 shadow-2xl rounded-xl p-10 flex flex-col items-center space-y-6 border-4 border-double border-white">
                <h1 class="text-4xl font-extrabold text-white tracking-widest drop-shadow-lg">
                    {"🎨 Login 🎨"}
                </h1>
                <form class="flex rounded-lg shadow-lg overflow-hidden">
                    <input
                        {oninput}
                        class="p-4 w-64 text-gray-100 placeholder-gray-400 font-medium focus:outline-none bg-gray-800 border-2 border-pink-300 focus:ring-2 focus:ring-yellow-400"
                        placeholder="Masukkan Username"
                    />
                    <Link<Route> to={Route::Chat}>
                        <button
                            {onclick}
                            disabled={username.len() < 1}
                            class="px-6 py-4 font-bold uppercase transition-all duration-300 ease-in-out bg-gradient-to-br from-orange-500 to-pink-600 text-white hover:from-green-400 hover:to-yellow-400 disabled:opacity-50"
                        >
                            {"Go Chatting!"}
                        </button>
                    </Link<Route>>
                </form>
            </div>
        </div>
    }       
}