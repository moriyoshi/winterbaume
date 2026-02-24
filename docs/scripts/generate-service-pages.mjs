#!/usr/bin/env node
/**
 * Copies crates/winterbaume-{slug}/README.md to docs/services/{slug}.md,
 * removing crate-only prose that does not belong in the public service docs.
 * Run with: node scripts/generate-service-pages.mjs
 */
import fs from 'fs'
import path from 'path'
import { fileURLToPath } from 'url'

const __dirname = path.dirname(fileURLToPath(import.meta.url))
const root = path.resolve(__dirname, '../../')
const cratesDir = path.join(root, 'crates')
const servicesDir = path.join(__dirname, '../services')

/** Crates that are infrastructure / support, not AWS service implementations. */
const SKIP = new Set([
  'winterbaume-core',
  'winterbaume-server',
  'winterbaume-stubs',
  'winterbaume-terraform',
  'winterbaume-tfstate',
  'winterbaume-partiql',
  'winterbaume-sqlengine-duckdb',
  'winterbaume-sqs-redis',
  'winterbaume-dynamodb-redis',
  'winterbaume-e2e-tests',
  'winterbaume-bedrock-flow-parser',
  'winterbaume-bedrock-flow-validator',
  'winterbaume-iam-rule-eval',
  'winterbaume-sfn-asl-eval',
  'winterbaume-wafv2-wcu-calculator',
  'winterbaume-wafv2-webacl-rule-parser',
  'winterbaume-ec2-generated',
])

fs.mkdirSync(servicesDir, { recursive: true })

const crates = fs.readdirSync(cratesDir)
  .filter(d => d.startsWith('winterbaume-') && !SKIP.has(d))
  .sort()

const generated = []

for (const crate of crates) {
  const readmePath = path.join(cratesDir, crate, 'README.md')
  if (!fs.existsSync(readmePath)) continue

  const content = fs.readFileSync(readmePath, 'utf8')
    .replace(/^<!--.*?-->\n\n?/s, '')
    .replace(/^<!-- END AUTO -->\n?/m, '')
    .replace(/^This crate is part of .*\n\n?/m, '')
    .replace(/^Winterbäume is not affiliated with, endorsed by, or sponsored by Amazon .*\n\n?/m, '')
    .replace(/^Coverage is generated from \[.*?\n\n?/m, '')
    .replace(/^See the workspace \[.*?\n\n?/m, '')
    .trimEnd() + '\n'
  const slug = crate.replace('winterbaume-', '')
  const outPath = path.join(servicesDir, `${slug}.md`)
  fs.writeFileSync(outPath, content)
  generated.push(slug)
  process.stdout.write(`  wrote services/${slug}.md\n`)
}

process.stdout.write(`\nGenerated ${generated.length} service pages.\n`)
