use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

struct Model {
    user_name: String,
    messages: Vec<String>,
    new_message: String,
    user_id: Option<String>,
}

enum Msg {
    RegisterUser,
    SendMessage,
    ReceiveMessage(String),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub user_id: Option<String>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            user_name: String::new(),
            messages: vec![],
            new_message: String::new(),
            user_id: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RegisterUser => {
                // Panggil backend untuk mendaftar pengguna
                let user_id = format!("{}", uuid::Uuid::new_v4());
                self.user_id = Some(user_id);
            }
            Msg::SendMessage => {
                if let Some(user_id) = &self.user_id {
                    // Panggil backend untuk mengirim pesan
                    self.messages.push(self.new_message.clone());
                    self.new_message.clear();
                }
            }
            Msg::ReceiveMessage(msg) => {
                self.messages.push(msg);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ "Teman Curhat" }</h1>
                {
                    if self.user_id.is_none() {
                        html! {
                            <div>
                                <input
                                    type="text"
                                    placeholder="Masukkan nama"
                                    oninput={ctx.link().callback(|e: InputData| Msg::RegisterUser)}
                                />
                                <button onclick={ctx.link().callback(|_| Msg::RegisterUser)}>{ "Daftar" }</button>
                            </div>
                        }
                    } else {
                        html! {
                            <div>
                                <textarea
                                    placeholder="Tulis pesan..."
                                    value={self.new_message.clone()}
                                    oninput={ctx.link().callback(|e: InputData| Msg::ReceiveMessage(e.value))}
                                />
                                <button onclick={ctx.link().callback(|_| Msg::SendMessage)}>{ "Kirim Pesan" }</button>
                                <div>
                                    <h3>{ "Pesan-pesan Anda:" }</h3>
                                    <ul>
                                        { for self.messages.iter().map(|msg| html!{ <li>{ msg }</li> }) }
                                    </ul>
                                </div>
                            </div>
                        }
                    }
                }
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
