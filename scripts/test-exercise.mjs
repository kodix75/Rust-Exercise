#!/usr/bin/env node
/**
 * Run the test of one exercise (or all of them).
 *
 *   npm run test:ex -- ex01
 *   npm run test:ex -- all
 *
 * The tests inside the exercise files are marked `#[ignore]` so that
 * `npm test` (and CI) stay green while the exercises are unsolved. This
 * script runs them anyway with `--ignored`.
 */
import { spawnSync } from "node:child_process";
import { existsSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = dirname(dirname(fileURLToPath(import.meta.url)));
const binDir = join(root, "src", "bin");
const EXERCISE = /^ex(\d{2})\.rs$/;

function exercises() {
  if (!existsSync(binDir)) return [];
  return readdirSync(binDir)
    .filter((f) => EXERCISE.test(f))
    .map((f) => f.replace(/\.rs$/, ""))
    .sort();
}

const useColor = process.stdout.isTTY && !process.env.NO_COLOR;
const paint = (code) => (text) => (useColor ? `\x1b[${code}m${text}\x1b[0m` : text);
const green = paint("32");
const red = paint("31");
const bold = paint("1");
const dim = paint("2");

const available = exercises();
const name = process.argv[2];

if (!name || name === "--help" || name === "-h") {
  console.log("Usage: npm run test:ex -- <exNN|all>");
  console.log("");
  console.log(available.length ? `Available: ${available.join(", ")}` : "No exercises found.");
  process.exit(name ? 0 : 1);
}

function runOne(exercise) {
  console.log(bold(`\n▶ ${exercise}`));
  const result = spawnSync(
    "cargo",
    ["test", "--bin", exercise, "--", "--ignored", "--nocapture"],
    { cwd: root, stdio: "inherit" },
  );
  return result.status === 0;
}

const targets = name === "all" ? available : [name];
const unknown = targets.filter((t) => !available.includes(t));

if (unknown.length) {
  console.error(`${red("✖")} unknown exercise: ${unknown.join(", ")}`);
  console.error(dim(`  Available: ${available.join(", ")}`));
  process.exit(1);
}

let failed = 0;
for (const exercise of targets) {
  if (!runOne(exercise)) failed += 1;
}

console.log("");
if (failed) {
  console.log(red(bold(`✖ ${failed}/${targets.length} exercise test(s) failed`)));
  console.log(dim("  Read the first assertion that failed, fix the code, run again.\n"));
  process.exit(1);
}

console.log(green(bold(`✓ ${targets.length} exercise test(s) passed`)));
console.log(dim("  Tick the exercise in the README, then commit.\n"));
