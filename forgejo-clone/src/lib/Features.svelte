<script>
  const features = [
    {
      title: 'Security, Scaling, Federation & Privacy',
      description: 'Forgejo focuses on security, scaling, federation and privacy. Built with modern technologies to ensure your code is safe and accessible.',
      icon: 'shield',
      bgColor: 'bg-blue-50'
    },
    {
      title: 'Take Back Control',
      description: 'Self-host your projects and get everyone involved in delivering quality software. Own your development process without depending on proprietary platforms.',
      icon: 'control',
      bgColor: 'bg-green-50'
    },
    {
      title: 'Ease of Use',
      description: "Ease of use is important to get things done efficiently. Forgejo's user experience is designed for collaboration and productivity.",
      icon: 'users',
      bgColor: 'bg-purple-50'
    },
    {
      title: 'Liberation',
      description: 'Liberate your software from proprietary shackles. Forgejo offers a familiar environment to GitHub users for a smooth transition to a platform you own.',
      icon: 'freedom',
      bgColor: 'bg-orange-50'
    },
    {
      title: 'Community Driven',
      description: 'Get involved in various domains including localization, code, federation, releases management, user research, UX, community management, documentation, web design, and governance.',
      icon: 'community',
      bgColor: 'bg-pink-50'
    }
  ];
</script>

<section class="py-20 bg-white">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    <h2 class="text-4xl font-bold text-center text-gray-900 mb-16">
      Why Choose Forgejo?
    </h2>

    <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
      {#each features as feature}
        <div class="p-8 rounded-lg {feature.bgColor} hover:shadow-lg transition-shadow duration-300">
          <div class="mb-4">
            {#if feature.icon === 'shield'}
              <svg class="w-12 h-12" style="color: #e5873a" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"/>
              </svg>
            {:else if feature.icon === 'control'}
              <svg class="w-12 h-12" style="color: #e5873a" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
              </svg>
            {:else if feature.icon === 'users'}
              <svg class="w-12 h-12" style="color: #e5873a" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"/>
              </svg>
            {:else if feature.icon === 'freedom'}
              <svg class="w-12 h-12" style="color: #e5873a" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/>
              </svg>
            {:else if feature.icon === 'community'}
              <svg class="w-12 h-12" style="color: #e5873a" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"/>
              </svg>
            {/if}
          </div>
          <h3 class="text-2xl font-bold text-gray-900 mb-4">{feature.title}</h3>
          <p class="text-gray-700 leading-relaxed">{feature.description}</p>
        </div>
      {/each}
    </div>
  </div>
</section>
