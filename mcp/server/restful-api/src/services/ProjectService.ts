import { Service } from 'typedi';
// import { dynamic_messages, MESSAGES } from '../commons/constants/messages';
import { AppError } from '../commons/errors/AppError';
import axios from 'axios';
import { OpenAI } from 'openai';
import { CommonResponseDTO } from '../dtos/common.dto';
import { CONFIGS } from '../commons/configs';
import { AiRequestDataDTO } from '../dtos/project.dto';

@Service()
export default class ProjectService {
    constructor(
    ){}
    public async queryFetchedProjects(prompt: string)
    : Promise<CommonResponseDTO> 
    {
        const openai = new OpenAI({
          apiKey: CONFIGS.OpenAI.KEY,
          baseURL: CONFIGS.OpenAI.URL
        });
        try {
          const apiResponse = await axios.get("https://voice-korea-api.dev.voice-korea.com/v2/landing?param-type=read&action=find-one")
          const listOfProjects = apiResponse.data.projects

          const summary = await openai.chat.completions.create({
                model: CONFIGS.OpenAI.MODEL,
                messages: [
                  {
                    role: "system",
                    content: "You are an assistant that only returns short, direct answers. No extra words."
                  },
                  {
                    role: "user",
                    content: `Here is a list of projects: ${JSON.stringify(listOfProjects)}\n\nQuestion: ${prompt}\nAnswer only:`
                  }
                ],
                temperature:0
              } as any);
              
          return {
            isSuccess: true,
            message: null,
            data: summary.choices[0].message
          }
          } catch (err: any) {
            console.log({err})
            throw new AppError("Something went wrong!");
          }
    }
}