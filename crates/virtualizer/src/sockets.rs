use crate::exports::wasi::sockets::ip_name_lookup;
use crate::exports::wasi::sockets::types;
use crate::exports::wasi::sockets::types::*;
use crate::policy::{authorize_and_execute, Action};
use crate::VirtualizationProxy;


impl From<crate::wasi::sockets::types::IpAddressFamily> for crate::exports::wasi::sockets::types::IpAddressFamily {
    fn from(value: crate::wasi::sockets::types::IpAddressFamily) -> Self {
        match value {
            crate::wasi::sockets::types::IpAddressFamily::Ipv4 => crate::exports::wasi::sockets::types::IpAddressFamily::Ipv4,
            crate::wasi::sockets::types::IpAddressFamily::Ipv6 => crate::exports::wasi::sockets::types::IpAddressFamily::Ipv6,
        }
    }
}
impl From<crate::exports::wasi::sockets::types::IpAddressFamily> for crate::wasi::sockets::types::IpAddressFamily {
    fn from(value: crate::exports::wasi::sockets::types::IpAddressFamily) -> Self {
        match value {
            crate::exports::wasi::sockets::types::IpAddressFamily::Ipv4 => crate::wasi::sockets::types::IpAddressFamily::Ipv4,
            crate::exports::wasi::sockets::types::IpAddressFamily::Ipv6 => crate::wasi::sockets::types::IpAddressFamily::Ipv6,
        }
    }
}
impl From<crate::wasi::sockets::types::ErrorCode> for crate::exports::wasi::sockets::types::ErrorCode {
    fn from(value: crate::wasi::sockets::types::ErrorCode) -> Self {
        match value {
            crate::wasi::sockets::types::ErrorCode::AccessDenied => crate::exports::wasi::sockets::types::ErrorCode::AccessDenied,
            crate::wasi::sockets::types::ErrorCode::NotSupported => crate::exports::wasi::sockets::types::ErrorCode::NotSupported,
            crate::wasi::sockets::types::ErrorCode::InvalidArgument => crate::exports::wasi::sockets::types::ErrorCode::InvalidArgument,
            crate::wasi::sockets::types::ErrorCode::OutOfMemory => crate::exports::wasi::sockets::types::ErrorCode::OutOfMemory,
            crate::wasi::sockets::types::ErrorCode::Timeout => crate::exports::wasi::sockets::types::ErrorCode::Timeout,
            crate::wasi::sockets::types::ErrorCode::InvalidState => crate::exports::wasi::sockets::types::ErrorCode::InvalidState,
            crate::wasi::sockets::types::ErrorCode::AddressNotBindable => crate::exports::wasi::sockets::types::ErrorCode::AddressNotBindable,
            crate::wasi::sockets::types::ErrorCode::AddressInUse => crate::exports::wasi::sockets::types::ErrorCode::AddressInUse,
            crate::wasi::sockets::types::ErrorCode::RemoteUnreachable => crate::exports::wasi::sockets::types::ErrorCode::RemoteUnreachable,
            crate::wasi::sockets::types::ErrorCode::ConnectionRefused => crate::exports::wasi::sockets::types::ErrorCode::ConnectionRefused,
            crate::wasi::sockets::types::ErrorCode::ConnectionBroken => crate::exports::wasi::sockets::types::ErrorCode::ConnectionBroken,
            crate::wasi::sockets::types::ErrorCode::ConnectionReset => crate::exports::wasi::sockets::types::ErrorCode::ConnectionReset,
            crate::wasi::sockets::types::ErrorCode::ConnectionAborted => crate::exports::wasi::sockets::types::ErrorCode::ConnectionAborted,
            crate::wasi::sockets::types::ErrorCode::DatagramTooLarge => crate::exports::wasi::sockets::types::ErrorCode::DatagramTooLarge,
            crate::wasi::sockets::types::ErrorCode::Other(s) => crate::exports::wasi::sockets::types::ErrorCode::Other(s),
        }
    }
}
impl From<crate::exports::wasi::sockets::types::ErrorCode> for crate::wasi::sockets::types::ErrorCode {
    fn from(value: crate::exports::wasi::sockets::types::ErrorCode) -> Self {
        match value {
            crate::exports::wasi::sockets::types::ErrorCode::AccessDenied => crate::wasi::sockets::types::ErrorCode::AccessDenied,
            crate::exports::wasi::sockets::types::ErrorCode::NotSupported => crate::wasi::sockets::types::ErrorCode::NotSupported,
            crate::exports::wasi::sockets::types::ErrorCode::InvalidArgument => crate::wasi::sockets::types::ErrorCode::InvalidArgument,
            crate::exports::wasi::sockets::types::ErrorCode::OutOfMemory => crate::wasi::sockets::types::ErrorCode::OutOfMemory,
            crate::exports::wasi::sockets::types::ErrorCode::Timeout => crate::wasi::sockets::types::ErrorCode::Timeout,
            crate::exports::wasi::sockets::types::ErrorCode::InvalidState => crate::wasi::sockets::types::ErrorCode::InvalidState,
            crate::exports::wasi::sockets::types::ErrorCode::AddressNotBindable => crate::wasi::sockets::types::ErrorCode::AddressNotBindable,
            crate::exports::wasi::sockets::types::ErrorCode::AddressInUse => crate::wasi::sockets::types::ErrorCode::AddressInUse,
            crate::exports::wasi::sockets::types::ErrorCode::RemoteUnreachable => crate::wasi::sockets::types::ErrorCode::RemoteUnreachable,
            crate::exports::wasi::sockets::types::ErrorCode::ConnectionRefused => crate::wasi::sockets::types::ErrorCode::ConnectionRefused,
            crate::exports::wasi::sockets::types::ErrorCode::ConnectionBroken => crate::wasi::sockets::types::ErrorCode::ConnectionBroken,
            crate::exports::wasi::sockets::types::ErrorCode::ConnectionReset => crate::wasi::sockets::types::ErrorCode::ConnectionReset,
            crate::exports::wasi::sockets::types::ErrorCode::ConnectionAborted => crate::wasi::sockets::types::ErrorCode::ConnectionAborted,
            crate::exports::wasi::sockets::types::ErrorCode::DatagramTooLarge => crate::wasi::sockets::types::ErrorCode::DatagramTooLarge,
            crate::exports::wasi::sockets::types::ErrorCode::Other(s) => crate::wasi::sockets::types::ErrorCode::Other(s),
        }
    }
}
impl From<crate::wasi::sockets::types::IpSocketAddress> for crate::exports::wasi::sockets::types::IpSocketAddress {
    fn from(value: crate::wasi::sockets::types::IpSocketAddress) -> Self {
        match value {
            crate::wasi::sockets::types::IpSocketAddress::Ipv4(v) => crate::exports::wasi::sockets::types::IpSocketAddress::Ipv4(crate::exports::wasi::sockets::types::Ipv4SocketAddress {
                port: v.port,
                address: v.address,
            }),
            crate::wasi::sockets::types::IpSocketAddress::Ipv6(v) => crate::exports::wasi::sockets::types::IpSocketAddress::Ipv6(crate::exports::wasi::sockets::types::Ipv6SocketAddress {
                port: v.port,
                flow_info: v.flow_info,
                address: v.address,
                scope_id: v.scope_id,
            }),
        }
    }
}
impl From<crate::exports::wasi::sockets::types::IpSocketAddress> for crate::wasi::sockets::types::IpSocketAddress {
    fn from(value: crate::exports::wasi::sockets::types::IpSocketAddress) -> Self {
        match value {
            crate::exports::wasi::sockets::types::IpSocketAddress::Ipv4(v) => crate::wasi::sockets::types::IpSocketAddress::Ipv4(crate::wasi::sockets::types::Ipv4SocketAddress {
                port: v.port,
                address: v.address,
            }),
            crate::exports::wasi::sockets::types::IpSocketAddress::Ipv6(v) => crate::wasi::sockets::types::IpSocketAddress::Ipv6(crate::wasi::sockets::types::Ipv6SocketAddress {
                port: v.port,
                flow_info: v.flow_info,
                address: v.address,
                scope_id: v.scope_id,
            }),
        }
    }
}
impl From<crate::wasi::sockets::types::IpAddress> for crate::exports::wasi::sockets::types::IpAddress {
    fn from(value: crate::wasi::sockets::types::IpAddress) -> Self {
        match value {
            crate::wasi::sockets::types::IpAddress::Ipv4(v) => crate::exports::wasi::sockets::types::IpAddress::Ipv4(v),
            crate::wasi::sockets::types::IpAddress::Ipv6(v) => crate::exports::wasi::sockets::types::IpAddress::Ipv6(v),
        }
    }
}
impl From<crate::exports::wasi::sockets::types::IpAddress> for crate::wasi::sockets::types::IpAddress {
    fn from(value: crate::exports::wasi::sockets::types::IpAddress) -> Self {
        match value {
            crate::exports::wasi::sockets::types::IpAddress::Ipv4(v) => crate::wasi::sockets::types::IpAddress::Ipv4(v),
            crate::exports::wasi::sockets::types::IpAddress::Ipv6(v) => crate::wasi::sockets::types::IpAddress::Ipv6(v),
        }
    }
}
impl From<crate::wasi::sockets::ip_name_lookup::ErrorCode> for crate::exports::wasi::sockets::ip_name_lookup::ErrorCode {
    fn from(value: crate::wasi::sockets::ip_name_lookup::ErrorCode) -> Self {
        match value {
            crate::wasi::sockets::ip_name_lookup::ErrorCode::AccessDenied => crate::exports::wasi::sockets::ip_name_lookup::ErrorCode::AccessDenied,
            crate::wasi::sockets::ip_name_lookup::ErrorCode::InvalidArgument => crate::exports::wasi::sockets::ip_name_lookup::ErrorCode::InvalidArgument,
            crate::wasi::sockets::ip_name_lookup::ErrorCode::NameUnresolvable => crate::exports::wasi::sockets::ip_name_lookup::ErrorCode::NameUnresolvable,
            crate::wasi::sockets::ip_name_lookup::ErrorCode::TemporaryResolverFailure => crate::exports::wasi::sockets::ip_name_lookup::ErrorCode::TemporaryResolverFailure,
            crate::wasi::sockets::ip_name_lookup::ErrorCode::PermanentResolverFailure => crate::exports::wasi::sockets::ip_name_lookup::ErrorCode::PermanentResolverFailure,
            crate::wasi::sockets::ip_name_lookup::ErrorCode::Other(s) => crate::exports::wasi::sockets::ip_name_lookup::ErrorCode::Other(s),
        }
    }
}
impl From<crate::exports::wasi::sockets::ip_name_lookup::ErrorCode> for crate::wasi::sockets::ip_name_lookup::ErrorCode {
    fn from(value: crate::exports::wasi::sockets::ip_name_lookup::ErrorCode) -> Self {
        match value {
            crate::exports::wasi::sockets::ip_name_lookup::ErrorCode::AccessDenied => crate::wasi::sockets::ip_name_lookup::ErrorCode::AccessDenied,
            crate::exports::wasi::sockets::ip_name_lookup::ErrorCode::InvalidArgument => crate::wasi::sockets::ip_name_lookup::ErrorCode::InvalidArgument,
            crate::exports::wasi::sockets::ip_name_lookup::ErrorCode::NameUnresolvable => crate::wasi::sockets::ip_name_lookup::ErrorCode::NameUnresolvable,
            crate::exports::wasi::sockets::ip_name_lookup::ErrorCode::TemporaryResolverFailure => crate::wasi::sockets::ip_name_lookup::ErrorCode::TemporaryResolverFailure,
            crate::exports::wasi::sockets::ip_name_lookup::ErrorCode::PermanentResolverFailure => crate::wasi::sockets::ip_name_lookup::ErrorCode::PermanentResolverFailure,
            crate::exports::wasi::sockets::ip_name_lookup::ErrorCode::Other(s) => crate::wasi::sockets::ip_name_lookup::ErrorCode::Other(s),
        }
    }
}


