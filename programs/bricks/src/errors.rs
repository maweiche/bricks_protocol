use anchor_lang::error_code;

#[error_code]
pub enum SetupError {
    #[msg("You are not authorized to perform this action")]
    Unauthorized,
    #[msg("You are already verified!")]
    ProfileAlreadyVerified,
    #[msg("You have a Non-Upgradable Membership Type!")]
    InvalidMembership,
    #[msg("You used an invalid condition")]
    InvalidCondition,
    #[msg("You used an invalid object type")]
    InvalidObjectType,
    #[msg("You used an invalid type")]
    InvalidType,

}

#[error_code]
pub enum BuyingError {
    #[msg("You already bought more than 500$ worth of fraction, to buy more you need to do KYC")]
    NotVerified,
    #[msg("Listing is not Live yet, come back later!")]
    NotTimeYet,
    #[msg("Overflow")]
    Overflow,
    #[msg("Underflow")]
    Underflow,
    #[msg("The amount offered does not match the initial token price")]
    PriceMismatch,
    #[msg("Signature authority mismatch")]
    SignatureAuthorityMismatch,
    #[msg("Invalid instruction")]
    InvalidInstruction,
}