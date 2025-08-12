use sqlite::Connection;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref EZPR_FILE_INSTANCE: Mutex<Option<EZPRFile>> = Mutex::new(None);
}

pub trait IEZPRFile {
    /// Creates a new file instance in program memory. Overwrites
    /// previously opened file, so before calling this function,
    /// code should check if the end-user is okay with overwriting
    /// the previously opened file.
    fn new_file() -> anyhow::Result<()>;

    /// Saves the currently opened file to disk. If a file is
    /// not open, simply return Ok(). Any errors in the process
    /// are returned as Err().
    fn save_file<P>(file_path: P) -> anyhow::Result<()>
    where
        P: ToString;

    /// Load a file on disk into program memory. This
    /// function will ensure that the file exists
    /// before attemtpting to load it. Any errors
    /// with this process are reported as Err().
    fn load_file<P>(file_path: P) -> anyhow::Result<()>
    where
        P: ToString;

    /// Closes the currently opened file without saving any changes.
    /// Any errors are reported as Err().
    fn close_file() -> anyhow::Result<()>;

    /// Returns `true` if a file is currently loaded into
    /// program memory, `false` otherwise.
    fn is_file_loaded() -> bool {
        false
    }
}

pub struct EZPRFile {
    /// The filename to which this database should
    /// be saved. If this is None, treat this as
    /// "Save As".
    ///
    /// @TODO: Come back to this spec.
    filename: String,

    /// A connection to a SQLite DB instance, which
    /// should (under normal circumstances) be opened
    /// with the query parameter "?mode=memory".
    ///
    /// @TODO: Come back to this spec.
    sqlite_connection: Connection,
}

impl IEZPRFile for EZPRFile {
    fn close_file() -> anyhow::Result<()> {
        todo!()
    }

    fn is_file_loaded() -> bool {
        EZPR_FILE_INSTANCE.lock().unwrap().is_some()
    }

    fn load_file<P>(file_path: P) -> anyhow::Result<()>
    where
        P: ToString,
    {
        let path = file_path.to_string();
        let file = std::fs::File::open(&path)?;
        drop(file);

        // Now if we're here, we can be certain that the
        // file exists, now open it as a sqlite database.
        let connection = sqlite::open(&path)?;

        // @TODO - In here, we should check to make sure that
        // the file we're opening is in the correct db version.
        // If not, have some way of updating it to the current
        // spec.

        let file_instance = Self {
            filename: path.clone(),
            sqlite_connection: connection,
        };
        let mut file_lock = match EZPR_FILE_INSTANCE.lock() {
            Ok(l) => l,
            Err(e) => return Err(anyhow::anyhow!("{}", e)),
        };

        *file_lock = Some(file_instance);

        Ok(())
    }

    fn new_file() -> anyhow::Result<()> {
        let tmp = "tmp.ezpr";

        let connection = sqlite::open(tmp)?;

        // @TODO - In here, we should check to make sure that
        // the file we're opening is in the correct db version.
        // If not, have some way of updating it to the current
        // spec.

        let file_instance = Self {
            filename: tmp.to_string(),
            sqlite_connection: connection,
        };
        let mut file_lock = match EZPR_FILE_INSTANCE.lock() {
            Ok(l) => l,
            Err(e) => return Err(anyhow::anyhow!("{}", e)),
        };

        *file_lock = Some(file_instance);

        Ok(())
    }

    fn save_file<P>(file_path: P) -> anyhow::Result<()>
    where
        P: ToString,
    {
        todo!()
    }
}
