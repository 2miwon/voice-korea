use crate::service::user_service::UserService;
use bdk::prelude::*;
use indexmap::IndexMap;
use models::{
    deliberation_comment::{
        DeliberationComment, DeliberationCommentQuery, DeliberationCommentSummary,
    },
    deliberation_project::DeliberationProject,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CommentTree {
    pub id: i64,
    pub created_at: i64,
    pub updated_at: i64,

    pub comment: String,
    pub parent_id: i64,

    pub replies: i64,
    pub likes: i64,
    pub liked: bool,
    pub nickname: Option<String>,
    pub children: Vec<CommentTree>,
}

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    #[allow(dead_code)]
    id: ReadOnlySignal<i64>,

    #[allow(dead_code)]
    summary: Resource<DeliberationProject>,

    pub comments: Resource<Vec<DeliberationCommentSummary>>,

    pub comment_trees: Signal<Vec<CommentTree>>,

    pub user: UserService,
}

impl Controller {
    pub fn init(lang: Language, id: ReadOnlySignal<i64>) -> std::result::Result<Self, RenderError> {
        let user: UserService = use_context();

        let summary = use_server_future(move || {
            let id = id();

            async move {
                let endpoint = crate::config::get().api_url;
                DeliberationProject::get_client(endpoint)
                    .get(id)
                    .await
                    .unwrap_or_default()
            }
        })?;

        let comments = use_server_future(move || {
            let id = id();

            async move {
                let endpoint = crate::config::get().api_url;
                DeliberationComment::get_client(endpoint)
                    .query(
                        id,
                        DeliberationCommentQuery {
                            //FIXME: use pagination
                            size: 100,
                            bookmark: None,
                            action: None,
                            parent_id: None,
                        },
                    )
                    .await
                    .unwrap_or_default()
                    .items
            }
        })?;

        let mut ctrl = Self {
            lang,
            id,
            summary,
            comments,

            comment_trees: use_signal(|| vec![]),
            user,
        };

        use_effect(move || {
            let comments = ctrl.parsing_comments(comments().unwrap_or_default());
            ctrl.comment_trees.set(comments);
        });

        use_context_provider(|| ctrl);

        Ok(ctrl)
    }

    pub fn parsing_comments(&self, comments: Vec<DeliberationCommentSummary>) -> Vec<CommentTree> {
        let mut map: IndexMap<i64, CommentTree> = IndexMap::new();
        let mut roots = Vec::new();

        for comment in comments.into_iter() {
            let nickname = if let Some(user) = comment.user.get(0) {
                user.nickname.clone()
            } else {
                None
            };
            map.insert(
                comment.id,
                CommentTree {
                    id: comment.id,
                    created_at: comment.created_at,
                    updated_at: comment.updated_at,

                    comment: comment.comment,
                    parent_id: comment.parent_id,
                    nickname,
                    replies: comment.replies,
                    likes: comment.likes,
                    liked: comment.liked,
                    children: Vec::new(),
                },
            );
        }

        let mut orphan_children = Vec::new();
        let mut parent_child_pairs = Vec::new();

        for comment in map.values() {
            if comment.parent_id == 0 {
                roots.push(comment.clone());
            } else {
                parent_child_pairs.push((comment.parent_id, comment.clone()));
            }
        }

        for (parent_id, child) in parent_child_pairs.clone() {
            if let Some(parent) = map.get_mut(&parent_id) {
                parent.children.push(child);
            } else {
                orphan_children.push(child);
            }
        }

        roots = map
            .values()
            .filter(|comment| comment.parent_id == 0)
            .cloned()
            .collect();

        roots.extend(orphan_children);

        roots
    }

    pub async fn like_comment(&mut self, id: i64) {
        let user_id = (self.user.user_id)();

        if user_id == 0 {
            btracing::error!("login is required");
            return;
        }

        let project_id = self.id();
        match DeliberationComment::get_client(&crate::config::get().api_url)
            .like(project_id, id)
            .await
        {
            Ok(_) => {
                self.comments.restart();
            }
            Err(e) => {
                btracing::error!("like comment failed with error: {:?}", e);
            }
        };

        self.comments.restart();
    }

    pub async fn send_comment(&mut self, comment: String) {
        let user_id = (self.user.user_id)();

        if user_id == 0 {
            btracing::error!("login is required");
            return;
        }

        let project_id = self.id();
        match DeliberationComment::get_client(&crate::config::get().api_url)
            .comment(project_id, comment)
            .await
        {
            Ok(_) => {
                self.comments.restart();
            }
            Err(e) => {
                btracing::error!("send comment failed with error: {:?}", e);
            }
        };

        self.comments.restart();
    }

    pub async fn send_reply(&mut self, comment_id: i64, reply: String) {
        let user_id = (self.user.user_id)();

        if user_id == 0 {
            btracing::error!("login is required");
            return;
        }

        let project_id = self.id();
        match DeliberationComment::get_client(&crate::config::get().api_url)
            .reply_to_comment(project_id, comment_id, reply)
            .await
        {
            Ok(_) => {
                self.comments.restart();
            }
            Err(e) => {
                btracing::error!("send reply failed with error: {:?}", e);
            }
        };
    }
}
