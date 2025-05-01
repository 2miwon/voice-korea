import "reflect-metadata";
import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ListToolsRequestSchema,
} from "@modelcontextprotocol/sdk/types.js";
import { Container } from "typedi";
import ProjectService from "./services/ProjectService.js";
import { projectTools } from "./tools/ProjectTools.js";

const server = new Server({
  name: "voice-korea-mcp-tool",
  version: "1.0.0",
}, {
  capabilities: {
    tools: {}
  }
});

server.setRequestHandler(ListToolsRequestSchema, async () => {
  const tools = [...projectTools];
    return {
        tools
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

      // Find the surveys in a project
      else if (name === "get_surveys_in_a_project") {
        const { question, id } = args;
        const projectService = Container.get(ProjectService);
        return await projectService.getProjectSurveys(id, question);
      }

      // Find projects by their title
      else if (name === "search_projects_by_title") {
        const { question, title } = args;
        const projectService = Container.get(ProjectService);
        return await projectService.searchProjects(title, question);
      }

      else if(name === "fetch_latest_projects"){
        const { question } = args;
        const projectService = Container.get(ProjectService);
        return await projectService.fetchLatestProjects(question);
      }


      return {
        content: [{ type: "text", text: `Error No matching tool handler found!` }]
      };
});

const transport = new StdioServerTransport();
await server.connect(transport); 