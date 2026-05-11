import { defineConfig, type HeadConfig } from 'vitepress'
import fs from 'fs'
import path from 'path'
import { fileURLToPath } from 'url'

const __dirname = path.dirname(fileURLToPath(import.meta.url))
const siteTitle = 'Winterbäume'
const siteDescription = 'Stateful AWS service mocking for aws-sdk-rust'
const siteUrl = process.env.VITEPRESS_SITE_URL ?? 'https://winterbau.me/'
const ogImagePath = '/ogp.jpg'
const ogImageUrl = new URL(ogImagePath, siteUrl).href

function pageUrl(relativePath: string) {
  if (relativePath === 'index.md') {
    return new URL('/', siteUrl).href
  }

  const urlPath = relativePath
    .replace(/(^|\/)index\.md$/, '$1')
    .replace(/\.md$/, '.html')

  return new URL(urlPath, siteUrl).href
}

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
  title: siteTitle,
  description: siteDescription,
  head: [
    ['meta', { name: 'theme-color', content: '#4d75ec' }],
  ],
  transformPageData(pageData) {
    const title = pageData.title && pageData.title !== siteTitle
      ? `${pageData.title} | ${siteTitle}`
      : siteTitle
    const description = pageData.description || siteDescription
    const url = pageUrl(pageData.relativePath)
    const head = (pageData.frontmatter.head ??= []) as HeadConfig[]

    head.push(
      ['meta', { property: 'og:url', content: url }],
      ['meta', { property: 'og:title', content: title }],
      ['meta', { property: 'og:description', content: description }],
      ['meta', { property: 'og:type', content: 'website' }],
      ['meta', { property: 'og:locale', content: 'en_US' }],
      ['meta', { property: 'og:site_name', content: siteTitle }],
      ['meta', { property: 'og:image', content: ogImageUrl }],
      ['meta', { property: 'og:image:secure_url', content: ogImageUrl }],
      ['meta', { property: 'og:image:type', content: 'image/jpeg' }],
      ['meta', { property: 'og:image:width', content: '1200' }],
      ['meta', { property: 'og:image:height', content: '630' }],
      ['meta', { property: 'og:image:alt', content: 'Winterbäume: stateful AWS service mocking for Rust integration tests' }],
      ['meta', { name: 'twitter:card', content: 'summary_large_image' }],
      ['meta', { name: 'twitter:title', content: title }],
      ['meta', { name: 'twitter:description', content: description }],
      ['meta', { name: 'twitter:image', content: ogImageUrl }],
      ['meta', { name: 'twitter:image:alt', content: 'Winterbäume: stateful AWS service mocking for Rust integration tests' }],
      ['link', { rel: 'canonical', href: url }],
    )
  },
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

  },
})
