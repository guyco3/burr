#[macro_export]
macro_rules! bidirectional_wasi_enum {
    ($host_type:ty, $guest_type:ty, $($variant:ident),+) => {
        impl From<$host_type> for $guest_type {
            fn from(host: $host_type) -> Self {
                match host {
                    $( <$host_type>::$variant => <$guest_type>::$variant, )+
                }
            }
        }
        impl From<$guest_type> for $host_type {
            fn from(guest: $guest_type) -> Self {
                match guest {
                    $( <$guest_type>::$variant => <$host_type>::$variant, )+
                }
            }
        }
    };
}
