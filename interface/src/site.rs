use axum::{routing::post, Router};
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <link rel="stylesheet" href="../../style.css" />
        // Top navigation
        <div class="topnav">

            // Centered link
            <div class="topnav-centered">
                <a href="#home" class="active">Home</a>
            </div>

                // Left-aligned links (default)
                <a href="#news">News</a>
                <a href="#contact">Contact</a>

            // Right-aligned links
            <div class="topnav-right">
                <a href="#search">Search</a>
                <a href="#about">About</a>
            </div>

        </div>
        <p> "Hello World" </p>
        <h1> "Hello World" </h1>
    }
}

pub async fn start_server() {
    let conf = get_configuration(Some("./interface/Cargo.toml"))
        .await
        .unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, |cx| view! { cx, <App/> })
        .with_state(leptos_options);

    log!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
