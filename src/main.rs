/// This does use Syn + Parse crate, which increase build times. But Askama's attribute (procedural)
/// macro uses those anyway.
use const_format::formatcp;
use std::io;

mod entry;
mod fs;
mod server;

/// Environment variable name that contains the port number assigned by Deta.Space.
const ENV_PORT: &'static str = "PORT";
const DEFAULT_PORT: &'static str = "8080";

/// Environment variable name that contains private key ("data key", formerly known as "project
/// key", generated by Deta.Space. (See also
/// <https://deta.space/docs/en/build/fundamentals/data-storage#manual-setup>).
const ENV_DATA_KEY: &'static str = "DETA_PROJECT_KEY";

/// Environment variable name that contains "salt", so that users whom you give write hashes can't
/// brute-force your Deta.Space private key.
const ENV_SALT: &'static str = "SALT";

// Directory names here don't have a trailing slash.
//
const TMP: &'static str = "/tmp";
const DIRS: &'static str = formatcp!("{TMP}/wdav_dirs");

// Leading URL "segments" (top level directories). Warp requires them NOT to contain any slash.
const READ: &'static str = "read";
const WRITE: &'static str = "write";
const ADMIN: &'static str = "admin";
const ADD: &'static str = "add";

// Directories containing symlinks. These constants could use `const_format` crate. But that
// involves quote + syn = long build times. TODO reconsider because of Tokio, or don't use Tokio
// attrib. macro.
const SYMLINKS: &'static str = "/tmp/wdav_symlinks";
const SYMLINKS_WRITE: &'static str = formatcp!("{SYMLINKS}/{WRITE}");
const SYMLINKS_READ: &'static str = formatcp!("{SYMLINKS}/{READ}");

const CLEANUP_IN_PROGRESS: &'static str = formatcp!("{SYMLINKS}/CLEANUP_IN_PROGRESS");

#[tokio::main]
pub(crate) async fn main() -> io::Result<()> {
    server::main().await
}
