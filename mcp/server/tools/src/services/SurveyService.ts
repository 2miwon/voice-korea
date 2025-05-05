import { Service } from 'typedi';
import { makeApiCall } from '../commons/utils/axios.js';

@Service()
export default class SurveyService {
    public async getProjectSurveys(id: number, question: string)
    {
        try {
            const project = await makeApiCall(`/deliberations/${id}/sample-surveys?param-type=read&action=get-by-id`, { method: 'GET' })
            if (!project) {
              return {
                content: [{ type: "text", text: `No surveys found with project ID ${id}` }]
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

    public async getProjectFinalSurveys(id: number, question: string)
    {
        try {
            const project = await makeApiCall(`/deliberations/${id}/final-surveys?param-type=read&action=get-by-id`, { method: 'GET' })
            if (!project) {
              return {
                content: [{ type: "text", text: `No final survey found with project ID ${id}` }]
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

    public async getProjectFinalSurveyRecommendation(id: number, question: string)
    {
        try {
            const project = await makeApiCall(`/deliberations/${id}/drafts?param-type=read&action=get-by-id`, { method: 'GET' })
            if (!project) {
              return {
                content: [{ type: "text", text: `No final survey recommendation found with project ID ${id}` }]
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