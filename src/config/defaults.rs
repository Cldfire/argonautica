use futures_cpupool::CpuPool;
use num_cpus;

use config::{Backend, Variant, Version};

/// Returns a [`CpuPool`](https://docs.rs/futures-cpupool/0.1.8/futures_cpupool/struct.CpuPool.html)
/// with threads equal to the number of logical cores on your machine
#[inline(always)]
pub fn default_cpu_pool() -> CpuPool {
    CpuPool::new(num_cpus::get())
}

#[cfg(feature = "serde")]
pub(crate) fn default_cpu_pool_serde() -> Option<CpuPool> {
    None
}

/// Returns the number of logical cores on your machine
#[inline(always)]
pub fn default_lanes() -> u32 {
    num_cpus::get() as u32
}

/// Returns the number of logical cores on your machine
#[inline(always)]
pub fn default_threads() -> u32 {
    num_cpus::get() as u32
}

/// [`Backend::C`](enum.Backend.html#variant.C)
pub const DEFAULT_BACKEND: Backend = Backend::C;

/// `32_u32`
pub const DEFAULT_HASH_LENGTH: u32 = 32;

/// `128_u32`
pub const DEFAULT_ITERATIONS: u32 = 128;

/// `4096_u32`
pub const DEFAULT_MEMORY_SIZE: u32 = 4_096;

/// `32_u32`
pub const DEFAULT_SALT_LENGTH: u32 = 32;

/// `false`
pub const DEFAULT_OPT_OUT_OF_RANDOM_SALT: bool = false;

/// `false`
pub const DEFAULT_OPT_OUT_OF_SECRET_KEY: bool = false;

/// `false`
pub const DEFAULT_PASSWORD_CLEARING: bool = false;

/// `false`
pub const DEFAULT_SECRET_KEY_CLEARING: bool = false;

/// [`Variant::Argon2id`](enum.Variant.html#variant.Argon2id)
pub const DEFAULT_VARIANT: Variant = Variant::Argon2id;

/// [`Version::_0x13`](enum.Version.html#variant._0x13)
pub const DEFAULT_VERSION: Version = Version::_0x13;
