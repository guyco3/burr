use anyhow::Result;
use std::fs;
use std::path::Path;

const DEFAULT_POLICY: &str = include_str!("assets/default_policy.cedar");

pub fn ensure_policy_exists(policy_path: &Path) -> Result<()> {
    if !policy_path.exists() {
        fs::write(&policy_path, DEFAULT_POLICY)?;
        log::info!(
            "Generated default deny-all policy at {}",
            policy_path.display()
        );
    } else {
        log::info!("Using existing policy at {}", policy_path.display());
    }
    Ok(())
}
