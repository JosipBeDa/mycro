use crate::db::models::user::{SortOptions, User};
use derive_new::new;
use hextacy::web::http::response::Response;
use serde::{Deserialize, Serialize};
use validify::Validify;

#[derive(Debug, Deserialize, Validify)]
#[serde(rename_all = "camelCase")]
pub struct GetUsersPaginated {
    #[validate(range(min = 1., max = 65_535.))]
    pub page: Option<u16>,
    #[validate(range(min = 1., max = 65_535.))]
    pub per_page: Option<u16>,
    pub sort_by: Option<SortOptions>,
}

#[derive(Debug, Serialize, new)]
pub struct UserResponse {
    users: Vec<User>,
}

impl Response<'_> for UserResponse {}