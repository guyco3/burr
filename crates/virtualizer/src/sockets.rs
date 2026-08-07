use crate::exports::wasi::sockets::ip_name_lookup;
use crate::exports::wasi::sockets::types;
use crate::exports::wasi::sockets::types::*;
use crate::policy::{authorize_and_execute, Action};
use crate::VirtualizationProxy;

pub struct ProxyTcpSocket {
    pub inner: crate::wasi::sockets::types::TcpSocket,
}
impl types::GuestTcpSocket for ProxyTcpSocket {
    fn create(address_family: IpAddressFamily) -> Result<TcpSocket, ErrorCode> {
        let inner = crate::wasi::sockets::types::TcpSocket::create(unsafe {
            std::mem::transmute(address_family)
        })
        .map_err(|e| unsafe { std::mem::transmute(e) })?;
        Ok(TcpSocket::new(ProxyTcpSocket { inner }))
    }
    fn bind(&self, local_address: IpSocketAddress) -> Result<(), ErrorCode> {
        let ip_str = match &local_address {
            IpSocketAddress::Ipv4(v4) => format!(
                "{}.{}.{}.{}",
                v4.address.0, v4.address.1, v4.address.2, v4.address.3
            ),
            IpSocketAddress::Ipv6(v6) => format!("{:?}", v6.address),
        };
        let port = match &local_address {
            IpSocketAddress::Ipv4(v4) => v4.port,
            IpSocketAddress::Ipv6(v6) => v6.port,
        };
        authorize_and_execute(
            &[Action::SocketConnect { ip: ip_str, port }],
            || ErrorCode::AccessDenied,
            || {
                let res: Result<(), ErrorCode> = unsafe {
                    std::mem::transmute(
                        self.inner
                            .bind(unsafe { std::mem::transmute(local_address) }),
                    )
                };
                res
            },
        )?
    }
    async fn connect(&self, remote_address: IpSocketAddress) -> Result<(), ErrorCode> {
        let ip_str = match &remote_address {
            IpSocketAddress::Ipv4(v4) => format!(
                "{}.{}.{}.{}",
                v4.address.0, v4.address.1, v4.address.2, v4.address.3
            ),
            IpSocketAddress::Ipv6(v6) => format!("{:?}", v6.address),
        };
        let port = match &remote_address {
            IpSocketAddress::Ipv4(v4) => v4.port,
            IpSocketAddress::Ipv6(v6) => v6.port,
        };
        authorize_and_execute(
            &[Action::SocketConnect { ip: ip_str, port }],
            || ErrorCode::AccessDenied,
            || async {
                unsafe {
                    std::mem::transmute(
                        self.inner
                            .connect(unsafe { std::mem::transmute(remote_address) })
                            .await,
                    )
                }
            },
        )?
        .await
    }
    fn listen(&self) -> Result<wit_bindgen::rt::async_support::StreamReader<TcpSocket>, ErrorCode> {
        unsafe { std::mem::transmute(self.inner.listen()) }
    }
    fn send(
        &self,
        data: wit_bindgen::rt::async_support::StreamReader<u8>,
    ) -> wit_bindgen::rt::async_support::FutureReader<Result<(), ErrorCode>> {
        unsafe { std::mem::transmute(self.inner.send(data)) }
    }
    fn receive(
        &self,
    ) -> (
        wit_bindgen::rt::async_support::StreamReader<u8>,
        wit_bindgen::rt::async_support::FutureReader<Result<(), ErrorCode>>,
    ) {
        unsafe { std::mem::transmute(self.inner.receive()) }
    }
    fn get_local_address(&self) -> Result<IpSocketAddress, ErrorCode> {
        unsafe { std::mem::transmute(self.inner.get_local_address()) }
    }
    fn get_remote_address(&self) -> Result<IpSocketAddress, ErrorCode> {
        unsafe { std::mem::transmute(self.inner.get_remote_address()) }
    }
    fn get_is_listening(&self) -> bool {
        unsafe { std::mem::transmute(self.inner.get_is_listening()) }
    }
    fn get_address_family(&self) -> IpAddressFamily {
        unsafe { std::mem::transmute(self.inner.get_address_family()) }
    }
    fn set_listen_backlog_size(&self, value: u64) -> Result<(), ErrorCode> {
        unsafe { std::mem::transmute(self.inner.set_listen_backlog_size(value)) }
    }
    fn get_keep_alive_enabled(&self) -> Result<bool, ErrorCode> {
        unsafe { std::mem::transmute(self.inner.get_keep_alive_enabled()) }
    }
    fn set_keep_alive_enabled(&self, value: bool) -> Result<(), ErrorCode> {
        unsafe { std::mem::transmute(self.inner.set_keep_alive_enabled(value)) }
    }
    fn get_keep_alive_idle_time(&self) -> Result<Duration, ErrorCode> {
        unsafe { std::mem::transmute(self.inner.get_keep_alive_idle_time()) }
    }
    fn set_keep_alive_idle_time(&self, value: Duration) -> Result<(), ErrorCode> {
        unsafe { std::mem::transmute(self.inner.set_keep_alive_idle_time(value)) }
    }
    fn get_keep_alive_interval(&self) -> Result<Duration, ErrorCode> {
        unsafe { std::mem::transmute(self.inner.get_keep_alive_interval()) }
    }
    fn set_keep_alive_interval(&self, value: Duration) -> Result<(), ErrorCode> {
        unsafe { std::mem::transmute(self.inner.set_keep_alive_interval(value)) }
    }
    fn get_keep_alive_count(&self) -> Result<u32, ErrorCode> {
        unsafe { std::mem::transmute(self.inner.get_keep_alive_count()) }
    }
    fn set_keep_alive_count(&self, value: u32) -> Result<(), ErrorCode> {
        unsafe { std::mem::transmute(self.inner.set_keep_alive_count(value)) }
    }
    fn get_hop_limit(&self) -> Result<u8, ErrorCode> {
        unsafe { std::mem::transmute(self.inner.get_hop_limit()) }
    }
    fn set_hop_limit(&self, value: u8) -> Result<(), ErrorCode> {
        unsafe { std::mem::transmute(self.inner.set_hop_limit(value)) }
    }
    fn get_receive_buffer_size(&self) -> Result<u64, ErrorCode> {
        unsafe { std::mem::transmute(self.inner.get_receive_buffer_size()) }
    }
    fn set_receive_buffer_size(&self, value: u64) -> Result<(), ErrorCode> {
        unsafe { std::mem::transmute(self.inner.set_receive_buffer_size(value)) }
    }
    fn get_send_buffer_size(&self) -> Result<u64, ErrorCode> {
        unsafe { std::mem::transmute(self.inner.get_send_buffer_size()) }
    }
    fn set_send_buffer_size(&self, value: u64) -> Result<(), ErrorCode> {
        unsafe { std::mem::transmute(self.inner.set_send_buffer_size(value)) }
    }
}

