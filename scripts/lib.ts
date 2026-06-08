export type Command = {
  cmd: string[];
  label?: string;
};

export const examples = [
  "examples/all-macros/app.flour",
  "examples/bounce/app.flour",
  "examples/counter/app.flour",
] as const;

export const projectExamples = [
  "examples/hood-wars",
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
