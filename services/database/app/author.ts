import dotenv = require('dotenv');
import express = require('express');
import { Author } from "./models/author";

dotenv.config();


export default function () {
    const router: express.Router = express.Router();

    router.get('/', (req, res) => {
        res.status(200)
            .send('<h1>Hello authors 👋️</h1>');
    });

    router.post('/new', (req, res) => {
        res.status(201)
            .send('todo, add a new author');
    });

    router.get('/:authorId', (req, res) => {
        const authorId: String = req.params.authorId;
        res.status(200)
            .send(`todo, retrieve author ${authorId}`);
    });

    router.get('/all', (req, res) => {
        res.status(200)
            .send('todo, retrieve all authors');
    });

    router.delete('/:authorId', (req, res) => {
        const authorId: String = req.params.authorId;
        res.status(200)
            .send(`todo, delete ${authorId}`);
    });

    router.get('/test/', (req, res) => {
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
    });

    return router;
}
