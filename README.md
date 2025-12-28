# dogs_management_api
Axum api for management of dogs


# **🐕 cURL Commands to Test Your Dog API**

Here are `curl` commands to test all your endpoints:

## **🌐 Root endpoint (GET /)**
```bash
curl -X GET http://localhost:8000/
```
**Expected response:** `"Welcome to Dog Management API!"`

## **📋 1. LIST all dogs (GET /dogs)**
```bash
curl -X GET http://localhost:8000/dogs \
  -H "Accept: application/json"
```
**Expected response:** `[]` (empty initially) or array of dogs.

## **➕ 2. CREATE a dog (POST /dogs)**
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

## **🔍 3. GET a dog by ID (GET /dogs/{id})**
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

## **✏️ 4. UPDATE a dog (PUT /dogs/{id})**
```bash
# Update dog with ID 1
curl -X PUT http://localhost:8000/dogs/1 \
  -H "Content-Type: application/json" \
  -d '{"name":"Fido Updated","age":4}'

# Try to update non-existent dog
curl -X PUT http://localhost:8000/dogs/999 \
  -H "Content-Type: application/json" \
  -d '{"name":"Ghost","age":10}'
```

**Successful response (200 OK):**
```json
{"id":1,"name":"Fido Updated","age":4}
```

## **🗑️ 5. DELETE a dog (DELETE /dogs/{id})**
```bash
# Delete dog with ID 2
curl -X DELETE http://localhost:8000/dogs/2

# Try to delete non-existent dog
curl -X DELETE http://localhost:8000/dogs/999
```

**Responses:**
- **204 No Content:** If deleted successfully (no body)
- **404 Not Found:** If ID doesn't exist
