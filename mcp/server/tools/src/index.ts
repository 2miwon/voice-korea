import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ListToolsRequestSchema,
} from "@modelcontextprotocol/sdk/types.js";
import { Container } from "typedi";
import ProjectService from "./services/projectService.js";

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
          name: "get_project_by_id",
          description: "Fetch Voice Korea Project by its ID",
          inputSchema: {
            type: "object",
            properties: {
              question: { type: "string" },
              id: { type: "number" }
            },
            required: ["question","id"]
          }
        }
      ]
      };
});

server.setRequestHandler(CallToolRequestSchema, async (request) => {
      const { name, arguments: args } = request.params as any;
      // Find a project by its ID
      if (name === "get_project_by_id") {
        const { question, id } = args;
        const projectService = Container.get(ProjectService);
        return await projectService.getProjectById(id, question);
      }

      else if (name === "get_surveys_in_a_project") {
        const { question, id } = args;
        const projectService = Container.get(ProjectService);
        return await projectService.getProjectSurveys(id, question);
      }
      return {
        content: [{ type: "text", text: `Error No matching tool handler found!` }]
      };
});

const transport = new StdioServerTransport();
await server.connect(transport); 