use crate::app::core::auth::data::{
    ChangePassword, ChangePasswordPayload, Credentials, CredentialsPayload, EmailToken,
    EmailTokenPayload, ForgotPassword, ForgotPasswordPayload, ForgotPasswordVerify,
    ForgotPasswordVerifyPayload, Logout, Otp, OtpPayload, RegistrationData,
    RegistrationDataPayload, ResendRegToken, ResendRegTokenPayload, ResetPassword,
    ResetPasswordPayload,
};
use crate::app::core::auth::native::AuthenticationContract;
use crate::app::router::AppResponse;
use crate::error::Error;
use crate::helpers::request::extract_session;
use actix_web::{web, HttpRequest, Responder};
use tracing::info;
use validify::Validify;

/// Verifies the user's login credentials and either establishes a session if the user
/// doesn't have 2FA or prompts the user for their 2FA pass if they have it set up
pub async fn login<T: AuthenticationContract>(
    data: web::Json<CredentialsPayload>,
    service: web::Data<T>,
) -> Result<impl Responder, Error> {
    let credentials = Credentials::validify(data.0)?;
    info!("Credentials login : {}", credentials.email);
    service.login(credentials).await.map(AppResponse)
}

/// Starts the registration process for the user and sends an email containing a temporary
/// token used to complete the registration
pub async fn start_registration<T: AuthenticationContract>(
    data: web::Json<RegistrationDataPayload>,
    service: web::Data<T>,
) -> Result<impl Responder, Error> {
    let registration = RegistrationData::validify(data.0)?;
    info!("Start Registration: {:?}", registration);
    service
        .start_registration(registration)
        .await
        .map(AppResponse)
}

/// Verifies the user's registration token
pub async fn verify_registration_token<T: AuthenticationContract>(
    data: web::Query<EmailTokenPayload>,
    service: web::Data<T>,
) -> Result<impl Responder, Error> {
    let email_token = EmailToken::validify(data.0)?;
    info!("Verify registration token: {:?}", email_token);
    service
        .verify_registration_token(email_token)
        .await
        .map(AppResponse)
}

/// Resend the user's registration token in case it expired
pub async fn resend_registration_token<T: AuthenticationContract>(
    data: web::Json<ResendRegTokenPayload>,
    service: web::Data<T>,
) -> Result<impl Responder, Error> {
    let reg_token = ResendRegToken::validify(data.0)?;
    info!("Resend registration token: {:?}", reg_token.email);
    service
        .resend_registration_token(reg_token)
        .await
        .map(AppResponse)
}

/// Sets the user's OTP secret. Requires a valid session to be established beforehand
pub async fn set_otp_secret<T: AuthenticationContract>(
    req: HttpRequest,
    service: web::Data<T>,
) -> Result<impl Responder, Error> {
    let session = extract_session(req)?;
    info!("Registering OTP secret for: {}", session.user_id);
    service.set_otp_secret(session).await.map(AppResponse)
}

/// Verifies the user's OTP if they have 2FA enabled
pub async fn verify_otp<T: AuthenticationContract>(
    data: web::Json<OtpPayload>,
    service: web::Data<T>,
) -> Result<impl Responder, Error> {
    let otp = Otp::validify(data.0)?;
    info!("OTP login : {:?}", otp);
    service.verify_otp(otp).await.map(AppResponse)
}

/// Changes the user's password and purges all their sessions
pub async fn change_password<T: AuthenticationContract>(
    data: web::Json<ChangePasswordPayload>,
    req: HttpRequest,
    service: web::Data<T>,
) -> Result<impl Responder, Error> {
    let change_pw = ChangePassword::validify(data.0)?;
    let session = extract_session(req)?;
    info!("Updating password for {}", session.user_id);
    service
        .change_password(session, change_pw)
        .await
        .map(AppResponse)
}

/// Sends a forgot password token via email
pub async fn forgot_password<T: AuthenticationContract>(
    data: web::Json<ForgotPasswordPayload>,
    service: web::Data<T>,
) -> Result<impl Responder, Error> {
    let forgot_pw = ForgotPassword::validify(data.0)?;
    info!("Forgot password, sending token to {}", forgot_pw.email);
    service.forgot_password(forgot_pw).await.map(AppResponse)
}

/// Changes the user's password and purges all their sessions
pub async fn verify_forgot_password<T: AuthenticationContract>(
    data: web::Json<ForgotPasswordVerifyPayload>,
    service: web::Data<T>,
) -> Result<impl Responder, Error> {
    let forgot_pw_v = ForgotPasswordVerify::validify(data.0)?;
    info!("Forgot password, setting new");
    service
        .verify_forgot_password(forgot_pw_v)
        .await
        .map(AppResponse)
}

/// Changes the user's password and purges all their sessions
pub async fn reset_password<T: AuthenticationContract>(
    data: web::Query<ResetPasswordPayload>,
    service: web::Data<T>,
) -> Result<impl Responder, Error> {
    let reset_pw = ResetPassword::validify(data.0)?;
    info!("Resetting password token: {:?}", reset_pw);
    service.reset_password(reset_pw).await.map(AppResponse)
}

/// Logs the user out. Optionally purges their sessions, Requires a valid session to be established beforehand
pub async fn logout<T: AuthenticationContract>(
    data: web::Json<Logout>,
    req: HttpRequest,
    service: web::Data<T>,
) -> Result<impl Responder, Error> {
    let session = extract_session(req)?;
    info!("Logging out {}", session.user_id);
    service.logout(session, data.0).await.map(AppResponse)
}