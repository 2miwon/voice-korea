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
        name: "get_surveys_in_a_project",
        description: "Fetch Voice Korea's Project's surveys by the ID of the project",
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
      }
];
