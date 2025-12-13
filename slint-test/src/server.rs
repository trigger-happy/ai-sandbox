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
            <title>Slint Test Application</title>
            <style>
                body {
                    font-family: Arial, sans-serif;
                    margin: 0;
                    padding: 0;
                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                    min-height: 100vh;
                    display: flex;
                    flex-direction: column;
                }
                .header {
                    background: rgba(255, 255, 255, 0.1);
                    padding: 20px;
                    text-align: center;
                    color: white;
                    backdrop-filter: blur(10px);
                }
                .header h1 {
                    margin: 0 0 10px 0;
                    font-size: 32px;
                }
                .header p {
                    margin: 5px 0;
                    opacity: 0.9;
                }
                .app-container {
                    flex: 1;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    padding: 20px;
                }
                #slint-app {
                    background: white;
                    border-radius: 10px;
                    box-shadow: 0 10px 40px rgba(0,0,0,0.3);
                    overflow: hidden;
                }
                .info {
                    background: rgba(255, 255, 255, 0.1);
                    color: white;
                    padding: 15px;
                    text-align: center;
                    backdrop-filter: blur(10px);
                }
            </style>
        </head>
        <body>
            <div class="header">
                <h1>🚀 Slint Test Application</h1>
                <p>Interactive Browser-Based UI Components</p>
            </div>

            <div class="app-container">
                <div id="slint-app">
                    <!-- Slint UI will be rendered here -->
                    <canvas id="canvas"></canvas>
                </div>
            </div>

            <div class="info">
                <p>✓ Web Server Running | Built with Slint, Rust, Axum & WebAssembly</p>
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
    println!("🚀 Starting Slint Test Web Server...");

    let app = Router::new()
        .route("/", get(handler))
        .route("/health", get(health))
        .nest_service("/pkg", ServeDir::new("pkg"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to port 3000");

    println!("✓ Web server started at http://127.0.0.1:3000");
    println!("📊 Health check available at http://127.0.0.1:3000/health");
    println!("🌐 Open http://127.0.0.1:3000 in your browser to see the Slint UI");
    println!("\nPress Ctrl+C to stop");

    axum::serve(listener, app).await.unwrap();
}
