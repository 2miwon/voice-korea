import { Tool } from "@modelcontextprotocol/sdk/types.js";

export const projectTools: Tool[] = [
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
      },
      {
        name: "search_projects_by_title",
        description: "Fetch Voice Korea's Projects by their title",
        inputSchema: {
          type: "object",
          properties: {
            question: { type: "string" },
            title: { type: "number" }
          },
          required: ["question","title"]
        }
      },
      {
        name: "fetch_latest_projects",
        description: "Fetch Most recent Voice Korea Projects",
        inputSchema: {
          type: "object",
          properties: {
            question: { type: "string" },
          },
          required: ["question"]
        }
      },
];
