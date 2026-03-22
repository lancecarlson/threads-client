use reqwest::Client;

use crate::error::{Result, ThreadsError};
use crate::types::*;

const BASE_URL: &str = "https://graph.threads.net/v1.0";

pub struct ThreadsClient {
    http: Client,
    access_token: String,
}

impl ThreadsClient {
    pub fn new(access_token: impl Into<String>) -> Self {
        Self {
            http: Client::new(),
            access_token: access_token.into(),
        }
    }

    // --- Profiles ---

    pub async fn get_profile(
        &self,
        user_id: &str,
        fields: &[&str],
    ) -> Result<UserProfile> {
        let url = format!("{BASE_URL}/{user_id}");
        let resp = self
            .http
            .get(&url)
            .query(&[
                ("fields", fields.join(",")),
                ("access_token", self.access_token.clone()),
            ])
            .send()
            .await?;
        self.check_response(resp).await
    }

    pub async fn lookup_profile(
        &self,
        username: &str,
        fields: &[&str],
    ) -> Result<PublicProfile> {
        let url = format!("{BASE_URL}/profile_lookup");
        let resp = self
            .http
            .get(&url)
            .query(&[
                ("username", username.to_string()),
                ("fields", fields.join(",")),
                ("access_token", self.access_token.clone()),
            ])
            .send()
            .await?;
        self.check_response(resp).await
    }

    // --- Threads ---

    pub async fn get_threads(
        &self,
        user_id: &str,
        fields: &[&str],
        since: Option<&str>,
        until: Option<&str>,
        limit: Option<u32>,
    ) -> Result<PaginatedResponse<Thread>> {
        let url = format!("{BASE_URL}/{user_id}/threads");
        let mut query = vec![
            ("fields".to_string(), fields.join(",")),
            ("access_token".to_string(), self.access_token.clone()),
        ];
        if let Some(s) = since {
            query.push(("since".to_string(), s.to_string()));
        }
        if let Some(u) = until {
            query.push(("until".to_string(), u.to_string()));
        }
        if let Some(l) = limit {
            query.push(("limit".to_string(), l.to_string()));
        }
        let resp = self.http.get(&url).query(&query).send().await?;
        self.check_response(resp).await
    }

    pub async fn get_thread(
        &self,
        media_id: &str,
        fields: &[&str],
    ) -> Result<Thread> {
        let url = format!("{BASE_URL}/{media_id}");
        let resp = self
            .http
            .get(&url)
            .query(&[
                ("fields", fields.join(",")),
                ("access_token", self.access_token.clone()),
            ])
            .send()
            .await?;
        self.check_response(resp).await
    }

    pub async fn get_profile_posts(
        &self,
        username: &str,
        fields: &[&str],
        since: Option<&str>,
        until: Option<&str>,
        limit: Option<u32>,
    ) -> Result<PaginatedResponse<Thread>> {
        let url = format!("{BASE_URL}/profile_posts");
        let mut query = vec![
            ("username".to_string(), username.to_string()),
            ("fields".to_string(), fields.join(",")),
            ("access_token".to_string(), self.access_token.clone()),
        ];
        if let Some(s) = since {
            query.push(("since".to_string(), s.to_string()));
        }
        if let Some(u) = until {
            query.push(("until".to_string(), u.to_string()));
        }
        if let Some(l) = limit {
            query.push(("limit".to_string(), l.to_string()));
        }
        let resp = self.http.get(&url).query(&query).send().await?;
        self.check_response(resp).await
    }

    pub async fn get_ghost_posts(
        &self,
        user_id: &str,
        fields: &[&str],
    ) -> Result<PaginatedResponse<Thread>> {
        let url = format!("{BASE_URL}/{user_id}/ghost_posts");
        let resp = self
            .http
            .get(&url)
            .query(&[
                ("fields", fields.join(",")),
                ("access_token", self.access_token.clone()),
            ])
            .send()
            .await?;
        self.check_response(resp).await
    }

    // --- Publishing ---

    pub async fn create_container(
        &self,
        user_id: &str,
        params: &CreateContainerParams,
    ) -> Result<CreateContainerResponse> {
        let url = format!("{BASE_URL}/{user_id}/threads");
        let mut form: Vec<(String, String)> =
            vec![("access_token".to_string(), self.access_token.clone())];
        let serialized = serde_json::to_value(params)
            .map_err(|e| ThreadsError::Api {
                code: None,
                message: format!("Failed to serialize params: {e}"),
            })?;
        if let serde_json::Value::Object(map) = serialized {
            for (k, v) in map {
                let val = match &v {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    other => other.to_string(),
                };
                form.push((k, val));
            }
        }
        let resp = self.http.post(&url).form(&form).send().await?;
        self.check_response(resp).await
    }

    pub async fn publish(
        &self,
        user_id: &str,
        creation_id: &str,
    ) -> Result<PublishResponse> {
        let url = format!("{BASE_URL}/{user_id}/threads_publish");
        let resp = self
            .http
            .post(&url)
            .form(&[
                ("creation_id", creation_id),
                ("access_token", &self.access_token),
            ])
            .send()
            .await?;
        self.check_response(resp).await
    }

