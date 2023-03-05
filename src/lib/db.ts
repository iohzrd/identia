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
  return await db.select(query, bindValues ?? []);
}
