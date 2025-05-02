import "reflect-metadata";
import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ListToolsRequestSchema,
} from "@modelcontextprotocol/sdk/types.js";
import { projectTools } from "./tools/ProjectTools.js";
import { surveyTools } from "./tools/SurveyTools.js";
import { toolControllers } from "./controllers/index.js";

const server = new Server({
  name: "voice-korea-mcp-tool",
  version: "1.0.0",
}, {
  capabilities: {
    tools: {}
  }
});

server.setRequestHandler(ListToolsRequestSchema, async () => {
  const tools = [...projectTools, ...surveyTools];
    return {
        tools
      };
});

server.setRequestHandler(CallToolRequestSchema, async (request) => {
      const { name, arguments: args } = request.params as any;
      try {
        const controllerFn = toolControllers[name];
        if (!controllerFn) {
          return {
            content: [{ type: "text", text: `Error: No matching tool controller found for "${name}"` }]
          };
        }
    
        const response = await controllerFn(args);
        return response;
    
      } catch (error: any) {
        return {
          content: [{ type: "text", text: `Error handling tool "${name}": ${error.message}` }]
        };
      }
});

const transport = new StdioServerTransport();
await server.connect(transport); 