//! The build script requires that a user has provided a valid
//! feature flag. If they provide too many feature flags, we fail.

use std::env;

// Keep this in sync with the available features
static SUPPORTED_FEATURES: &[&str] = &["imxrt1010", "imxrt1060", "imxrt1064"];

fn main() {
    let feature_count = SUPPORTED_FEATURES
        .iter()
        .cloned()
        .map(str::to_uppercase)
        .map(|feature| format!("CARGO_FEATURE_{}", feature))
        .map(|cargo_feature| env::var(cargo_feature).is_ok() as i32)
        .sum();

    if 0 == feature_count {
        panic!(
            "No feature selected! Available features: {}",
            SUPPORTED_FEATURES.join(" | ")
        );
    } else if feature_count > 1 {
        panic!(
            "Too many features selected! Select one feature from the feature list: {}",
            SUPPORTED_FEATURES.join(" | ")
        );
    }
}
