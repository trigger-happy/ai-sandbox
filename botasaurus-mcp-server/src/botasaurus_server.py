#!/usr/bin/env python3
"""
Botasaurus MCP Server

An MCP server that provides web scraping tools using the Botasaurus framework.
Designed to bypass anti-bot measures and fetch web content reliably.
"""

import asyncio
from typing import Optional
from mcp.server.fastmcp import FastMCP

# Initialize MCP server
mcp = FastMCP(
    name="Botasaurus Web Scraper",
    version="1.0.0"
)


@mcp.tool()
def fetch_page(
    url: str,
    bypass_cloudflare: bool = False,
    wait_for_selector: Optional[str] = None,
    timeout: int = 30
) -> str:
    """
    Fetch web page content using Botasaurus driver with full browser automation.

    This method uses a real browser to bypass anti-bot measures like Cloudflare,
    DataDome, and other bot detection systems. Best for protected pages but slower.

    Args:
        url: The URL to fetch
        bypass_cloudflare: Enable Cloudflare bypass mode (default: False)
        wait_for_selector: Optional CSS selector to wait for before capturing content
        timeout: Maximum time to wait in seconds (default: 30)

    Returns:
        The HTML content of the page

    Examples:
        - fetch_page("https://example.com")
        - fetch_page("https://protected-site.com", bypass_cloudflare=True)
        - fetch_page("https://example.com", wait_for_selector=".content")
    """
    from botasaurus.browser import browser, Driver

    @browser(
        close_on_crash=True,
        block_images=True,  # Faster loading on free tier
        reuse_driver=False,  # Avoid memory issues on free tier
        headless=True
    )
    def scrape_page(driver: Driver, data):
        url = data['url']
        bypass_cf = data.get('bypass_cloudflare', False)
        selector = data.get('wait_for_selector')
        timeout_val = data.get('timeout', 30)

        # Navigate to the page
        if bypass_cf:
            driver.get(url, bypass_cloudflare=True, wait=timeout_val)
        else:
            driver.get(url, wait=timeout_val)

        # Wait for specific selector if provided
        if selector:
            driver.wait_for_element(selector, timeout=timeout_val)

        # Get the page HTML
        html = driver.page_html
        return html

    # Execute the scraper
    result = scrape_page({
        'url': url,
        'bypass_cloudflare': bypass_cloudflare,
        'wait_for_selector': wait_for_selector,
        'timeout': timeout
    })

    return result


@mcp.tool()
def fetch_page_fast(
    url: str,
    user_agent: Optional[str] = None
) -> str:
    """
    Fetch web page content using HTTP requests (faster but less stealthy).

    This method uses humanized HTTP requests without a browser. Much faster
    and lighter on resources, but won't work on heavily protected sites.
    Best for simple pages without anti-bot protection.

    Args:
        url: The URL to fetch
        user_agent: Optional custom user agent string

    Returns:
        The HTML content of the page

    Examples:
        - fetch_page_fast("https://example.com")
        - fetch_page_fast("https://api.example.com/data")
    """
    from botasaurus.request import request, Request

    @request(
        close_on_crash=True
    )
    def scrape_with_request(request: Request, data):
        url = data['url']
        ua = data.get('user_agent')

        # Set custom user agent if provided
        if ua:
            response = request.get(url, headers={'User-Agent': ua})
        else:
            response = request.get(url)

        return response.text

    # Execute the request
    result = scrape_with_request({
        'url': url,
        'user_agent': user_agent
    })

    return result


