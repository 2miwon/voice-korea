import { projectController } from "./ProjectController.js";
import { surveyController } from "./SurveyController.js";

export const toolControllers = {
  ...projectController,
  ...surveyController
};
