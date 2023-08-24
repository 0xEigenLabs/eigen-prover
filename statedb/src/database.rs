use crate::database::nodes::dsl::nodes;
use crate::database::program::dsl::program;
use crate::database_model::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use plonky::field_gl::Fr;
use plonky::to_hex;
use std::env;
use utils::scalar::{normalize_to_n_format, prepend_zeros, byte2char};

pub struct DatabaseConnection {
    connection: PgConnection,
    in_use: bool,
}

impl DatabaseConnection {
    pub fn new() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn = PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        DatabaseConnection {
            connection: conn,
            in_use: true,
        }
    }

    fn read_program(&mut self, key: &String) -> Result<String, diesel::result::Error> {
        let result = program
            .find(key)
            .select(Program::as_select())
            .first(&mut self.connection)
            .optional();
        match result {
            Ok(Some(pg)) => Ok(pg.data),
            Ok(None) => Err(diesel::NotFound),
            Err(e) => Err(e),
        }
    }

    fn read_nodes(&mut self, key: &String) -> Result<String, diesel::result::Error> {
        let result = nodes
            .find(key)
            .select(Nodes::as_select())
            .first(&mut self.connection)
            .optional();
        match result {
            Ok(Some(pg)) => Ok(pg.data),
            Ok(None) => Err(diesel::NotFound),
            Err(e) => Err(e),
        }
    }

    pub fn read_remote(
        &mut self,
        is_program: bool,
        key: &String,
    ) -> Result<String, diesel::result::Error> {
        match is_program {
            true => self.read_program(key),
            _ => self.read_nodes(key),
        }
    }

    fn write_program(
        &mut self,
        key: String,
        value: String,
        update: bool,
    ) -> Result<usize, diesel::result::Error> {
        let new_pro = Program {
            hash: key,
            data: value,
        };
        let res = match update {
            true => diesel::insert_into(program)
                .values(&new_pro)
                .on_conflict(crate::database_model::program::hash)
                .do_update()
                .set(&new_pro)
                .execute(&mut self.connection)?,
            _ => diesel::insert_into(program)
                .values(&new_pro)
                .on_conflict_do_nothing()
                .execute(&mut self.connection)?,
        };
        Ok(res)
    }

    fn write_nodes(
        &mut self,
        key: String,
        value: String,
        update: bool,
    ) -> Result<usize, diesel::result::Error> {
        let new_pro = Nodes {
            hash: key,
            data: value,
        };
        let res = match update {
            true => diesel::insert_into(nodes)
                .values(&new_pro)
                .on_conflict(crate::database_model::nodes::hash)
                .do_update()
                .set(&new_pro)
                .execute(&mut self.connection)?,
            _ => diesel::insert_into(nodes)
                .values(&new_pro)
                .on_conflict_do_nothing()
                .execute(&mut self.connection)?,
        };
        Ok(res)
    }

    pub fn write_remote(
        &mut self,
        is_program: bool,
        key: String,
        value: String,
        update: bool,
    ) -> Result<usize, diesel::result::Error> {
        match is_program {
            true => self.write_program(key, value, update),
            _ => self.write_nodes(key, value, update),
        }
    }

    pub fn write(&mut self, key: &String, value: &Vec<Fr>, update: bool) {
        let key = normalize_to_n_format(key, 64).to_lowercase();
        let mut value_str = String::from("");
        for v in value {
            value_str.push_str(&prepend_zeros(&to_hex(v), 16));
        }
        self.write_remote(false, key, value_str, update);
    }

    pub fn read_tree_remote(key: &String, keys: Vec<u64>, level: u64) -> Result<u64, diesel::result::Error> {
        let mut rkey = String::from("");
        for i in (level as usize)..keys.len() {
            let aux: u8 = keys[i] & 0xFF;
            if aux > 1 {
                log::info!("Database::read_tree_remote(), found invalid keys value = {} at position {}", aux, i);
                return Err(diesel::result::QueryResult);
            }
            rkey.push(byte2char(aux));
        }


        return Ok(0)
    }
}
