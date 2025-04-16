import { Inject, Service } from 'typedi';
import { Controller, Route, Post, Security, Request, Tags, Example, Get, Path, Delete } from "tsoa";
// import { serverErrorResponse, successResponse } from '../common/helpers/responseHandlers';
import { Logger } from '../commons/configs/winston_logger';
@Tags("Withdrawal Bank Accounts")
@Route("withdrawal-accounts")
@Service()
export class WithdrawalAccountController extends Controller {
    constructor(
        @Inject(()=> Logger) private readonly logger: Logger,
        // private readonly withdrawalAccountService: WithdrawalAccountService,
    ){
        super()
        this.logger = new Logger(WithdrawalAccountController.name);
    }

    @Post("/")
    public async createNewWithdrawalAccount(@Request() req: any)
    // : Promise<CustomApiResponse>
    {
        try {
            // const user_id = req.authId;
            // const createAccountData: CreateWithdrawalAccountDTO ={
            //     userId: req.userId,
            //     bankName: req.bankName,
            //     bankCode: req.bankCode,
            //     accountNumber: req.accountNumber,
            //     accountName: req.accountName,
            //     currency: req.currency
            // }
            // const newWallet = await this.withdrawalAccountService.addWithdrawalAccount(user_id, createAccountData);
            // const { message, account } = newWallet;
            // this.logger.info({
            //     activity_type: ACTIVITY_TYPES.WITHDRAWAL_ACCOUNT.CREATION,
            //     message,
            //     metadata: {
            //         account: {
            //             id: account?.id
            //         }
            //     }
            // });
                
            //     if (newWallet.isSuccess) {
            //         if (newWallet.isSuccess) {
            //             this.setStatus(201)
            //             return successResponse(message as string, account, 201)
            //         }
            //     }
                this.setStatus(400);
            // return successResponse(message as string)
        } catch (error: any) {
            //    this.logger.error({
            //     activity_type: ACTIVITY_TYPES.WITHDRAWAL_ACCOUNT.CREATION,
            //     message: error.message,
            //     metadata: {}
            // });
            //      return serverErrorResponse(MESSAGES.COMMON.INTERNAL_SERVER_ERROR);
            // }
    }

}
    
}