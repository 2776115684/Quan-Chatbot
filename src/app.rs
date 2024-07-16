use std::cell::RefCell;
use std::rc::Rc;

use futures::stream::SplitSink;
use leptos::*;
use leptos_meta::*;

mod components;
use components::chat_area::ChatArea;
use components::type_area::TypeArea;

use crate::model::conversation::{Conversation, Message};

#[component]
pub fn App() -> impl IntoView {
    // 提供管理样式表、标题、meta标签等的上下文
    provide_meta_context();

    // 允许任何组件通过上下文获取深色模式状态
    let (dark_mode, _) = create_signal(true); // 设置深色模式状态的信号
    provide_context(dark_mode);

    // 创建一个会话状态和设置会话状态的函数
    let (conversation, set_conversation) = create_signal(Conversation::new());

    use futures::{SinkExt, StreamExt};
    use gloo_net::websocket::futures::WebSocket;
    use gloo_net::websocket::Message::Text as Txt;
    let client: Rc<RefCell<Option<SplitSink<WebSocket, gloo_net::websocket::Message>>>> =
        Default::default();

    // 克隆客户端引用，用于在闭包中使用
    let client_clone_baby = client.clone();
    create_effect(move |_| {
        // 获取当前窗口的位置对象
        let location = web_sys::window().unwrap().location();
        // 获取主机名
        let hostname = location
            .hostname()
            .expect("failed to retrieve origin hostname");
        // 构建 WebSocket URL
        let ws_url = format!("ws://{hostname}:3000/ws");

        // 打开 WebSocket 连接
        let connection = WebSocket::open(&format!("{ws_url}"))
            .expect("failed to establish WebSocket connection");

        // 分割 WebSocket 连接为发送器和接收器
        let (sender, mut recv) = connection.split();
        spawn_local(async move {
            // 异步接收消息
            while let Some(msg) = recv.next().await {
                match msg {
                    Ok(Txt(msg)) => {
                        // 更新会话状态，附加收到的消息
                        set_conversation.update(move |c| {
                            c.messages.last_mut().unwrap().text.push_str(&msg);
                        });
                    }
                    _ => {
                        break;
                    }
                }
            }
        });

        // 将发送器保存到客户端引用中
        *client_clone_baby.borrow_mut() = Some(sender);
    });

    // 定义发送消息的动作
    let send = create_action(move |new_message: &String| {
        // 创建用户消息
        let user_message = Message {
            text: new_message.clone(),
            user: true,
        };
        // 更新会话状态，添加新的用户消息
        set_conversation.update(move |c| {
            c.messages.push(user_message);
        });

        // 克隆客户端引用并发送消息
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

    // 创建效果，当发送动作的输入发生变化时，添加一个空的 AI 消息
    create_effect(move |_| {
        if let Some(_) = send.input().get() {
            let model_message = Message {
                text: "...".to_string(), // 等待 AI 响应时显示 "..."
                user: false,
            };

            set_conversation.update(move |c| {
                c.messages.push(model_message);
            });
        }
    });

    // 渲染视图
    view! {
        <Stylesheet id="leptos" href="/pkg/quan-chatbot.css"/>

        <Title text="Quan-Chatbot"/>
        <ChatArea conversation/>
        <TypeArea send/>
    }
}
