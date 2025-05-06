use dioxus_translate::translate;

translate! {
    Translate;

    text: {
        ko: "메인 페이지",
        en: "Main Page"
    }
}

translate! {
    ReviewSectionTranslate;

    participation_review: {
        ko: "참여 후기",
        en: "Participation Review"
    }

    anonymity: {
        ko: "익명",
        en: "Anonymity"
    }
}

translate! {
    InquirySectionTranslate;

    inquiry_title: {
        ko: "공정하고 투명한 의사결정을 위한 공론조사 솔루션, VOICE KOREA",
        en: "Deliberation survey solution for fair and transparent decision-making, VOICE KOREA"
    }

    inquiry_description: {
        ko: "VOICE KOREA는 DAO 블록체인 기술을 활용하여, 데이터 조작 없이 신뢰할 수 있는 공론조사를 제공합니다.\n귀 기관에 최적화된 공론조사 시스템을 원하신다면, 지금 문의하세요!",
        en: "VOICE KOREA utilizes DAO blockchain technology to provide reliable deliberation surveys without data manipulation.\nIf you want a deliberation survey system optimized for your organization, contact us now!"
    }

    blockchain_info_1: {
        ko: "DAO 기반의 분산형 의사결정",
        en: "DAO-based decentralized decision-making"
    }
    blockchain_info_2: {
        ko: "데이터 조작 걱정 없는 블록체인 기술",
        en: "Blockchain technology without worrying about data manipulation"
    }
    blockchain_info_3: {
        ko: "소통을 위한 직관적인 인터페이스",
        en: "Intuitive interface for communication"
    }

    name: {
        ko: "이름",
        en: "Name"
    }
    name_hint: {
        ko: "이름을 입력해주세요.",
        en: "Please Enter Your Name"
    }

    email: {
        ko: "이메일",
        en: "Email"
    }
    email_hint: {
        ko: "이메일을 입력해주세요.",
        en: "Please Enter Your Email"
    }
    email_error: {
        ko: "이메일 형식에 맞게 입력해주세요.",
        en: "Please enter your email address in the correct email format."
    }

    message: {
        ko: "메세지",
        en: "Message"
    }
    message_hint: {
        ko: "문의하고 싶은 내용을 입력해주세요.",
        en: "Please enter the information you would like to inquire about."
    }
    message_error: {
        ko: "문의 내용을 입력해주세요.",
        en: "Please enter your inquiry details."
    }

    inquiry: {
        ko: "문의하기",
        en: "Inquiry"
    }
}

translate! {
    OpinionInstitutionTranslate;

    institution: {
        ko: "정책 결정 기관",
        en: "Policy Making Institution"
    }
    institution_description: {
        ko: "정책 결정 기관은 중요한 문제를 논의하고 정책을 결정하는 곳으로, 여러분의 의견을 반영한 공론조사를 진행합니다. ",
        en: "The policy-making body is a place where important issues are discussed and policies are decided, and deliberation surveys are conducted that reflect your opinions."
    }
}

