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
            <title>Forgejo - Beyond coding. We forge.</title>
            <style>
                * {
                    margin: 0;
                    padding: 0;
                    box-sizing: border-box;
                }
                html, body {
                    margin: 0;
                    padding: 0;
                    width: 100%;
                    height: 100%;
                    overflow: hidden;
                    background: #161B22;
                }
                canvas {
                    display: block;
                    width: 100%;
                    height: 100%;
                }
            </style>
        </head>
        <body>
            <canvas id="canvas"></canvas>
            <script>
                // Set canvas size immediately before module loads
                (function() {
                    const canvas = document.getElementById('canvas');
                    canvas.width = window.innerWidth;
                    canvas.height = window.innerHeight;
                    console.log('Canvas set to:', canvas.width, 'x', canvas.height);
                })();
            </script>
            <script type="module">
                import init from '/pkg/slint_test.js';
                init().catch(console.error);
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
