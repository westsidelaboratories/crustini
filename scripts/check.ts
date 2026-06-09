import { exampleApps, fixtureApps, fixtureProjects, runAll } from "./lib";

await runAll([
  {
    cmd: ["cargo", "fmt", "--all", "--check"],
    label: "cargo fmt --all --check",
  },
  {
    cmd: ["cargo", "check", "--workspace"],
    label: "cargo check --workspace",
  },
  {
    cmd: ["cargo", "test", "--workspace"],
    label: "cargo test --workspace",
  },
  {
    cmd: ["cargo", "clippy", "--workspace", "--all-targets", "--", "-D", "warnings"],
    label: "cargo clippy --workspace --all-targets",
  },
  {
    cmd: ["bun", "run", "check:scripts"],
    label: "typecheck root scripts",
  },
  ...fixtureApps.map((fixture) => ({
    cmd: ["cargo", "run", "--bin", "rx", "--", "bake", fixture],
    label: `bake fixture ${fixture}`,
  })),
  ...fixtureProjects.map((fixture) => ({
    cmd: ["cargo", "run", "--bin", "rx", "--", "bake", fixture],
    label: `bake fixture ${fixture}`,
  })),
  ...exampleApps.map((example) => ({
    cmd: ["cargo", "run", "--bin", "rx", "--", "bake", example],
    label: `bake example ${example}`,
  })),
  {
    cmd: ["bun", "--filter", "@crustini/site", "check"],
    label: "astro check",
  },
]);
