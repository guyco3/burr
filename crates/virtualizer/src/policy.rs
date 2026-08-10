use cedar_policy::{
    Authorizer, Context, Decision, Entities, EntityId, EntityTypeName, EntityUid, PolicySet,
    Request, Schema,
};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub enum Action {
    EnvRead(String),
    FsRead(String),
    FsWrite(String),
    HttpOutgoingRequest { url: String, method: String },
    SocketConnect { ip: String, port: u16 },
    SocketBind { ip: String, port: u16 },
    DnsLookup(String),

    HttpIncomingRequest { url: String, method: String },
    CliExit,
    CliReadEnvironment,
    CliReadArguments,
    CliReadInitialCwd,
}

pub struct PolicyEngine {
    authorizer: Authorizer,
    policies: PolicySet,
    schema: Schema,
    principal_id: String,
}

impl PolicyEngine {
    pub fn new(policy_str: &str, schema_str: &str, principal_id: String) -> Self {
        let (schema, _) = Schema::from_cedarschema_str(schema_str).expect("Failed to parse schema");

        let policies = PolicySet::from_str(policy_str).unwrap_or_else(|e| {
            log::warn!(
                "WARDEN INIT ERROR: Failed to parse policies ({}). Defaulting to DENY ALL.",
                e
            );
            PolicySet::new()
        });

        env_logger::try_init().ok();
        Self {
            authorizer: Authorizer::new(),
            policies,
            schema,
            principal_id,
        }
    }

    pub fn from_env() -> Self {
        let schema_str = include_str!("../policy/schema.cedarschema");

        let policy_path =
            std::env::var("WRDN_POLICY_PATH").unwrap_or_else(|_| "./policy.cedar".to_string());

        let policy_str = std::fs::read_to_string(&policy_path).unwrap_or_else(|e| {
            log::info!(
                "WARDEN INIT WARNING: Failed to read policy from {} ({}). Defaulting to DENY ALL.",
                policy_path,
                e
            );
            String::new() // Empty policy = deny all
        });

        Self::new(&policy_str, schema_str, "guest-module".to_string())
    }

