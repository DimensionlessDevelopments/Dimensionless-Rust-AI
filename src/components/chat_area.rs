// Dimensionless Developments Rust Ai

use leptos::{*, html::Div};

use crate::model::conversation::Conversation;

const USER_MESSAGE_STYLE: &str = "px-3 py-2 bg-orange-500 rounded text-white";
const MODEL_MESSAGE_STYLE: &str = "px-3.5 py-2 bg-blue-100 rounded text-gray-900";

const CHAT_AREA_CLASS: &str = "h-full w-full flex flex-col overflow-y-auto p-5";
const CHAT_AREA_LIGHT_MODE_COLORS: &str = "bg-gray-50";
const CHAT_AREA_DARK_MODE_COLORS: &str = "bg-gradient-to-br from-gray-900 via-gray-800 to-gray-900";

#[component]
pub fn ChatArea(conversation: ReadSignal<Conversation>) -> impl IntoView {
    let dark_mode = use_context::<ReadSignal<bool>>()
        .expect("should be able to get dark mode state");

    let chat_div_ref = create_node_ref::<Div>();
    
    // Track message count to trigger scroll without reading full conversation
    let message_count = create_memo(move |_| conversation.get().messages.len());
    
    create_effect(move |_| {
        let _ = message_count.get();
        if let Some(div) = chat_div_ref.get() {
            div.set_scroll_top(div.scroll_height());
        }
    });

    let chat_area_class = Signal::derive(move || {
        if dark_mode.get() {
            format!("{CHAT_AREA_CLASS} {CHAT_AREA_DARK_MODE_COLORS}")
        } else {
            format!("{CHAT_AREA_CLASS} {CHAT_AREA_LIGHT_MODE_COLORS}")
        }
    });

    view! {
        <div class={chat_area_class.get()} node_ref=chat_div_ref>
            <div class="w-full pb-40">
                {move || {
                    let messages = conversation.get().messages;
                    messages.iter().map(|message| {
                        if message.user {
                            view! {
                                <div class="flex gap-2.5 justify-end mb-4">
                                    <div class="grid">
                                        <div class="text-right text-gray-400 text-sm font-semibold leading-snug pb-1">
                                            {&message.sender_name}
                                        </div>
                                        <div class={USER_MESSAGE_STYLE}>
                                            <h2 class="text-white text-sm font-normal leading-snug">
                                                {&message.text}
                                            </h2>
                                        </div>
                                        <div class="text-right text-gray-500 text-xs font-normal leading-4 py-1">
                                            {&message.timestamp}
                                        </div>
                                    </div>
                                </div>
                            }
                        } else {
                            // Split message text into paragraphs for better formatting
                            let text = message.text.clone(); // Clone to own the string
                            let paragraphs: Vec<String> = text.split("\n\n").map(|s| s.to_string()).collect();
                            
                            view! {
                                <div class="flex gap-2.5 mb-4">
                                    <img src="https://cdn-icons-png.freepik.com/512/1404/1404288.png" alt="assistant" class="w-10 h-10 rounded-full" />
                                    <div class="grid">
                                        <h5 class="text-gray-400 text-sm font-semibold leading-snug pb-1">
                                            {&message.sender_name}
                                        </h5>
                                        <div class={MODEL_MESSAGE_STYLE}>
                                            {paragraphs.iter().map(|para| {
                                                // Split by single newlines for line breaks within paragraphs
                                                let lines: Vec<String> = para.split('\n').map(|s| s.to_string()).collect();
                                                view! {
                                                    <div class="mb-3 last:mb-0">
                                                        {lines.iter().enumerate().map(|(i, line)| {
                                                            view! {
                                                                <>
                                                                    <span class="text-sm font-normal leading-snug">{line.clone()}</span>
                                                                    {if i < lines.len() - 1 {
                                                                        view! { <br /> }.into_view()
                                                                    } else {
                                                                        view! { <></> }.into_view()
                                                                    }}
                                                                </>
                                                            }
                                                        }).collect_view()}
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </div>
                                        <div class="text-gray-500 text-xs font-normal leading-4 py-1">
                                            {&message.timestamp}
                                        </div>
                                    </div>
                                </div>
                            }
                        }
                    }).collect_view()
                }}
            </div>
        </div>
    }
}