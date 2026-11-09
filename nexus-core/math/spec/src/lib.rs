//! # spec (math)
//!
//! The shared contract for the math library, written once.
//!
//! Every mode (backup, default, performant) must provide the same types with
//! the same names and signatures. The contract tests and the benchmarks live
//! here as macros, and each mode crate runs them against its own types. A
//! mode that is missing a method or behaves differently fails to compile or
//! fails its tests. They are added here as the types are built.
