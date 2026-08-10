use crate::exports::wasi::filesystem::types::{DescriptorFlags, OpenFlags};
fn check(f: DescriptorFlags, o: OpenFlags) {
    let _ = f.contains(DescriptorFlags::WRITE);
    let _ = f.contains(DescriptorFlags::READ);
    let _ = o.contains(OpenFlags::CREATE);
}
