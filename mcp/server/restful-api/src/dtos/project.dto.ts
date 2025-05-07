import { Type } from "class-transformer";
import { IsBoolean, IsNumber, IsOptional, IsString, ValidateNested } from "class-validator";
import { Example } from "tsoa";

export class ContextDTO {
    @IsString()
    user!: string;
  
    @IsString()
    session!: string;
  }

export class Message {
    role!: 'user' | 'system' | 'assistant';
    content!: string;
};
export class AiMessageDTO {
    @IsString()
    role!: string;

    @IsString()
    content!: string;
}

export class AiRequestDataDTO {
    @IsString()
    model!: string;

    @ValidateNested()
    @Type(() => AiMessageDTO)
    messages?: AiMessageDTO[];

    @IsNumber()
    temperature!: number;

    @ValidateNested()
    @Type(() => ContextDTO)
    context?: ContextDTO;
}

export class ProjectListReqDTO {
    @IsString()
    @Example("What is the most recent project")
    prompt!: string;

    @ValidateNested()
    @Example({"user": "Boniface","session":"session_239kidk"})
    @Type(() => ContextDTO)
    context?: ContextDTO;

    @IsNumber()
    @Example(1.5)
    temperature!: number;
}