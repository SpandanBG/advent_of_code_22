import type { Express } from 'express';

export function setupPugViewEngine(app: Express) {
    app.set('views', "src/views");
    app.set('view engine', 'pug');
}