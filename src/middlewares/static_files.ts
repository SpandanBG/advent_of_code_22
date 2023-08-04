import { static as Static } from 'express';
import type { Express } from 'express';

export function setupStaticFilesDir(app: Express) {
    app.use(Static("res"));
}