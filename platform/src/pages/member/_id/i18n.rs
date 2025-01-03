use crate::utils::context::Language;

pub struct MemberDetailTranslate {
    pub organization_management: String,
    pub team_member_management: String,
    pub see_detail: String,
    pub register_date: String,

    pub privacy: String,
    pub name: String,
    pub group: String,
    pub role: String,
    pub email: String,
    pub save: String,
    pub remove_team_member: String,

    pub participation_record: String,
    pub search_info: String,
    pub item: String,
    pub project: String,
    pub panel: String,
    pub period: String,
    pub status: String,
}

pub fn translate(lang: Language) -> MemberDetailTranslate {
    match lang {
        Language::En => MemberDetailTranslate {
            organization_management: "Organization Management".to_string(),
            team_member_management: "Team Member Management".to_string(),
            see_detail: "See Detail".to_string(),
            register_date: "Register Date".to_string(),

            privacy: "Privacy".to_string(),
            name: "Name".to_string(),
            group: "Group".to_string(),
            role: "Role".to_string(),
            email: "Email".to_string(),
            save: "Save".to_string(),
            remove_team_member: "Remove Team Member".to_string(),

            participation_record: "Participation Record".to_string(),
            search_info: "Please search the project name.".to_string(),
            item: "Item".to_string(),
            project: "Project".to_string(),
            panel: "Panel".to_string(),
            period: "Period".to_string(),
            status: "Status".to_string(),
        },
        Language::Ko => MemberDetailTranslate {
            organization_management: "조직 관리".to_string(),
            team_member_management: "팀원 관리".to_string(),
            see_detail: "자세히 보기".to_string(),
            register_date: "등록된 날짜".to_string(),

            privacy: "개인정보".to_string(),
            name: "이름".to_string(),
            group: "그룹".to_string(),
            role: "역할".to_string(),
            email: "이메일".to_string(),
            save: "저장하기".to_string(),
            remove_team_member: "팀원 삭제하기".to_string(),

            participation_record: "참여 기록".to_string(),
            search_info: "프로젝트명을 검색해주세요.".to_string(),
            item: "항목".to_string(),
            project: "프로젝트".to_string(),
            panel: "패널".to_string(),
            period: "기간".to_string(),
            status: "상태".to_string(),
        },
    }
}
