use crate::VirtualizationProxy;
use crate::policy::{Action, PolicyEngine};

impl crate::exports::wasi::cli::exit::Guest for VirtualizationProxy {
    fn exit(status: Result<(), ()>) -> () {
        let policy = crate::policy::get_engine();
        let _ = policy.authorize(&Action::CliExit);
        crate::wasi::cli::exit::exit(status)
    }
    fn exit_with_code(status: u8) -> () {
        let policy = crate::policy::get_engine();
        let _ = policy.authorize(&Action::CliExit);
        crate::wasi::cli::exit::exit_with_code(status)
    }
}


impl crate::exports::wasi::cli::environment::Guest for VirtualizationProxy {
    fn get_environment() -> Vec<(String, String)> {
        let policy = crate::policy::get_engine();
        let env = crate::wasi::cli::environment::get_environment();
        env.into_iter().filter(|(k, _)| {
            policy.authorize(&Action::EnvRead(k.clone())).is_ok()
        }).collect()
    }
    
    fn get_arguments() -> Vec<String> {
        let policy = crate::policy::get_engine();
        let _ = policy.authorize(&Action::CliReadArguments);
        crate::wasi::cli::environment::get_arguments()
    }
    
    fn get_initial_cwd() -> Option<String> {
        let policy = crate::policy::get_engine();
        let _ = policy.authorize(&Action::CliReadInitialCwd);
        crate::wasi::cli::environment::get_initial_cwd()
    }
}

