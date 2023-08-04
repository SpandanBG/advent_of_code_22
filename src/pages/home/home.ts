import type { RequestHandler } from 'express';
import questionsData from '../../data';
import { Methods } from '../../models';
import type { RouteI } from '../../models';

const HomePage: RequestHandler = (_, res) => {
    res.render("index", { questions: questionsData });
}

const HomePageRoute: RouteI = {
    path: "/",
    method: Methods.GET,
    handler: [HomePage],
}

export { HomePageRoute };