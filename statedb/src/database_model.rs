use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::database_model::nodes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Nodes {
    pub hash: String,
    pub data: String,
}

#[derive(Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::database_model::program)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Program {
    pub hash: String,
    pub data: String,
}

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
