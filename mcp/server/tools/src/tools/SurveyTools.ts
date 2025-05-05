import { Tool } from "@modelcontextprotocol/sdk/types.js";

export const surveyTools: Tool[] = [
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
        name: "get_final_surveys_in_a_project",
        description: "Fetch Voice Korea's Project's final surveys by the ID of the project",
        inputSchema: {
          type: "object",
          properties: {
            question: { type: "string" },
            id: { type: "number" }
          },
          required: ["question","id"]
        }
      }
];
