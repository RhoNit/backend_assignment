table! {
    customers (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    orders (id) {
        id -> Int4,
        customer_id -> Int4,
        created_at -> Int4,
        status -> Text,
        premium -> Bool,
    }
}

table! {
    payments (id) {
        id -> Int4,
        order_id -> Int4,
        created_at -> Int4,
        status -> Text,
    }
}

joinable!(orders -> customers (customer_id));
joinable!(payments -> orders (order_id));

allow_tables_to_appear_in_same_query!(
    customers,
    orders,
    payments,
);
