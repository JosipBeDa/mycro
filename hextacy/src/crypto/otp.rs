use super::CryptoError;
use data_encoding::BASE32;
use tracing::debug;

/// Generates an OTP secret of length 160
pub fn generate_secret() -> String {
    debug!("Generating OTP secret");
    thotp::encoding::encode(&thotp::generate_secret(160), BASE32)
}

/// Generates a QR code svg with the given secret
pub fn generate_totp_qr_code(
    secret: &str,
    user_email: &str,
    label: &str,
    issuer: &str,
) -> Result<String, CryptoError> {
    debug!("Generating TOTP QR");
    let uri = thotp::qr::otp_uri(
        "totp",
        secret,
        &format!("{label}:{user_email}"),
        issuer,
        None,
    )?;
    thotp::qr::generate_code_svg(&uri, None, None, thotp::qr::EcLevel::M).map_err(Into::into)
}

/// Verifies a timed OTP against the given secret
pub fn verify_otp(password: &str, secret: &str) -> Result<bool, CryptoError> {
    debug!("Verifying TOTP {password}");
    let secret = BASE32.decode(secret.as_bytes())?;
    thotp::verify_totp(password, &secret, 0)
        .map_err(Into::into)
        .map(|(res, _)| res)
}