    // --- Replies ---

    pub async fn get_replies(
        &self,
        media_id: &str,
        fields: &[&str],
        reverse: Option<bool>,
    ) -> Result<PaginatedResponse<Thread>> {
        let url = format!("{BASE_URL}/{media_id}/replies");
        let mut query = vec![
            ("fields".to_string(), fields.join(",")),
            ("access_token".to_string(), self.access_token.clone()),
        ];
        if let Some(r) = reverse {
            query.push(("reverse".to_string(), r.to_string()));
        }
        let resp = self.http.get(&url).query(&query).send().await?;
        self.check_response(resp).await
    }

    pub async fn get_conversation(
        &self,
        media_id: &str,
        fields: &[&str],
        reverse: Option<bool>,
    ) -> Result<PaginatedResponse<Thread>> {
        let url = format!("{BASE_URL}/{media_id}/conversation");
        let mut query = vec![
            ("fields".to_string(), fields.join(",")),
            ("access_token".to_string(), self.access_token.clone()),
        ];
        if let Some(r) = reverse {
            query.push(("reverse".to_string(), r.to_string()));
        }
        let resp = self.http.get(&url).query(&query).send().await?;
        self.check_response(resp).await
    }

    pub async fn get_pending_replies(
        &self,
        media_id: &str,
        fields: &[&str],
        reverse: Option<bool>,
        approval_status: Option<&str>,
    ) -> Result<PaginatedResponse<Thread>> {
        let url = format!("{BASE_URL}/{media_id}/pending_replies");
        let mut query = vec![
            ("fields".to_string(), fields.join(",")),
            ("access_token".to_string(), self.access_token.clone()),
        ];
        if let Some(r) = reverse {
            query.push(("reverse".to_string(), r.to_string()));
        }
        if let Some(s) = approval_status {
            query.push(("approval_status".to_string(), s.to_string()));
        }
        let resp = self.http.get(&url).query(&query).send().await?;
        self.check_response(resp).await
    }

    pub async fn manage_reply(
        &self,
        reply_id: &str,
        hide: bool,
    ) -> Result<SuccessResponse> {
        let url = format!("{BASE_URL}/{reply_id}/manage_reply");
        let resp = self
            .http
            .post(&url)
            .form(&[
                ("hide", &hide.to_string() as &str),
                ("access_token", &self.access_token),
            ])
            .send()
            .await?;
        self.check_response(resp).await
    }

    pub async fn manage_pending_reply(
        &self,
        reply_id: &str,
        approve: bool,
    ) -> Result<SuccessResponse> {
        let url = format!("{BASE_URL}/{reply_id}/manage_pending_reply");
        let action = if approve { "approve" } else { "deny" };
        let resp = self
            .http
            .post(&url)
            .form(&[
                ("approval_status", action),
                ("access_token", &self.access_token),
            ])
            .send()
            .await?;
        self.check_response(resp).await
    }

    // --- Insights ---

    pub async fn get_media_insights(
        &self,
        media_id: &str,
        metrics: &[&str],
    ) -> Result<InsightsResponse> {
        let url = format!("{BASE_URL}/{media_id}/insights");
        let resp = self
            .http
            .get(&url)
            .query(&[
                ("metric", metrics.join(",")),
                ("access_token", self.access_token.clone()),
            ])
            .send()
            .await?;
        self.check_response(resp).await
    }

    pub async fn get_user_insights(
        &self,
        user_id: &str,
        metrics: &[&str],
        since: Option<i64>,
        until: Option<i64>,
    ) -> Result<InsightsResponse> {
        let url = format!("{BASE_URL}/{user_id}/threads_insights");
        let mut query = vec![
            ("metric".to_string(), metrics.join(",")),
            ("access_token".to_string(), self.access_token.clone()),
        ];
        if let Some(s) = since {
            query.push(("since".to_string(), s.to_string()));
        }
        if let Some(u) = until {
            query.push(("until".to_string(), u.to_string()));
        }
        let resp = self.http.get(&url).query(&query).send().await?;
        self.check_response(resp).await
    }

    // --- Pagination ---

    pub async fn fetch_next_page<T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<T> {
        let resp = self.http.get(url).send().await?;
        self.check_response(resp).await
    }

    // --- Internal ---

    async fn check_response<T: serde::de::DeserializeOwned>(
        &self,
        resp: reqwest::Response,
    ) -> Result<T> {
        if resp.status().is_success() {
            let body = resp.text().await?;
            serde_json::from_str(&body).map_err(|e| ThreadsError::Api {
                code: None,
                message: format!("Failed to parse response: {e}"),
            })
        } else {
            let body = resp.text().await.unwrap_or_default();
            if let Ok(err) = serde_json::from_str::<ApiErrorBody>(&body) {
                if let Some(detail) = err.error {
                    return Err(ThreadsError::Api {
                        code: detail.code,
                        message: detail.message.unwrap_or(body),
                    });
                }
            }
            Err(ThreadsError::Api {
                code: None,
                message: body,
            })
        }
    }
}
