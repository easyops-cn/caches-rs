use crate::lfu::tinylfu::TinyLFUError;
use crate::lru::CacheError;
use core::fmt::{Debug, Display, Formatter};

pub enum WTinyLFUError {
    InvalidCountMinWidth(u64),
    InvalidSamples(usize),
    InvalidWindowCacheSize(usize),
    InvalidProbationaryCacheSize(usize),
    InvalidProtectedCacheSize(usize),
    InvalidFalsePositiveRatio(f64),
    /// None Key Hasher for TinyLFU
    InvalidKeyHasher,

    /// Invalid cache size
    InvalidSize(usize),
    /// Invalid recent ratio for [`TwoQueueCache`]
    InvalidRecentRatio(f64),
    /// Invalid ghost ratio for [`TwoQueueCache`]
    InvalidGhostRatio(f64),

    Unknown,
}

impl WTinyLFUError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            WTinyLFUError::InvalidSamples(v) => write!(f, "invalid number of samples: {}", *v),
            WTinyLFUError::InvalidCountMinWidth(v) => {
                write!(f, "invalid count main sketch width: {}", *v)
            }
            WTinyLFUError::InvalidWindowCacheSize(v) => {
                write!(f, "invalid window cache size: {}", *v)
            }
            WTinyLFUError::InvalidProbationaryCacheSize(v) => {
                write!(f, "invalid probationary cache size: {}", *v)
            }
            WTinyLFUError::InvalidProtectedCacheSize(v) => {
                write!(f, "invalid protected cache size: {}", *v)
            }
            WTinyLFUError::InvalidFalsePositiveRatio(v) => write!(
                f,
                "invalid false positive ratio: {}, which should be in range (0.0, 1.0)",
                *v
            ),

            WTinyLFUError::InvalidKeyHasher => write!(f, "Invalid key hasher, which must be set.",),

            WTinyLFUError::InvalidSize(size) => write!(f, "invalid cache size {}", *size),
            WTinyLFUError::InvalidRecentRatio(r) => write!(f, "invalid recent ratio {}", *r),
            WTinyLFUError::InvalidGhostRatio(r) => write!(f, "invalid ghost ratio {}", *r),

            WTinyLFUError::Unknown => write!(f, "Unknown error"),
        }
    }
}

impl From<CacheError> for WTinyLFUError {
    fn from(c_e: CacheError) -> Self {
        match c_e {
            CacheError::InvalidSize(size) => WTinyLFUError::InvalidSize(size),
            CacheError::InvalidRecentRatio(r) => WTinyLFUError::InvalidRecentRatio(r),
            CacheError::InvalidGhostRatio(r) => WTinyLFUError::InvalidGhostRatio(r),
        }
    }
}

impl From<TinyLFUError> for WTinyLFUError {
    fn from(t_e: TinyLFUError) -> Self {
        match t_e {
            TinyLFUError::InvalidCountMinWidth(e) => WTinyLFUError::InvalidCountMinWidth(e),
            TinyLFUError::InvalidSamples(e) => WTinyLFUError::InvalidSamples(e),
            TinyLFUError::InvalidFalsePositiveRatio(e) => {
                WTinyLFUError::InvalidFalsePositiveRatio(e)
            }
            TinyLFUError::InvalidKeyHasher => WTinyLFUError::InvalidKeyHasher,
        }
    }
}

impl Display for WTinyLFUError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.fmt(f)
    }
}

impl Debug for WTinyLFUError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.fmt(f)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for WTinyLFUError {}
