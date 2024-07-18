use diesel::prelude::*;
use diesel::RunQueryDsl;

//use crate::schema::encumbrance_certificates_searchlog;
use crate::db;
use crate::error_handler::LandeedError;
use crate::schema::payments::{self, dsl, table};

#[derive(Queryable, Debug)]
pub struct Payment {
    pub id: i32,
    pub order_id: i32,
    pub created_at: i32,
    pub status: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = payments)]
pub struct NewPayment {
    pub order_id: i32,
    pub created_at: i32,
    pub status: String,
}

impl Payment {
    pub fn create(payment: NewPayment) -> Result<Self, LandeedError> {
        let mut conn = db::connection()?;
        let payment = diesel::insert_into(table)
            .values(&payment)
            .get_result(&mut conn)?;
        Ok(payment)
    }
    pub fn find(id: i32) -> Result<Self, LandeedError> {
        let mut conn = db::connection()?;
        let payment = dsl::payments.filter(dsl::id.eq(id)).first(&mut conn)?;
        Ok(payment)
    }
    pub fn all() -> Result<Vec<Self>, LandeedError> {
        let mut conn = db::connection()?;
        let payments = dsl::payments.order(dsl::id.desc()).load(&mut conn)?;
        Ok(payments)
    }

    // a new function to fetch payments by order_id
    pub fn find_by_order_id(order_id: i32) -> Result<Vec<Self>, LandeedError>  {
        let mut conn = db::connection()?;
        
        let payments = 
            dsl::payments
                .filter(dsl::order_id.eq(order_id))
                .load(&mut conn)?;

        Ok(payments)
    }
}
