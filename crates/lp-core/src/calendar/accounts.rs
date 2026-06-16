use keyring::Entry;
use thiserror::Error;

const SERVICE: &str = "LifePlanner";

#[derive(Error, Debug)]
pub enum AccountError {
    #[error("Keyring error: {0}")]
    Keyring(String),
}

pub fn store_password(account_id: &str, password: &str) -> Result<(), AccountError> {
    Entry::new(SERVICE, account_id)
        .map_err(|e| AccountError::Keyring(e.to_string()))?
        .set_password(password)
        .map_err(|e| AccountError::Keyring(e.to_string()))
}

pub fn get_password(account_id: &str) -> Result<Option<String>, AccountError> {
    match Entry::new(SERVICE, account_id)
        .map_err(|e| AccountError::Keyring(e.to_string()))?
        .get_password()
    {
        Ok(pw) => Ok(Some(pw)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(AccountError::Keyring(e.to_string())),
    }
}

pub fn delete_password(account_id: &str) {
    Entry::new(SERVICE, account_id).ok()
        .and_then(|e| e.delete_credential().ok());
}
