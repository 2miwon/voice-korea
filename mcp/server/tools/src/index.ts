import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ErrorCode,
  ListToolsRequestSchema,
  McpError,
} from "@modelcontextprotocol/sdk/types.js";
import axios from 'axios';
import { makeApiCall } from "./commons/utils/axios.js";

const server = new Server({
  name: "voice-korea-mcp-tool",
  version: "1.0.0",
}, {
  capabilities: {
    tools: {}
  }
});

server.setRequestHandler(ListToolsRequestSchema, async () => {
    return {
        tools: [
          {
          name: "calculate_sum",
          description: "Add two numbers together",
          inputSchema: {
            type: "object",
            properties: {
              a: { type: "number" },
              b: { type: "number" }
            },
            required: ["a", "b"]
          }
        },
        {
          name: "search_voice_korea_projects",
          description: "Search Voice Korea Projects",
          inputSchema: {
            type: "object",
            properties: {
              question: { type: "string" }
            },
            required: ["question"]
          }
        }
      ]
      };
});

server.setRequestHandler(CallToolRequestSchema, async (request) => {
    if (request.params.name === "calculate_sum") {
        const { a, b } = request.params.arguments as any;
        if (typeof a !== 'number' || typeof b !== 'number') {
          throw new Error("Invalid arguments: a and b must be numbers");
        }
        return { toolResult: a + b };
      }
      try {
        if (request.params.name === "search_voice_korea_projects") {
          const { data } = await makeApiCall(`/landing?param-type=read&action=find-one`, { method: 'GET' })
          const listOfProjects = data.projects || [];
          return {
            content: [
              {
                type: "text",
                text: JSON.stringify(listOfProjects, null, 2)
              }
            ]
          };
        }
        throw new Error("Tool not defined");
      } catch (error) {
        throw new Error("Failed to fetch projects");
      }
});

const transport = new StdioServerTransport();
await server.connect(transport); 