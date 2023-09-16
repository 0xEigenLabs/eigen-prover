diesel::table! {
    nodes (hash) {
        hash -> Varchar,
        data -> Varchar,
    }
}

diesel::table! {
    program (hash) {
        hash -> Varchar,
        data -> Varchar,
    }
}
