use crate::models::{Node, Program};
use anyhow::bail;
use plonky::field_gl::Fr;
use plonky::to_hex;
use sqlx::{any::AnyPoolOptions, Any, Pool};
use std::env;
use utils::{
    errors::{EigenError, Result},
    scalar::{h4_to_string, normalize_to_n_format, prepend_zeros},
};

const DEFAULT_ROOT_KEY: &str = "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";

pub struct Database {
    /// The database connection pool
    pool: Pool<Any>,
    /// The database URL
    url: String,
    /// The root key
    pub key: String,
}

impl Database {
    pub async fn new(url: Option<String>) -> Self {
        let url = url.unwrap_or(env::var("DATABASE_URL").expect("DATABASE_URL must be set"));
        let key = env::var("ROOT_KEY").unwrap_or(DEFAULT_ROOT_KEY.to_string());

        let pool = AnyPoolOptions::new()
            .connect(&url)
            .await
            .expect("Could not build connection pool");

        // Acquire a connection
        pool.acquire().await.expect("Could not acquire connection");

        Database { url, pool, key }
    }

    pub async fn read_program(&mut self, key: &str) -> Result<String> {
        log::debug!("read nodes: {}", key);
        // let key: Vec<u8> = key.into();
        // let result = program
        //     .find(key)
        //     .select(Program::as_select())
        //     .first(&mut self.pool.get().unwrap())
        //     .optional();
        // match result {
        //     Ok(Some(pg)) => Ok(String::from_utf8_lossy(&pg.data).to_string()),
        //     Ok(None) => bail!(EigenError::DatabaseError(diesel::NotFound)),
        //     Err(e) => bail!(EigenError::DatabaseError(e)),
        // }

        let key: Vec<u8> = key.into();
        let result = sqlx::query_as::<_, Program>("SELECT * FROM programs WHERE hash = ?")
            .bind(key)
            .fetch_optional(&self.pool)
            .await
            .map_err(EigenError::DatabaseError)?;
        match result {
            Some(pg) => Ok(String::from_utf8_lossy(&pg.data).to_string()),
            None => bail!(EigenError::DatabaseError(sqlx::Error::RowNotFound)),
        }
    }

    pub async fn read_nodes(&mut self, key: &str) -> Result<String> {
        log::debug!("read nodes: {}", key);
        // let key: Vec<u8> = key.into();
        // let result = nodes
        //     .find(key)
        //     .select(Nodes::as_select())
        //     .first(&mut self.pool.get().unwrap())
        //     .optional();
        // match result {
        //     Ok(Some(pg)) => Ok(String::from_utf8_lossy(&pg.data).to_string()),
        //     Ok(None) => bail!(EigenError::DatabaseError(diesel::NotFound)),
        //     Err(e) => bail!(EigenError::DatabaseError(e)),
        // }
        let key: Vec<u8> = key.into();
        let result = sqlx::query_as::<_, Node>("SELECT * FROM nodes WHERE hash = ?")
            .bind(key)
            .fetch_optional(&self.pool)
            .await
            .map_err(EigenError::DatabaseError)?;
        match result {
            Some(node) => Ok(String::from_utf8_lossy(&node.data).to_string()),
            None => bail!(EigenError::DatabaseError(sqlx::Error::RowNotFound)),
        }
    }

    pub async fn read_remote(&mut self, is_program: bool, key: &str) -> Result<String> {
        // match is_program {
        //     true => self.read_program(key).await,
        //     _ => self.read_nodes(key).await,
        // }
        if is_program {
            self.read_program(key).await
        } else {
            self.read_nodes(key).await
        }
    }

    pub async fn write_program(
        &mut self,
        key: &str,
        value: &Vec<u8>,
        update: bool,
    ) -> Result<usize> {
        // let new_pro = Program {
        //     hash: key.to_string().into(),
        //     data: value.clone(),
        // };
        // log::debug!("write program: {}=>{:?}", key, value);
        // let res = match update {
        //     true => diesel::insert_into(program)
        //         .values(&new_pro)
        //         .on_conflict(crate::schema::state::program::hash)
        //         .do_update()
        //         .set(&new_pro)
        //         .execute(&mut self.pool.get().unwrap())?,
        //     _ => diesel::insert_into(program)
        //         .values(&new_pro)
        //         .on_conflict_do_nothing()
        //         .execute(&mut self.pool.get().unwrap())?,
        // };
        // Ok(res)
        let new_pro = Program {
            hash: key.to_string().into(),
            data: value.clone(),
        };

        log::debug!("write program: {}=>{:?}", key, value);
        let result = match update {
            true => sqlx::query(
                "INSERT INTO programs (hash, data) VALUES ($1, $2) ON CONFLICT (hash) DO UPDATE SET data = $2",
            )
            .bind(&new_pro.hash)
            .bind(&new_pro.data)
            .execute(&self.pool)
            .await
            .map_err(EigenError::DatabaseError)?,
            _ => sqlx::query(
                "INSERT INTO programs (hash, data) VALUES ($1, $2) ON CONFLICT (hash) DO NOTHING",
            )
            .bind(&new_pro.hash)
            .bind(&new_pro.data)
            .execute(&self.pool)
            .await
            .map_err(EigenError::DatabaseError)?,
        };

        Ok(result.rows_affected() as usize)
    }

