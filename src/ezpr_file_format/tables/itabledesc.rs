//! Table interface for end user
//!
//! Any tables being put into the EZPR File
//! Format should be defined with the
//! `sql_table_description` macro, so that
//! they have corresponding table information
//! and a TableRow struct for getting queries.

#[macro_export]
macro_rules! sql_table_description {
    (
        TABLE $i:ident COLUMNS
        $($tt:tt)*
    ) => {
        sql_table_description!(
            @inside STRUCT $i -> $($tt)*
        );
    };

    // Just after matching first arm - start getting parameters
    (
        @inside STRUCT $i:ident -> $colname:ident: $type:ty => { $sql_type:literal }
        $($tt:tt)*
    ) => {
        sql_table_description!(
            @repetition STRUCT $i { $colname }
            { $type }
            { $sql_type }
            -> $($tt)*
        );
    };

    (
        @repetition STRUCT $i:ident { $($colname_list:ident),* }
        { $($type_list:ty),* }
        { $($sql_params:literal),* }
        -> $colname:ident: $type:ty => { $sql_type:literal }
        $($tt:tt)*
    ) => {
        sql_table_description!(
            @repetition STRUCT $i { $($colname_list),* , $colname }
            { $($type_list),* , $type }
            { $($sql_params),* , $sql_type }
            -> $($tt)*
        );
    };

    (
        @repetition STRUCT $i:ident { $($colname_list:ident),* }
        { $($type_list:ty),* }
        { $($sql_params:literal),* }
        ->
    ) => {
        // End case - do funny list
        paste::paste! {
            pub struct [<$i Table>] {}

            impl [<$i Table>] {
                $(
                    pub fn [<get_ $colname_list _name>]() -> &'static str {
                        std::stringify!($colname_list)
                    }
                )*

                pub fn get_table_name() -> &'static str {
                    std::stringify!($i)
                }

                pub fn get_create_table_command() -> String {
                    let mut s = format!("CREATE TABLE {} (",
                        std::stringify!($i),
                    );
                    let mut ctr = 0;
                    $(
                        s = format!("{}{} {}",
                            s,
                            if ctr == 0 { "" } else {","},
                            std::concat!(std::stringify!($colname_list), " ", $sql_params),
                        );
                        ctr += 1;
                    )*

                    s = s + ");";
                    s
                }
            }
        
            pub struct [<$i TableRow>] {
                $(
                    $colname_list : $type_list,
                )*
            }

            impl [<$i TableRow>] {
                $(
                    pub fn [<get_ $colname_list>](&self) -> & $type_list {
                        &self.$colname_list
                    }
                )*
            }

            impl TryFrom<sqlite::Row> for [<$i TableRow>] {
                type Error = sqlite::Error;

                fn try_from(value: sqlite::Row) -> Result<Self, Self::Error> {
                    $(
                        crate::try_from_line!(value, $colname_list, $type_list);
                    )*

                    Ok( Self {
                        $($colname_list),*
                    })
                }
            }
        }
    };
}

#[macro_export]
macro_rules! try_from_line {
    ($value_varname:ident, $colname:ident, String) => {
        let $colname = $value_varname.try_read::<&str, _>(std::stringify!($colname))?.to_string();
    };

    ($value_varname:ident, $colname:ident, $type:ty) => {
        let $colname = $value_varname.try_read::<$type, _>(std::stringify!($colname))?;
    };
}
