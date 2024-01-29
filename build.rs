//! The build script requires that a user has provided a valid
//! feature flag. If they provide too many feature flags, we fail.

use std::env;

// Keep this in sync with the available features
static SUPPORTED_FEATURES: &[&str] = &[
    "imxrt1010",
    "imxrt1020",
    "imxrt1050",
    "imxrt1060",
    "imxrt1064",
    "imxrt1170",
];

fn main() {
    let features: Vec<_> = env::vars()
        .map(|(key, _)| key)
        .flat_map(|key| key.strip_prefix("CARGO_FEATURE_").map(str::to_lowercase))
        .collect();

    let feature_count = features.len();

    if 0 == feature_count {
        panic!(
            "No feature selected! Available features: {}",
            SUPPORTED_FEATURES.join(" | ")
        );
    } else if feature_count > 1 {
        panic!(
            "Too many features selected! Detected features {:?}. Select one feature from the feature list: {}",
            features,
            SUPPORTED_FEATURES.join(" | ")
        );
    }
}
