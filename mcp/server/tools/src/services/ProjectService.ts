import { Service } from 'typedi';
import { makeApiCall } from '../commons/utils/axios.js';

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

}