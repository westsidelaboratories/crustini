import { readdir, rm } from "node:fs/promises";
import { join } from "node:path";
import { hasFlag, repoRoot } from "./lib";

const root = repoRoot();
const dryRun = hasFlag("--dry-run");

const rootPaths = ["target", "apps/site/dist", "apps/site/.astro"];
const nestedDirs = [".bakery", ".crustini", ".oven"];

async function removePath(path: string): Promise<void> {
  if (dryRun) {
    console.log(`would remove ${path}`);
    return;
  }

  await rm(join(root, path), { recursive: true, force: true });
  console.log(`removed ${path}`);
}

async function removeNested(parent: string, dirname: string): Promise<void> {
  let entries: string[];
  try {
    entries = await readdir(join(root, parent));
  } catch {
    return;
  }

  for (const entry of entries) {
    await removePath(join(parent, entry, dirname));
  }
}

for (const path of rootPaths) {
  await removePath(path);
}

for (const dirname of nestedDirs) {
  await removeNested("examples", dirname);
}

