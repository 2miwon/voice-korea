import 'reflect-metadata';
import express from "express";
import cors from "cors";
import dotenv from "dotenv";
import { CONFIGS } from "./commons/configs";
import { rateLimitMiddleware } from "./middlewares/rateLimitMiddleware";
import swaggerUi from "swagger-ui-express";
import { readFileSync } from 'fs';
import { join} from 'path';
import { RegisterRoutes } from "./swagger/routes";
import { corsOptions } from './commons/configs/cors';
const swaggerDocument = JSON.parse(
    readFileSync(join(process.cwd(), 'src/swagger/swagger.json'), 'utf-8')
);

dotenv.config();
const app = express();

app.use(cors(corsOptions));
app.use(rateLimitMiddleware)

app.get("/", (req:any, res:any) => res.send(`MCP service is UP!`));
if (CONFIGS.NODE_ENV !== 'prod' && CONFIGS.NODE_ENV !== 'production'){
    app.use('/swagger/api', swaggerUi.serve, swaggerUi.setup(swaggerDocument));
}
app.use(express.json());
RegisterRoutes(app);
app.listen(CONFIGS.SERVER_PORT, () => {
    console.log(`MCP service is running on http://localhost:${CONFIGS.SERVER_PORT}`);
});