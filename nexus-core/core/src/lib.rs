//! # nexus-core
//!
//! The central library of the Nexus engine. It contains no systems of its
//! own: it brings the smaller libraries together under one name, so the
//! engine only ever writes `nexus_core::...`.
//!
//! Every system exists in three modes, and all three are available side by
//! side:
//!
//! ```text
//! nexus_core::math::backup::*       <- nexus-core/math/backup
//! nexus_core::math::default::*      <- nexus-core/math/default
//! nexus_core::math::performant::*   <- nexus-core/math/performant
//! ```
//!
//! Each system is added here as a module when it is built, in build order.
//! The engine itself is separate and will depend on this crate later.

/// Vectors, matrices, quaternions, rays and transforms (build order #1).
pub mod math {
    /// The Backup mode: industry-standard, reliable, the reference.
    pub use nexus_math_backup as backup;
    /// The Default mode: best overall, the recommended one.
    pub use nexus_math_default as default;
    /// The Performant mode: raw speed above all.
    pub use nexus_math_performant as performant;
}

#[cfg(test)]
mod tests {
    #[test]
    fn math_modes_are_all_present() {
        assert_eq!(crate::math::backup::MODE, "backup");
        assert_eq!(crate::math::default::MODE, "default");
        assert_eq!(crate::math::performant::MODE, "performant");
    }
}
