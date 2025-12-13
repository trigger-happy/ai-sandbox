# Forgejo Website Layout Emulation

This document describes the Forgejo website layout emulation created in Slint.

## Overview

The layout emulates the [Forgejo.org](https://forgejo.org/) website using Rust and Slint, focusing on accurately recreating the visual design and layout structure.

## Design Elements Implemented

### Color Palette
Based on research of Forgejo branding:
- **Primary Orange**: `#ff6600` - Main brand color, used for CTAs and highlights
- **Dark Orange**: `#e25822` - Used for gradients and emphasis
- **Light Orange**: `#ff8533` - Lighter accent color
- **Text Dark**: `#2c3e50` - Primary text color
- **Text Light**: `#6b7280` - Secondary text color
- **Backgrounds**: White (`#ffffff`) and light gray (`#f9fafb`)
- **Borders**: `#e5e7eb` for subtle borders

### Layout Sections

#### 1. Header / Navigation Bar
- Height: 70px
- White background with subtle drop shadow
- Contains:
  - Logo (orange rectangle with simplified flame icon)
  - Brand name "Forgejo"
  - Navigation menu (Features, Download, Documentation, Community, Blog)
  - "Get Started" CTA button in primary orange

#### 2. Hero Section
- Height: 500px
- Gradient background from white to light orange tint (`#fff5f0`)
- Features:
  - Main headline: "Beyond coding. We forge."
  - Subtitle: "A self-hosted lightweight software forge."
  - Supporting text
  - Two CTA buttons: "Download" (filled) and "Learn More" (outlined)
  - Abstract illustration using orange circles

#### 3. Features Section
- White background
- Grid layout with 6 feature cards (2 rows × 3 columns)
- Each card includes:
  - Colored icon background
  - Feature title
  - Descriptive text
- Features highlighted:
  1. **Self-Hosted** (Orange) - Control your data
  2. **Lightweight** (Blue) - Efficient and fast
  3. **Community-Driven** (Green) - Built by community
  4. **Privacy First** (Red) - No tracking
  5. **Federation Ready** (Purple) - ActivityPub support
  6. **Easy Migration** (Amber) - GitHub-compatible

#### 4. Call to Action Section
- Height: 300px
- Orange gradient background (#ff6600 to #e25822)
- White text
- Two CTAs: "Download Forgejo" and "Read the Docs"

#### 5. Footer
- Dark background (`#1f2937`)
- Four columns:
  1. Forgejo branding and description
  2. Resources (Documentation, Download, Releases)
  3. Community (Codeberg, Forum, Matrix Chat)
  4. About (Blog, Governance, License)
- Copyright notice at bottom

## Technical Implementation

### Technologies Used
- **Slint 1.14**: UI framework
- **Rust**: Programming language
- **WebAssembly**: Browser deployment target
- **Tokio/Axum**: Web server for hosting

### Key Features
- Fully scrollable layout using `ScrollView`
- Responsive spacing and padding
- Modern card-based design with shadows
- Gradient backgrounds
- Clean typography hierarchy

### Building and Running

1. **Build for WebAssembly**:
   ```bash
   cargo build --target wasm32-unknown-unknown --lib
   wasm-pack build --target web --out-dir pkg
   ```

2. **Run the development server**:
   ```bash
   cargo run --bin server
   ```

3. **Access the application**:
   Open `http://127.0.0.1:3000` in a web browser

## Design Fidelity

The layout captures the essential design elements of Forgejo:
- Clean, modern aesthetic
- Strong brand identity with orange accents
- Clear visual hierarchy
- Card-based feature presentation
- Professional color palette
- Emphasis on open source and community values

## References

Research sources:
- [Forgejo Website Repository](https://codeberg.org/forgejo/website) - Built with Astro/Tailwind
- [Forgejo Branding](https://codeberg.org/forgejo/meta) - Logo and brand guidelines
- [Forgejo Documentation](https://forgejo.org/docs/)
- Community discussions on Codeberg about design and branding

## Future Enhancements

Potential improvements:
- Add hover effects and animations
- Implement actual navigation functionality
- Add the Forgejo mascot illustration by David Revoy
- Create responsive layouts for different screen sizes
- Add dark mode support
