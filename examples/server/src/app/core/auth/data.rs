use crate::{db::models::user::User, helpers::validation::EMAIL_REGEX};
use hextacy::{web::http::response::Response, Constructor, HttpResponse};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use validify::Validify;

#[derive(Debug, Clone, Deserialize, Validify)]
/// Received on initial login
pub struct Credentials {
    #[validate(regex(EMAIL_REGEX))]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
    pub remember: bool,
}

#[derive(Debug, Clone, Deserialize, Validify)]
/// Received when registering
pub struct RegistrationData {
    #[validate(regex(EMAIL_REGEX))]
    pub email: String,
    #[modify(trim)]
    #[validate(length(min = 4))]
    pub username: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validify)]
/// Received when resending reg token
pub struct ResendRegToken {
    #[validate(regex(EMAIL_REGEX))]
    pub email: String,
}

#[derive(Debug, Deserialize, Validify)]
/// Received when verifying a one time password
pub struct Otp {
    #[validate(length(equal = 6))]
    pub password: String,
    pub token: String,
    pub remember: bool,
}

#[derive(Debug, Deserialize, Validify)]
/// Received when updating a password
pub struct ChangePassword {
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validify)]
/// Received when a user forgot their password
pub struct ForgotPassword {
    #[validate(regex(EMAIL_REGEX))]
    pub email: String,
}

#[derive(Debug, Deserialize, Validify)]
/// Received when a user asks to reset their password via email
pub struct ResetPassword {
    pub token: String,
}

#[derive(Debug, Deserialize, Validify)]
/// Received when verifying registration token
pub struct EmailToken {
    pub token: String,
}

#[derive(Debug, Deserialize, Validify)]
/// Received when verifying registration token
pub struct ForgotPasswordVerify {
    #[validate(length(min = 8))]
    pub password: String,
    pub token: String,
}

#[derive(Debug, Deserialize)]
/// Received when the user wants to logout
pub struct Logout {
    pub purge: bool,
}

#[derive(Debug, Clone, Deserialize, Validify)]
pub struct OAuthCodeExchange {
    #[modify(trim)]
    #[validate(length(min = 1))]
    pub code: String,
}

/*

RESPONSES

*/

/// Sent when the user successfully authenticates with credentials and has 2FA enabled
#[derive(Debug, Serialize, Constructor, HttpResponse)]
pub struct TwoFactorAuthResponse<'a> {
    username: &'a str,
    token: &'a str,
    remember: bool,
}

/// Sent when the user exceeds the maximum invalid login attempts
#[derive(Debug, Serialize, Constructor, HttpResponse)]
pub struct FreezeAccountResponse<'a> {
    email: &'a str,
    message: &'a str,
}

/// Sent when a user registers for the very first time
#[derive(Debug, Serialize, Constructor, HttpResponse)]
pub struct RegistrationStartResponse<'a> {
    message: &'a str,
    username: &'a str,
    email: &'a str,
}

/// Sent when the user completely authenticates
#[derive(Debug, Serialize, Constructor, HttpResponse)]
pub struct AuthenticationSuccessResponse {
    user: User,
}
