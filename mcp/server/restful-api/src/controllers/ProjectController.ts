import { Inject, Service } from 'typedi';
import { Controller, Route, Post, Request, Tags, Body } from "tsoa";
import { serverErrorResponse, successResponse } from '../commons/helpers/responseHanders';
import { Logger } from '../commons/configs/winston_logger';
import { CustomApiResponse } from '../dtos/common.dto';
import ProjectService from '../services/ProjectService';
import { CONFIGS } from '../commons/configs';
import { AiMessageDTO, AiRequestDataDTO, ProjectListReqDTO } from '../dtos/project.dto';
@Tags("Projects")
@Route("projects")
@Service()
export class WithdrawalAccountController extends Controller {
    constructor(
        @Inject(()=> Logger) private readonly logger: Logger,
        private readonly projectService: ProjectService,
    ){
        super()
        this.logger = new Logger(WithdrawalAccountController.name);
    }

    @Post("/")
    public async createNewWithdrawalAccount(@Body() req: ProjectListReqDTO)
    : Promise<CustomApiResponse>
    {
        const { prompt, temperature, context } = req;

        try {            
            const res = await this.projectService.queryFetchedProjects(prompt)
            this.setStatus(200);
            const message = "";
            return successResponse(message,res.data)
        } catch (error: any) {
            return serverErrorResponse("Something went wrong!");
    }

}
    
}