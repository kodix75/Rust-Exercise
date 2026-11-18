#!/usr/bin/env node
/**
 * npm test — verify the whole workspace.
 *
 * Runs the structure checks, cargo check, cargo test, and (when installed)
 * rustfmt and clippy. Exits non-zero on the first category that fails.
 */
import { spawnSync } from "node:child_process";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = dirname(dirname(fileURLToPath(import.meta.url)));
const useColor = process.stdout.isTTY && !process.env.NO_COLOR;
const paint = (code) => (text) => (useColor ? `\x1b[${code}m${text}\x1b[0m` : text);
const green = paint("32");
const red = paint("31");
const yellow = paint("33");
const dim = paint("2");
const bold = paint("1");

const results = [];

function hasCommand(cmd, args = ["--version"]) {
  const probe = spawnSync(cmd, args, { cwd: root, stdio: "ignore" });
  return probe.status === 0;
}

function step(label, fn) {
  const started = Date.now();
  const ok = fn();
  const ms = Date.now() - started;
  results.push({ label, ok, ms });
  const mark = ok ? green("✓") : red("✖");
  console.log(`${mark} ${bold(label)} ${dim(`(${ms} ms)`)}`);
  return ok;
}

function skip(label, why) {
  results.push({ label, ok: true, skipped: true });
  console.log(`${yellow("○")} ${bold(label)} ${dim(`skipped — ${why}`)}`);
}

function cargo(args) {
  const run = spawnSync("cargo", args, { cwd: root, stdio: "inherit" });
  return run.status === 0;
}

function structureProblems() {
  const problems = [];

  const cargoToml = join(root, "Cargo.toml");
  if (!existsSync(cargoToml)) {
    problems.push("Cargo.toml is missing");
  } else if (!/name\s*=\s*"rust-exercise"/.test(readFileSync(cargoToml, "utf8"))) {
    problems.push('Cargo.toml: package name should be "rust-exercise"');
  }

  const gitignore = join(root, ".gitignore");
  if (!existsSync(gitignore) || !readFileSync(gitignore, "utf8").includes("/target")) {
    problems.push(".gitignore must ignore /target");
  }

  const docsDir = join(root, "docs", "exercises");
  if (!existsSync(docsDir)) {
    problems.push("docs/exercises/ is missing");
    return problems;
  }

  const guides = readdirSync(docsDir)
    .filter((f) => f.endsWith(".md") && f !== "README.md")
    .map((f) => readFileSync(join(docsDir, f), "utf8"))
    .join("\n");

  if (!guides.trim()) problems.push("docs/exercises/ has no guides");

  const binDir = join(root, "src", "bin");
  if (!existsSync(binDir)) {
    problems.push("src/bin/ is missing");
  } else {
    for (const file of readdirSync(binDir)) {
      if (file === ".gitkeep") continue;
      if (!/^ex\d{2}\.rs$/.test(file)) {
        problems.push(`src/bin/${file}: exercise files must be named exNN.rs`);
        continue;
      }
      const number = file.slice(2, 4);
      if (!guides.includes(`Ex ${number} —`)) {
        problems.push(`src/bin/${file}: no guide found (expected "Ex ${number} —" in docs/exercises/)`);
      }
    }
  }

  return problems;
}

console.log(bold("\n🦀 Rust Exercises — verification\n"));

step("structure", () => {
  const problems = structureProblems();
  if (problems.length) {
    for (const p of problems) console.error(`   ${red("•")} ${p}`);
    return false;
  }
  return true;
});

step("cargo check", () => cargo(["check", "--all-targets"]));
step("cargo test", () => cargo(["test"]));

if (hasCommand("cargo", ["fmt", "--version"])) {
  step("cargo fmt --check", () => cargo(["fmt", "--all", "--", "--check"]));
} else {
  skip("cargo fmt --check", "rustfmt not installed (sudo dnf install rustfmt)");
}

if (hasCommand("cargo", ["clippy", "--version"])) {
  step("cargo clippy", () => cargo(["clippy", "--all-targets", "--", "-D", "warnings"]));
} else {
  skip("cargo clippy", "clippy not installed (sudo dnf install clippy)");
}

const failed = results.filter((r) => !r.ok);
const skipped = results.filter((r) => r.skipped);
const passed = results.length - failed.length - skipped.length;

console.log("");
if (failed.length) {
  console.log(red(bold(`✖ ${failed.length} check(s) failed`)) + dim(`, ${passed} passed`));
  console.log(red(`  ${failed.map((r) => r.label).join(", ")}`));
  process.exit(1);
}

console.log(green(bold("✓ all checks passed")) + dim(` — ${passed} ran, ${skipped.length} skipped`));
console.log(dim("\nAdd an exercise: docs/exercises/ · run one: npm run ex -- ex01\n"));
