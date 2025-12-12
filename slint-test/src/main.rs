slint::include_modules!();

use axum::{
    routing::get,
    Router,
    response::Html,
};
use tokio::runtime::Runtime;

async fn handler() -> Html<&'static str> {
    Html(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Slint Test Server</title>
            <style>
                body {
                    font-family: Arial, sans-serif;
                    max-width: 800px;
                    margin: 50px auto;
                    padding: 20px;
                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                    color: white;
                }
                .container {
                    background: rgba(255, 255, 255, 0.1);
                    padding: 30px;
                    border-radius: 10px;
                    backdrop-filter: blur(10px);
                }
                h1 {
                    margin-top: 0;
                }
                .status {
                    background: rgba(46, 204, 113, 0.3);
                    padding: 15px;
                    border-radius: 5px;
                    margin: 20px 0;
                }
                ul {
                    line-height: 1.8;
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>🚀 Slint Test Web Server</h1>
                <div class="status">
                    <strong>✓ Server Status:</strong> Running on port 3000
                </div>
                <h2>About This Application</h2>
                <p>This is a Rust application that combines:</p>
                <ul>
                    <li><strong>Slint UI</strong> - A native desktop UI framework</li>
                    <li><strong>Axum</strong> - A modern web server framework</li>
                    <li><strong>Tokio</strong> - An asynchronous runtime</li>
                </ul>
                <h2>UI Components</h2>
                <p>The Slint desktop UI includes various components for testing:</p>
                <ul>
                    <li>Buttons with click handlers</li>
                    <li>Text input fields (LineEdit)</li>
                    <li>CheckBoxes with state binding</li>
                    <li>Sliders with value display</li>
                    <li>SpinBox for numeric input</li>
                    <li>ComboBox with multiple options</li>
                    <li>Colored rectangles and visual elements</li>
                    <li>TabWidget for component organization</li>
                    <li>Progress indicators</li>
                </ul>
                <p>Check the desktop window to interact with all components!</p>
            </div>
        </body>
        </html>
    "#)
}

async fn health() -> &'static str {
    "OK"
}

fn start_web_server() {
    // Create a new Tokio runtime for the web server
    let rt = Runtime::new().unwrap();

    std::thread::spawn(move || {
        rt.block_on(async {
            let app = Router::new()
                .route("/", get(handler))
                .route("/health", get(health));

            let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
                .await
                .unwrap();

            println!("🌐 Web server started at http://127.0.0.1:3000");
            println!("📊 Health check available at http://127.0.0.1:3000/health");

            axum::serve(listener, app).await.unwrap();
        });
    });
}

fn main() -> Result<(), slint::PlatformError> {
    println!("🚀 Starting Slint Test Application...");

    // Start the web server in a background thread
    start_web_server();

    // Give the server a moment to start
    std::thread::sleep(std::time::Duration::from_millis(500));

    println!("🎨 Launching Slint UI...");

    // Create and show the Slint UI
    let ui = AppWindow::new()?;

    // Set initial values
    ui.set_status_message("Application initialized successfully!".into());

    println!("✓ Application ready!");
    println!("  - Desktop UI is now visible");
    println!("  - Web server is running at http://127.0.0.1:3000");

    // Run the UI (this blocks until the window is closed)
    ui.run()
}