@mcp.tool()
def google_fetch(
    url: str,
    timeout: int = 30
) -> str:
    """
    Fetch web page via Google referrer for better success rate.

    Visits the page through Google search, which can help bypass some
    bot detection systems that whitelist Google traffic. Uses browser automation.

    Args:
        url: The URL to fetch
        timeout: Maximum time to wait in seconds (default: 30)

    Returns:
        The HTML content of the page

    Examples:
        - google_fetch("https://example.com")
        - google_fetch("https://news-site.com/article", timeout=45)
    """
    from botasaurus.browser import browser, Driver

    @browser(
        close_on_crash=True,
        block_images=True,
        reuse_driver=False,
        headless=True
    )
    def scrape_via_google(driver: Driver, data):
        url = data['url']
        timeout_val = data.get('timeout', 30)

        # Navigate via Google
        driver.google_get(url, wait=timeout_val)

        # Get the page HTML
        html = driver.page_html
        return html

    # Execute the scraper
    result = scrape_via_google({
        'url': url,
        'timeout': timeout
    })

    return result


@mcp.tool()
def extract_text(
    url: str,
    selector: str,
    bypass_cloudflare: bool = False,
    timeout: int = 30
) -> str:
    """
    Extract text content from a specific element on a web page.

    Uses browser automation to navigate to a page and extract text from
    an element matching the provided CSS selector.

    Args:
        url: The URL to fetch
        selector: CSS selector for the element to extract text from
        bypass_cloudflare: Enable Cloudflare bypass mode (default: False)
        timeout: Maximum time to wait in seconds (default: 30)

    Returns:
        The text content of the matched element

    Examples:
        - extract_text("https://example.com", "h1")
        - extract_text("https://example.com", ".article-content", bypass_cloudflare=True)
    """
    from botasaurus.browser import browser, Driver

    @browser(
        close_on_crash=True,
        block_images=True,
        reuse_driver=False,
        headless=True
    )
    def extract_element_text(driver: Driver, data):
        url = data['url']
        sel = data['selector']
        bypass_cf = data.get('bypass_cloudflare', False)
        timeout_val = data.get('timeout', 30)

        # Navigate to the page
        if bypass_cf:
            driver.get(url, bypass_cloudflare=True, wait=timeout_val)
        else:
            driver.get(url, wait=timeout_val)

        # Wait for and extract text from element
        driver.wait_for_element(sel, timeout=timeout_val)
        text = driver.get_text(sel)

        return text

    # Execute the scraper
    result = extract_element_text({
        'url': url,
        'selector': selector,
        'bypass_cloudflare': bypass_cloudflare,
        'timeout': timeout
    })

    return result


@mcp.tool()
def fetch_multiple_pages(
    urls: list[str],
    bypass_cloudflare: bool = False,
    max_parallel: int = 3
) -> dict:
    """
    Fetch multiple web pages in parallel using browser automation.

    Limited to 3 parallel requests by default to avoid overwhelming
    free tier resources. Returns a dictionary mapping URLs to their content.

    Args:
        urls: List of URLs to fetch
        bypass_cloudflare: Enable Cloudflare bypass mode for all requests
        max_parallel: Maximum parallel requests (default: 3, max: 5 for free tier)

    Returns:
        Dictionary mapping each URL to its HTML content

    Examples:
        - fetch_multiple_pages(["https://example.com", "https://example.org"])
        - fetch_multiple_pages(["https://site1.com", "https://site2.com"], bypass_cloudflare=True)
    """
    from botasaurus.browser import browser, Driver

    # Limit parallel requests for free tier
    max_parallel = min(max_parallel, 5)

    @browser(
        close_on_crash=True,
        block_images=True,
        reuse_driver=False,
        headless=True,
        parallel=max_parallel
    )
    def scrape_pages(driver: Driver, data):
        url = data['url']
        bypass_cf = data.get('bypass_cloudflare', False)

        # Navigate to the page
        if bypass_cf:
            driver.get(url, bypass_cloudflare=True)
        else:
            driver.get(url)

        # Return URL and HTML
        return {
            'url': url,
            'html': driver.page_html
        }

    # Prepare data for parallel execution
    data_list = [{'url': url, 'bypass_cloudflare': bypass_cloudflare} for url in urls]

    # Execute the scraper
    results = scrape_pages(data_list)

    # Convert to dictionary format
    return {result['url']: result['html'] for result in results}


if __name__ == "__main__":
    # Run the MCP server using stdio transport (standard for MCP)
    mcp.run(transport="stdio")