    pub async fn write_nodes(&mut self, key: &str, value: &str, update: bool) -> Result<usize> {
        log::debug!("write node: {}=>{}", key, value);
        let new_pro = Node {
            hash: key.to_string().into(),
            data: value.to_string().into(),
        };
        // let res = match update {
        //     true => diesel::insert_into(nodes)
        //         .values(&new_pro)
        //         .on_conflict(crate::schema::state::nodes::hash)
        //         .do_update()
        //         .set(&new_pro)
        //         .execute(&mut self.pool.get().unwrap())?,
        //     _ => diesel::insert_into(nodes)
        //         .values(&new_pro)
        //         .on_conflict_do_nothing()
        //         .execute(&mut self.pool.get().unwrap())?,
        // };
        // Ok(res)

        let result = match update {
            true => sqlx::query(
                "INSERT INTO nodes (hash, data) VALUES ($1, $2) ON CONFLICT (hash) DO UPDATE SET data = $2",
            )
            .bind(&new_pro.hash)
            .bind(&new_pro.data)
            .execute(&self.pool)
            .await
            .map_err(EigenError::DatabaseError)?,
            _ => sqlx::query(
                "INSERT INTO nodes (hash, data) VALUES ($1, $2) ON CONFLICT (hash) DO NOTHING",
            )
            .bind(&new_pro.hash)
            .bind(&new_pro.data)
            .execute(&self.pool)
            .await
            .map_err(EigenError::DatabaseError)?,
        };

        Ok(result.rows_affected() as usize)
    }

    pub async fn write_remote(
        &mut self,
        is_program: bool,
        key: &str,
        value: &str,
        update: bool,
    ) -> Result<usize> {
        // match is_program {
        //     true => {
        //         self.write_program(key, &value.to_string().into(), update)
        //             .await
        //     }
        //     _ => self.write_nodes(key, value, update).await,
        // }
        if is_program {
            self.write_program(key, &value.to_string().into(), update)
                .await
        } else {
            self.write_nodes(key, value, update).await
        }
    }

    pub async fn write(&mut self, key: &str, value: &Vec<Fr>, update: bool) -> Result<usize> {
        let key = normalize_to_n_format(key, 64).to_lowercase();
        let mut value_str = String::from("");
        for v in value {
            value_str.push_str(&prepend_zeros(&to_hex(v), 16));
        }
        log::debug!("write: {} => {}", key, value_str);
        self.write_remote(false, &key, &value_str, update).await
    }

    pub async fn read(&mut self, key: &[Fr; 4]) -> Result<Vec<Fr>> {
        let key = h4_to_string(key);
        let key = normalize_to_n_format(&key, 64).to_lowercase();
        let s_data = self.read_remote(false, &key).await?;
        log::debug!("read: {} => {}", key, s_data);

        assert_eq!(s_data.len() % 16, 0);
        let mut res = vec![];
        for i in (0..s_data.len()).step_by(16) {
            let aux = u64::from_str_radix(&s_data[i..(i + 16)], 16).unwrap();
            res.push(Fr::from(aux));
        }

        Ok(res)
    }

    pub async fn set_program(&mut self, key: &str, data: &Vec<u8>, update: bool) -> Result<usize> {
        let key = normalize_to_n_format(key, 64).to_lowercase();
        self.write_remote(true, &key, &hex::encode(data), update)
            .await
    }

    pub async fn get_program(&mut self, key: &str) -> Result<Vec<u8>> {
        let key = normalize_to_n_format(key, 64).to_lowercase();
        let hex_data = self.read_remote(true, &key).await?;
        Ok(hex::decode(hex_data)?)
    }
}

impl std::fmt::Debug for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Database {{ url: {} }}", self.url)
    }
}
