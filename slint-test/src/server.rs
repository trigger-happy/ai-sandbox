use axum::{
    routing::get,
    Router,
    response::Html,
};
use tower_http::services::ServeDir;

async fn handler() -> Html<&'static str> {
    Html(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="UTF-8">
            <title>Forgejo – Beyond coding. We forge.</title>
            <style>
                * {
                    margin: 0;
                    padding: 0;
                    box-sizing: border-box;
                }
                body {
                    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
                    margin: 0;
                    padding: 0;
                    background: #1a1a2e;
                    min-height: 100vh;
                    display: flex;
                    flex-direction: column;
                }
                .app-container {
                    flex: 1;
                    display: flex;
                    justify-content: center;
                    align-items: flex-start;
                }
                #slint-app {
                    width: 100%;
                    height: 100vh;
                    overflow: hidden;
                }
            </style>
        </head>
        <body>
            <div class="app-container">
                <div id="slint-app">
                    <canvas id="canvas"></canvas>
                </div>
            </div>

            <script type="module">
                import init from '/pkg/slint_test.js';

                async function run() {
                    await init();
                }

                run().catch(console.error);
            </script>
        </body>
        </html>
    "#)
}

async fn health() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    println!("Starting Forgejo UI Demo Server...");

    let app = Router::new()
        .route("/", get(handler))
        .route("/health", get(health))
        .nest_service("/pkg", ServeDir::new("pkg"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to port 3000");

    println!("Web server started at http://127.0.0.1:3000");
    println!("Health check available at http://127.0.0.1:3000/health");
    println!("Open http://127.0.0.1:3000 in your browser to see the Forgejo UI");
    println!("\nPress Ctrl+C to stop");

    axum::serve(listener, app).await.unwrap();
}