pub struct ProxyTcpSocket {
    pub inner: crate::wasi::sockets::types::TcpSocket,
}
impl types::GuestTcpSocket for ProxyTcpSocket {
    fn create(address_family: IpAddressFamily) -> Result<TcpSocket, ErrorCode> {
        let inner = crate::wasi::sockets::types::TcpSocket::create(address_family.into())
        ?;
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
                            .bind(local_address.into()),
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
                            .connect(remote_address.into())
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
        self.inner.get_local_address().map(|a| a.into()).map_err(Into::into)
    }
    fn get_remote_address(&self) -> Result<IpSocketAddress, ErrorCode> {
        self.inner.get_remote_address().map(|a| a.into()).map_err(Into::into)
    }
    fn get_is_listening(&self) -> bool {
        self.inner.get_is_listening()
    }
    fn get_address_family(&self) -> IpAddressFamily {
        self.inner.get_address_family().into()
    }
    fn set_listen_backlog_size(&self, value: u64) -> Result<(), ErrorCode> {
        self.inner.set_listen_backlog_size(value).map_err(Into::into)
    }
    fn get_keep_alive_enabled(&self) -> Result<bool, ErrorCode> {
        self.inner.get_keep_alive_enabled().map_err(Into::into)
    }
    fn set_keep_alive_enabled(&self, value: bool) -> Result<(), ErrorCode> {
        self.inner.set_keep_alive_enabled(value).map_err(Into::into)
    }
    fn get_keep_alive_idle_time(&self) -> Result<Duration, ErrorCode> {
        self.inner.get_keep_alive_idle_time().map_err(Into::into)
    }
    fn set_keep_alive_idle_time(&self, value: Duration) -> Result<(), ErrorCode> {
        self.inner.set_keep_alive_idle_time(value).map_err(Into::into)
    }
    fn get_keep_alive_interval(&self) -> Result<Duration, ErrorCode> {
        self.inner.get_keep_alive_interval().map_err(Into::into)
    }
    fn set_keep_alive_interval(&self, value: Duration) -> Result<(), ErrorCode> {
        self.inner.set_keep_alive_interval(value).map_err(Into::into)
    }
    fn get_keep_alive_count(&self) -> Result<u32, ErrorCode> {
        self.inner.get_keep_alive_count().map_err(Into::into)
    }
    fn set_keep_alive_count(&self, value: u32) -> Result<(), ErrorCode> {
        self.inner.set_keep_alive_count(value).map_err(Into::into)
    }
    fn get_hop_limit(&self) -> Result<u8, ErrorCode> {
        self.inner.get_hop_limit().map_err(Into::into)
    }
    fn set_hop_limit(&self, value: u8) -> Result<(), ErrorCode> {
        self.inner.set_hop_limit(value).map_err(Into::into)
    }
    fn get_receive_buffer_size(&self) -> Result<u64, ErrorCode> {
        self.inner.get_receive_buffer_size().map_err(Into::into)
    }
    fn set_receive_buffer_size(&self, value: u64) -> Result<(), ErrorCode> {
        self.inner.set_receive_buffer_size(value).map_err(Into::into)
    }
    fn get_send_buffer_size(&self) -> Result<u64, ErrorCode> {
        self.inner.get_send_buffer_size().map_err(Into::into)
    }
    fn set_send_buffer_size(&self, value: u64) -> Result<(), ErrorCode> {
        self.inner.set_send_buffer_size(value).map_err(Into::into)
    }
}

