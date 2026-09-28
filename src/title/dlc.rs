pub const ADDCONT_PATH: &str = "/user/addcont";

#[derive(Debug)]
pub struct DLC {
    pub id: String,

    // Installed ac.pkg
    pub package_path: String,

    // Present only if decrypted DLC is currently mounted
    pub mounted_path: Option<String>,
}

// // Find DLCs
// let mounts = match ftp.nlst(Some(ADDCONT_PATH)) {
//     Ok(mounts) => mounts,
//     Err(_) => todo!(),
//     // Err(error) => return Err(error),
// };
// println!("{:?}", mounts);
