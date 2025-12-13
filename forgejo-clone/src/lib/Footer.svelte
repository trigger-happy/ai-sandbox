<script>
  const currentYear = new Date().getFullYear();

  const footerLinks = {
    product: [
      { text: 'Download', href: '/download' },
      { text: 'Documentation', href: '/docs' },
      { text: 'FAQ', href: '/faq' },
      { text: 'Releases', href: '/releases' }
    ],
    community: [
      { text: 'Blog', href: '/blog' },
      { text: 'Code', href: 'https://codeberg.org/forgejo/forgejo' },
      { text: 'Chat', href: '/chat' },
      { text: 'Forum', href: '/forum' }
    ],
    about: [
      { text: 'About Forgejo', href: '/about' },
      { text: 'Governance', href: '/governance' },
      { text: 'Sustainability', href: '/sustainability' },
      { text: 'Contact', href: '/contact' }
    ]
  };
</script>

<footer class="bg-gray-900 text-gray-300 py-12">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    <div class="grid md:grid-cols-4 gap-8 mb-8">
      <!-- Brand -->
      <div>
        <div class="flex items-center space-x-2 mb-4">
          <svg class="w-8 h-8" style="color: #e5873a" viewBox="0 0 24 24" fill="currentColor">
            <path d="M12 2L2 7v10c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V7l-10-5zm0 2.18l8 4v8.82c0 4.52-3.13 8.88-8 10-4.87-1.12-8-5.48-8-10V8.18l8-4z"/>
            <circle cx="12" cy="12" r="3"/>
          </svg>
          <span class="text-xl font-bold text-white">Forgejo</span>
        </div>
        <p class="text-sm">
          Beyond coding. We forge.
        </p>
      </div>

      <!-- Product Links -->
      <div>
        <h3 class="text-white font-semibold mb-4">Product</h3>
        <ul class="space-y-2">
          {#each footerLinks.product as link}
            <li>
              <a href={link.href} class="footer-link transition-colors duration-200">
                {link.text}
              </a>
            </li>
          {/each}
        </ul>
      </div>

      <!-- Community Links -->
      <div>
        <h3 class="text-white font-semibold mb-4">Community</h3>
        <ul class="space-y-2">
          {#each footerLinks.community as link}
            <li>
              <a href={link.href} class="footer-link transition-colors duration-200">
                {link.text}
              </a>
            </li>
          {/each}
        </ul>
      </div>

      <!-- About Links -->
      <div>
        <h3 class="text-white font-semibold mb-4">About</h3>
        <ul class="space-y-2">
          {#each footerLinks.about as link}
            <li>
              <a href={link.href} class="footer-link transition-colors duration-200">
                {link.text}
              </a>
            </li>
          {/each}
        </ul>
      </div>
    </div>

    <!-- Bottom Bar -->
    <div class="border-t border-gray-800 pt-8 flex flex-col md:flex-row justify-between items-center">
      <p class="text-sm mb-4 md:mb-0">
        © {currentYear} Codeberg e.V. and contributors. Free and Open Source Software under the MIT License.
      </p>
      <div class="flex space-x-6">
        <a href="/privacy" class="footer-link text-sm transition-colors duration-200">
          Privacy Policy
        </a>
        <a href="/terms" class="footer-link text-sm transition-colors duration-200">
          Terms of Service
        </a>
      </div>
    </div>
  </div>
</footer>

<style>
  .footer-link:hover {
    color: #e5873a;
  }
</style>
