import { runAll } from "./lib";

await runAll([
  {
    cmd: ["cargo", "build", "--workspace"],
    label: "cargo build --workspace",
  },
  {
    cmd: ["bun", "--filter", "@crustini/site", "build"],
    label: "astro build",
  },
]);

