use models::{
    deliberation_response::DeliberationResponse, response::Answer, ParsedQuestion, Question,
};

pub fn group_responses_by_question(
    questions: &Vec<Question>,
    responses: &Vec<DeliberationResponse>,
) -> Vec<(String, ParsedQuestion)> {
    let mut parsed_questions: Vec<(String, ParsedQuestion)> = questions
        .iter()
        .map(|question| match question.clone() {
            Question::MultipleChoice(inner) => {
                let title = inner.title;
                let options = inner.options;
                let count = vec![0; options.len()];
                (
                    title,
                    ParsedQuestion::MultipleChoice {
                        answers: options,
                        response_count: count,
                    },
                )
            }
            Question::SingleChoice(inner) => {
                let title = inner.title;
                let options = inner.options;
                let count = vec![0; options.len()];
                (
                    title,
                    ParsedQuestion::SingleChoice {
                        answers: options,
                        response_count: count,
                    },
                )
            }
            Question::ShortAnswer(inner) => {
                (inner.title, ParsedQuestion::ShortAnswer { answers: vec![] })
            }
            Question::Subjective(inner) => {
                (inner.title, ParsedQuestion::Subjective { answers: vec![] })
            }
        })
        .collect();

    for responses in responses.iter() {
        for (i, answer) in responses.answers.iter().enumerate() {
            parsed_questions.get_mut(i).map(|(_, parsed_question)| {
                match parsed_question {
                    ParsedQuestion::SingleChoice { response_count, .. } => {
                        if let Answer::SingleChoice { answer } = answer {
                            response_count[*answer as usize] += 1;
                        }
                    }
                    ParsedQuestion::MultipleChoice { response_count, .. } => {
                        if let Answer::MultipleChoice { answer } = answer {
                            for ans in answer {
                                response_count[*ans as usize] += 1;
                            }
                        }
                    }
                    ParsedQuestion::ShortAnswer { answers } => {
                        if let Answer::ShortAnswer { answer } = answer {
                            answers.push(answer.clone());
                        }
                    }
                    ParsedQuestion::Subjective { answers } => {
                        if let Answer::Subjective { answer } = answer {
                            answers.push(answer.clone());
                        }
                    }
                };
            });
        }
    }
    parsed_questions
}
