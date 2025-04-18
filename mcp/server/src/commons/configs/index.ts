import dotenv from "dotenv";
import { AppError } from "../errors/AppError";
dotenv.config();

enum EnvironmentKeys {
    NODE_ENV = 'NODE_ENV',
    OPENAI_API_KEY = 'OPENAI_API_KEY',
    PORT = 'PORT',
}

export function get(key: EnvironmentKeys): string {
    const envKey = EnvironmentKeys[key];
    const value = process.env[envKey];
    if(!value) {
        throw new AppError(`Environment variable ${envKey} is not set`);
    }
    return process.env[envKey] as string;
}

export const CONFIGS ={
    NODE_ENV: get(EnvironmentKeys.NODE_ENV),
    APP_NAME:"MCP Service",
    SERVER_PORT: get(EnvironmentKeys.PORT),
    OpenAI:{
        KEY: get(EnvironmentKeys.OPENAI_API_KEY),
        URL: 'https://openrouter.ai/api/v1',
        MODEL: 'meta-llama/llama-3-8b-instruct'
    },
    IS_PRODUCTION: process.env.NODE_ENV === "prod" || process.env.NODE_ENV === "production" ? true : false,
    HTTP_ALLOWED_HEADERS: [
        "Content-Type",
        "Authorization",
        "Origin",
        "Accept",
        "X-Requested-With",
        "x-jwt-token",
        "x-jwt-refresh-token",
        "Content-Length",
        "Accept-Language",
        "Accept-Encoding",
        "Connection",
        "Access-Control-Allow-Origin"
    ],
    HTTP_METHODS:["GET", "PUT", "POST", "DELETE", "OPTIONS"],
    SERVICES_COR_ORIGINS: [
        "http://localhost:3000"
    ]
}