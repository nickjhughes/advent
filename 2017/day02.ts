import fs from "node:fs/promises";

async function getInput() {
  return await fs.readFile("input02", { encoding: "ascii" });
}

async function part1() {
  const input = await getInput();
  const total = input
    .split("\n")
    .filter((l) => l.length > 0)
    .map((line) => {
      const numbers = line.split(/\s+/).map((n) => parseInt(n));
      return Math.max(...numbers) - Math.min(...numbers);
    })
    .reduce((s, a) => s + a, 0);
  console.log(total);
}

async function part2() {
  const input = await getInput();
  const total = input
    .split("\n")
    .filter((l) => l.length > 0)
    .map((line) => {
      const numbers = line.split(/\s+/).map((n) => parseInt(n));
      for (const a of numbers) {
        for (const b of numbers) {
          if (a === b) {
            continue;
          }
          if (a / b > 0 && a % b === 0) {
            return a / b;
          }
        }
      }
      throw new Error("no evenly divisible pair of numbers found");
    })
    .reduce((s, a) => s + a, 0);
  console.log(total);
}

part1()
  .then(() => {
    part2()
      .then(() => {
        process.exit();
      })
      .catch((err) => {
        throw err;
      });
  })
  .catch((err) => {
    throw err;
  });
