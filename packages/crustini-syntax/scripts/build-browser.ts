import { copyFile, mkdir } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const repoRoot = resolve(root, "../..");
const outdir = resolve(root, "dist");
const outfile = resolve(outdir, "crustini-highlight.js");
const liveDocsCopy = resolve(repoRoot, "apps/site/public/live/crs-highlight.js");

await mkdir(outdir, { recursive: true });

const result = await Bun.build({
  entrypoints: [resolve(root, "src/browser-global.ts")],
  outdir,
  naming: "crustini-highlight.js",
  target: "browser",
  format: "iife",
  minify: false,
  sourcemap: "none",
  banner: `/* Crustini .flour syntax highlighter
   No dependencies after bundling. Browser + plain script usage.

   Usage:
   <link rel="stylesheet" href="themes/crustini-dark.css">
   <script src="dist/crustini-highlight.js"></script>
   <pre class="crs-code"><code class="language-flour">app! Main { ... }</code></pre>
   <script>CrustiniHighlight.highlightAll();</script>
*/`,
});

if (!result.success) {
  for (const log of result.logs) {
    console.error(log);
  }
  process.exit(1);
}

await copyFile(outfile, liveDocsCopy);

console.log(`built ${outfile}`);
console.log(`synced ${liveDocsCopy}`);
