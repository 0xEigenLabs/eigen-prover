use crate::schema::state::{nodes, program};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = nodes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Nodes {
    pub hash: Vec<u8>,
    pub data: Vec<u8>,
}

#[derive(Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = program)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Program {
    pub hash: Vec<u8>,
    pub data: Vec<u8>,
}
