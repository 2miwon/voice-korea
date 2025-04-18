export class CommonResponseDTO {
    isSuccess!: boolean;
    data?: any;
    message!: string|null;
}

export interface CustomApiResponse{
    message?: string;
    data?: any, 
    status_code: number;
    }