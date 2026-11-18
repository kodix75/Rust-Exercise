#!/usr/bin/env node
/**
 * Run one exercise:  npm run ex -- ex01
 * Pass program arguments after the exercise name:  npm run ex -- ex10 -- add 3 4
 */
import { spawnSync } from "node:child_process";
import { readdirSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const root = dirname(dirname(fileURLToPath(import.meta.url)));
const binDir = join(root, "src", "bin");

const EXERCISE = /^ex(\d{2})\.rs$/;

function exercises() {
  try {
    return readdirSync(binDir)
      .filter((f) => EXERCISE.test(f))
      .map((f) => f.replace(/\.rs$/, ""))
      .sort();
  } catch {
    return [];
  }
}

const name = process.argv[2];
const extra = process.argv.slice(3);
const available = exercises();

if (!name || name === "--help" || name === "-h") {
  console.log("Usage: npm run ex -- exNN [-- program args]");
  console.log("");
  console.log(
    available.length
      ? `Available: ${available.join(", ")}`
      : "No exercises yet — create src/bin/ex01.rs (see docs/exercises/ex01.md).",
  );
  process.exit(name ? 0 : 1);
}

if (!EXERCISE.test(`${name}.rs`)) {
  console.error(`✖ "${name}" is not an exercise name. Expected exNN, e.g. ex01.`);
  process.exit(1);
}

if (!available.includes(name)) {
  console.error(`✖ ${name} does not exist yet.`);
  console.error(
    available.length
      ? `   Available: ${available.join(", ")}`
      : "   Create src/bin/ex01.rs first — the guide is in docs/exercises/ex01.md",
  );
  process.exit(1);
}

const args = ["run", "--bin", name];
if (extra.length) args.push("--", ...extra);

const result = spawnSync("cargo", args, { cwd: root, stdio: "inherit" });
if (result.error) {
  console.error(`✖ could not run cargo: ${result.error.message}`);
  process.exit(1);
}
process.exit(result.status ?? 1);