pub struct ProxyUdpSocket {
    pub inner: crate::wasi::sockets::types::UdpSocket,
}
impl types::GuestUdpSocket for ProxyUdpSocket {
    fn create(address_family: IpAddressFamily) -> Result<UdpSocket, ErrorCode> {
        let inner = crate::wasi::sockets::types::UdpSocket::create(address_family.into())
        ?;
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
                            .bind(local_address.into()),
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
                            .connect(remote_address.into()),
                    )
                };
                res
            },
        )?
    }
    fn disconnect(&self) -> Result<(), ErrorCode> {
        self.inner.disconnect().map_err(Into::into)
    }
    async fn send(
        &self,
        data: Vec<u8>,
        remote_address: Option<IpSocketAddress>,
    ) -> Result<(), ErrorCode> {
        self.inner.send(data, remote_address.map(Into::into)).await.map_err(Into::into)
    }
    async fn receive(&self) -> Result<(Vec<u8>, IpSocketAddress), ErrorCode> {
        self.inner.receive().await.map(|(d, a)| (d, a.into())).map_err(Into::into)
    }
    fn get_local_address(&self) -> Result<IpSocketAddress, ErrorCode> {
        self.inner.get_local_address().map(|a| a.into()).map_err(Into::into)
    }
    fn get_remote_address(&self) -> Result<IpSocketAddress, ErrorCode> {
        self.inner.get_remote_address().map(|a| a.into()).map_err(Into::into)
    }
    fn get_address_family(&self) -> IpAddressFamily {
        self.inner.get_address_family().into()
    }
    fn get_unicast_hop_limit(&self) -> Result<u8, ErrorCode> {
        self.inner.get_unicast_hop_limit().map_err(Into::into)
    }
    fn set_unicast_hop_limit(&self, value: u8) -> Result<(), ErrorCode> {
        self.inner.set_unicast_hop_limit(value).map_err(Into::into)
    }
    fn get_receive_buffer_size(&self) -> Result<u64, ErrorCode> {
        self.inner.get_receive_buffer_size().map_err(Into::into)
    }
    fn set_receive_buffer_size(&self, value: u64) -> Result<(), ErrorCode> {
        self.inner.set_receive_buffer_size(value).map_err(Into::into)
    }
    fn get_send_buffer_size(&self) -> Result<u64, ErrorCode> {
        self.inner.get_send_buffer_size().map_err(Into::into)
    }
    fn set_send_buffer_size(&self, value: u64) -> Result<(), ErrorCode> {
        self.inner.set_send_buffer_size(value).map_err(Into::into)
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

                let addrs = addrs?;
                Ok(addrs
                    .into_iter()
                    .map(|a| a.into())
                    .collect())
            },
        )?
        .await
    }
}
