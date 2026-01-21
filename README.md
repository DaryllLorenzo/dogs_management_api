# Dogs Management API

Axum API for management of dogs with PostgreSQL database.

## Features

- Full CRUD operations for dogs
- PostgreSQL database with connection pooling  
- Database migrations with SQLx
- Environment-based configuration
- Proper HTTP status codes and error handling
- Type-safe database queries
- Three-layer architecture (Router → Service → Repository)
- Modular project structure

## Tech Stack

- **Framework**: Axum 0.8
- **Database**: PostgreSQL with SQLx
- **Async Runtime**: Tokio
- **Serialization**: Serde
- **Configuration**: dotenvy

## Project Structure

```
src/
├── main.rs                # Application entry point
└── dog/                  # Dogs domain module
    ├── mod.rs            # Module exports
    ├── model.rs          # Data models (Dog, DogPayload, DogPatchPayload)
    ├── repository.rs     # Database access layer
    ├── service.rs        # Business logic layer  
    └── router.rs         # HTTP handlers and routing
```

## Architecture

The application follows a clean architecture pattern:

1. **Router Layer** (`router.rs`): HTTP handlers and route definitions
2. **Service Layer** (`service.rs`): Business logic and validation
3. **Repository Layer** (`repository.rs`): Database operations
4. **Model Layer** (`model.rs`): Data structures and DTOs

## Getting Started

### Prerequisites
- Rust 1.75+
- PostgreSQL 14+
- Cargo package manager

### Installation

1. Clone the repository:
```bash
git clone https://github.com/DaryllLorenzo/dogs_management_api.git
cd dogs_management_api
```

2. Set up environment variables:
```bash
cp .env.example .env
# Edit .env with your database credentials
```

3. Run database migrations and start the server:
```bash
cargo run
```

## API Endpoints

### Root endpoint (GET /)
```bash
curl -X GET http://localhost:8000/
```
**Response:** `"Welcome to Dog Management API!"`

### Health check (GET /health)
```bash
curl -X GET http://localhost:8000/health
```
**Response:** `"OK"`

### 1. List all dogs (GET /dogs)
```bash
curl -X GET http://localhost:8000/api/dogs \
  -H "Accept: application/json"
```

Optional pagination parameters:
- `page`: Page number (default: 1)
- `limit`: Items per page (default: 20)

```bash
curl -X GET "http://localhost:8000/api/dogs?page=1&limit=10"
```

### 2. Create a dog (POST /dogs)
```bash
curl -X POST http://localhost:8000/api/dogs \
  -H "Content-Type: application/json" \
  -d '{"name":"Fido","age":3}'
```

**Validation rules:**
- Name cannot be empty
- Age must be between 0 and 30

**Response (201 Created):**
```json
{"id":1,"name":"Fido","age":3}
```

### 3. Get a dog by ID (GET /dogs/{id})
```bash
curl -X GET http://localhost:8000/api/dogs/1 \
  -H "Accept: application/json"
```

**Successful response (200 OK):**
```json
{"id":1,"name":"Fido","age":3}
```

**Error response (404 Not Found):** For non-existent ID

### 4. Full update a dog (PUT /dogs/{id})
```bash
curl -X PUT http://localhost:8000/api/dogs/1 \
  -H "Content-Type: application/json" \
  -d '{"name":"Fido Updated","age":4}'
```

**Successful response (200 OK):**
```json
{"id":1,"name":"Fido Updated","age":4}
```

**Validation:** Same as POST endpoint

### 5. Partial update a dog (PATCH /dogs/{id})
```bash
curl -X PATCH http://localhost:8000/api/dogs/1 \
  -H "Content-Type: application/json" \
  -d '{"name":"Fido Patched"}'
```

**Successful response (200 OK):**
```json
{"id":1,"name":"Fido Patched","age":4}
```

**Note:** Only provided fields are updated

### 6. Delete a dog (DELETE /dogs/{id})
```bash
curl -X DELETE http://localhost:8000/api/dogs/2
```

**Responses:**
- **204 No Content:** If deleted successfully
- **404 Not Found:** If ID doesn't exist

### 7. List all breeds (GET /breeds)
```bash
curl -X GET http://localhost:8000/api/breeds \
  -H "Accept: application/json"
```

### 8. Create a breed (POST /breeds)
```bash
curl -X POST http://localhost:8000/api/breeds \
  -H "Content-Type: application/json" \
  -d '{"name":"Pitbull"}'
```


## Error Handling

The API returns appropriate HTTP status codes:

- **200 OK**: Successful operation
- **201 Created**: Resource created successfully
- **204 No Content**: Resource deleted successfully
- **400 Bad Request**: Validation error (e.g., empty name, invalid age)
- **404 Not Found**: Resource not found
- **500 Internal Server Error**: Server-side error

## Development

### Running Tests
```bash
cargo test
```

### Database Migrations
Migrations are automatically run on application startup via SQLx. To create a new migration:

1. Create SQL file in `migrations/` directory
2. The migration will run automatically when the server starts


## Future Improvements

1. JWT authentication and authorization
2. API documentation with OpenAPI/Swagger
3. Request/response logging middleware
4. Docker containerization
5. Integration tests