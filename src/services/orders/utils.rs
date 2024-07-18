use crate::models;

use crate::orders::v1::{
    GetOrderResponse, 
    GetPaymentResponse, 
    GetOnlyPaymentDetails
};

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

// utility function to construct a reponse type of GetOnlyPaymentDetails
pub fn get_all_payments_response_for_an_order(order_id: i32) -> Vec<GetOnlyPaymentDetails> {
    // fetch all payments for a particular order (by order_id)
    let payments = models::Payment::find_by_order_id(order_id);

    // construct reponse as per GetOnlyPaymentDetails structure
    let payment_details_reponse = 
        payments.unwrap()
            .into_iter()
            .map(|payment| GetOnlyPaymentDetails {
                payment_id: payment.id,
                created_at: payment.created_at,
                status: payment.status
            })
            .collect();
    
    payment_details_reponse
}
