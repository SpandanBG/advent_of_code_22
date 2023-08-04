import express from "express";
import setupMiddlewares from "./middlewares";
import setupRoutes from './router';

const app = express();
const port = 8080;

setupMiddlewares(app);
setupRoutes(app);

app.listen(port, () => {
    console.log(`Web Server ready on port ${port}`);
});