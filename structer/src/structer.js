/*
 * This script fetches any JSON from the web and prints a legal Rust struct
 * declaration. If any fields have a name that happens to be a Rust keyword,
 * the field is included but commented out. Theoretically the output can simply
 * be copy-pasted into a Rust file.
 * Requires node version >= 14 to run.
 */
import fetch from "node-fetch";

const args = process.argv;
const endpoint = args.length > 2 ? args[2] : "users/DaniloHP/repos";
const url = `https://api.github.com/${endpoint}`;
fetch(url).then((res) =>
  res.json().then((j) => {
    console.log("use serde::Deserialize;")
    Array.isArray(j) ? iterObj(j[0], "Repo") : iterObj(j, "Repo");
    // if it's an array, only call on the first item
  })
);

const toRustType = (val) => {
  if (val === null) return "Option<String>"; // not perfect
  switch (typeof val) {
    case "number":
      return "i32";
    case "string":
      return "Option<String>";
    case "boolean":
      return "bool";
    default:
      return "";
  }
};

// obj is the JSON object from the GitHub API
const iterObj = (obj, structName) => {
  let soFar = "";
  soFar += `\n#[derive(Debug, Deserialize)]\npub struct ${structName} {\n`;
  Object.entries(obj).forEach((e) => {
    const [key, val] = e;
    const pref = keywords.has(key) ? " // " : "    ";
    // prefix with a comment if a key has a name that is reserved in Rust
    if (val && typeof val === "object") {
      const tcStruct = key[0].toUpperCase() + key.substring(1);
      // ^ Titlecase, as Rust likes its struct names
      soFar += `${pref}pub ${key}: Option<${tcStruct}>,\n`;
      iterObj(obj[key], tcStruct);
    } else {
      soFar += `${pref}pub ${key}: ${toRustType(val)},\n`;
    }
  });
  soFar = soFar.substring(0, soFar.length - 2) + "\n}";
  console.log(soFar);
};

const keywords = new Set([
  "as",
  "break",
  "const",
  "continue",
  "crate",
  "else",
  "enum",
  "extern",
  "false",
  "fn",
  "for",
  "if",
  "impl",
  "in",
  "let",
  "loop",
  "match",
  "mod",
  "move",
  "mut",
  "pub",
  "ref",
  "return",
  "self",
  "Self",
  "static",
  "struct",
  "super",
  "trait",
  "true",
  "type",
  "unsafe",
  "use",
  "where",
  "while",
]);
