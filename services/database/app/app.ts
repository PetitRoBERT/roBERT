// lib/app.ts
import express = require('express');
import Database from "./database";
import dotenv = require('dotenv');
import author from './author';

dotenv.config();
const PORT = process.env.DB_PORT || 3000;

new Database();

// Create a new express application instance
const app: express.Application = express();

app.get('/', (req, res) => {
    res.send('<h1>Hello World 👋️</h1><ul><li>authors/</li></ul>');
});

app.use('/authors', author());

app.listen(PORT, () => {
    console.log(`Live on port http://localhost:${PORT}/`);
});