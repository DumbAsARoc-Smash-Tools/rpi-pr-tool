pub trait IEZPRFile {

    /// Creates a new file instance in program memory. Overwrites
    /// previously opened file, so before calling this function,
    /// code should check if the end-user is okay with overwriting
    /// the previously opened file.
    fn new_file() -> anyhow::Result<()>;

    /// Saves the currently opened file to disk. If a file is
    /// not open, simply return Ok(). Any errors in the process
    /// are returned as Err().
    fn save_file<P>(file_path: P) -> anyhow::Result<()> where P: AsRef<std::path::Path>;

    /// Load a file on disk into program memory. Any errors
    /// with this process are reported as Err().
    fn load_file<P>(file_path: P) -> anyhow::Result<()> where P: AsRef<std::path::Path>;

    /// Closes the currently opened file without saving any changes.
    /// Any errors are reported as Err().
    fn close_file() -> anyhow::Result<()>;

    /// Returns `true` if a file is currently loaded into
    /// program memory, `false` otherwise.
    fn is_file_loaded() -> bool { false }
}
