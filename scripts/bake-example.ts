import { arg, run } from "./lib";

const app = arg(0, "examples/bounce/app.flour");

await run({
  cmd: ["cargo", "run", "--bin", "rx", "--", "bake", app],
  label: `bake ${app}`,
});
