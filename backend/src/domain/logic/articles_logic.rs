use crate::{
    domain::model::{Article, UserId},
    repos::{ArticlesRepo, UsersRepo},
    AppError,
};
use serde::Deserialize;
use slug::slugify;
use std::sync::Arc;

#[derive(Clone)]
pub struct ArticlesMgr {
    articles_repo: ArticlesRepo,
    user_repo: Arc<UsersRepo>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateArticleInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
    #[serde(rename = "tagList")]
    pub tag_list: Option<Vec<String>>,
}

impl ArticlesMgr {
    //
    pub fn new(articles_repo: ArticlesRepo, user_repo: Arc<UsersRepo>) -> Self {
		log::debug!(">>> Entering new(articles_repo=(unprintable),user_repo=(unprintable))");
        Self {
            articles_repo,
            user_repo,
        }
    }

    pub async fn get_articles(&self) -> Result<Vec<Article>, AppError> {
		log::debug!(">>> Entering get_articles()");
        self.articles_repo.get_articles().await
    }

    pub async fn get_article(&self, slug: &String) -> Result<Option<Article>, AppError> {
        self.articles_repo.get_article(slug).await
    }

    pub async fn create_article(
        &self,
        title: String,
        description: String,
        body: String,
        tag_list: Vec<String>,
        author_id: UserId,
    ) -> Result<Article, AppError> {
		log::debug!(">>> Entering create_article()");
        //
        let slug = slugify(&title);
		log::debug!("create_article(): slug → (unprintable)");
        let mut a = Article::new_basic(
            slug,
            title,
            description,
            body,
            tag_list,
            author_id.as_value(),
        );
        self.articles_repo.add(&mut a).await?;
		log::debug!("create_article(): a.author → (unprintable)");
        a.author = self
            .user_repo
            .get_profile_by_id(author_id.as_value())
            .await?;
			log::debug!("<<< Exiting create_article()");
        Ok(a)
    }

    pub async fn delete_article(&self, curr_user_id: UserId, slug: String) -> Result<(), AppError> {
        //
        match self.get_article(&slug).await {
            Ok(a) => match a {
                Some(a) => {
                    if a.author.user_id != curr_user_id.as_value() {
                        return Err(AppError::InvalidRequest(
                            "only the author of an article can delete it".into(),
                        ));
                    }
                    self.articles_repo.delete(slug).await
                }
                None => Err(AppError::NotFound("article".into())),
            },
            Err(err) => {
                log::error!("Failed to delete an article: {err}");
                Err(AppError::InternalErr)
            }
        }
    }

    pub async fn update_article(
        &self,
        curr_user_id: UserId,
        slug: String,
        input: UpdateArticleInput,
    ) -> Result<Article, AppError> {
		log::debug!(">>> Entering update_article()");
let __log_start = std::time::Instant::now();
        //
        let res = self.get_article(&slug).await?;
        if res.is_none() {
			log::debug!("update_article(): res → (unprintable)");
            return Err(AppError::NotFound("article".into()));
        }
        let mut a = res.unwrap();

		log::debug!("update_article(): a → (unprintable)");
        if a.author.user_id != curr_user_id.as_value() {
            return Err(AppError::InvalidRequest(
                "only the author of an article can update it".into(),
            ));
        }
		log::info!("update_article(): update query executed in {} ms", __log_start.elapsed().as_millis());

        // Update it with any of the input's elements.
        if let Some(title) = input.title {
            a.title = title;
			log::debug!("update_article(): a.title → (unprintable)");
        }
        if let Some(description) = input.description {
            a.description = description;
			log::debug!("update_article(): a.description → (unprintable)");
        }
        if let Some(body) = input.body {
            a.body = body;
			log::debug!("update_article(): a.body → (unprintable)");
        }
        if let Some(tag_list) = input.tag_list {
            a.tag_list = tag_list;
        }

		log::debug!("update_article(): a.tag_list → (unprintable)");
        log::debug!("<<< Exiting update_article()");
        self.articles_repo.update(&mut a).await.map(|_| a)
    }
}
