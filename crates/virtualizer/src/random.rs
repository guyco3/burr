use crate::VirtualizationProxy;
use crate::policy::{Action, PolicyEngine};

impl crate::exports::wasi::random::insecure::Guest for VirtualizationProxy {
    fn get_insecure_random_bytes(len: u64) -> Vec<u8> {
        let policy = crate::policy::get_engine();
        let _ = policy.authorize(&Action::RandomRead);
        crate::wasi::random::insecure::get_insecure_random_bytes(len)
    }
    
    fn get_insecure_random_u64() -> u64 {
        let policy = crate::policy::get_engine();
        let _ = policy.authorize(&Action::RandomRead);
        crate::wasi::random::insecure::get_insecure_random_u64()
    }
}

impl crate::exports::wasi::random::insecure_seed::Guest for VirtualizationProxy {
    fn get_insecure_seed() -> (u64, u64) {
        let policy = crate::policy::get_engine();
        let _ = policy.authorize(&Action::RandomRead);
        crate::wasi::random::insecure_seed::get_insecure_seed()
    }
}

impl crate::exports::wasi::random::random::Guest for VirtualizationProxy {
    fn get_random_bytes(len: u64) -> Vec<u8> {
        let policy = crate::policy::get_engine();
        let _ = policy.authorize(&Action::RandomRead);
        crate::wasi::random::random::get_random_bytes(len)
    }
    
    fn get_random_u64() -> u64 {
        let policy = crate::policy::get_engine();
        let _ = policy.authorize(&Action::RandomRead);
        crate::wasi::random::random::get_random_u64()
    }
}
