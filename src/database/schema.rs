diesel::table! {
    users (id) {
        id -> Integer,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        role -> Varchar,
        dir_id -> Varchar,
    }
}