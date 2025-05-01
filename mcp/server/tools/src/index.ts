import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ErrorCode,
  ListToolsRequestSchema,
  McpError,
} from "@modelcontextprotocol/sdk/types.js";
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
      try {
        if (name === "get_project_by_id") {
          const { question, id } = args;
          const project = await makeApiCall(`/projects/${id}`, { method: 'GET' })
          if (!project) {
            return {
              content: [{ type: "text", text: `No project found with ID ${id}` }]
            };
          }

          return {
            content: [
              {
                type: "text",
                text: `question asked is: ${question}, matching data project data is: ${JSON.stringify(project, null, 2)}`
              }
            ]
          };
        }
        return {
          content: [{ type: "text", text: `Error No matching tool handler found!` }]
        };
      } catch (error: any) {
        return {
          content: [{ type: "text", text: `Error fetching project: ${error.message}` }]
        };
      }
});

const transport = new StdioServerTransport();
await server.connect(transport); 