# ICAP Server Response Codes with Examples

This document lists **ICAP server response codes** as defined in RFC 3507, along with example responses including headers and optional data payloads.

---

## 📌 1xx – Informational

### 100 Continue
```http
ICAP/1.0 100 Continue
Encapsulated: null-body=0
```

---

## 📌 2xx – Success

### 200 OK (content modified)
```http
ICAP/1.0 200 OK
ISTag: "srv-123"
Encapsulated: res-hdr=0, res-body=137

HTTP/1.1 200 OK
Content-Type: text/html
Content-Length: 50

This content was modified by the ICAP server.
```

### 204 No Modifications
```http
ICAP/1.0 204 No Modifications
ISTag: "srv-123"
Encapsulated: null-body=0
```

---

## 📌 3xx – Redirection

### 302 Found
```http
ICAP/1.0 302 Found
ISTag: "srv-123"
Location: icap://icap.example.net/new-service
Encapsulated: null-body=0
```

### 304 Not Modified
```http
ICAP/1.0 304 Not Modified
ISTag: "srv-123"
Encapsulated: null-body=0
```

---

## 📌 4xx – Client Error

### 400 Bad Request
```http
ICAP/1.0 400 Bad Request
ISTag: "srv-123"
Encapsulated: null-body=0
```

### 403 Forbidden
```http
ICAP/1.0 403 Forbidden
ISTag: "srv-123"
Encapsulated: res-hdr=0, res-body=77

HTTP/1.1 403 Forbidden
Content-Type: text/html
Content-Length: 30

<html>Blocked by policy</html>
```

### 404 Not Found
```http
ICAP/1.0 404 Not Found
ISTag: "srv-123"
Encapsulated: null-body=0
```

### 405 Method Not Allowed
```http
ICAP/1.0 405 Method Not Allowed
Allow: REQMOD, RESPMOD, OPTIONS
ISTag: "srv-123"
Encapsulated: null-body=0
```

### 407 Proxy Authentication Required
```http
ICAP/1.0 407 Proxy Authentication Required
Proxy-Authenticate: Basic realm="icap-auth"
ISTag: "srv-123"
Encapsulated: null-body=0
```

### 409 Conflict
```http
ICAP/1.0 409 Conflict
ISTag: "srv-123"
Encapsulated: null-body=0
```

### 413 Request Too Large
```http
ICAP/1.0 413 Request Too Large
ISTag: "srv-123"
Encapsulated: null-body=0
```

### 415 Unsupported Media Type
```http
ICAP/1.0 415 Unsupported Media Type
ISTag: "srv-123"
Encapsulated: null-body=0
```

---

## 📌 5xx – Server Error

### 500 Server Error
```http
ICAP/1.0 500 Server Error
ISTag: "srv-123"
Encapsulated: null-body=0
```

### 501 Not Implemented
```http
ICAP/1.0 501 Not Implemented
ISTag: "srv-123"
Encapsulated: null-body=0
```

### 502 Bad Gateway
```http
ICAP/1.0 502 Bad Gateway
ISTag: "srv-123"
Encapsulated: null-body=0
```

### 503 Service Unavailable
```http
ICAP/1.0 503 Service Unavailable
Retry-After: 120
ISTag: "srv-123"
Encapsulated: null-body=0
```

### 505 ICAP Version Not Supported
```http
ICAP/1.0 505 ICAP Version Not Supported
ISTag: "srv-123"
Encapsulated: null-body=0
```

---
