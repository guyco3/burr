#[macro_export]
macro_rules! map_wasi_enum {
    ($host_type:ty, $guest_type:ty, $($variant:ident),+) => {
        impl From<$host_type> for $guest_type {
            fn from(host: $host_type) -> Self {
                match host {
                    $( <$host_type>::$variant => <$guest_type>::$variant, )+
                }
            }
        }
    };
}
