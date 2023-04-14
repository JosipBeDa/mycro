//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use crate::db::models::user::User;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(unique)]
    pub email: String,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: String,
    pub phone: Option<String>,
    pub password: Option<String>,
    pub otp_secret: Option<String>,
    pub frozen: bool,
    pub google_id: Option<String>,
    pub github_id: Option<String>,
    pub email_verified_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::oauth::Entity")]
    Oauth,
    #[sea_orm(has_many = "super::sessions::Entity")]
    Sessions,
}

impl Related<super::oauth::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Oauth.def()
    }
}

impl Related<super::sessions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sessions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for User {
    fn from(
        Model {
            id,
            email,
            username,
            first_name,
            last_name,
            role,
            phone,
            password,
            otp_secret,
            frozen,
            google_id,
            github_id,
            email_verified_at,
            created_at,
            updated_at,
        }: Model,
    ) -> Self {
        Self {
            id,
            email,
            username,
            first_name,
            last_name,
            role: role.into(),
            phone,
            password,
            otp_secret,
            frozen,
            google_id,
            github_id,
            email_verified_at: email_verified_at.map(|d| d.naive_utc()),
            created_at: created_at.naive_utc(),
            updated_at: updated_at.naive_utc(),
        }
    }
}