use crate::VirtualizationProxy;
use crate::policy::{Action, authorize_and_execute};
use crate::exports::wasi::filesystem::types;
use crate::exports::wasi::filesystem::preopens;
use crate::exports::wasi::filesystem::types::*;

pub struct ProxyDescriptor { pub inner: crate::wasi::filesystem::types::Descriptor }
impl types::GuestDescriptor for ProxyDescriptor {
    fn read_via_stream(
                        &self,
                        offset: Filesize,
                    ) -> (
                        wit_bindgen::rt::async_support::StreamReader<u8>,
                        wit_bindgen::rt::async_support::FutureReader<
                            Result<(), ErrorCode>,
                        >,
                    ) {
        unsafe { std::mem::transmute(self.inner.read_via_stream(offset)) }
    }
    fn write_via_stream(
                        &self,
                        data: wit_bindgen::rt::async_support::StreamReader<u8>,
                        offset: Filesize,
                    ) -> wit_bindgen::rt::async_support::FutureReader<
                        Result<(), ErrorCode>,
                    > {
        unimplemented!()
    }
    fn append_via_stream(
                        &self,
                        data: wit_bindgen::rt::async_support::StreamReader<u8>,
                    ) -> wit_bindgen::rt::async_support::FutureReader<
                        Result<(), ErrorCode>,
                    > {
        unimplemented!()
    }
    async fn advise(
                        &self,
                        offset: Filesize,
                        length: Filesize,
                        advice: Advice,
                    ) -> Result<(), ErrorCode> {
        let res: Result<(), ErrorCode> = unsafe { std::mem::transmute(self.inner.advise(offset, length, unsafe { std::mem::transmute(advice) }).await) };
        res
    }
    async fn sync_data(&self) -> Result<(), ErrorCode> {
        let res: Result<(), ErrorCode> = unsafe { std::mem::transmute(self.inner.sync_data().await) };
        res
    }

