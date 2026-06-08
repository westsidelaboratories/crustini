import { arg, hasFlag, run } from "./lib";

const port = arg(0, "4321");
const dryRun = hasFlag("--dry-run");

if (!/^\d+$/.test(port)) {
  throw new Error(`expected numeric port, got ${port}`);
}

const proc = Bun.spawn(["lsof", "-ti", `tcp:${port}`], {
  stdout: "pipe",
  stderr: "pipe",
});

const output = await new Response(proc.stdout).text();
await proc.exited;

const pids = output
  .split(/\s+/)
  .map((pid) => pid.trim())
  .filter(Boolean);

if (pids.length === 0) {
  console.log(`port ${port}: clean`);
} else if (dryRun) {
  console.log(`port ${port}: would kill ${pids.join(" ")}`);
} else {
  await run({
    cmd: ["kill", ...pids],
    label: `kill port ${port}: ${pids.join(" ")}`,
  });
}

