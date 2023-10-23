use crate::models::{Nodes, Program};
use crate::schema::state::nodes::dsl::nodes;
use crate::schema::state::program::dsl::program;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use plonky::field_gl::Fr;
use plonky::to_hex;
use std::env;
use utils::{
    errors::{EigenError, Result},
    scalar::{byte2string, h4_to_string, normalize_to_n_format, prepend_zeros, string2ba},
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
        let result = program
            .find(key)
            .select(Program::as_select())
            .first(&mut self.pool.get().unwrap())
            .optional();
        match result {
            Ok(Some(pg)) => Ok(pg.data),
            Ok(None) => Err(EigenError::DatabaseError(diesel::NotFound)),
            Err(e) => Err(EigenError::DatabaseError(e)),
        }
    }

    pub fn read_nodes(&mut self, key: &str) -> Result<String> {
        let result = nodes
            .find(key)
            .select(Nodes::as_select())
            .first(&mut self.pool.get().unwrap())
            .optional();
        match result {
            Ok(Some(pg)) => Ok(pg.data),
            Ok(None) => Err(EigenError::DatabaseError(diesel::NotFound)),
            Err(e) => Err(EigenError::DatabaseError(e)),
        }
    }

    pub fn read_remote(&mut self, is_program: bool, key: &str) -> Result<String> {
        match is_program {
            true => self.read_program(key),
            _ => self.read_nodes(key),
        }
    }

    pub fn write_program(&mut self, key: &str, value: &str, update: bool) -> Result<usize> {
        let new_pro = Program {
            hash: key.to_string(),
            data: value.to_string(),
        };
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
        let new_pro = Nodes {
            hash: key.to_string(),
            data: value.to_string(),
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

    pub fn write_remote(
        &mut self,
        is_program: bool,
        key: &str,
        value: &str,
        update: bool,
    ) -> Result<usize> {
        match is_program {
            true => self.write_program(key, value, update),
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

    /*
    pub fn write_get_tree_function(&mut self) {
        let db_nodes_table_name = "state.nodes";

        let query = format!(r#"
    "create or replace function get_tree (root_hash bytea, remaining_key bytea)\n" +
    "   returns setof state.nodes\n" +
    "   language plpgsql\n" +
    "as $$\n" +
    "declare\n" +
    "	current_hash bytea;\n" +
    "	current_row {}%rowtype;\n" +
    "	remaining_key_length integer;\n" +
    "	remaining_key_bit integer;\n" +
    "	byte_71 integer;\n" +
    "	aux_integer integer;\n" +
    "begin\n" +
    "	remaining_key_length = octet_length(remaining_key);\n" +
    "	current_hash = root_hash;\n" +

    "	-- For every bit (0 or 1) in remaining key\n" +
    "	for counter in 0..(remaining_key_length-1) loop\n" +

    "		-- Get the current_hash row and store it into current_row\n" +
    "		select * into current_row from {} where hash = current_hash;\n" +
    "		if not found then\n" +
    "			raise EXCEPTION 'Hash % not found', current_hash;\n" +
    "		end if;\n" +

    "		-- Return it as a result\n" +
    "		return next current_row;\n" +

    "		-- Data should be a byte array of 12x8 bytes (12 field elements)\n" +
    "		-- Check data length is exactly 12 field elements\n" +
    "		if (octet_length(current_row.data) != 12*8) then\n" +
    "			raise EXCEPTION 'Hash % got invalid data size %', current_hash, octet_length(current_row.data);\n" +
    "		end if;\n" +
    //	-- Check that last 3 field elements are zero
    //	--if (substring(current_row.data from 89 for 8) != E'\\x0000000000000000') then
    //	--	RAISE EXCEPTION 'Hash % got non-null 12th field element data=%', current_hash, current_row.data;
    //	--end if;
    //	--if (substring(current_row.data from 81 for 8) != E'\\x0000000000000000') then
    //	--	RAISE EXCEPTION 'Hash % got non-null 11th field element data=%', current_hash, current_row.data;
    //	--end if;
    //	--if (substring(current_row.data from 73 for 8) != E'\\x0000000000000000') then
    //	--	RAISE EXCEPTION 'Hash % got non-null 10th field element data=%', current_hash, current_row.data;
    //	--end if;
    "		-- If last 4 field elements are 0000, this is an intermediate node\n" +
    "		byte_71 = get_byte(current_row.data, 71);\n" +
    "		case byte_71\n" +
    "		when 0 then\n" +

    "			-- If the next remaining key is a 0, take the left sibling way, if it is a 1, take the right one\n" +
    "			remaining_key_bit = get_byte(remaining_key, counter);\n" +
    "			case remaining_key_bit\n" +
    "			when 0 then\n" +
    "				current_hash =\n" +
    "					substring(current_row.data from 25 for 8) ||\n" +
    "					substring(current_row.data from 17 for 8) ||\n" +
    "					substring(current_row.data from 9 for 8) ||\n" +
    "					substring(current_row.data from 1 for 8);\n" +
    "			when 1 then\n" +
    "				current_hash =\n" +
    "					substring(current_row.data from 57 for 8) ||\n" +
    "					substring(current_row.data from 49 for 8) ||\n" +
    "					substring(current_row.data from 41 for 8) ||\n" +
    "					substring(current_row.data from 33 for 8);\n" +
    "			else\n" +
    "				raise EXCEPTION 'Invalid remaining key bit at position % with value %', counter, remaining_key_bit ;\n" +
    "			end case;\n" +

    "			-- If the hash is a 0, we reached the end of the branch\n" +
    "			if (get_byte(current_hash, 0) = 0) and\n" +
    "			   (get_byte(current_hash, 1) = 0) and\n" +
    "			   (get_byte(current_hash, 2) = 0) and\n" +
    "			   (get_byte(current_hash, 3) = 0) and\n" +
    "			   (get_byte(current_hash, 4) = 0) and\n" +
    "			   (get_byte(current_hash, 5) = 0) and\n" +
    "			   (get_byte(current_hash, 6) = 0) and\n" +
    "			   (get_byte(current_hash, 7) = 0) and\n" +
    "			   (get_byte(current_hash, 8) = 0) and\n" +
    "			   (get_byte(current_hash, 9) = 0) and\n" +
    "			   (get_byte(current_hash, 10) = 0) and\n" +
    "			   (get_byte(current_hash, 11) = 0) and\n" +
    "			   (get_byte(current_hash, 12) = 0) and\n" +
    "			   (get_byte(current_hash, 13) = 0) and\n" +
    "			   (get_byte(current_hash, 14) = 0) and\n" +
    "			   (get_byte(current_hash, 15) = 0) and\n" +
    "			   (get_byte(current_hash, 16) = 0) and\n" +
    "			   (get_byte(current_hash, 17) = 0) and\n" +
    "			   (get_byte(current_hash, 18) = 0) and\n" +
    "			   (get_byte(current_hash, 19) = 0) and\n" +
    "			   (get_byte(current_hash, 20) = 0) and\n" +
    "			   (get_byte(current_hash, 21) = 0) and\n" +
    "			   (get_byte(current_hash, 22) = 0) and\n" +
    "			   (get_byte(current_hash, 23) = 0) and\n" +
    "			   (get_byte(current_hash, 24) = 0) and\n" +
    "			   (get_byte(current_hash, 25) = 0) and\n" +
    "			   (get_byte(current_hash, 26) = 0) and\n" +
    "			   (get_byte(current_hash, 27) = 0) and\n" +
    "			   (get_byte(current_hash, 28) = 0) and\n" +
    "			   (get_byte(current_hash, 29) = 0) and\n" +
    "			   (get_byte(current_hash, 30) = 0) and\n" +
    "			   (get_byte(current_hash, 31) = 0) then\n" +
    "			   return;\n" +
    "			end if;\n" +

    "		-- If last 4 field elements are 1000, this is a leaf node\n" +
    "		when 1 then	\n" +

    "			current_hash =\n" +
    "				substring(current_row.data from 57 for 8) ||\n" +
    "				substring(current_row.data from 49 for 8) ||\n" +
    "				substring(current_row.data from 41 for 8) ||\n" +
    "				substring(current_row.data from 33 for 8);\n" +
    "			select * into current_row from {} where hash = current_hash;\n" +
    "			if not found then\n" +
    "				raise EXCEPTION 'Hash % not found', current_hash;\n" +
    "			end if;\n" +
    "			return next current_row;\n" +
    "			return;\n" +

    "		else\n" +
    "			raise EXCEPTION 'Hash % got invalid 9th field element data=%', current_hash, current_row.data;\n" +
    "		end case;\n" +

    "	end loop;\n" +

    "	return;\n" +
    "end;$$\n"#,db_nodes_table_name, db_nodes_table_name, db_nodes_table_name);

        let result = diesel::sql_query(query).execute(&mut self.pool.get().unwrap());

        log::info!("Database::writeGetTreeFunction() {} returns {:?}", query, result);
        Ok(result)
    }
    */

    pub fn set_program(&mut self, key: &str, data: &Vec<u8>, update: bool) -> Result<usize> {
        let key = normalize_to_n_format(key, 64).to_lowercase();
        let mut s_data = String::from("");
        for d in data {
            s_data.push_str(&byte2string(*d));
        }

        self.write_remote(true, &key, &s_data, update)
    }

    pub fn get_program(&mut self, key: &str) -> Result<Vec<u8>> {
        let key = normalize_to_n_format(key, 64).to_lowercase();
        let s_data = self.read_remote(true, &key)?;
        Ok(string2ba(&s_data))
    }

    /*
    pub fn read_tree_remote(key: &str, keys: Vec<u64>, level: u64) -> Result<u64> {
        let mut rkey = String::from("");
        for i in (level as usize)..keys.len() {
            let aux = (keys[i] & 0xFF) as u8;
            if aux > 1 {
                log::info!(
                    "Database::read_tree_remote(), found invalid keys value = {} at position {}",
                    aux,
                    i
                );
                return Err(EigenError::InvalidValue(
                    "Invalid key when read_tree_remote".to_string(),
                ));
            }
            rkey.push(byte2char(aux >> 4));
            rkey.push(byte2char(aux & 0x0F));
        }

        let mut num_of_fields = 0;

        let query = ""



        return Ok(0);
    }
    */
}
