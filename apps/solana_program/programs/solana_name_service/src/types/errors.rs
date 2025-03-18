use anchor_lang::error_code;

#[error_code]
pub enum SnsError {
    #[msg("Unauthorized: You are not the owner of this domain.")]
    Unauthorised,

    #[msg("Invalid domain name format.")]
    InvalidDomainFormat,

    #[msg("Registration period expired.")]
    RegistrationExpired,
}
