# Forgejo Website Clone

A pixel-perfect recreation of the [Forgejo.org](https://forgejo.org/) homepage using Svelte and Tailwind CSS.

## Overview

This project recreates the official Forgejo website with modern web technologies, maintaining the exact look, feel, and branding of the original site.

**Forgejo** is a self-hosted lightweight software forge. Easy to install and low maintenance, it just does the job.

## Technology Stack

- **Svelte 5.43.8** - Modern reactive framework
- **Tailwind CSS 4.1.18** - Utility-first CSS framework
- **Vite 7.2.7** - Fast build tool and dev server

## Features

✨ **Accurate Design**
- Official Forgejo orange color (#e5873a)
- Authentic typography and spacing
- Original content and messaging

🎨 **Responsive Layout**
- Mobile-first design
- Tablet and desktop optimized
- Smooth transitions and hover effects

♿ **Accessibility**
- Semantic HTML
- ARIA labels
- Keyboard navigation

⚡ **Performance**
- Fast loading times
- Optimized bundle sizes
- Hot module replacement in development

## Project Structure

```
forgejo-clone/
├── src/
│   ├── lib/
│   │   ├── Header.svelte      # Navigation header
│   │   ├── Hero.svelte        # Hero section with CTA
│   │   ├── Features.svelte    # Feature cards
│   │   └── Footer.svelte      # Footer with links
│   ├── App.svelte             # Main app component
│   ├── app.css                # Tailwind CSS imports
│   └── main.js                # Entry point
├── index.html                 # HTML template
├── VALIDATION.md              # Validation report
└── package.json               # Dependencies
```

## Getting Started

### Prerequisites

- Node.js (v18 or higher)
- npm or yarn

### Installation

```bash
# Install dependencies
npm install
```

### Development

```bash
# Start development server
npm run dev

# Server will be available at http://localhost:5173
```

### Production Build

```bash
# Build for production
npm run build

# Preview production build
npm run preview
```

## Design Elements

### Brand Colors

- **Primary Orange**: #e5873a (Forgejo brand color)
- **Hover State**: #d4773a (darker orange)

### Sections

1. **Header** - Logo and navigation menu
2. **Hero** - Main tagline "Beyond coding. We forge." with CTAs
3. **Features** - Five feature cards highlighting:
   - Security, Scaling, Federation & Privacy
   - Take Back Control
   - Ease of Use
   - Liberation
   - Community Driven
4. **Footer** - Links organized by Product, Community, and About

## Validation

This recreation has been validated against the official Forgejo website. See [VALIDATION.md](./VALIDATION.md) for detailed verification.

## References

- [Forgejo Official Site](https://forgejo.org/)
- [Forgejo Documentation](https://forgejo.org/docs/)
- [Codeberg e.V.](https://codeberg.org/)

## License

This is a demonstration project recreating the Forgejo website design. Forgejo is Free and Open Source Software under the MIT License.

## Acknowledgments

- Original design by the Forgejo team
- Forgejo is brought to you by Codeberg e.V., a democratic non-profit organization
