mod itabledesc;

use crate::sql_table_description;

sql_table_description! {
    // The players table - holds all the
    // player data
    TABLE Players COLUMNS

        // The player's unique identifer (exclusive to the database)
        player_id: i64 => { "INTEGER PRIMARY KEY AUTOINCREMENT" }

        // The player's tag (primary name)
        player_tag: String => { "VARCHAR(30)" }
}

sql_table_description! {

    TABLE Tournaments COLUMNS

        tournament_id: i64 => { "INTEGER PRIMARY KEY AUTOINCREMENT" }

        tournament_name: String => { "VARCHAR(100)" }

        num_entrants: i64 => { "INTEGER" }

}

sql_table_description! {

    TABLE Aliases COLUMNS

        orig_player_id: i64 => { "INTEGER" }

        alias: String => { "VARCHAR(30)" }
}

sql_table_description! {

    TABLE Placements COLUMNS

        player_id: i64 => { "INTEGER" }

        tournament_id: i64 => { "INTEGER" }

        placement: i64 => { "INTEGER" }
}
