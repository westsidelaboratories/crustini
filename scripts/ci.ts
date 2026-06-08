import { runAll } from "./lib";

await runAll([
  {
    cmd: ["bun", "run", "check"],
    label: "repo check",
  },
  {
    cmd: ["bun", "run", "build"],
    label: "repo build",
  },
]);

