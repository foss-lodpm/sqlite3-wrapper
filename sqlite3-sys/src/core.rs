pub struct Database {
    pub rp: *mut crate::bindings::sqlite3,
}

pub use crate::bindings::SqlitePrimaryResult;
pub use crate::connection::Connection;
pub use crate::operations::Operations;