translate! {
    PriceSectionTranslate;

    free_title: {
        ko: "Free",
        en: "Free"
    }

    free_description: {
        ko: "무료로 공론조사에 참여하고, 사회적 의사결정에 기여하세요. 투명한 블록체인 시스템에서 당신의 의견을 안전하게 제출할 수 있습니다.",
        en: "Participate in deliberation surveys for free and contribute to social decision-making. You can safely submit your opinions in a transparent blockchain system."
    }

    won: {
        ko: "원",
        en: "Won"
    }

    free_info_label_1: {
        ko: "기본 공론조사 참여",
        en: "Participation in basic deliberation surveys"
    }
    free_info_description_1: {
        ko: "사회적 이슈에 의견을 제출할 수 있습니다.",
        en: "You can submit your opinion on social issues."
    }
    free_info_label_2: {
        ko: "투명한 블록체인 기록",
        en: "Transparent blockchain records"
    }
    free_info_description_2: {
        ko: "결과는 블록체인에 기록되어 안전하고 투명합니다.",
        en: "Results are recorded on the blockchain, making them secure and transparent."
    }
    free_info_label_3: {
        ko: "기본 보상",
        en: "basic reward"
    }
    free_info_description_3: {
        ko: "참여한 조사에 소량의 보상 지급",
        en: "A small amount of compensation is paid for participating in the survey."
    }
    start: {
        ko: "시작하기",
        en: "Start"
    }

    premium_title: {
        ko: "Premium",
        en: "Premium"
    }

    premium_description: {
        ko: "프리미엄 서비스로 더 많은 공론조사에 참여하고, 심층 분석 리포트와 더 큰 보상을 받으세요. 더 나은 사회적 결정을 함께 만들어갈 수 있습니다.",
        en: "Participate in more deliberation polls, receive in-depth analysis reports, and receive greater rewards with our premium service. We can make better social decisions together."
    }

    premium_info_label_1: {
        ko: "무제한 공론조사 참여",
        en: "Participation in unlimited deliberation surveys"
    }
    premium_info_description_1: {
        ko: "유료 사용자는 다양한 중요 이슈에 대한 무제한 참여가 가능하며, 특별한 조사에도 참여할 수 있습니다.",
        en: "Paid users have unlimited participation in a variety of important issues and can participate in special surveys."
    }

    premium_info_label_2: {
        ko: "전문가 분석 레포트 제공",
        en: "Provides expert analysis reports"
    }
    premium_info_description_2: {
        ko: "조사 결과에 대한 심층 분석 및 맞춤형 보고서를 제공하여, 유저가 의사결정을 더 잘 할 수 있도록 돕습니다.",
        en: "We provide in-depth analysis of survey results and customized reports to help users make better decisions."
    }

    premium_info_label_3: {
        ko: "높은 참여 보상",
        en: "High participation rewards"
    }
    premium_info_description_3: {
        ko: "유료 사용자는 공론조사 참여에 대해 더 많은 보상을 받으며, 이는 블록체인 기반의 토큰으로 지급됩니다.",
        en: "Paid users receive more compensation for participating in deliberation polls, which are paid in blockchain-based tokens."
    }

    premium_info_label_4: {
        ko: "모든 고급 기능 사용 가능",
        en: "All advanced features available"
    }
    premium_info_description_4: {
        ko: "유료 서비스 이용자는 데이터 분석 도구, 심층 투표 시스템 등 모든 고급 기능에 접근하여 더 깊이 있는 참여가 가능합니다.",
        en: "Paid users have access to all advanced features, including data analysis tools and an in-depth voting system, for deeper engagement."
    }
}

translate! {
    InstitutionBoxTranslate;

    project: {
        ko: "프로젝트",
        en: "Project"
    }
    vote: {
        ko: "투표",
        en: "Vote"
    }
}

translate! {
    OpinionProjectTranslate;

    project: {
        ko: "프로젝트",
        en: "Project"
    }
    project_description: {
        ko: "여러분의 의견이 정책에 반영될 수 있도록 진행된 공론조사 프로젝트 목록입니다. 함께 살펴보고, 어떤 주제들이 논의되었는지 확인해 보세요.",
        en: "This is a list of deliberation survey projects conducted to ensure that your opinions are reflected in policies. Take a look together and see what topics were discussed."
    }
}

translate! {
    MoreButtonTranslate;

    more: {
        ko: "더보기",
        en: "More"
    }
}

translate! {
    ProjectBoxTranslate;

    participant: {
        ko: "참여자",
        en: "Participant"
    }

    vote: {
        ko: "투표",
        en: "Vote"
    }

    detail: {
        ko: "자세히 보기",
        en: "See Details"
    }
}

