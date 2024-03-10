use crate::models::{Node, Program};

use anyhow::bail;
use log::debug;
use plonky::field_gl::Fr;
use plonky::to_hex;
use sqlx::{any::AnyPoolOptions, Any, Pool};
use utils::{
    errors::{EigenError, Result},
    scalar::{h4_to_string, normalize_to_n_format, prepend_zeros},
};

pub const DEFAULT_ROOT_KEY: &str =
    "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";

/// The [Database] is a wrapper around the database connection pool
/// and provides methods to connect or disconnect to the database.
pub struct Database {
    /// The database connection pool
    pool: Pool<Any>,
    /// The database URL
    url: String,
    /// The root key
    pub key: String,
}

impl Database {
    /// Create a new database connection pool
    pub async fn new(url: &str, root_key: &str) -> Self {
        Database {
            url: String::from(url),
            pool: AnyPoolOptions::new()
                .connect(url)
                .await
                .expect("Could not build connection pool"),
            key: String::from(root_key),
        }
    }

    /// Read the program from the database by the given key
    pub async fn read_program(&self, key: &str) -> Result<String> {
        debug!("read program: {}", key);

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

    /// Read the nodes from the database by the given key
    pub async fn read_nodes(&self, key: &str) -> Result<String> {
        log::debug!("read nodes: {}", key);

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

    /// Write the program to the database with the given key and value
    pub async fn write_program(&self, key: &str, value: &Vec<u8>, update: bool) -> Result<usize> {
        log::debug!("write program: {}=>{:?}", key, value);

        let new_pro = Program {
            hash: key.to_string().into(),
            data: value.clone(),
        };

        let result = if update {
            sqlx::query(
                "INSERT INTO programs (hash, data) VALUES ($1, $2) ON CONFLICT (hash) DO UPDATE SET data = $2",
            )
            .bind(&new_pro.hash)
            .bind(&new_pro.data)
            .execute(&self.pool)
            .await
            .map_err(EigenError::DatabaseError)?
        } else {
            sqlx::query(
                "INSERT INTO programs (hash, data) VALUES ($1, $2) ON CONFLICT (hash) DO NOTHING",
            )
            .bind(&new_pro.hash)
            .bind(&new_pro.data)
            .execute(&self.pool)
            .await
            .map_err(EigenError::DatabaseError)?
        };

        Ok(result.rows_affected() as usize)
    }

    /// Write the nodes to the database with the given key and value
    pub async fn write_nodes(&self, key: &str, value: &str, update: bool) -> Result<usize> {
        log::debug!("write node: {}=>{}", key, value);

        let new_pro = Node {
            hash: key.to_string().into(),
            data: value.to_string().into(),
        };

        let result = if update {
            sqlx::query(
                "INSERT INTO nodes (hash, data) VALUES ($1, $2) ON CONFLICT (hash) DO UPDATE SET data = $2",
            )
            .bind(&new_pro.hash)
            .bind(&new_pro.data)
            .execute(&self.pool)
            .await
            .map_err(EigenError::DatabaseError)?
        } else {
            sqlx::query(
                "INSERT INTO nodes (hash, data) VALUES ($1, $2) ON CONFLICT (hash) DO NOTHING",
            )
            .bind(&new_pro.hash)
            .bind(&new_pro.data)
            .execute(&self.pool)
            .await
            .map_err(EigenError::DatabaseError)?
        };

        Ok(result.rows_affected() as usize)
    }

    /// Write the nodes to the database with the given key and value
    pub async fn write(&self, key: &str, value: &Vec<Fr>, update: bool) -> Result<usize> {
        let key = normalize_to_n_format(key, 64).to_lowercase();
        let mut value_str = String::from("");
        for v in value {
            value_str.push_str(&prepend_zeros(&to_hex(v), 16));
        }
        log::debug!("write: {} => {}", key, value_str);
        self.write_nodes(&key, &value_str, update).await
    }

    /// Read the nodes from the database by the given key
    pub async fn read(&self, key: &[Fr; 4]) -> Result<Vec<Fr>> {
        let key = h4_to_string(key);
        let key = normalize_to_n_format(&key, 64).to_lowercase();
        let s_data = self.read_nodes(&key).await?;
        log::debug!("read: {} => {}", key, s_data);

        assert_eq!(s_data.len() % 16, 0);
        let mut res = vec![];
        for i in (0..s_data.len()).step_by(16) {
            let aux = u64::from_str_radix(&s_data[i..(i + 16)], 16).unwrap();
            res.push(Fr::from(aux));
        }

        Ok(res)
    }

    /// Set program with the given key and data
    pub async fn set_program(&self, key: &str, data: &Vec<u8>, update: bool) -> Result<usize> {
        let key = normalize_to_n_format(key, 64).to_lowercase();
        self.write_program(&key, &hex::encode(data).into(), update)
            .await
    }

    /// Get program with the given key
    pub async fn get_program(&self, key: &str) -> Result<Vec<u8>> {
        let key = normalize_to_n_format(key, 64).to_lowercase();
        let hex_data = self.read_program(&key).await?;
        Ok(hex::decode(hex_data)?)
    }
}

impl std::fmt::Debug for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Database {{ url: {} }}", self.url)
    }
}
