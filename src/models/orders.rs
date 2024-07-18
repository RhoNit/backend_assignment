use diesel::prelude::*;
use diesel::RunQueryDsl;

//use crate::schema::encumbrance_certificates_searchlog;
use crate::db;
use crate::error_handler::LandeedError;
use crate::schema::orders::{self, dsl, table};

#[derive(Queryable, Debug)]
pub struct Order {
    pub id: i32,
    pub customer_id: i32,
    pub created_at: i32,
    pub status: String,
    pub premium: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = orders)]
pub struct NewOrder {
    pub customer_id: i32,
    pub created_at: i32,
    pub status: String,
    pub premium: bool,
}

impl Order {
    pub fn create(order: NewOrder) -> Result<Self, LandeedError> {
        let mut conn = db::connection()?;
        let order = diesel::insert_into(table)
            .values(&order)
            .get_result(&mut conn)?;
        Ok(order)
    }
    pub fn find(id: i32) -> Result<Self, LandeedError> {
        let mut conn = db::connection()?;
        let order = dsl::orders.filter(dsl::id.eq(id)).first(&mut conn)?;
        Ok(order)
    }
    pub fn all() -> Result<Vec<Self>, LandeedError> {
        let mut conn = db::connection()?;
        let orders = dsl::orders.order(dsl::id.desc()).load(&mut conn)?;
        Ok(orders)
    }
}
