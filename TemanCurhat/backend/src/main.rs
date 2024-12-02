use ic_cdk::api::{call, trap};
use ic_cdk::export::candid::CandidType;
use ic_cdk::export::Principal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(CandidType, Serialize, Deserialize, Debug)]
struct User {
    id: Principal,
    name: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
struct Message {
    sender: Principal,
    content: String,
}

struct ChatApp {
    users: HashMap<Principal, User>,
    chats: HashMap<Principal, Vec<Message>>, // Percakapan berdasarkan pengguna
}

impl ChatApp {
    fn new() -> Self {
        ChatApp {
            users: HashMap::new(),
            chats: HashMap::new(),
        }
    }

    fn register_user(&mut self, name: String) -> Principal {
        let user_id = Principal::self_authenticating();
        let user = User { id: user_id, name };
        self.users.insert(user_id.clone(), user);
        user_id
    }

    fn send_message(&mut self, user_id: Principal, content: String) {
        let message = Message {
            sender: user_id,
            content,
        };

        self.chats.entry(user_id).or_insert(Vec::new()).push(message);
    }

    fn get_messages(&self, user_id: Principal) -> Option<&Vec<Message>> {
        self.chats.get(&user_id)
    }

    fn find_random_user(&self, current_user_id: Principal) -> Option<Principal> {
        let other_users: Vec<Principal> = self.users.keys().cloned().collect();
        let random_user = other_users
            .into_iter()
            .filter(|&user_id| user_id != current_user_id)
            .next();

        random_user
    }
}

#[ic_cdk::query]
fn get_messages(user_id: Principal) -> Option<Vec<Message>> {
    let app = get_app();
    app.get_messages(user_id).cloned()
}

#[ic_cdk::update]
fn register_user(name: String) -> Principal {
    let mut app = get_app();
    app.register_user(name)
}

#[ic_cdk::update]
fn send_message(user_id: Principal, content: String) {
    let mut app = get_app();
    app.send_message(user_id, content)
}

#[ic_cdk::update]
fn find_random_user(current_user_id: Principal) -> Option<Principal> {
    let app = get_app();
    app.find_random_user(current_user_id)
}

// Helper function to get app state
fn get_app() -> &'static mut ChatApp {
    ic_cdk::storage::get_mut::<ChatApp>().unwrap()
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    ic_cdk::storage::stable_save((get_app(),)).unwrap();
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    let (app,): (ChatApp,) = ic_cdk::storage::stable_restore().unwrap();
    ic_cdk::storage::set(app);
}
