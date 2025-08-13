pub const PLAYERS_TABLE_NAME: &str = "players";
pub const PLAYERS_TABLE_CREATE_STATEMENT: &str = "
    CREATE TABLE players
        (playerID INTEGER PRIMARY KEY AUTOINCREMENT);
";

pub struct PlayerTableRow {
    pub playerID: i64,
}

impl TryFrom<sqlite::Row> for PlayerTableRow {
    type Error = sqlite::Error;

    fn try_from(value: sqlite::Row) -> Result<Self, Self::Error> {
        let primary_key = value.try_read::<i64, _>("playerID")?;

        Ok(Self {
            playerID: primary_key,
        })
    }
}
