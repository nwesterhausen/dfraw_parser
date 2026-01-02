/// Options for configuring the database client behavior.
#[derive(Debug, Clone, Default)]
pub struct ClientOptions {
    /// If true, the database will be wiped and re-initialized on startup.
    pub reset_database: bool,
    /// If true, existing raw definitions will be updated if the identifier exists in the module.
    pub overwrite_raws: bool,
}
