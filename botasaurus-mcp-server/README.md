# Botasaurus MCP Server

An MCP (Model Context Protocol) server that provides powerful web scraping capabilities using the [Botasaurus](https://github.com/omkarcloud/botasaurus) framework. Designed to bypass anti-bot measures and fetch web content reliably, even from protected sites.

## Features

- **Undefeatable Scraping**: Bypass Cloudflare, DataDome, and other bot detection systems
- **Multiple Fetching Methods**: Choose between browser automation or fast HTTP requests
- **Google Referrer Support**: Access pages via Google for better success rates
- **Text Extraction**: Extract specific elements using CSS selectors
- **Parallel Scraping**: Fetch multiple pages concurrently (free tier optimized)
- **Free Tier Optimized**: All features work on free tier with sensible defaults

## Tools Available

### 1. `fetch_page`
Fetch web pages using full browser automation. Best for protected sites.

**Parameters:**
- `url` (required): The URL to fetch
- `bypass_cloudflare` (optional): Enable Cloudflare bypass mode
- `wait_for_selector` (optional): CSS selector to wait for before capturing
- `timeout` (optional): Maximum wait time in seconds (default: 30)

**Example:**
```
fetch_page("https://example.com", bypass_cloudflare=true)
```

### 2. `fetch_page_fast`
Fetch web pages using HTTP requests. Much faster but won't bypass anti-bot measures.

**Parameters:**
- `url` (required): The URL to fetch
- `user_agent` (optional): Custom user agent string

**Example:**
```
fetch_page_fast("https://example.com")
```

### 3. `google_fetch`
Fetch web pages via Google referrer for improved success rates.

**Parameters:**
- `url` (required): The URL to fetch
- `timeout` (optional): Maximum wait time in seconds (default: 30)

**Example:**
```
google_fetch("https://news-site.com/article")
```

### 4. `extract_text`
Extract text content from specific elements on a page.

**Parameters:**
- `url` (required): The URL to fetch
- `selector` (required): CSS selector for the element
- `bypass_cloudflare` (optional): Enable Cloudflare bypass mode
- `timeout` (optional): Maximum wait time in seconds (default: 30)

**Example:**
```
extract_text("https://example.com", "h1")
```

### 5. `fetch_multiple_pages`
Fetch multiple web pages in parallel.

**Parameters:**
- `urls` (required): List of URLs to fetch
- `bypass_cloudflare` (optional): Enable Cloudflare bypass for all requests
- `max_parallel` (optional): Maximum parallel requests (default: 3, max: 5)

**Example:**
```
fetch_multiple_pages(["https://example.com", "https://example.org"])
```

## Installation

### Prerequisites
- Python 3.10 or higher
- pip

### Install Dependencies

```bash
cd botasaurus-mcp-server
pip install -r requirements.txt
```

### For Development

```bash
pip install -e .
```

## Usage

### With Claude Desktop

Add this configuration to your Claude Desktop config file:

**MacOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
**Windows**: `%APPDATA%/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "botasaurus": {
      "command": "python",
      "args": [
        "/absolute/path/to/botasaurus-mcp-server/src/botasaurus_server.py"
      ]
    }
  }
}
```

### With Other MCP Clients

The server uses stdio transport and follows the MCP specification. Run it directly:

```bash
python src/botasaurus_server.py
```

## Use Cases

### Scrape Protected Sites
```
Use fetch_page to get content from Cloudflare-protected sites:
fetch_page("https://protected-site.com", bypass_cloudflare=true)
```

### Fast Public Data Collection
```
Use fetch_page_fast for unprotected sites to save resources:
fetch_page_fast("https://api.example.com/public-data")
```

### Extract Specific Content
```
Get article titles or specific elements:
extract_text("https://blog.com/post", "h1.title")
```

### Bulk Scraping
```
Fetch multiple pages efficiently:
fetch_multiple_pages([
  "https://example.com/page1",
  "https://example.com/page2",
  "https://example.com/page3"
])
```

## Free Tier Optimizations

This server is optimized for free tier usage:

- **Image Blocking**: Images are blocked by default to reduce bandwidth
- **No Driver Reuse**: Prevents memory accumulation issues
- **Limited Parallelism**: Default max 3 parallel requests, hard cap at 5
- **Headless Mode**: All browsers run headless to save resources
- **Auto-cleanup**: Drivers close automatically on crash

## Troubleshooting

### Browser Not Found
Botasaurus will automatically download the required browser on first run. Ensure you have sufficient disk space (~300MB).

### Slow Performance
- Use `fetch_page_fast` for unprotected sites
- Reduce `max_parallel` for batch operations
- Consider increasing `timeout` for slow-loading pages

### Cloudflare Bypass Not Working
- Ensure `bypass_cloudflare=True` is set
- Some sites may require additional wait time
- Try using `google_fetch` as an alternative

## Architecture

```
botasaurus-mcp-server/
├── src/
│   ├── __init__.py
│   └── botasaurus_server.py    # Main MCP server implementation
├── requirements.txt              # Python dependencies
├── pyproject.toml               # Package configuration
└── README.md                    # This file
```

## Credits

- Built on [Botasaurus](https://github.com/omkarcloud/botasaurus) by Omkar Cloud
- Uses [MCP Python SDK](https://github.com/modelcontextprotocol/python-sdk)
- Developed for the Model Context Protocol ecosystem

## License

This MCP server implementation is provided as-is for use with the Botasaurus library. Please refer to Botasaurus's license for usage terms of the underlying scraping framework.

## Contributing

Contributions are welcome! Feel free to:
- Report bugs
- Suggest new features
- Submit pull requests
- Improve documentation

## Resources

- [Model Context Protocol Documentation](https://modelcontextprotocol.io/)
- [Botasaurus GitHub](https://github.com/omkarcloud/botasaurus)
- [MCP Python SDK](https://github.com/modelcontextprotocol/python-sdk)
