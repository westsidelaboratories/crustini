export type Command = {
  cmd: string[];
  label?: string;
};

export const exampleApps = [
  "examples/new-examples/sketch-pulse.flour",
  "examples/new-examples/keyboard-mover.flour",
  "examples/new-examples/mouse-follow.flour",
  "examples/hello-world",
  "examples/brick-breaker/app.flour",
  "examples/brick-breaker-plus/app.flour",
  "examples/mouse-orbit/app.flour",
  "examples/dodge-dots/app.flour",
] as const;

export const fixtureApps = [
  "fixtures/apps/all-macros/app.flour",
  "fixtures/apps/bounce/app.flour",
  "fixtures/apps/counter/app.flour",
] as const;

export const fixtureProjects = [
  "fixtures/projects/hood-wars",
] as const;

export async function run(command: Command): Promise<void> {
  const label = command.label ?? command.cmd.join(" ");
  console.log(`\n$ ${label}`);

  const proc = Bun.spawn(command.cmd, {
    cwd: repoRoot(),
    stdin: "inherit",
    stdout: "inherit",
    stderr: "inherit",
  });

  const exitCode = await proc.exited;
  if (exitCode !== 0) {
    throw new Error(`${label} failed with exit code ${exitCode}`);
  }
}

export async function runAll(commands: Command[]): Promise<void> {
  for (const command of commands) {
    await run(command);
  }
}

export function repoRoot(): string {
  return new URL("..", import.meta.url).pathname;
}

export function arg(index: number, fallback?: string): string {
  return Bun.argv[index + 2] ?? fallback ?? "";
}

export function hasFlag(name: string): boolean {
  return Bun.argv.includes(name);
}
