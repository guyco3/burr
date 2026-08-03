use std::fs;
use std::path::Path;
use anyhow::Result;

const DEFAULT_POLICY: &str = r#"// Default wrdn policy: Deny all actions
forbid(
    principal,
    action,
    resource
);
"#;

pub fn ensure_policy_exists(wrdn_pkg_dir: &Path) -> Result<()> {
    let policy_path = wrdn_pkg_dir.join("policy.cedar");
    if !policy_path.exists() {
        fs::write(&policy_path, DEFAULT_POLICY)?;
        println!("Generated default deny-all policy at {}", policy_path.display());
    } else {
        println!("Using existing policy at {}", policy_path.display());
    }
    Ok(())
}
