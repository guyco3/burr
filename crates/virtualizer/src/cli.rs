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


impl crate::exports::wasi::cli::stderr::Guest for VirtualizationProxy {
    fn write_via_stream(_data: wit_bindgen::rt::async_support::StreamReader<u8>) -> wit_bindgen::rt::async_support::FutureReader<Result<(), crate::wasi::cli::types::ErrorCode>> { 
        unimplemented!() 
    }
}

impl crate::exports::wasi::cli::stdin::Guest for VirtualizationProxy {
    fn read_via_stream() -> (wit_bindgen::rt::async_support::StreamReader<u8>, wit_bindgen::rt::async_support::FutureReader<Result<(), crate::exports::wasi::cli::stdin::ErrorCode>>) { 
        unimplemented!() 
    }
}

impl crate::exports::wasi::cli::stdout::Guest for VirtualizationProxy {
    fn write_via_stream(_data: wit_bindgen::rt::async_support::StreamReader<u8>) -> wit_bindgen::rt::async_support::FutureReader<Result<(), crate::wasi::cli::types::ErrorCode>> { 
        unimplemented!() 
    }
}

pub struct ProxyTerminalInput {
    pub inner: crate::wasi::cli::terminal_input::TerminalInput,
}
impl crate::exports::wasi::cli::terminal_input::GuestTerminalInput for ProxyTerminalInput {}
impl crate::exports::wasi::cli::terminal_input::Guest for VirtualizationProxy {
    type TerminalInput = ProxyTerminalInput;
}

pub struct ProxyTerminalOutput {
    pub inner: crate::wasi::cli::terminal_output::TerminalOutput,
}
impl crate::exports::wasi::cli::terminal_output::GuestTerminalOutput for ProxyTerminalOutput {}
impl crate::exports::wasi::cli::terminal_output::Guest for VirtualizationProxy {
    type TerminalOutput = ProxyTerminalOutput;
}

impl crate::exports::wasi::cli::terminal_stderr::Guest for VirtualizationProxy {
    fn get_terminal_stderr() -> Option<crate::exports::wasi::cli::terminal_output::TerminalOutput> {
        crate::wasi::cli::terminal_stderr::get_terminal_stderr().map(|inner| {
            crate::exports::wasi::cli::terminal_output::TerminalOutput::new(ProxyTerminalOutput { inner })
        })
    }
}

impl crate::exports::wasi::cli::terminal_stdin::Guest for VirtualizationProxy {
    fn get_terminal_stdin() -> Option<crate::exports::wasi::cli::terminal_input::TerminalInput> {
        crate::wasi::cli::terminal_stdin::get_terminal_stdin().map(|inner| {
            crate::exports::wasi::cli::terminal_input::TerminalInput::new(ProxyTerminalInput { inner })
        })
    }
}

impl crate::exports::wasi::cli::terminal_stdout::Guest for VirtualizationProxy {
    fn get_terminal_stdout() -> Option<crate::exports::wasi::cli::terminal_output::TerminalOutput> {
        crate::wasi::cli::terminal_stdout::get_terminal_stdout().map(|inner| {
            crate::exports::wasi::cli::terminal_output::TerminalOutput::new(ProxyTerminalOutput { inner })
        })
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

