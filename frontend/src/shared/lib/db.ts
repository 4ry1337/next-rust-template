import { Pool } from "pg";

export const pool = new Pool({
  user: process.env.POSTGRES_USER,
  password: process.env.POSTGRES_PASSWORD,
  database: process.env.POSTGRES_DB,
  port: Number(process.env.DB_PORT),
  host: process.env.DB_HOST,
  ssl: Boolean(process.env.POSTGRES_TLS)
});
