// Dimensionless Developments Rust Ai
// # Browser frontend UI

use std::cell::RefCell;
use std::rc::Rc;

use futures::stream::SplitSink;
use leptos::*;
use leptos_router::*;

use crate::components::chat_area::ChatArea;
use crate::components::type_area::TypeArea;
use crate::components::landing::Landing;
use crate::model::conversation::{Conversation, Message};

#[component]
pub fn App() -> impl IntoView {
    // Allow any component to get dark mode state via context
    let (dark_mode, _) = create_signal(true);
    provide_context(dark_mode);

    view! {
        <Router>
            <Routes>
                <Route path="/" view=Landing/>
                <Route path="/chat" view=Chat/>
            </Routes>
        </Router>
    }
}

#[component]
pub fn Chat() -> impl IntoView {
    let (conversation, set_conversation) = create_signal(Conversation::new());

    use gloo_net::websocket::futures::WebSocket;
    use gloo_net::websocket::Message::Text as Txt;
    use futures::{SinkExt, StreamExt};
    
    let client: Rc<RefCell<Option<SplitSink<WebSocket, gloo_net::websocket::Message>>>>
        = Default::default();

    let client_clone = client.clone();
    
    // Only run once on mount
    let _setup = create_resource(
        || (),
        move |_| {
            let client_clone = client_clone.clone();
            let set_conversation = set_conversation.clone();
            
            async move {
                let location = web_sys::window().unwrap().location();
                let hostname = location.hostname().expect("failed to retrieve origin hostname");
                let ws_url = format!("ws://{}:3000/ws", hostname);

                if let Ok(connection) = WebSocket::open(&ws_url) {
                    let (sender, mut recv) = connection.split();
                    
                    spawn_local(async move {
                        let mut buffer = String::new();
                        while let Some(msg) = recv.next().await {
                            match msg {
                                Ok(Txt(msg)) => {
                                    buffer.push_str(&msg);
                                    
                                    if buffer.len() >= 10 || buffer.contains('\n') {
                                        let text_to_add = buffer.clone();
                                        buffer.clear();
                                        
                                        set_conversation.update(move |c| {
                                            if let Some(last_msg) = c.messages.last_mut() {
                                                if !last_msg.user {
                                                    last_msg.text.push_str(&text_to_add);
                                                }
                                            }
                                        });
                                    }
                                }
                                _ => { 
                                    if !buffer.is_empty() {
                                        let text_to_add = buffer.clone();
                                        set_conversation.update(move |c| {
                                            if let Some(last_msg) = c.messages.last_mut() {
                                                if !last_msg.user {
                                                    last_msg.text.push_str(&text_to_add);
                                                }
                                            }
                                        });
                                    }
                                    break; 
                                }
                            }
                        }
                    });

                    *client_clone.borrow_mut() = Some(sender);
                }
            }
        },
    );

    let send = create_action(move |new_message: &String| {
        let user_message = Message {
            text: new_message.clone(),
            user: true,
            sender_name: "You".to_string(),
            timestamp: "now".to_string(),
        };
        set_conversation.update(move |c| {
            c.messages.push(user_message);
        });

        let client2 = client.clone();
        let msg = new_message.to_string();
        async move {
            client2
                .borrow_mut()
                .as_mut()
                .unwrap()
                .send(Txt(msg.to_string()))
                .await
                .map_err(|_| ServerFnError::ServerError("WebSocket issue".to_string()))
        }
    });

    create_effect(move |_| {
        if send.input().get().is_some() {
            let model_message = Message {
                text: String::new(),
                user: false,
                sender_name: "Assistant".to_string(),
                timestamp: "now".to_string(),
            };

            set_conversation.update(move |c| {
                c.messages.push(model_message);
            });
        }
    });

    view! {
        <div class="h-screen w-full flex flex-col overflow-hidden">
            <div class="flex-1 overflow-hidden">
                <ChatArea conversation/>
            </div>
            <TypeArea send/>
        </div>
    }
}