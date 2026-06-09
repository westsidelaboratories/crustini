import { arg, run } from "./lib";

const app = arg(0, "examples/hello-world");

await run({
  cmd: ["cargo", "run", "--bin", "rx", "--", "bake", app],
  label: `bake ${app}`,
});
