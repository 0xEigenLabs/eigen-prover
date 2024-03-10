#[derive(sqlx::FromRow)]
pub struct Node {
    pub hash: Vec<u8>,
    pub data: Vec<u8>,
}

#[derive(sqlx::FromRow)]
pub struct Program {
    pub hash: Vec<u8>,
    pub data: Vec<u8>,
}
