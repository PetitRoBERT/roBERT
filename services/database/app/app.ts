// lib/app.ts
import express = require('express');
import Database from "./database";
import { Author } from "./models/author";

new Database();

// Create a new express application instance
const app: express.Application = express();

app.get('/', (req, res) => {
    res.send('Hello World!');
});

app.get('/author/', (req, res) => {
    const firstname = req.query.firstname;
    const lastname = req.query.lastname;

    const author = new Author({
        firstname,
        lastname
    })

    author.save()
        .then((doc: any) => {
            console.log(doc);
            res.status(200)
                .send({
                    message: `Successfully added ${firstname} ${lastname} to DB 🍃`,
                    doc
                })
        })
        .catch((err: any) => {
            console.error(err);
            res.status(400)
                .send({
                    message: `Error :${err}`
                })
        });
})

app.listen(3000, () => {
    console.log('Live on port 3000');
});