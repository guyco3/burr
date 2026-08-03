use crate::VirtualizationProxy;
use crate::policy::{Action, PolicyEngine};

impl crate::exports::wasi::clocks::monotonic_clock::Guest for VirtualizationProxy {
    fn now() -> crate::exports::wasi::clocks::monotonic_clock::Mark {
        let policy = crate::policy::get_engine();
        let _ = policy.authorize(&Action::ClockReadMonotonic);
        crate::wasi::clocks::monotonic_clock::now()
    }
    
    fn get_resolution() -> crate::exports::wasi::clocks::monotonic_clock::Duration {
        crate::wasi::clocks::monotonic_clock::get_resolution()
    }
    
    async fn wait_until(when: crate::exports::wasi::clocks::monotonic_clock::Mark) {
        crate::wasi::clocks::monotonic_clock::wait_until(when).await
    }
    
    async fn wait_for(how_long: crate::exports::wasi::clocks::monotonic_clock::Duration) {
        crate::wasi::clocks::monotonic_clock::wait_for(how_long).await
    }
}

impl crate::exports::wasi::clocks::system_clock::Guest for VirtualizationProxy {
    fn now() -> crate::exports::wasi::clocks::system_clock::Instant {
        let policy = crate::policy::get_engine();
        let _ = policy.authorize(&Action::ClockReadSystem);
        unsafe { std::mem::transmute(crate::wasi::clocks::system_clock::now()) }
    }

    fn get_resolution() -> crate::exports::wasi::clocks::system_clock::Duration {
        crate::wasi::clocks::system_clock::get_resolution()
    }
}
