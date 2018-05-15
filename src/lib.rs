//! ## Overview
//!
//! `a2` is a Rust crate for hashing passwords that uses the cryptographically-secure
//! [Argon2 hashing algorithm](https://tools.ietf.org/html/draft-irtf-cfrg-argon2-03),
//! which won the [Password Hashing Competition](https://password-hashing.net/) in 2015 and is
//! comparable to other secure hashing algorithms such as [bcrypt](https://en.wikipedia.org/wiki/Bcrypt)
//! and [scrypt](https://en.wikipedia.org/wiki/Scrypt).
//!
//! `a2` was designed:
//! * to be easy to use,
//! * to have beginner-friendly documentation,
//! * to have sensible defaults, and
//! * to (as much as possible), follow Rust API "best practices" (whatever that means 😊)
//!
//! The library was built with a simple use-case in mind: hashing passwords for storage in a database for a
//! website. That said, `a2` is also feature-complete, meaning you should be able to to anything
//! with `a2` that you can do with the cannonical [C implementation](https://github.com/P-H-C/phc-winner-argon2)
//! of Argon2.
//!
//! ## Hashing
//!
//! Hashing passwords with `a2` is simple.  Just instantiate a default `Hasher`, provide it
//! with a password and a secret key, and then call the `hash` method.
//! ```
//! extern crate a2;
//!
//! fn main() {
//!     let mut hasher = a2::Hasher::default();
//!     let hash = hasher
//!         .with_password("P@ssw0rd")
//!         .with_secret_key("\
//!             secret key that you should really store in \
//!             an environment variable instead of in code, \
//!             but this is just an example\
//!         ")
//!         .hash()
//!         .unwrap();
//!     println!("{}", &hash);
//!     // 👆 prints a hash, which will be random as the default Hasher uses a random salt
//! }
//! ```
//! ## Verifying
//!
//! Verifying a password against a hash is equally as simple. Just instantiate a
//! default `Verifier`, provide it with the password and the hash you would like to compare,
//! provide it with the secret key that was used to create the hash, and then call the `verify`
//! method.
//! ```
//! extern crate a2;
//!
//! fn main() {
//!     let hash = "\
//!         $argon2id$v=19$m=4096,t=128,p=2\
//!         $/q7MXPB7VqmB1iRQvgg6g1Vz5Rr76qISATkCGafVnLU\
//!         $039phOrF/E5yzN67B2aCbXhRAcNMM1yKhhD8wtDMciY\
//!     ";
//!     let mut verifier = a2::Verifier::default();
//!     let is_valid = verifier
//!         .with_hash(hash)
//!         .with_password("P@ssw0rd")
//!         .with_secret_key("\
//!             secret key that you should really store in \
//!             an environment variable instead of in code, \
//!             but this is just an example\
//!         ")
//!         .verify()
//!         .unwrap();
//!     println!("{}", is_valid);
//!     // 👆 prints true
//! }
//! ```
//! ## Configuration
//!
//! The default configurations for `Hasher` and `Verfier` were chosen to be reasonably secure for
//! the general use-case of hashing passwords for storage in a database, but you may want to use
//! `a2` for a different use-case or you may just disagree with the chosen defaults. Customizing
//! `a2` configuration options to meet your needs is hopefully as easy and as intuitive as using
//! the default options.
//!
//! Here is a hashing example that shows how to use custom configuration options and also provides
//! color on each of the options.
//! ```
//! extern crate a2;
//!
//! use a2::config::{Backend, Variant, Version};
//!
//! fn main() {
//!     let mut hasher = a2::Hasher::default();
//!     hasher
//!         .configure_backend(Backend::C)
//!         // 👆 a2 is designed to support multiple backends (meaning multiple implementations
//!         // of the underlying Argon2 algorithm). Currently only Backend::C is supported, which
//!         // uses the cannonical Argon2 library written in C to actually do the work. In the
//!         // future hopefully a Rust backend will also be supported. For the moment, however,
//!         // you must use Backend::C, which is the default
//!         .configure_hash_length(32)
//!         // 👆 The hash length in bytes is configurable. The default is 32. This is probably
//!         // a good number to use. 16 is also probably fine. You probably shouldn't go below 16,
//!         // however
//!         .configure_iterations(128)
//!         // 👆 Argon2 has a notion of "iterations" or "time cost". All else equal and generally
//!         // speaking, the greater the number of iterations, the longer it takes to perform the
//!         // hash and the more secure the resulting hash. More iterations basically means more
//!         // CPU load. This and "memory size" (see below) are, again, generally speaking,
//!         // the two parameters to adjust in order to increase or decrease the security
//!         // of your hash. The default is 128 iterations, which was chosen because, along with
//!         // the default memory size of 4096, this leads to a hashing time of approximately
//!         // 500 milliseconds on the early-2014 Macbook Air that is the developer's machine.
//!         // If you're going to use a2 in production, you should probably tweak this parameter
//!         // (and the memory size parameter) in order to increase the time it takes to hash
//!         // to the maximum you can reasonably allow for your use-case (e.g. to probably about
//!         // 500 milliseconds for the use-case of hashing user passwords for a website)
//!         .configure_lanes(2)
//!         // 👆 Argon2 can break up its work into one or more "lanes" during some parts of
//!         // the hashing algorithm. If you configure it with multiple lanes and you also
//!         // use multiple threads (see below) the hashing algorithm will performed its
//!         // work in parallel in some parts, potentially speeding up the time it takes to
//!         // produce a hash without diminishing the security of the result. By default,
//!         // the number of lanes is set to the number of physical cores on your machine
//!         .configure_memory_size(4096)
//!         // 👆 Argon2 has a notion of "memory size" or "memory cost" (in kibibytes). All else
//!         // equal and generally speaking, the greater the memory size, the longer it takes to
//!         // perform the hash and the more secure the resulting hash. More memory size basically
//!         // means more memory used. This and "iterations" (see above) are, again, generally
//!         // speaking, the two parameters to adjust in order to increase or decrease the
//!         // security of your hash. The default is 4096 kibibytes, which was chosen because,
//!         // again, along with the default iterations of 128, this leads to a hashing time of
//!         // approximately 500 milliseconds on the early-2014 Macbook Air that is the
//!         // developer's machine. If you're going to use a2 in production, you should probably
//!         // tweak this parameter (and the iterations parameter) in order to increase the time
//!         // it takes to hash to the maximum you can reasonably allow for your use-case
//!         // (e.g. to probably about 500 milliseconds for the use-case of hashing user passwords
//!         // for a website)
//!         .configure_password_clearing(true)
//!         // 👆 By default, every time you call hash or hash_raw on a Hasher, the underying
//!         // bytes of the password you provided are completely erased, meaning you can no
//!         // longer access them and will have to provide a new password to Hasher in order
//!         // to call hash or hash_raw again. This is a sensible security measure designed to
//!         // to prevent you from keeping the password bytes around longer than you have to.
//!         // With this method, however, you can turn this security feature off by passing false.
//!         // This is not recommended
//!         .configure_secret_key_clearing(false)
//!         // 👆 It is also possible with a2 to have the underlying bytes of the secret key you
//!         // a Hasher completely erased after each call to hash or hash_raw. Unlike with
//!         // password clearing, however, this option is not turned on by default. Typically,
//!         // you'll want to use the same Hasher instance to hash multiple passwords. With
//!         // the default setting of secret key clearing set to false, you can provide your
//!         // Hasher with your secret key once and use it for multiple passwords. If you want
//!         // to be extra secure and force yourself to provide the secret key to Hasher every
//!         // time you hash a password, you can turn this feature on by passing true to this
//!         // method
//!         .configure_threads(2)
//!         // 👆 If you have configured a Hasher to use more than one lane (see above), you
//!         // can get the hashing algorithm to run in parallel during some parts of the
//!         // computation by setting the number of threads to be greater than one as well,
//!         // potentially speeding up the time it takes to produce a hash without diminishing
//!         // the security of the result. By default, the number of threads is set to the number
//!         // of physical cores on your machine
//!         .configure_variant(Variant::Argon2id)
//!         // 👆 Argon2 has three variants: Argon2d, Argon2i, and Argon2id. Here is how these
//!         // variants are explained in the RFC: "Argon2 has one primary variant: Argon2id,
//!         // and two supplementary variants: Argon2d and Argon2i. Argon2d uses data-dependent
//!         // memory access, which makes it suitable for ... applications with no threats from
//!         // side-channel timing attacks. Argon2i uses data-independent memory access, which
//!         // is preferred for password hashing and password-based key derivation. Argon2id
//!         // works as Argon2i for the first half of the first iteration over the memory, and
//!         // as Argon2d for the rest, thus providing both side-channel attack protection and
//!         // brute-force cost savings due to time-memory tradeoffs." If you do not know which
//!         // variant to use, use the default, which is Argon2id
//!         .configure_version(Version::_0x13)
//!         // 👆 Argon2 has two versions: 0x10 and 0x13. The latest version is 0x13 (as of 5/18).
//!         // Unless you have a very specific reason not to, you should always use the latest
//!         // version, which is also the default
//!         .opt_out_of_random_salt()
//!         // 👆 As a built-in "safety" mechanism, if you wish to use a non-random salt,
//!         // which is generally not a good idea, you must explicity call this method
//!         .opt_out_of_secret_key();
//!         // 👆 As a built-in "safety" mechanism, if you wish to not use a secret key,
//!         // which is generally not a good idea, you must explicity call this method
//!
//!     let hash = hasher
//!         .with_password("P@ssw0rd")
//!         .with_salt("somesalt")
//!         // 👆 A non-random salt, which is a bad idea, but possible
//!         // because we configured this Hasher with opt_out_of_random_salt
//!         .hash()
//!         .unwrap();
//!         // 👆 Notice we did not include a secret key, which is also a bad idea, but possible
//!         // because we configured this Hasher with opt_out_of_secret_key
//!
//!     println!("{}", &hash);
//!     // 👆 prints $argon2id$v=19$m=8192,t=256,p=2$c29tZXNhbHQ$TyX+9AspmkeMGLJRQdJozQ
//! }
//! ```
//! ## Installation
//!
//! Installing `a2` should be pretty straightforward:
//! * Include `a2 = "0.1.0"` in the dependencies section of your `Cargo.toml`, and
//! * Include `extern crate a2;` in your code (typically you would include it in either `lib.rs` or `main.rs`).
//!
//! That said, `a2` uses [cc](https://github.com/alexcrichton/cc-rs) to compile the cannonical
//! [C implemenation](https://github.com/P-H-C/phc-winner-argon2) of Argon2 into a
//! static archive during the build process. This means that you need a C compiler on your
//! machine in order to build `a2`. I can anticipate this causing issues for some users.
//! At the moment, all I can say is please submit an issue if `a2` fails to build on your machine and
//! I'll try to look into it, but to be honest, compiling C programs is not really an area of expertise
//! for me (so if anyone wants to help out in this area, that would be much appreciated!).
//!
//! `a2` was built using stable Rust 1.25.0 and most likely works on earlier versions
//! of Rust as well, but I'm not aware of exactly how far back it will go.
//!
//! ## Alternatives
//!
//! If `a2` isn't your cup of tea, other Rust crates that will do Argon2 hashing for you
//! include [argon2rs](https://github.com/bryant/argon2rs) and [rust-argon2](https://github.com/sru-systems/rust-argon2).
//! As already mentioned, there's also a cannonical [C implementation](https://github.com/P-H-C/phc-winner-argon2),
//! which `a2` actually uses under the covers if you're using the C backend. Finally, if you're interesting
//! in password hashing with a different algorithm, [rust-bcrypt](https://github.com/Keats/rust-bcrypt)
//! might be worth checking out.
//!
//! For what it's worth, besides API differences, one thing `a2` focuses on relative to other similar
//! libraries is the ability to easily create hashes using a secret key. Your mileage may vary,
//! but the crate's author found it somewhat difficult to create hashes using a secret key when
//! experimenting with alternative libraries.
#![allow(unknown_lints)]
#![warn(clippy)]
extern crate base64;
#[macro_use]
extern crate bitflags;
// extern crate blake2_rfc;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate nom;
extern crate num_cpus;
extern crate rand;
extern crate scopeguard;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod backend;
mod ffi;
mod hasher;
mod verifier;

pub mod config;
pub mod data;
pub mod error;
pub use hasher::Hasher;
pub mod output;
pub mod utils;
pub use verifier::Verifier;

// TODO: Check for errors before serializing
// TODO: Use rust for encoding / decoding always?
// TODO: Errors
// TODO: Verify compiler flags for build script
