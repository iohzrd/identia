import TOML from "@ltd/j-toml";
import fs from "fs";
import semver from "semver";

async function main() {
  let p_path = "package.json";
  let p_parsed = JSON.parse(fs.readFileSync(p_path));
  let new_version = semver.inc(p_parsed.version, "patch");
  p_parsed.version = new_version;
  fs.writeFileSync(p_path, JSON.stringify(p_parsed, null, 2));

  let p_lock_path = "package-lock.json";
  let p_lock_parsed = JSON.parse(fs.readFileSync(p_lock_path));
  p_lock_parsed.version = new_version;
  fs.writeFileSync(p_lock_path, JSON.stringify(p_lock_parsed, null, 2));

  let t_path = "src-tauri/tauri.conf.json";
  let t_parsed = JSON.parse(fs.readFileSync(t_path));
  t_parsed.package.version = new_version;
  fs.writeFileSync(t_path, JSON.stringify(t_parsed, null, 2));

  let parsed = TOML.parse(fs.readFileSync("src-tauri/Cargo.toml"));
  parsed.package.version = new_version;
  let opts = {
    newline: "\n",
    newlineAround: "section",
  };
  let out = TOML.stringify(parsed, opts).replace("\n", "").replaceAll("'", '"');
  fs.writeFileSync("src-tauri/Cargo.toml", out);
}

main().catch((e) => {
  throw e;
});
