import dotenv from "dotenv";
import { AppError } from "../errors/AppError.js";
dotenv.config();

enum EnvironmentKeys {
    NODE_ENV = 'NODE_ENV',
    OPENAI_API_KEY = 'OPENAI_API_KEY',
    PORT = 'PORT',
    BASE_API_URL='BASE_API_URL'
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
    BASE_API_URL: get(EnvironmentKeys.BASE_API_URL),
    IS_PRODUCTION: process.env.NODE_ENV === "prod" || process.env.NODE_ENV === "production" ? true : false,
}