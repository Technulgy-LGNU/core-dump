pub mod ai_types;
pub mod cp_types;
mod legacy_ai;

// The currently deployed AI crates still use the reference-based command API.
// Keep that stable at `core_dump::types::*`; the newer world/intent model is
// available explicitly through `types::ai_types`.
pub use legacy_ai::*;
