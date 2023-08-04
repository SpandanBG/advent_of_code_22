import type { RequestHandler } from 'express';

enum Methods {
    GET,
    POST,
}

interface RouteI {
    path: string,
    method: Methods,
    handler: RequestHandler[];
}

export { Methods };
export type { RouteI };