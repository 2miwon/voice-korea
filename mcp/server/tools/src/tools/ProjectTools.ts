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
            title: { type: "string" }
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
      {
        name: "fetch_project_deliberations",
        description: "Fetch the deliberations under a Voice Korea's Project using the projects ID",
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
        name: "fetch_project_discussions",
        description: "Fetch the discussions under a Voice Korea's Project using the projects ID",
        inputSchema: {
          type: "object",
          properties: {
            question: { type: "string" },
            id: { type: "number" }
          },
          required: ["question","id"]
        }
      },
];
