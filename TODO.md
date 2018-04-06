- update `last_access_date` on get
- Refactor `tmpshare::storage::get_path(hash) -> Result<PathBuf, _>` to
  `tmpshare::storage::get(hash) -> Result<Metadata,_>`
- Use failure crate
- Implement list command
- Implement Trees, so we can serve collections of files
- Implement list HTTP endpoint (with authentication)
- Add cleanup method
- Unit tests
- Documentation