// use std::mem;

// use blake2_rfc::blake2b::Blake2b;
// use failure;

// use data::{Read, ReadPrivate, Salt};
// use hasher::Hasher;

// fn h0(hasher: &mut Hasher) -> Result<[u8; 72], failure::Error> {
//     if hasher.salt().is_random() {
//         let len = hasher.salt().len();
//         hasher.set_salt(Salt::random(len)?);
//     }
//     let input = &[
//         &u32_to_bytes(hasher.config().lanes()),
//         &u32_to_bytes(hasher.config().hash_length()),
//         &u32_to_bytes(hasher.config().memory_size()),
//         &u32_to_bytes(hasher.config().iterations()),
//         &u32_to_bytes(hasher.config().version() as u32),
//         &u32_to_bytes(hasher.config().variant() as u32),
//         &u32_to_bytes(hasher.password().len()),
//         hasher.password().as_bytes(),
//         &u32_to_bytes(hasher.salt().len()),
//         hasher.salt().as_bytes(),
//         &u32_to_bytes(hasher.secret_key().len()),
//         hasher.secret_key().as_bytes(),
//         &u32_to_bytes(hasher.additional_data().len()),
//         hasher.additional_data().as_bytes(),
//     ];
//     let mut blake2b = Blake2b::new(64);
//     for i in input {
//         blake2b.update(i);
//     }
//     let mut buffer = [0u8; 72];
//     {
//         let buffer_slice = &mut buffer[0..64];
//         buffer_slice.copy_from_slice(blake2b.finalize().as_bytes());
//     }
//     Ok(buffer)
// }

// fn u32_to_bytes(x: u32) -> [u8; 4] {
//     unsafe { mem::transmute(x.to_le()) }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_h0() {
//         let mut hasher = Hasher::default();
//         hasher
//             .with_password("P@ssw0rd")
//             .with_secret_key("secret");
//         let output = h0(&mut hasher).unwrap();
//         println!("{:?}", &output[..]);
//     }
// }
