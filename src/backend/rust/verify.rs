#![allow(dead_code)]

use error::Error;
use verifier::Verifier;

pub(crate) fn verify_rust(verifier: &mut Verifier) -> Result<bool, Error> {
    let _ = verifier;
    unimplemented!();
}