translate! {
    OpinionFeatureTranslate;

    title: {
        ko: "VOICE KOREA의 공론 조사 특징",
        en: "Characteristics of VOICE KOREA’s deliberation survey"
    }

    sub_title_1: {
        ko: "신뢰할 수 있는 분산화 시스템",
        en: "Trustworthy Decentralized System"
    }
    sub_description_1: {
        ko: "블록체인 기술로 모든 결정과 참여가 분산되어 저장되고 공개됩니다. 이는 누구도 부당하게 조작할 수 없다는 것을 보장합니다. 여러분의 목소리는 안전하게 기록됩니다.",
        en: "With blockchain technology, all decisions and participation are distributed, stored, and made public. This ensures that no one can tamper with it. Your voice is safely recorded."
    }

    sub_title_2: {
        ko: "투명하고 변하지 않는 기록",
        en: "Transparent and unchanging record"
    }
    sub_description_2: {
        ko: "여러분이 남긴 의견이나 선택은 블록체인에 기록되어 언제든지 확인할 수 있습니다. 이로써 모든 과정이 투명하고 신뢰할 수 있게 보장됩니다.",
        en: "Any comments or choices you make are recorded on the blockchain and can be checked at any time. This ensures that the entire process is transparent and trustworthy."
    }

    sub_title_3: {
        ko: "모두가 평등하게 참여하는 환경",
        en: "An environment where everyone participates equally"
    }
    sub_description_3: {
        ko: "DAO는 중앙 권력 없이 모두가 평등하게 의견을 낼 수 있는 환경을 제공합니다. 이로 인해 모든 참여자가 의사결정에 영향력을 행사할 수 있습니다.",
        en: "DAO provides an environment where everyone can express their opinions equally without a central authority. This allows all participants to have influence on decisions."
    }

    description: {
        ko: "DAO 블록체인 기술을 통해 모든 결정은 투명하고 신뢰할 수 있는 방식으로 이루어지며, 사용자의 의견은 안전하게 기록합니다.\nVOICE KOREA는 중앙 권력 없이도 모든 참여자들은 평등하게 의견을 나누고, 중요한 사회적 결정에 참여할 수 있는 공간을 제공합니다.",
        en: "Through DAO blockchain technology, all decisions are made in a transparent and trustworthy manner, and user opinions are safely recorded.\nVOICE KOREA provides a space where all participants can share opinions equally and participate in important social decisions without a central authority."
    }
}

translate! {
    MainBannerTranslate;

    title: {
        ko: "당신의 목소리가 변화를 이끕니다.",
        en: "Your voice drives change."
    }

    description: {
        ko: "보이스코리아에서 당신의 의견은 단순한 소리가 아닌, 사회를 바꾸는 힘이 됩니다.\n함께 목소리를 낼 때, 우리는 더 나은 미래를 만들어갈 수 있습니다.",
        en: "At Voice Korea, your opinion is not just a voice, it becomes the power to change society.\nWhen we speak out together, we can create a better future."
    }

    button: {
        ko: "공론 조사 만들기",
        en: "Create a deliberation survey"
    }
}

translate! {
    GuideLineTranslate;

    guideline: {
        ko: "가이드라인",
        en: "Guideline"
    }

    guideline_desc: {
        ko: "공론조사 참여 및 설계 콘솔에 대한 자세한 가이드를 다운로드하실 수 있습니다. 해당 가이드를 통해 플랫폼 사용법과 공론 조사 설계 방법을 쉽게 이해할 수 있습니다.\n파일을 다운로드하여 공론조사와 설계 콘솔에 대한 중요한 정보를  확인하고, 참여 및 설계를 더욱 효과적으로 진행하세요.",
        en: "You can download detailed guides on participating in deliberation surveys and using the design console. Through this guide, you can easily understand how to use the platform and design deliberation surveys.\nDownload the file to check important information about the deliberation survey and design console, and participate and design more effectively."
    }

    public_opinion_participation_guide: {
        ko: "공론 조사 참여 가이드",
        en: "Deliberation Participation Guide"
    }

    public_opinion_survey_design_console_guide: {
        ko: "공론 조사 설계 콘솔 가이드",
        en: "Deliberation Survey Design Console Guide"
    }
}
