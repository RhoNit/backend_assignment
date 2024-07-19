## Thematic Diagram of request/response associated with GetOrderWithAllPayments rpc

![diagram](screenshots/thematic-diagram.png)

## Payments Table Data
![payments-table-data](screenshots/payments_table.png)

## API Testing
![request/response](screenshots/api_testing.png)

## API Documentation (`GetOrderWithAllPayments`)
### Request
```json
{
	"order_id": 2
}
```

### Response
```json
{
	"payments_list": [
		{
			"payment_id": 1,
			"created_at": 1609491661,
			"status": "Completed"
		},
		{
			"payment_id": 2,
			"created_at": 1672563661,
			"status": "Refunded"
		}
	],
	"order": {
		"order_id": 2,
		"created_at": 1609491661,
		"status": "Completed",
		"premium": true
	}
}
```