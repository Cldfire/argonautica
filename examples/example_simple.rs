extern crate dotenv;
#[macro_use]
extern crate failure;
extern crate jasonus;

use std::env;

use jasonus::input::SecretKey;

// Helper method to load the secret key from a .env file. Used in `main` below.
fn load_secret_key() -> Result<SecretKey, failure::Error> {
    let dotenv_path = env::current_dir()?.join("examples").join("example.env");
    dotenv::from_path(&dotenv_path).map_err(|e| format_err!("{}", e))?;
    let base64_encoded_secret_key = env::var("SECRET_KEY")?;
    Ok(SecretKey::from_base64_encoded_str(
        &base64_encoded_secret_key,
    )?)
}

fn main() -> Result<(), failure::Error> {
    let secret_key = load_secret_key()?;
    let mut hasher = jasonus::Hasher::default();
    let hash = hasher
        .with_password("P@ssw0rd")
        .with_secret_key(&secret_key)
        .hash()?;
    println!("{}", &hash);

    let mut verifier = jasonus::Verifier::default();
    let is_valid = verifier
        .with_hash(&hash)
        .with_password("P@ssw0rd")
        .with_secret_key(&secret_key)
        .verify()?;

    assert!(is_valid);
    Ok(())
}
