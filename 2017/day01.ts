import fs from "node:fs/promises";

async function getInput() {
  return await fs.readFile("input01", { encoding: "ascii" });
}

async function part1() {
  let input = await getInput();
  input = input.replace("\n", "");
  let total = 0;
  for (let i = 0; i < input.length; i++) {
    const j = (i + 1) % input.length;
    if (input.charAt(i) === input.charAt(j)) {
      total += parseInt(input.charAt(i));
    }
  }
  console.log(total);
}

async function part2() {
  let input = await getInput();
  input = input.replace("\n", "");
  let total = 0;
  for (let i = 0; i < input.length; i++) {
    const j = (i + Math.floor(input.length / 2)) % input.length;
    if (input.charAt(i) === input.charAt(j)) {
      total += parseInt(input.charAt(i));
    }
  }
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
