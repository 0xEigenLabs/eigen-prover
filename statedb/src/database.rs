use crate::models::{Nodes, Program};
use crate::schema::state::nodes::dsl::nodes;
use crate::schema::state::program::dsl::program;
use anyhow::bail;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use plonky::field_gl::Fr;
use plonky::to_hex;
use std::env;
use utils::{
    errors::{EigenError, Result},
    scalar::{h4_to_string, normalize_to_n_format, prepend_zeros},
};
pub struct Database {
    pool: Pool<ConnectionManager<PgConnection>>,
    database_url: String,
    _in_use: bool,
    pub db_state_root_key: String,
}

impl Default for Database {
    fn default() -> Self {
        Self::new(None)
    }
}

impl std::fmt::Debug for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Database:\n\turl: {}\n\tin_use: {}",
            self.database_url, self._in_use
        )
    }
}

impl Database {
    pub fn new(url: Option<String>) -> Self {
        let database_url = match url {
            Some(x) => x,
            _ => env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        };
        let manager = ConnectionManager::<PgConnection>::new(database_url.clone());
        Database {
            pool: Pool::builder()
                .test_on_check_out(true)
                .build(manager)
                .expect("Could not build connection pool"),
            database_url,
            _in_use: true,
            db_state_root_key: "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
                .to_string(),
        }
    }

    pub fn read_program(&mut self, key: &str) -> Result<String> {
        log::debug!("read nodes: {}", key);
        let key: Vec<u8> = key.into();
        let result = program
            .find(key)
            .select(Program::as_select())
            .first(&mut self.pool.get().unwrap())
            .optional();
        match result {
            Ok(Some(pg)) => Ok(String::from_utf8_lossy(&pg.data).to_string()),
            Ok(None) => bail!(EigenError::DatabaseError(diesel::NotFound)),
            Err(e) => bail!(EigenError::DatabaseError(e)),
        }
    }

    pub fn read_nodes(&mut self, key: &str) -> Result<String> {
        log::debug!("read nodes: {}", key);
        let key: Vec<u8> = key.into();
        let result = nodes
            .find(key)
            .select(Nodes::as_select())
            .first(&mut self.pool.get().unwrap())
            .optional();
        match result {
            Ok(Some(pg)) => Ok(String::from_utf8_lossy(&pg.data).to_string()),
            Ok(None) => bail!(EigenError::DatabaseError(diesel::NotFound)),
            Err(e) => bail!(EigenError::DatabaseError(e)),
        }
    }

    pub fn read_remote(&mut self, is_program: bool, key: &str) -> Result<String> {
        match is_program {
            true => self.read_program(key),
            _ => self.read_nodes(key),
        }
    }

    pub fn write_program(&mut self, key: &str, value: &Vec<u8>, update: bool) -> Result<usize> {
        let new_pro = Program {
            hash: key.to_string().into(),
            data: value.clone(),
        };
        log::debug!("write program: {}=>{:?}", key, value);
        let res = match update {
            true => diesel::insert_into(program)
                .values(&new_pro)
                .on_conflict(crate::schema::state::program::hash)
                .do_update()
                .set(&new_pro)
                .execute(&mut self.pool.get().unwrap())?,
            _ => diesel::insert_into(program)
                .values(&new_pro)
                .on_conflict_do_nothing()
                .execute(&mut self.pool.get().unwrap())?,
        };
        Ok(res)
    }

    pub fn write_nodes(&mut self, key: &str, value: &str, update: bool) -> Result<usize> {
        log::debug!("write node: {}=>{}", key, value);
        let new_pro = Nodes {
            hash: key.to_string().into(),
            data: value.to_string().into(),
        };
        let res = match update {
            true => diesel::insert_into(nodes)
                .values(&new_pro)
                .on_conflict(crate::schema::state::nodes::hash)
                .do_update()
                .set(&new_pro)
                .execute(&mut self.pool.get().unwrap())?,
            _ => diesel::insert_into(nodes)
                .values(&new_pro)
                .on_conflict_do_nothing()
                .execute(&mut self.pool.get().unwrap())?,
        };
        Ok(res)
    }

    pub async fn write_nodes2(&mut self, key: &str, value: &str, update: bool) -> Result<usize> {
        todo!()
    }

    pub fn write_remote(
        &mut self,
        is_program: bool,
        key: &str,
        value: &str,
        update: bool,
    ) -> Result<usize> {
        match is_program {
            true => self.write_program(key, &value.to_string().into(), update),
            _ => self.write_nodes(key, value, update),
        }
    }

    pub fn write(&mut self, key: &str, value: &Vec<Fr>, update: bool) -> Result<usize> {
        let key = normalize_to_n_format(key, 64).to_lowercase();
        let mut value_str = String::from("");
        for v in value {
            value_str.push_str(&prepend_zeros(&to_hex(v), 16));
        }
        log::debug!("write: {} => {}", key, value_str);
        self.write_remote(false, &key, &value_str, update)
    }

    pub fn read(&mut self, key: &[Fr; 4]) -> Result<Vec<Fr>> {
        let key = h4_to_string(key);
        let key = normalize_to_n_format(&key, 64).to_lowercase();
        let s_data = self.read_remote(false, &key)?;
        log::debug!("read: {} => {}", key, s_data);

        assert_eq!(s_data.len() % 16, 0);
        let mut res = vec![];
        for i in (0..s_data.len()).step_by(16) {
            let aux = u64::from_str_radix(&s_data[i..(i + 16)], 16).unwrap();
            res.push(Fr::from(aux));
        }

        Ok(res)
    }

    pub fn set_program(&mut self, key: &str, data: &Vec<u8>, update: bool) -> Result<usize> {
        let key = normalize_to_n_format(key, 64).to_lowercase();
        self.write_remote(true, &key, &hex::encode(data), update)
    }

    pub fn get_program(&mut self, key: &str) -> Result<Vec<u8>> {
        let key = normalize_to_n_format(key, 64).to_lowercase();
        let hex_data = self.read_remote(true, &key)?;
        Ok(hex::decode(hex_data)?)
    }
}
