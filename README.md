# **🐕 Dogs Management API**

Axum API for management of dogs with PostgreSQL database.

## **🚀 Features**

- ✅ Full CRUD operations for dogs
- ✅ PostgreSQL database with connection pooling  
- ✅ Database migrations with SQLx
- ✅ Environment-based configuration
- ✅ Proper HTTP status codes and error handling
- ✅ Type-safe database queries

## **🔜 Planned Improvements**

1. **Modular project structure**
2. **Better error handling**
3. **JWT authentication**
4. **API documentation** with OpenAPI/Swagger

## **🛠️ Tech Stack**

- **Framework**: Axum 0.8
- **Database**: PostgreSQL with SQLx
- **Async Runtime**: Tokio
- **Serialization**: Serde
- **Configuration**: dotenvy

## **🚀 Getting Started**

### **Prerequisites**
- Rust 1.70+
- PostgreSQL 14+
- Cargo

### **Installation**

1. **Clone the repository:**
```bash
git clone https://github.com/DaryllLorenzo/dogs_management_api.git
cd dogs_management_api
```

2. **Set up environment variables:**
```bash
cp .env.example .env
# Edit .env with your database credentials
```

3. **Run the server (migrations run automatically):**
```bash
cargo run
```

## **📋 API Testing with cURL**

### **🌐 Root endpoint (GET /)**
```bash
curl -X GET http://localhost:8000/
```
**Expected response:** `"Welcome to Dog Management API!"`

### **📋 1. LIST all dogs (GET /dogs)**
```bash
curl -X GET http://localhost:8000/dogs \
  -H "Accept: application/json"
```
**Expected response:** `[]` (empty initially) or array of dogs.

### **➕ 2. CREATE a dog (POST /dogs)**
```bash
# Create a basic dog
curl -X POST http://localhost:8000/dogs \
  -H "Content-Type: application/json" \
  -d '{"name":"Fido","age":3}'

```

**Expected response (201 Created):**
```json
{"id":1,"name":"Fido","age":3}
```

### **🔍 3. GET a dog by ID (GET /dogs/{id})**
```bash
# Get dog with ID 1
curl -X GET http://localhost:8000/dogs/1 \
  -H "Accept: application/json"

# Try to get non-existent dog
curl -X GET http://localhost:8000/dogs/999 \
  -H "Accept: application/json"
```
**Successful response (200 OK):**
```json
{"id":1,"name":"Fido","age":3}
```
**Error (404 Not Found):** For non-existent ID

### **✏️ 4. UPDATE a dog (PUT /dogs/{id})**
```bash
# Update dog with ID 1
curl -X PUT http://localhost:8000/dogs/1 \
  -H "Content-Type: application/json" \
  -d '{"name":"Fido Updated","age":4}'
```

**Successful response (200 OK):**
```json
{"id":1,"name":"Fido Updated","age":4}
```

### **🗑️ 5. DELETE a dog (DELETE /dogs/{id})**
```bash
# Delete dog with ID 2
curl -X DELETE http://localhost:8000/dogs/2

# Try to delete non-existent dog
curl -X DELETE http://localhost:8000/dogs/999
```

**Responses:**
- **204 No Content:** If deleted successfully (no body)
- **404 Not Found:** If ID doesn't exist

