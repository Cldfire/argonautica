#![allow(non_camel_case_types)]

use config::{Backend, Variant, Version};

/// Available backends
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub enum argonautica_backend_t {
    /// TODO:
    ARGONAUTICA_C = 0,

    /// TODO:
    ARGONAUTICA_RUST = 1,
}

impl From<argonautica_backend_t> for Backend {
    fn from(backend: argonautica_backend_t) -> Backend {
        match backend {
            argonautica_backend_t::ARGONAUTICA_C => Backend::C,
            argonautica_backend_t::ARGONAUTICA_RUST => Backend::Rust,
        }
    }
}

/// Argonautica errors
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub enum argonautica_error_t {
    /// TODO:
    ARGONAUTICA_OK = 0,

    /// TODO:
    ARGONAUTICA_ERROR1 = 1,

    /// TODO:
    ARGONAUTICA_ERROR2 = 2,
}

/// Available argon2 variants
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub enum argonautica_variant_t {
    /// TODO:
    ARGONAUTICA_ARGON2D = 1,

    /// TODO:
    ARGONAUTICA_ARGON2I = 2,

    /// TODO:
    ARGONAUTICA_ARGON2ID = 3,
}

impl From<argonautica_variant_t> for Variant {
    fn from(variant: argonautica_variant_t) -> Variant {
        match variant {
            argonautica_variant_t::ARGONAUTICA_ARGON2D => Variant::Argon2d,
            argonautica_variant_t::ARGONAUTICA_ARGON2I => Variant::Argon2i,
            argonautica_variant_t::ARGONAUTICA_ARGON2ID => Variant::Argon2id,
        }
    }
}

/// Available argon2 versions
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub enum argonautica_version_t {
    /// TODO:
    ARGONAUTICA_0x10 = 13,

    /// TODO:
    ARGONAUTICA_0x13 = 16,
}

impl From<argonautica_version_t> for Version {
    fn from(version: argonautica_version_t) -> Version {
        match version {
            argonautica_version_t::ARGONAUTICA_0x10 => Version::_0x10,
            argonautica_version_t::ARGONAUTICA_0x13 => Version::_0x13,
        }
    }
}
