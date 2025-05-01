import { Service } from 'typedi';
import { makeApiCall } from '../commons/utils/axios.js';
import { CONFIGS } from '../commons/configs/index.js';

@Service()
export default class ProjectService {
    constructor(
    ){}
    public async getProjectById(id: number, question: string)
    {
        try {
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
        } catch (error: any) {
            return {
              content: [{ type: "text", text: `Error fetching project: ${error.message}` }]
            };
          }
    }

    public async getProjectSurveys(id: number, question: string)
    {
        try {
            const project = await makeApiCall(`/projects/deliberations/${id}/sample-surveys?param-type=read&action=get-by-id`, { method: 'GET' })
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
        } catch (error: any) {
            return {
              content: [{ type: "text", text: `Error fetching project: ${error.message}` }]
            };
          }
    }

    public async searchProjects(title: string, question: string)
    {
        try {
          const maxLimit = CONFIGS.MAX_LIMIT
            const project = await makeApiCall(`/projects?action=search&bookmark=1&size=${maxLimit}&title=${title}&param-type=query`, { method: 'GET' })
            if (!project.items) {
              return {
                content: [{ type: "text", text: `No project found with title ${title}` }]
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
        } catch (error: any) {
            return {
              content: [{ type: "text", text: `Error fetching project: ${error.message}` }]
            };
          }
    }

    public async fetchLatestProjects(question: string)
    {
        try {
          const maxLimit = CONFIGS.MAX_LIMIT
            const project = await makeApiCall(`/projects?size=${maxLimit}&param-type=query`, { method: 'GET' })
            if (!project.items) {
              return {
                content: [{ type: "text", text: `No projects found!` }]
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
        } catch (error: any) {
            return {
              content: [{ type: "text", text: `Error fetching project: ${error.message}` }]
            };
          }
    }
}