import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

export default defineConfig({
  site: 'https://edithatogo.github.io',
  base: '/fe-reader',
  integrations: [
    starlight({
      title: 'Fe Reader',
      description:
        'Bleeding-edge local-first PDF workflow platform with privacy, provenance, and automation safety.',
      editLink: {
        baseUrl: 'https://github.com/edithatogo/fe-reader/edit/main/docs-site/src/content/docs/',
      },
      social: [{ icon: 'github', label: 'GitHub', href: 'https://github.com/edithatogo/fe-reader' }],
      sidebar: [
        {
          label: 'Start',
          items: [
            { label: 'Fe Reader', slug: 'index' },
            { label: 'Architecture', slug: 'architecture' },
            { label: 'Native macOS Shell', slug: 'native-macos-shell' },
            { label: 'Release Quality', slug: 'release-quality' },
            { label: 'Release Pipeline', slug: 'release-pipeline' },
            { label: 'Stable Desktop Release', slug: 'stable-desktop-release' },
            { label: 'PDF Baseline Parity', slug: 'pdf-baseline-parity' },
            { label: 'Mobile Public Launch', slug: 'mobile-public-launch' },
            { label: 'Frontier Intelligence', slug: 'frontier-intelligence-governance' },
            { label: 'Opt-in Collaboration', slug: 'opt-in-collaboration-sync' },
            { label: 'Rendering Performance', slug: 'rendering-performance-promotion' },
            { label: 'Automation Safety', slug: 'automation-safety' },
          ],
        },
        {
          label: 'Contracts',
          items: [{ label: 'Rendering', slug: 'contracts/rendering' }],
        },
      ],
    }),
  ],
});
