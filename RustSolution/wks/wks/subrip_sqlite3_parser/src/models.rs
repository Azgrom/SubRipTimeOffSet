#[derive(Queryable)]
pub struct SubRipRegistry {
    pub id: i32,
    pub filename: String,
    pub content: String,
    pub published: bool,
}

use super::schema::subrip_reg;

#[derive(Insertable)]
#[table_name = "subrip_reg"]
pub struct NewPSubRipRegistry<'a> {
    pub filename: &'a str,
    pub content: &'a str,
}
