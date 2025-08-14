use crate::sql_table_description;

sql_table_description! {
    TABLE Player COLUMNS
        player_id: i64 => { "INTEGER PRIMARY KEY AUTOINCREMENT" }
        player_tag: String => { "VARCHAR(30)" }
}

// pub const PLAYERS_TABLE_NAME: &str = "players";
// pub const PLAYERS_TABLE_CREATE_STATEMENT: &str =
//     "
//     CREATE TABLE players
//         (
//             playerID INTEGER PRIMARY KEY AUTOINCREMENT,
//             playerTag VARCHAR(30)
//         );
// ";

// pub struct PlayerTableRow {
//     /// The primary key corresponding to
//     /// this player
//     player_id: i64,

//     /// The tag (primary name) attached to
//     /// this player. Does not have to be
//     /// unique.
//     player_tag: String,
// }

// impl Default for PlayerTableRow {
//     fn default() -> Self {
//         Self {
//             player_id: 0,
//             player_tag: "".to_string(),
//         }
//     }
// }

// impl PlayerTableRow {
//     /// Returns the SQL Primary Key ID
//     /// number attached to the queried
//     /// player row.
//     pub fn get_player_id(&self) -> i64 {
//         self.player_id
//     }

//     /// Returns the tag (primary name) of
//     /// the queried player
//     pub fn get_player_tag(&self) -> &String {
//         &self.player_tag
//     }
// }

// impl TryFrom<sqlite::Row> for PlayerTableRow {
//     type Error = sqlite::Error;

//     fn try_from(value: sqlite::Row) -> Result<Self, Self::Error> {
//         let primary_key = value.try_read::<i64, _>("playerID")?;
//         let tag = value.try_read::<&str, _>("playerTag")?;

//         Ok(Self {
//             player_id: primary_key,
//             player_tag: tag.to_string(),
//         })
//     }
// }
