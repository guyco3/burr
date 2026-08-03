use cedar_policy::{Authorizer, Context, Decision, Entities, EntityId, EntityTypeName, EntityUid, PolicySet, Request, Schema};
use std::str::FromStr;
use std::collections::HashMap;

pub enum Action {
    EnvRead(String),
    FsRead(String),
    FsWrite(String),
    HttpOutgoingRequest { url: String, method: String },
    SocketConnect { ip: String, port: u16 },
    DnsLookup(String),

    // Old fallbacks to keep things compiling while we migrate
    HttpIncomingRequest { url: String, method: String },
    CliExit,
    CliReadEnvironment,
    CliReadArguments,
    CliReadInitialCwd,
    ClockReadMonotonic,
    ClockReadSystem,
    RandomRead,
}

pub struct PolicyEngine {
    authorizer: Authorizer,
    policies: PolicySet,
    schema: Schema,
}

impl PolicyEngine {
    pub fn new() -> Self {
        let schema_str = include_str!("../policy/schema.cedarschema");
        let (schema, _) = Schema::from_cedarschema_str(schema_str).expect("Failed to parse schema");
        
        let policy_path = "/tmp/policy.cedar";
        let policy_str = std::fs::read_to_string(policy_path)
            .unwrap_or_else(|e| {
                println!("WARDEN INIT WARNING: Failed to read policy from {} ({}). Defaulting to DENY ALL.", policy_path, e);
                String::new() // Empty policy = deny all
            });
            
        let policies = PolicySet::from_str(&policy_str).unwrap_or_else(|e| {
            eprintln!("WARDEN INIT ERROR: Failed to parse policies ({}). Defaulting to DENY ALL.", e);
            PolicySet::new()
        });
        
        Self {
            authorizer: Authorizer::new(),
            policies,
            schema,
        }
    }

    pub fn authorize(&self, action_req: &Action) -> Result<(), ()> {
        let principal_id = "telemetry-demo";
        let principal = EntityUid::from_type_name_and_id(
            EntityTypeName::from_str("Warden::Module").unwrap(),
            EntityId::from_str(principal_id).unwrap(),
        );

        let (action_str, resource_str, mut ctx_map) = match action_req {
            Action::EnvRead(key) => {
                let mut map = HashMap::new();
                map.insert("key".to_string(), cedar_policy::RestrictedExpression::from_str(&format!("\"{}\"", key)).unwrap());
                ("env_read", "environment", map)
            },
            Action::FsRead(path) => {
                let mut map = HashMap::new();
                map.insert("path".to_string(), cedar_policy::RestrictedExpression::from_str(&format!("\"{}\"", path)).unwrap());
                ("fs_read", "filesystem", map)
            },
            Action::FsWrite(path) => {
                let mut map = HashMap::new();
                map.insert("path".to_string(), cedar_policy::RestrictedExpression::from_str(&format!("\"{}\"", path)).unwrap());
                ("fs_write", "filesystem", map)
            },
            Action::HttpOutgoingRequest { url, method } | Action::HttpIncomingRequest { url, method } => {
                let mut map = HashMap::new();
                map.insert("url".to_string(), cedar_policy::RestrictedExpression::from_str(&format!("\"{}\"", url)).unwrap());
                map.insert("method".to_string(), cedar_policy::RestrictedExpression::from_str(&format!("\"{}\"", method)).unwrap());
                ("http_request", "network", map)
            },
            Action::SocketConnect { ip, port } => {
                let mut map = HashMap::new();
                map.insert("ip".to_string(), cedar_policy::RestrictedExpression::from_str(&format!("\"{}\"", ip)).unwrap());
                map.insert("port".to_string(), cedar_policy::RestrictedExpression::from_str(&format!("{}", port)).unwrap());
                ("socket_connect", "network", map)
            },
            Action::DnsLookup(hostname) => {
                let mut map = HashMap::new();
                map.insert("hostname".to_string(), cedar_policy::RestrictedExpression::from_str(&format!("\"{}\"", hostname)).unwrap());
                ("dns_lookup", "network", map)
            },
            // Fallback: allow all benign actions that don't have strict Cedar policies in this MVP
            Action::CliExit | Action::CliReadEnvironment | Action::CliReadArguments | 
            Action::CliReadInitialCwd | Action::ClockReadMonotonic | Action::ClockReadSystem | Action::RandomRead => {
                return Ok(());
            }
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
        let request = Request::new(principal, action, resource, context, Some(&self.schema)).unwrap();
        let entities = Entities::empty();
        
        let answer = self.authorizer.is_authorized(&request, &self.policies, &entities);
        
        let decision_str = match answer.decision() {
            Decision::Allow => "ALLOW",
            Decision::Deny => "DENY",
        };
        
        let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        let log = format!(
            r#"{{"timestamp": {}, "module": "{}", "action": "{}", "resource": "{}", "decision": "{}"}}"#,
            timestamp, principal_id, action_str, resource_str, decision_str
        );
        println!("[WARDEN AUDIT] {}", log);
        
        match answer.decision() {
            Decision::Allow => Ok(()),
            Decision::Deny => Err(()),
        }
    }
}

pub static POLICY_ENGINE: std::sync::OnceLock<PolicyEngine> = std::sync::OnceLock::new();

pub fn get_engine() -> &'static PolicyEngine {
    POLICY_ENGINE.get_or_init(|| PolicyEngine::new())
}
