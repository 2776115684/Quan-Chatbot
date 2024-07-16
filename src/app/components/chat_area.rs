use leptos::{html::Div, *};

use crate::model::conversation::Conversation;

// 定义不同颜色模式下的 CSS 类
const USER_MESSAGE_DARK_MODE_COLORS: &str = "bg-blue-500 text-white";
const USER_MESSAGE_LIGHT_MODE_COLORS: &str = "bg-blue-700 text-white";
const USER_MESSAGE_CLASS: &str = "max-w-md p-4 mb-5 rounded-lg self-end";

const MODEL_MESSAGE_LIGHT_MODE_COLORS: &str = "bg-gray-200 text-black";
const MODEL_MESSAGE_DARK_MODE_COLORS: &str = "bg-zinc-700 text-white";
const MODEL_MESSAGE_CLASS: &str = "max-w-md p-4 mb-5 rounded-lg self-start";

const CHAT_AREA_CLASS: &str = "h-screen pb-24 w-full flex flex-col overflow-y-auto p-5";
const CHAT_AREA_LIGHT_MODE_COLORS: &str = "border-gray-300 bg-gray-100";
const CHAT_AREA_DARK_MODE_COLORS: &str = "border-zinc-700 bg-zinc-900";

#[component]
pub fn ChatArea(conversation: ReadSignal<Conversation>) -> impl IntoView {
    // 从上下文中获取 dark_mode 状态
    let dark_mode =
        use_context::<ReadSignal<bool>>().expect("should be able to get dark mode state");

    // 根据 dark_mode 状态动态生成用户消息的 CSS 类
    let user_message_class = Signal::derive(move || {
        if dark_mode.get() {
            format!("{USER_MESSAGE_CLASS} {USER_MESSAGE_DARK_MODE_COLORS}")
        } else {
            format!("{USER_MESSAGE_CLASS} {USER_MESSAGE_LIGHT_MODE_COLORS}")
        }
    });

    // 根据 dark_mode 状态动态生成 AI 消息的 CSS 类
    let model_message_class = Signal::derive(move || {
        if dark_mode.get() {
            format!("{MODEL_MESSAGE_CLASS} {MODEL_MESSAGE_DARK_MODE_COLORS}")
        } else {
            format!("{MODEL_MESSAGE_CLASS} {MODEL_MESSAGE_LIGHT_MODE_COLORS}")
        }
    });

    // 根据 dark_mode 状态动态生成 AI 消息的 CSS 类
    let chat_div_ref = create_node_ref::<Div>();

    // 监视 conversation 状态的变化，并在对话更新时滚动到底部
    create_effect(move |_| {
        conversation.get();
        if let Some(div) = chat_div_ref.get() {
            div.set_scroll_top(div.scroll_height());
        }
    });

    // 根据 dark_mode 状态动态生成聊天区域的 CSS 类
    let chat_area_class = Signal::derive(move || {
        if dark_mode.get() {
            format!("{CHAT_AREA_CLASS} {CHAT_AREA_DARK_MODE_COLORS}")
        } else {
            format!("{CHAT_AREA_CLASS} {CHAT_AREA_LIGHT_MODE_COLORS}")
        }
    });

    // 使用 view! 宏来生成组件的视图
    view! {
          <div class={chat_area_class.get()} node_ref=chat_div_ref>
          {
            // 遍历 conversation 中的消息，动态生成每条消息的视图
            move || conversation.get().messages.iter().map(move |message| {
              let class_str = if message.user { user_message_class.get() } else { model_message_class.get()};
              view! {
                <div class={class_str}>
                  {message.text.clone()}
                </div>
              }
            }).collect::<Vec<_>>()
          }
        </div>
    }
}
