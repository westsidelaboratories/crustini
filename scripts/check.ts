import { examples, projectExamples, runAll } from "./lib";

await runAll([
  {
    cmd: ["cargo", "check", "--workspace"],
    label: "cargo check --workspace",
  },
  {
    cmd: ["bun", "run", "check:scripts"],
    label: "typecheck root scripts",
  },
  ...examples.map((example) => ({
    cmd: ["cargo", "run", "--bin", "rx", "--", "bake", example],
    label: `bake ${example}`,
  })),
  ...projectExamples.map((example) => ({
    cmd: ["cargo", "run", "--bin", "rx", "--", "bake", example],
    label: `bake ${example}`,
  })),
  {
    cmd: ["bun", "--filter", "@crustini/site", "check"],
    label: "astro check",
  },
]);
