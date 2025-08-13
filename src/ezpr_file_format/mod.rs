mod tables;

use anyhow::anyhow;
use sqlite::Connection;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref EZPR_FILE_INSTANCE: Mutex<Option<EZPRFile>> = Mutex::new(None);
}

pub trait IEZPRFile {
    /// Creates a new file instance. Overwrites
    /// previously opened file, so before calling this function,
    /// code should check if the end-user is okay with overwriting
    /// the previously opened file.
    fn new_file<P>(file_path: P) -> anyhow::Result<()>
    where
        P: ToString;

    /// Saves the currently opened file to disk. If a file is
    /// not open, simply return Ok(). Any errors in the process
    /// are returned as Err().
    fn save_file<P>(file_path: P) -> anyhow::Result<()>
    where
        P: ToString,
    {
        Err(anyhow!("Saving manually is not implemented."))
    }

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
    /// be saved.
    filename: String,

    /// A connection to a SQLite DB instance, which
    /// should be opened to the location on disk
    /// correlating to `filename`.
    sqlite_connection: Connection,
}

impl EZPRFile {
    fn set_up_sqlite_tables(ezpr_file: &Self) -> anyhow::Result<()> {
        let conn = &ezpr_file.sqlite_connection;

        // Ensure DB is completely cleared
        conn.execute(
            "
            PRAGMA writable_schema = 1;
            DELETE FROM sqlite_master;
            PRAGMA writable_schema = 0;
            VACUUM;
            PRAGMA integrity_check;
        ",
        )?;

        // Create player table
        conn.execute(tables::PLAYERS_TABLE_CREATE_STATEMENT)?;

        // // Below is an example of how to do a query
        // // (for my future reference)

        // for i in 0..50 {
        //     let mut query = conn.prepare(format!(
        //         "INSERT INTO {} (playerTag) VALUES (:name);",
        //         tables::PLAYERS_TABLE_NAME
        //     ))?;
        //     query.bind::<(&'static str, sqlite::Value)>((":name", i.to_string().into()))?;
        //     'sql_execute: while let Ok(status) = query.next() {
        //         if status == sqlite::State::Done {
        //             break 'sql_execute;
        //         }
        //     }
        //     println!("{}", i);
        // }

        // let players_query =
        //     conn.prepare(format!("SELECT * FROM {};", tables::PLAYERS_TABLE_NAME))?;

        // use tables::PlayerTableRow;

        // for row in players_query
        //     .into_iter()
        //     .map(|row| PlayerTableRow::try_from(row.unwrap()).unwrap())
        // {
        //     println!(
        //         "Player found in DB: ID = {}, Name = {}",
        //         row.get_player_id(),
        //         row.get_player_tag()
        //     );
        // }

        Ok(())
    }
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

    fn new_file<P>(file_path: P) -> anyhow::Result<()>
    where
        P: ToString,
    {
        let path = file_path.to_string();
        let connection = sqlite::open(&path)?;

        // @TODO - In here, we should check to make sure that
        // the file we're opening is in the correct db version.
        // If not, have some way of updating it to the current
        // spec.

        let file_instance = Self {
            filename: path.clone(),
            sqlite_connection: connection,
        };

        {
            // Separate context for the future ownership
            // of the sqlite connection.
            EZPRFile::set_up_sqlite_tables(&file_instance)?;
        }

        let mut file_lock = match EZPR_FILE_INSTANCE.lock() {
            Ok(l) => l,
            Err(e) => return Err(anyhow::anyhow!("{}", e)),
        };

        *file_lock = Some(file_instance);

        Ok(())
    }
}
