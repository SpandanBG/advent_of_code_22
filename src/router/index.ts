import type { Express } from 'express';

import { Methods } from '../models';
import type { RouteI } from "../models";

import { HomePageRoute } from '../pages/home';
import { SolvePageRoute } from '../pages/solve';

const routes: RouteI[] = [
    HomePageRoute,
    ...SolvePageRoute
]

export default function (app: Express) {
    routes.forEach(({ path, method, handler}) => {
        switch (method) {
            case Methods.GET:
                app.get(path, handler);
                break;
            case Methods.POST:
                app.post(path, handler);
                break;
        }
    });
};