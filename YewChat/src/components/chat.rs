use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::{User, services::websocket::WebsocketService};
use crate::services::event_bus::EventBus;

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    wss: WebsocketService,
    messages: Vec<MessageData>,
    _producer: Box<dyn Bridge<EventBus>>,
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                )
                                .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    //log::debug!("got input: {:?}", input.value());
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);
        html! {
            <div
                style="
                    background-color: #fefefe;
                    background-image:
                        radial-gradient(circle at 15% 20%, rgba(255, 0, 85, 0.5) 0%, transparent 40%),
                        radial-gradient(circle at 85% 25%, rgba(0, 200, 255, 0.5) 0%, transparent 40%),
                        radial-gradient(circle at 40% 80%, rgba(255, 255, 0, 0.5) 0%, transparent 40%),
                        radial-gradient(circle at 70% 70%, rgba(0, 255, 150, 0.5) 0%, transparent 40%),
                        radial-gradient(circle at 30% 50%, rgba(128, 0, 255, 0.5) 0%, transparent 40%);
                    background-size: cover;
                    background-repeat: no-repeat;
                "
                class="flex w-screen h-screen text-white"
            >
                // Sidebar user
                <div class="flex-none w-60 h-full bg-gray-900 bg-opacity-80 p-4 overflow-y-auto">
                    <div class="text-2xl font-bold mb-4 border-b pb-2">{"🧑‍🤝‍🧑 Users"}</div>
                    {
                        self.users.iter().map(|u| {
                            html!{
                                <div class="flex items-center bg-gray-800 rounded-lg p-2 mb-3 shadow-md">
                                    <img class="w-10 h-10 rounded-full mr-3" src={u.avatar.clone()} alt="avatar"/>
                                    <div class="text-sm">{ &u.name }</div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
    
                // Chat area
                <div class="grow flex flex-col h-full">
                    <div class="w-full h-16 flex items-center px-6 border-b border-white/20 bg-gray-900 bg-opacity-80 text-lg font-semibold">
                        {"💬 WebChat!"}
                    </div>
    
                    // Message bubbles
                    <div class="flex-grow p-6 overflow-y-auto space-y-4">
                        {
                            self.messages.iter().map(|m| {
                                let user = self.users.iter().find(|u| u.name == m.from).unwrap();
                                html! {
                                    <div class="flex items-start space-x-3 bg-white bg-opacity-20 p-4 rounded-2xl w-fit max-w-[60%] backdrop-blur-md text-white shadow-lg">
                                        <img class="w-8 h-8 rounded-full" src={user.avatar.clone()} />
                                        <div>
                                            <div class="font-semibold text-sm">{ &m.from }</div>
                                            <div class="text-sm">
                                                {
                                                    if m.message.ends_with(".gif") {
                                                        html! { <img class="mt-2 rounded-md" src={m.message.clone()} /> }
                                                    } else {
                                                        html! { <span>{ &m.message }</span> }
                                                    }
                                                }
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
    
                    // Input
                    <div class="w-full h-16 flex items-center px-4 bg-gray-900 bg-opacity-90 border-t border-white/10">
                        <input
                            ref={self.chat_input.clone()}
                            type="text"
                            placeholder="Tulis pesanmu..."
                            class="flex-grow p-3 rounded-full bg-white bg-opacity-20 text-white placeholder-gray-300 focus:outline-none focus:ring-2 focus:ring-pink-400 backdrop-blur"
                            required=true
                        />
                        <button onclick={submit} class="ml-3 p-3 bg-gradient-to-br from-pink-500 to-orange-500 hover:from-green-400 hover:to-yellow-300 rounded-full shadow-lg">
                            <svg fill="none" stroke="white" stroke-width="2" viewBox="0 0 24 24" class="w-5 h-5">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M3 10l9 4 9-4-9-4-9 4z" />
                                <path stroke-linecap="round" stroke-linejoin="round" d="M3 10v6l9 4 9-4v-6" />
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        }
    }    
}