pub struct ProxyUdpSocket {
    pub inner: crate::wasi::sockets::types::UdpSocket,
}
impl types::GuestUdpSocket for ProxyUdpSocket {
    fn create(address_family: IpAddressFamily) -> Result<UdpSocket, ErrorCode> {
        let inner = crate::wasi::sockets::types::UdpSocket::create(unsafe {
            std::mem::transmute(address_family)
        })
        .map_err(|e| unsafe { std::mem::transmute(e) })?;
        Ok(UdpSocket::new(ProxyUdpSocket { inner }))
    }
    fn bind(&self, local_address: IpSocketAddress) -> Result<(), ErrorCode> {
        let ip_str = match &local_address {
            IpSocketAddress::Ipv4(v4) => format!(
                "{}.{}.{}.{}",
                v4.address.0, v4.address.1, v4.address.2, v4.address.3
            ),
            IpSocketAddress::Ipv6(v6) => format!("{:?}", v6.address),
        };
        let port = match &local_address {
            IpSocketAddress::Ipv4(v4) => v4.port,
            IpSocketAddress::Ipv6(v6) => v6.port,
        };
        authorize_and_execute(
            &[Action::SocketConnect { ip: ip_str, port }],
            || ErrorCode::AccessDenied,
            || {
                let res: Result<(), ErrorCode> = unsafe {
                    std::mem::transmute(
                        self.inner
                            .bind(unsafe { std::mem::transmute(local_address) }),
                    )
                };
                res
            },
        )?
    }
    fn connect(&self, remote_address: IpSocketAddress) -> Result<(), ErrorCode> {
        let ip_str = match &remote_address {
            IpSocketAddress::Ipv4(v4) => format!(
                "{}.{}.{}.{}",
                v4.address.0, v4.address.1, v4.address.2, v4.address.3
            ),
            IpSocketAddress::Ipv6(v6) => format!("{:?}", v6.address),
        };
        let port = match &remote_address {
            IpSocketAddress::Ipv4(v4) => v4.port,
            IpSocketAddress::Ipv6(v6) => v6.port,
        };
        authorize_and_execute(
            &[Action::SocketConnect { ip: ip_str, port }],
            || ErrorCode::AccessDenied,
            || {
                let res: Result<(), ErrorCode> = unsafe {
                    std::mem::transmute(
                        self.inner
                            .connect(unsafe { std::mem::transmute(remote_address) }),
                    )
                };
                res
            },
        )?
    }
    fn disconnect(&self) -> Result<(), ErrorCode> {
        unsafe { std::mem::transmute(self.inner.disconnect()) }
    }
    async fn send(
        &self,
        data: Vec<u8>,
        remote_address: Option<IpSocketAddress>,
    ) -> Result<(), ErrorCode> {
        unsafe {
            std::mem::transmute(
                self.inner
                    .send(data, unsafe { std::mem::transmute(remote_address) })
                    .await,
            )
        }
    }
    async fn receive(&self) -> Result<(Vec<u8>, IpSocketAddress), ErrorCode> {
        unsafe { std::mem::transmute(self.inner.receive().await) }
    }
    fn get_local_address(&self) -> Result<IpSocketAddress, ErrorCode> {
        unsafe { std::mem::transmute(self.inner.get_local_address()) }
    }
    fn get_remote_address(&self) -> Result<IpSocketAddress, ErrorCode> {
        unsafe { std::mem::transmute(self.inner.get_remote_address()) }
    }
    fn get_address_family(&self) -> IpAddressFamily {
        unsafe { std::mem::transmute(self.inner.get_address_family()) }
    }
    fn get_unicast_hop_limit(&self) -> Result<u8, ErrorCode> {
        unsafe { std::mem::transmute(self.inner.get_unicast_hop_limit()) }
    }
    fn set_unicast_hop_limit(&self, value: u8) -> Result<(), ErrorCode> {
        unsafe { std::mem::transmute(self.inner.set_unicast_hop_limit(value)) }
    }
    fn get_receive_buffer_size(&self) -> Result<u64, ErrorCode> {
        unsafe { std::mem::transmute(self.inner.get_receive_buffer_size()) }
    }
    fn set_receive_buffer_size(&self, value: u64) -> Result<(), ErrorCode> {
        unsafe { std::mem::transmute(self.inner.set_receive_buffer_size(value)) }
    }
    fn get_send_buffer_size(&self) -> Result<u64, ErrorCode> {
        unsafe { std::mem::transmute(self.inner.get_send_buffer_size()) }
    }
    fn set_send_buffer_size(&self, value: u64) -> Result<(), ErrorCode> {
        unsafe { std::mem::transmute(self.inner.set_send_buffer_size(value)) }
    }
}

impl types::Guest for VirtualizationProxy {
    type TcpSocket = ProxyTcpSocket;
    type UdpSocket = ProxyUdpSocket;
}

impl ip_name_lookup::Guest for VirtualizationProxy {
    async fn resolve_addresses(
        name: String,
    ) -> Result<Vec<ip_name_lookup::IpAddress>, ip_name_lookup::ErrorCode> {
        authorize_and_execute(
            &[Action::DnsLookup(name.clone())],
            || ip_name_lookup::ErrorCode::NameUnresolvable,
            || async {
                log::debug!("WARDEN: Before inner.resolve_addresses for name: {}", name);
                let addrs =
                    crate::wasi::sockets::ip_name_lookup::resolve_addresses(name.clone()).await;
                log::debug!(
                    "WARDEN: After inner.resolve_addresses. Result: {}",
                    addrs.is_ok()
                );

                let addrs = addrs.map_err(|e| unsafe { std::mem::transmute(e) })?;
                Ok(addrs
                    .into_iter()
                    .map(|a| unsafe { std::mem::transmute(a) })
                    .collect())
            },
        )?
        .await
    }
}
