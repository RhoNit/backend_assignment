use crate::models;

use crate::orders::v1::{GetOrderResponse, GetPaymentResponse};

impl From<models::Order> for GetOrderResponse {
    fn from(order: models::Order) -> GetOrderResponse {
        GetOrderResponse {
            order_id: order.id,
            created_at: order.created_at,
            status: order.status,
            premium: order.premium,
        }
    }
}

pub fn get_payment_response(payment: models::Payment, order: models::Order) -> GetPaymentResponse {
    GetPaymentResponse {
        payment_id: payment.id,
        order_id: payment.order_id,
        created_at: payment.created_at,
        status: payment.status,
        order: Some(GetOrderResponse::from(order)),
    }
}