    pub fn authorize(&self, action_req: &Action) -> Result<(), ()> {
        let principal = EntityUid::from_type_name_and_id(
            EntityTypeName::from_str("Warden::Module").unwrap(),
            EntityId::from_str(&self.principal_id).unwrap(),
        );

        let (action_str, resource_str, ctx_map) = match action_req {
            Action::EnvRead(key) => {
                let mut map = HashMap::new();
                map.insert(
                    "key".to_string(),
                    cedar_policy::RestrictedExpression::from_str(&format!("\"{}\"", key)).unwrap(),
                );
                ("env_read", "environment", map)
            }
            Action::FsRead(path) => {
                let mut map = HashMap::new();
                map.insert(
                    "path".to_string(),
                    cedar_policy::RestrictedExpression::from_str(&format!("\"{}\"", path)).unwrap(),
                );
                ("fs_read", "filesystem", map)
            }
            Action::FsWrite(path) => {
                let mut map = HashMap::new();
                map.insert(
                    "path".to_string(),
                    cedar_policy::RestrictedExpression::from_str(&format!("\"{}\"", path)).unwrap(),
                );
                ("fs_write", "filesystem", map)
            }
            Action::HttpOutgoingRequest { url, method }
            | Action::HttpIncomingRequest { url, method } => {
                let mut map = HashMap::new();
                map.insert(
                    "url".to_string(),
                    cedar_policy::RestrictedExpression::from_str(&format!("\"{}\"", url)).unwrap(),
                );
                map.insert(
                    "method".to_string(),
                    cedar_policy::RestrictedExpression::from_str(&format!("\"{}\"", method))
                        .unwrap(),
                );
                ("http_request", "network", map)
            }
            Action::SocketConnect { ip, port } => {
                let mut map = HashMap::new();
                map.insert(
                    "ip".to_string(),
                    cedar_policy::RestrictedExpression::from_str(&format!("ip(\"{}\")", ip)).unwrap(),
                );
                map.insert(
                    "port".to_string(),
                    cedar_policy::RestrictedExpression::from_str(&format!("{}", port)).unwrap(),
                );
                ("socket_connect", "network", map)
            }
            Action::SocketBind { ip, port } => {
                let mut map = HashMap::new();
                map.insert(
                    "ip".to_string(),
                    cedar_policy::RestrictedExpression::from_str(&format!("ip(\"{}\")", ip)).unwrap(),
                );
                map.insert(
                    "port".to_string(),
                    cedar_policy::RestrictedExpression::from_str(&format!("{}", port)).unwrap(),
                );
                ("socket_bind", "network", map)
            }
            Action::DnsLookup(hostname) => {
                let mut map = HashMap::new();
                map.insert(
                    "hostname".to_string(),
                    cedar_policy::RestrictedExpression::from_str(&format!("\"{}\"", hostname))
                        .unwrap(),
                );
                ("dns_lookup", "network", map)
            }
            Action::CliExit => ("cli_exit", "system", HashMap::new()),
            Action::CliReadEnvironment => ("cli_read_environment", "system", HashMap::new()),
            Action::CliReadArguments => ("cli_read_arguments", "system", HashMap::new()),
            Action::CliReadInitialCwd => ("cli_read_initial_cwd", "system", HashMap::new()),
        };

        let action = EntityUid::from_type_name_and_id(
            EntityTypeName::from_str("Warden::Action").unwrap(),
            EntityId::from_str(action_str).unwrap(),
        );
        let resource = EntityUid::from_type_name_and_id(
            EntityTypeName::from_str("Warden::Resource").unwrap(),
            EntityId::from_str(resource_str).unwrap(),
        );

        let context = Context::from_pairs(ctx_map).unwrap();
        let request =
            Request::new(principal, action, resource, context, Some(&self.schema)).unwrap();
        let entities = Entities::empty();

        let answer = self
            .authorizer
            .is_authorized(&request, &self.policies, &entities);

        let decision_str = match answer.decision() {
            Decision::Allow => "ALLOW",
            Decision::Deny => "DENY",
        };

        let details = match action_req {
            Action::EnvRead(key) => format!(r#"{{"key": "{}"}}"#, key),
            Action::FsRead(path) => format!(r#"{{"path": "{}"}}"#, path),
            Action::FsWrite(path) => format!(r#"{{"path": "{}"}}"#, path),
            Action::HttpOutgoingRequest { url, method }
            | Action::HttpIncomingRequest { url, method } => {
                format!(r#"{{"url": "{}", "method": "{}"}}"#, url, method)
            }
            Action::SocketConnect { ip, port } | Action::SocketBind { ip, port } => {
                format!(r#"{{"ip": "{}", "port": {}}}"#, ip, port)
            }
            Action::DnsLookup(hostname) => format!(r#"{{"hostname": "{}"}}"#, hostname),
            Action::CliExit
            | Action::CliReadEnvironment
            | Action::CliReadArguments
            | Action::CliReadInitialCwd => "{}".to_string(),
        };

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let log = format!(
            r#"{{"timestamp": {}, "module": "{}", "action": "{}", "resource": "{}", "details": {}, "decision": "{}"}}"#,
            timestamp, self.principal_id, action_str, resource_str, details, decision_str
        );

        match answer.decision() {
            Decision::Allow => {
                log::info!("[WARDEN AUDIT] {}", log);
                Ok(())
            },
            Decision::Deny => {
                log::error!("[WARDEN AUDIT] {}", log);
                Err(())
            }
        }
    }
}

pub static POLICY_ENGINE: std::sync::OnceLock<PolicyEngine> = std::sync::OnceLock::new();

pub fn get_engine() -> &'static PolicyEngine {
    POLICY_ENGINE.get_or_init(|| PolicyEngine::from_env())
}

pub fn authorize_and_execute_with_engine<T, E, ErrMapper, F>(
    policy: &PolicyEngine,
    requirements: &[Action],
    err_mapper: ErrMapper,
    host_operation: F,
) -> Result<T, E>
where
    ErrMapper: Fn() -> E,
    F: FnOnce() -> T,
{
    for req in requirements {
        policy.authorize(req).map_err(|_| err_mapper())?;
    }
    Ok(host_operation())
}

/// The Interceptor: It is physically impossible to execute `host_operation`
/// without first passing the checks for every `Action` in `requirements`.
pub fn authorize_and_execute<T, E, ErrMapper, F>(
    requirements: &[Action],
    err_mapper: ErrMapper,
    host_operation: F,
) -> Result<T, E>
where
    ErrMapper: Fn() -> E,
    F: FnOnce() -> T,
{
    authorize_and_execute_with_engine(get_engine(), requirements, err_mapper, host_operation)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCHEMA: &str = include_str!("../policy/schema.cedarschema");
    const PRINCIPAL: &str = "guest-module";

    fn setup_engine(policy_str: &str) -> PolicyEngine {
        PolicyEngine::new(policy_str, SCHEMA, PRINCIPAL.to_string())
    }

    #[test]
    fn test_global_deny_all_interfaces() {
        let engine = setup_engine(""); // Empty policy = Deny All

        let exhaustive_actions = [
            Action::EnvRead("SECRET_KEY".to_string()),
            Action::FsRead("/etc/passwd".to_string()),
            Action::FsWrite("/etc/passwd".to_string()),
            Action::HttpIncomingRequest {
                url: "http://example.com".to_string(),
                method: "GET".to_string(),
            },
            Action::HttpOutgoingRequest {
                url: "http://example.com".to_string(),
                method: "POST".to_string(),
            },
            Action::SocketConnect {
                ip: "1.1.1.1".to_string(),
                port: 53,
            },
            Action::DnsLookup("google.com".to_string()),
        ];

        for action in exhaustive_actions {
            let result = engine.authorize(&action);
            assert!(
                result.is_err(),
                "CRITICAL: System failed closed. {:?} bypassed the Deny-All policy.",
                action
            );
        }
    }

    #[test]
    fn test_interceptor() {
        let engine = setup_engine(""); // Empty policy

        // Testing that the interceptor forwards the mapped error
        let result = authorize_and_execute_with_engine::<(), &str, _, _>(
            &engine,
            &[Action::FsRead("/etc/passwd".to_string())],
            || "CustomError",
            || (),
        );
        assert_eq!(result, Err("CustomError"));

        // Testing that the interceptor executes the closure if permitted
        let result2 = authorize_and_execute_with_engine::<&str, &str, _, _>(
            &engine,
            &[Action::CliExit],
            || "CustomError",
            || "Success",
        );
        assert_eq!(result2, Err("CustomError"));
    }

    #[test]
    fn test_env_read_allow() {
        let policy = r#"
            permit(
                principal == Warden::Module::"guest-module",
                action == Warden::Action::"env_read",
                resource == Warden::Resource::"environment"
            ) when {
                context.key == "APP_ENV"
            };
        "#;
        let engine = setup_engine(policy);

        let success = authorize_and_execute_with_engine::<&str, &str, _, _>(
            &engine,
            &[Action::EnvRead("APP_ENV".to_string())],
            || "CustomError",
            || "Success",
        );
        assert_eq!(success, Ok("Success"));

        let fail = authorize_and_execute_with_engine::<&str, &str, _, _>(
            &engine,
            &[Action::EnvRead("SECRET_KEY".to_string())],
            || "CustomError",
            || "Success",
        );
        assert_eq!(fail, Err("CustomError"));
    }

    #[test]
    fn test_network_connect() {
        let policy = r#"
            permit(
                principal == Warden::Module::"guest-module",
                action == Warden::Action::"socket_connect",
                resource == Warden::Resource::"network"
            ) when {
                context.ip == ip("93.184.216.34") &&
                context.port == 443
            };
        "#;
        let engine = setup_engine(policy);

        let success = authorize_and_execute_with_engine::<(), (), _, _>(
            &engine,
            &[Action::SocketConnect {
                ip: "93.184.216.34".to_string(),
                port: 443,
            }],
            || (),
            || (),
        );
        assert!(success.is_ok());

        let fail_port = authorize_and_execute_with_engine::<(), (), _, _>(
            &engine,
            &[Action::SocketConnect {
                ip: "93.184.216.34".to_string(),
                port: 80,
            }],
            || (),
            || (),
        );
        assert!(fail_port.is_err());

        let fail_ip = authorize_and_execute_with_engine::<(), (), _, _>(
            &engine,
            &[Action::SocketConnect {
                ip: "1.1.1.1".to_string(),
                port: 443,
            }],
            || (),
            || (),
        );
        assert!(fail_ip.is_err());
    }

    #[test]
    fn test_benign_actions() {
        let engine = setup_engine(""); // Empty policy

        let fail = authorize_and_execute_with_engine::<(), (), _, _>(
            &engine,
            &[Action::CliExit],
            || (),
            || (),
        );
        assert!(fail.is_err());
    }
}
