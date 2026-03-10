use serde::{Deserialize, Serialize};

// --- Request enums ---

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaType {
    Text,
    Image,
    Video,
    Carousel,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplyControl {
    Everyone,
    AccountsYouFollow,
    MentionedOnly,
}

// --- Request params ---

#[derive(Debug, Clone, Serialize)]
pub struct CreateContainerParams {
    pub media_type: MediaType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_carousel_item: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_control: Option<ReplyControl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_attachment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_attachment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_reply_approvals: Option<bool>,
}

// --- Response structs ---

#[derive(Debug, Clone, Deserialize)]
pub struct UserProfile {
    pub id: Option<String>,
    pub username: Option<String>,
    pub name: Option<String>,
    pub threads_profile_picture_url: Option<String>,
    pub threads_biography: Option<String>,
    pub is_verified: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PublicProfile {
    pub username: Option<String>,
    pub name: Option<String>,
    pub profile_picture_url: Option<String>,
    pub biography: Option<String>,
    pub follower_count: Option<i64>,
    pub likes_count: Option<i64>,
    pub quotes_count: Option<i64>,
    pub reposts_count: Option<i64>,
    pub views_count: Option<i64>,
    pub is_verified: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Owner {
    pub id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Thread {
    pub id: Option<String>,
    pub media_product_type: Option<String>,
    pub media_type: Option<String>,
    pub media_url: Option<String>,
    pub permalink: Option<String>,
    pub owner: Option<Owner>,
    pub username: Option<String>,
    pub text: Option<String>,
    pub timestamp: Option<String>,
    pub shortcode: Option<String>,
    pub thumbnail_url: Option<String>,
    pub children: Option<PaginatedResponse<Thread>>,
    pub is_quote_post: Option<bool>,
    pub has_replies: Option<bool>,
    pub is_reply: Option<bool>,
    pub root_post: Option<Box<Thread>>,
    pub replied_to: Option<Box<Thread>>,
    pub hide_status: Option<String>,
    pub reply_audience: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub paging: Option<Paging>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Paging {
    pub cursors: Option<Cursors>,
    pub next: Option<String>,
    pub previous: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Cursors {
    pub before: Option<String>,
    pub after: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateContainerResponse {
    pub id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PublishResponse {
    pub id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SuccessResponse {
    pub success: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InsightsResponse {
    pub data: Vec<Insight>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Insight {
    pub name: Option<String>,
    pub period: Option<String>,
    pub values: Option<Vec<InsightValue>>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub id: Option<String>,
    pub total_value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InsightValue {
    pub value: Option<serde_json::Value>,
    pub end_time: Option<String>,
}

// --- Internal error parsing ---

#[derive(Debug, Deserialize)]
pub(crate) struct ApiErrorBody {
    pub error: Option<ApiErrorDetail>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ApiErrorDetail {
    pub message: Option<String>,
    pub code: Option<i64>,
}
