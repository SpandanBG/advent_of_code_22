import type { Express } from 'express';
import bodyParser from "body-parser";

export function setupBodyPaser(app: Express) {
    // Add JSON parser
    app.use(bodyParser.json());

    // Add x-www-form-encoded body parser
    app.use(bodyParser.urlencoded({ extended: true }));
}