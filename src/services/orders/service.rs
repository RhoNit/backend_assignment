use tonic::{Request, Response, Status};

use crate::models;
use crate::orders::v1::{
    order_service_server::OrderService, Empty, GetOrderRequest, GetOrderResponse,
    GetPaymentRequest, GetPaymentResponse, ListOrderResponse, GetOrderWithAllPaymentsResponse
};

use super::utils::{ get_payment_response, get_all_payments_response_for_an_order };

#[derive(Default)]
pub struct OrderServer {}

#[tonic::async_trait]
impl OrderService for OrderServer {
    async fn get_order(
        &self,
        request: Request<GetOrderRequest>,
    ) -> Result<Response<GetOrderResponse>, Status> {
        println!("Got a request: {:#?}", &request);

        let GetOrderRequest { order_id } = request.into_inner();

        let order = models::Order::find(order_id)?;

        Ok(Response::new(GetOrderResponse::from(order)))
    }
    async fn list_orders(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<ListOrderResponse>, Status> {
        println!("Got a request: {:#?}", &request);

        let orders = models::Order::all()?;

        Ok(Response::new(ListOrderResponse {
            orders: orders
                .into_iter()
                .map(|order| GetOrderResponse::from(order))
                .collect(),
        }))
    }
    async fn get_payment(
        &self,
        request: Request<GetPaymentRequest>,
    ) -> Result<Response<GetPaymentResponse>, Status> {
        println!("Got a request: {:#?}", &request);

        let GetPaymentRequest { payment_id } = request.into_inner();

        let payment = models::Payment::find(payment_id)?;
        let order = models::Order::find(payment.order_id)?;

        Ok(Response::new(get_payment_response(payment, order)))
    }

    // handler function to get an order along with all its payments
    async fn get_order_with_all_payments(
        &self,
        request: Request<GetOrderRequest>
    ) -> Result<Response<GetOrderWithAllPaymentsResponse>, Status> {
        println!("Got a request: {:#?}", &request);

        let order_id = request.into_inner().order_id;
        let order = models::Order::find(order_id)?;

        let payments_resp = get_all_payments_response_for_an_order(order_id);
        
        Ok(Response::new(GetOrderWithAllPaymentsResponse {
            order: Some(GetOrderResponse::from(order)),
            payments_list: payments_resp
        }))
    }
}
