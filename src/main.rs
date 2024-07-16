use cfg_if::cfg_if;

use api::ws;
pub mod api;
pub mod model;

// 配置 SSR 环境的 main 函数
#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 设置调试日志级别，仅在生产环境中禁用
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    use actix_files::Files;
    use actix_web::*;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use quan_chatbot_lib::app::*;

    let conf = get_configuration(None).await.unwrap(); // 获取配置
    let addr = conf.leptos_options.site_addr; // 获取站点地址
                                              // Generate the list of routes in your Leptos App
    let routes = generate_route_list(|| view! { <App/> });

    // 静态 CSS 文件服务
    #[get("/style.css")]
    async fn css() -> impl Responder {
        actix_files::NamedFile::open_async("./style/output.css").await
    }

    // 获取语言模型，并将其包装在 Actix Web 的 Data 中以便共享
    let model = web::Data::new(get_language_model());
    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .app_data(model.clone())
            .service(css)
            .route("/ws", web::get().to(ws))
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .leptos_routes(
                leptos_options.to_owned(),
                routes.to_owned(),
                || view! { <App/> },
            )
            .service(Files::new("/", site_root))
    })
    .bind(&addr)?
    .run()
    .await
}

// 使用 cfg_if 宏，根据不同的编译配置导入和使用不同的代码
cfg_if! {
    if #[cfg(feature = "ssr")] {
        use llm::models::Llama;
        use actix_web::*;
        use std::env;
        use dotenv::dotenv;

        // 获取语言模型的函数
        fn get_language_model() -> Llama {
            use std::path::PathBuf;
            dotenv().ok();
            let model_path = env::var("MODEL_PATH").expect("MODEL_PATH must be set"); // 获取模型路径
            let model_parameters = llm::ModelParameters {
                prefer_mmap: true,
                context_size: 2048,
                lora_adapters: None,
                use_gpu: true,
                gpu_layers: None,
                rope_overrides: None,
                n_gqa: None,
            };

            llm::load::<Llama>(
                &PathBuf::from(&model_path),
                llm::TokenizerSource::Embedded,
                model_parameters,
                llm::load_progress_callback_stdout,
            )
            .unwrap_or_else(|err| {
                panic!("Failed to load model from {model_path:?}: {err}")
            })
        }
    }
}

// 当既不启用 SSR 也不启用 CSR 时的 main 函数
#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // 客户端 main 函数，仅用于纯客户端测试
}

// 仅启用 CSR 时的 main 函数
#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // 客户端 main 函数，通常用于 `trunk serve`
    use leptos::*;
    use quan_chatbot_lib::app::*;
    use wasm_bindgen::prelude::wasm_bindgen;

    console_error_panic_hook::set_once(); // 设置错误处理

    leptos::mount_to_body(move || {
        // 挂载 Leptos 组件
        view! {<App/> }
    });
}
