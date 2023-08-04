import type { RequestHandler } from 'express';
import solutions from '../../../solutions/pkg';
import { Methods } from '../../models';
import type { RouteI } from '../../models';


const SolvePage: RequestHandler = (req, res) => {
    const { title } = req.query;
    res.render("modal", { title, solve: req.params });
}

const SolveProblem: RequestHandler = (req, res) => {
    const { input } = req.body;
    const { day, part } = req.params;
    const solveMethod = solutions[`${day}_${part}` as keyof typeof solutions];

    if (typeof solveMethod !== "function") {
        return res.status(404).send();
    }

    const t0 = performance.now();
    const ans = solveMethod(input);
    const t1 = performance.now() - t0;

    const timeTaken  = `${t1.toFixed(3)}ms`;

    res.render("components/answer/answer", { ans, timeTaken });
}

const SolvePageRoute: RouteI[] = [
    {
        path: "/solve/:day/:part",
        method: Methods.GET,
        handler: [SolvePage],
    },
    {
        path: "/solve/:day/:part",
        method: Methods.POST,
        handler: [SolveProblem],
    }
]

export { SolvePageRoute };