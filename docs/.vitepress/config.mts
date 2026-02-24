import { defineConfig } from 'vitepress'
import fs from 'fs'
import path from 'path'
import { fileURLToPath } from 'url'

const __dirname = path.dirname(fileURLToPath(import.meta.url))

/** Read docs/services/ and build a sidebar item list, grouped A–Z. */
function servicesSidebar() {
  const servicesDir = path.join(__dirname, '../services')
  if (!fs.existsSync(servicesDir)) return []

  const slugs = fs.readdirSync(servicesDir)
    .filter(f => f.endsWith('.md'))
    .map(f => f.replace(/\.md$/, ''))
    .sort()

  // Group by first letter
  const groups: Record<string, typeof slugs> = {}
  for (const slug of slugs) {
    const letter = slug[0].toUpperCase()
    if (!groups[letter]) groups[letter] = []
    groups[letter].push(slug)
  }

  return Object.entries(groups).map(([letter, items]) => ({
    text: letter,
    collapsed: true,
    items: items.map(slug => ({
      text: slug,
      link: `/services/${slug}`,
    })),
  }))
}

export default defineConfig({
  title: 'Winterbäume',
  description: 'Stateful AWS service mocking for aws-sdk-rust',
  themeConfig: {
    nav: [
      { text: 'Guide', link: '/guide/getting-started' },
      { text: 'Services', link: '/reference/services' },
      { text: 'Developer', link: '/developer/' },
    ],

    sidebar: {
      '/guide/': [
        {
          text: 'Guide',
          items: [
            { text: 'Getting Started', link: '/guide/getting-started' },
            { text: 'Library Mode', link: '/guide/library-mode' },
            { text: 'Server Mode', link: '/guide/server-mode' },
            { text: 'State Management', link: '/guide/state-management' },
            { text: 'Smithy Mocks Integration', link: '/guide/smithy-mocks' },
            { text: 'Pluggable Backends', link: '/guide/pluggable-backends' },
          ],
        },
      ],

      '/reference/': [
        {
          text: 'Reference',
          items: [
            { text: 'Service Coverage', link: '/reference/services' },
            { text: 'Terraform Coverage', link: '/reference/terraform' },
            { text: 'Architecture', link: '/reference/architecture' },
          ],
        },
      ],

      '/services/': [
        {
          text: 'Services',
          items: [{ text: '← Coverage table', link: '/reference/services' }],
        },
        ...servicesSidebar(),
      ],

      '/developer/': [
        {
          text: 'Developer',
          items: [
            { text: 'Overview', link: '/developer/' },
            { text: 'Architecture', link: '/developer/architecture' },
            { text: 'Adding a Service', link: '/developer/adding-a-service' },
            { text: 'Smithy Codegen', link: '/developer/smithy-codegen' },
            { text: 'Testing', link: '/developer/testing' },
            { text: 'Backends', link: '/developer/backends' },
          ],
        },
      ],
    },

    socialLinks: [
      { icon: 'github', link: 'https://github.com/moriyoshi/winterbaume' },
    ],

    footer: {
      message: 'Released under the Apache-2.0 License. This project is not affiliated with or endorsed by Amazon Web Services.',
    },

    search: {
      provider: 'local',
    },

    head: [
      ['link', { rel: 'icon', type: 'image/svg+xml', href: '/vitepress-logo-mini.svg' }],
      ['link', { rel: 'icon', type: 'image/png', href: '/vitepress-logo-mini.png' }],
      ['meta', { name: 'theme-color', content: '#5f67ee' }],
    ],

    transformPageData: (pageData, ctx) => {
      const url = new URL(pageData.relativePath.replace(/(?:(^|\/)index)?\.md$/, '$1'), siteUrl).href;
      const site = resolveSiteDataByRoute(ctx.siteConfig.site, pageData.relativePath);
      const title = pageData.title ? `${pageData.title} | VitePress` : site.title;
      const description = pageData.description || site.description;
      const locale = localeToOgLocaleMap[site.localeIndex || 'root'];
      ((pageData.frontmatter.head ??= []) as HeadConfig[]).push(
        ['meta', { property: 'og:url', content: url }],
        ['meta', { property: 'og:title', content: title }],
        ['meta', { property: 'og:description', content: description }],
        ['meta', { property: 'og:type', content: 'website' }],
        ['meta', { property: 'og:locale', content: locale }],
        ['meta', { property: 'og:site_name', content: 'VitePress' }],
        ['meta', { property: 'og:image', content: ogImage }],
        ['meta', { property: 'og:image:secure_url', content: ogImage }],
        ['meta', { property: 'og:image:type', content: 'image/jpeg' }],
        ['meta', { property: 'og:image:width', content: '1280' }],
        ['meta', { property: 'og:image:height', content: '640' }],
        ['meta', { property: 'og:image:alt', content: 'VitePress' }],
        ['link', { rel: 'canonical', href: url }],
      ) ;
    },
  },
})
