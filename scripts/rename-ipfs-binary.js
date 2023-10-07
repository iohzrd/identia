// const execa = require("execa");
import fs from "fs";

let extension = "";
if (process.platform === "win32") {
  extension = ".exe";
}

async function main() {
  const { execa } = await import("execa");
  const rustInfo = (await execa("rustc", ["-vV"])).stdout;
  const targetTriple = /host: (\S+)/g.exec(rustInfo)[1];
  if (!targetTriple) {
    console.error("Failed to determine platform target triple");
  }
  fs.copyFileSync(
    `node_modules/kubo/kubo/ipfs${extension}`,
    `node_modules/kubo/kubo/ipfs-${targetTriple}${extension}`
  );
}

main().catch((e) => {
  throw e;
});
