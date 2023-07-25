import Database from "tauri-plugin-sql-api";
import type { QueryResult } from "tauri-plugin-sql-api";

let db: Database;
const loadDB = Database.load("sqlite:sqlite.db").then((instance) => {
  db = instance;
  return db;
});

export async function execute(
  query: string,
  bindValues?: unknown[]
): Promise<QueryResult> {
  await loadDB;
  return await db.execute(query, bindValues ?? []);
}

export async function select(
  query: string,
  bindValues?: unknown[]
): Promise<QueryResult> {
  await loadDB;
  // return await db.select(query, bindValues ?? []);
  const array: {}[] = await db.select(query, bindValues ?? []);
  array.forEach((element) => {
    Object.entries(element).forEach(([key, value]) => {
      console.log(`${key} ${value}`); // "a 5", "b 7", "c 9"
      console.log("value");
      console.log(typeof value);
      console.log(value);
      if (
        typeof value === "string" &&
        (value.startsWith("{") || value.startsWith("[")) &&
        (value.endsWith("}") || value.endsWith("]"))
      ) {
        try {
          element[key] = JSON.parse(value);
        } catch (error) {
          console.error(error);
        }
      }
    });
  });

  console.log("select", array);
  return array;
}
