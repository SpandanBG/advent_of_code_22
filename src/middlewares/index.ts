import type { Express } from "express";

import { setupBodyPaser } from "./request_parser";
import { setupPugViewEngine } from "./view_engine";
import { setupStaticFilesDir } from "./static_files";

export default function (app: Express) {
    [setupBodyPaser, setupPugViewEngine, setupStaticFilesDir].forEach(fn => fn(app));
}