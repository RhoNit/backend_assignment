# Project Setup and building
## 1. Manually (cloning and building)
### i. Clone the repo
```bash
git clone --depth 1 -b master https://github.com/RhoNit/backend_assignment.git
```

### ii. Change directory to `backend_assignment`
```bash
cd backend_assignment
```

### iii. Install `diesel_cli` and install all the crates defined in `Cargo.toml`
```cargo
cargo install diesel_cli --no-default-features --features postgres 
```
N.B. Make sure you have `libpq`, `libssl` etc development packages installed beforehand
```cargo
cargo build
```

### iv. Connect your DB server and then set up diesel
```
diesel setup
```
After settip up diesel, you can check your DB server. Database would be created and all the migrations would get executed.

### v. Run the application
```
cargo run
```

<hr>

# OR

## 2. Using `docker-compose` and `Dockerfile`
```
docker-compose up --build
```
docker-compose first would look for the `Dockerfile` in the current context and would build an image from it. Then two containers would get created for following services `db` and `backend`

<hr>
<br>
<br>

## Thematic Diagram of request/response associated with GetOrderWithAllPayments rpc

![diagram](screenshots/thematic-diagram.png)

<br>

## Payments Table Data
![payments-table-data](screenshots/payments_table.png)

<br>

## API Testing
![request/response](screenshots/api_testing.png)

<br>

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