    async fn get_flags(&self) -> Result<DescriptorFlags, ErrorCode> {
        unsafe { std::mem::transmute(self.inner.get_flags().await) }
    }
    async fn get_type(&self) -> Result<DescriptorType, ErrorCode> {
        let res: Result<DescriptorType, ErrorCode> = unsafe { std::mem::transmute(self.inner.get_type().await) };
        res
    }
    async fn set_size(&self, size: Filesize) -> Result<(), ErrorCode> {
        unsafe { std::mem::transmute(self.inner.set_size(size).await) }
    }
    async fn set_times(
                        &self,
                        data_access_timestamp: NewTimestamp,
                        data_modification_timestamp: NewTimestamp,
                    ) -> Result<(), ErrorCode> {
        unsafe { std::mem::transmute(self.inner.set_times(unsafe { std::mem::transmute(data_access_timestamp) }, unsafe { std::mem::transmute(data_modification_timestamp) }).await) }
    }
    fn read_directory(
                        &self,
                    ) -> (
                        wit_bindgen::rt::async_support::StreamReader<DirectoryEntry>,
                        wit_bindgen::rt::async_support::FutureReader<
                            Result<(), ErrorCode>,
                        >,
                    ) {
        let (s, f) = self.inner.read_directory();
        unsafe { (std::mem::transmute(s), std::mem::transmute(f)) }
    }
    async fn sync(&self) -> Result<(), ErrorCode> {
        unsafe { std::mem::transmute(self.inner.sync().await) }
    }
    async fn create_directory_at(
                        &self,
                        path: String,
                    ) -> Result<(), ErrorCode> {
        authorize_and_execute(
            &[Action::FsWrite(path.clone())],
            || ErrorCode::Access,
            || async {
                unsafe { std::mem::transmute(self.inner.create_directory_at(path).await) }
            }
        )?.await
    }
    async fn stat(&self) -> Result<DescriptorStat, ErrorCode> {
        unsafe { std::mem::transmute(self.inner.stat().await) }
    }
    async fn stat_at(
                        &self,
                        path_flags: PathFlags,
                        path: String,
                    ) -> Result<DescriptorStat, ErrorCode> {
        authorize_and_execute(
            &[Action::FsRead(path.clone())],
            || ErrorCode::Access,
            || async {
                unsafe { std::mem::transmute(self.inner.stat_at(unsafe { std::mem::transmute(path_flags) }, path).await) }
            }
        )?.await
    }
    async fn set_times_at(
                        &self,
                        path_flags: PathFlags,
                        path: String,
                        data_access_timestamp: NewTimestamp,
                        data_modification_timestamp: NewTimestamp,
                    ) -> Result<(), ErrorCode> {
        authorize_and_execute(
            &[Action::FsWrite(path.clone())],
            || ErrorCode::Access,
            || async {
                unsafe { std::mem::transmute(self.inner.set_times_at(unsafe { std::mem::transmute(path_flags) }, path, unsafe { std::mem::transmute(data_access_timestamp) }, unsafe { std::mem::transmute(data_modification_timestamp) }).await) }
            }
        )?.await
    }
    async fn link_at(
                        &self,
                        old_path_flags: PathFlags,
                        old_path: String,
                        new_descriptor: DescriptorBorrow<'_>,
                        new_path: String,
                    ) -> Result<(), ErrorCode> {
        authorize_and_execute(
            &[Action::FsRead(old_path.clone()), Action::FsWrite(new_path.clone())],
            || ErrorCode::Access,
            || async {
                unsafe { std::mem::transmute(self.inner.link_at(unsafe { std::mem::transmute(old_path_flags) }, old_path, &new_descriptor.get::<ProxyDescriptor>().inner, new_path).await) }
            }
        )?.await
    }
    async fn open_at(
                        &self,
                        path_flags: PathFlags,
                        path: String,
                        open_flags: OpenFlags,
                        flags: DescriptorFlags,
                    ) -> Result<Descriptor, ErrorCode> {
        authorize_and_execute(
            &[Action::FsRead(path.clone())],
            || ErrorCode::Access,
            || async {
                let inner = self.inner.open_at(unsafe { std::mem::transmute(path_flags) }, path, unsafe { std::mem::transmute(open_flags) }, unsafe { std::mem::transmute(flags) }).await
                    .map_err(|e| unsafe { std::mem::transmute(e) })?;
                Ok(Descriptor::new(ProxyDescriptor { inner }))
            }
        )?.await
    }
    async fn readlink_at(
                        &self,
                        path: String,
                    ) -> Result<String, ErrorCode> {
        authorize_and_execute(
            &[Action::FsRead(path.clone())],
            || ErrorCode::Access,
            || async {
                unsafe { std::mem::transmute(self.inner.readlink_at(path).await) }
            }
        )?.await
    }
    async fn remove_directory_at(
                        &self,
                        path: String,
                    ) -> Result<(), ErrorCode> {
        authorize_and_execute(
            &[Action::FsWrite(path.clone())],
            || ErrorCode::Access,
            || async {
                unsafe { std::mem::transmute(self.inner.remove_directory_at(path).await) }
            }
        )?.await
    }
    async fn rename_at(
                        &self,
                        old_path: String,
                        new_descriptor: DescriptorBorrow<'_>,
                        new_path: String,
                    ) -> Result<(), ErrorCode> {
        authorize_and_execute(
            &[Action::FsWrite(old_path.clone()), Action::FsWrite(new_path.clone())],
            || ErrorCode::Access,
            || async {
                unsafe { std::mem::transmute(self.inner.rename_at(old_path, &new_descriptor.get::<ProxyDescriptor>().inner, new_path).await) }
            }
        )?.await
    }
    async fn symlink_at(
                        &self,
                        old_path: String,
                        new_path: String,
                    ) -> Result<(), ErrorCode> {
        authorize_and_execute(
            &[Action::FsWrite(new_path.clone())],
            || ErrorCode::Access,
            || async {
                unsafe { std::mem::transmute(self.inner.symlink_at(old_path, new_path).await) }
            }
        )?.await
    }
    async fn unlink_file_at(
                        &self,
                        path: String,
                    ) -> Result<(), ErrorCode> {
        authorize_and_execute(
            &[Action::FsWrite(path.clone())],
            || ErrorCode::Access,
            || async {
                unsafe { std::mem::transmute(self.inner.unlink_file_at(path).await) }
            }
        )?.await
    }
    async fn is_same_object(&self, other: DescriptorBorrow<'_>) -> bool {
        unsafe { std::mem::transmute(self.inner.is_same_object(&other.get::<ProxyDescriptor>().inner).await) }
    }
    async fn metadata_hash(
                        &self,
                    ) -> Result<MetadataHashValue, ErrorCode> {
        unsafe { std::mem::transmute(self.inner.metadata_hash().await) }
    }
    async fn metadata_hash_at(
                        &self,
                        path_flags: PathFlags,
                        path: String,
                    ) -> Result<MetadataHashValue, ErrorCode> {
        authorize_and_execute(
            &[Action::FsRead(path.clone())],
            || ErrorCode::Access,
            || async {
                unsafe { std::mem::transmute(self.inner.metadata_hash_at(unsafe { std::mem::transmute(path_flags) }, path).await) }
            }
        )?.await
    }
}

impl types::Guest for VirtualizationProxy {
    type Descriptor = ProxyDescriptor;
}

impl preopens::Guest for VirtualizationProxy {
    fn get_directories() -> Vec<(types::Descriptor, String)> {
        crate::wasi::filesystem::preopens::get_directories().into_iter().map(|(d, s)| (types::Descriptor::new(ProxyDescriptor { inner: d }), s)).collect()
    }
}
