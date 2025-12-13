<script>
  // Navigation items
  const navItems = [
    { text: 'Download', href: '/download' },
    { text: 'Docs', href: '/docs' },
    { text: 'Blog', href: '/blog' },
    { text: 'FAQ', href: '/faq' },
    { text: 'About', href: '/about' }
  ];
</script>

<header class="bg-white border-b border-gray-200">
  <nav class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    <div class="flex justify-between items-center h-16">
      <!-- Logo -->
      <div class="flex items-center">
        <a href="/" class="flex items-center space-x-2">
          <svg class="w-8 h-8" style="color: #e5873a" viewBox="0 0 24 24" fill="currentColor">
            <path d="M12 2L2 7v10c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V7l-10-5zm0 2.18l8 4v8.82c0 4.52-3.13 8.88-8 10-4.87-1.12-8-5.48-8-10V8.18l8-4z"/>
            <circle cx="12" cy="12" r="3"/>
          </svg>
          <span class="text-xl font-bold text-gray-900">Forgejo</span>
        </a>
      </div>

      <!-- Navigation Links -->
      <div class="hidden md:flex items-center space-x-8">
        {#each navItems as item}
          <a
            href={item.href}
            class="text-gray-700 transition-colors duration-200 hover-forgejo"
          >
            {item.text}
          </a>
        {/each}
      </div>

      <!-- Mobile menu button -->
      <div class="md:hidden">
        <button class="text-gray-700" aria-label="Open menu">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/>
          </svg>
        </button>
      </div>
    </div>
  </nav>
</header>

<style>
  .hover-forgejo:hover {
    color: #e5873a;
  }
</style>
