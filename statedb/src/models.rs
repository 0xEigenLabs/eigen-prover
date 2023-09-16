use crate::schema::{nodes, program};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = nodes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Nodes {
    pub hash: String,
    pub data: String,
}

#[derive(Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = program)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Program {
    pub hash: String,
    pub data: String,
}
