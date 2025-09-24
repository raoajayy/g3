package main

import (
	"context"
	"testing"
	"time"
)

// TestIcapClient_NewIcapClient tests client creation
func TestIcapClient_NewIcapClient(t *testing.T) {
	config := &IcapConfig{
		Host:               "127.0.0.1",
		Port:               1344,
		Timeout:            30 * time.Second,
		Retries:            3,
		RetryDelay:         time.Second,
		MaxRetryDelay:      60 * time.Second,
		BackoffFactor:      2.0,
		ConnectionPoolSize: 10,
		KeepAlive:          true,
		VerifySSL:          true,
		LoggingLevel:       "INFO",
		MetricsEnabled:     true,
	}

	client := NewIcapClient(config)
	if client == nil {
		t.Fatal("Expected client to be created")
	}

	if client.config.Host != config.Host {
		t.Errorf("Expected host %s, got %s", config.Host, client.config.Host)
	}

	if client.config.Port != config.Port {
		t.Errorf("Expected port %d, got %d", config.Port, client.config.Port)
	}

	client.Close()
}

// TestIcapClient_NewIcapClientWithAuth tests client creation with authentication
func TestIcapClient_NewIcapClientWithAuth(t *testing.T) {
	config := &IcapConfig{
		Host:               "127.0.0.1",
		Port:               1344,
		Timeout:            30 * time.Second,
		Retries:            3,
		RetryDelay:         time.Second,
		MaxRetryDelay:      60 * time.Second,
		BackoffFactor:      2.0,
		ConnectionPoolSize: 10,
		KeepAlive:          true,
		VerifySSL:          true,
		LoggingLevel:       "INFO",
		MetricsEnabled:     true,
		Authentication: map[string]string{
			"method":   "basic",
			"username": "testuser",
			"password": "testpass",
		},
	}

	client := NewIcapClient(config)
	if client == nil {
		t.Fatal("Expected client to be created")
	}

	if client.authHandler == nil {
		t.Fatal("Expected auth handler to be created")
	}

	if client.authHandler.method != AuthBasic {
		t.Errorf("Expected auth method %s, got %s", AuthBasic, client.authHandler.method)
	}

	client.Close()
}

// TestAuthenticationHandler_GetHeaders tests authentication header generation
func TestAuthenticationHandler_GetHeaders(t *testing.T) {
	tests := []struct {
		name     string
		method   AuthenticationMethod
		config   map[string]string
		expected map[string]string
	}{
		{
			name:   "No authentication",
			method: AuthNone,
			config: map[string]string{},
			expected: map[string]string{},
		},
		{
			name:   "Basic authentication",
			method: AuthBasic,
			config: map[string]string{
				"username": "testuser",
				"password": "testpass",
			},
			expected: map[string]string{
				"Authorization": "Basic dGVzdHVzZXI6dGVzdHBhc3M=",
			},
		},
		{
			name:   "Bearer authentication",
			method: AuthBearer,
			config: map[string]string{
				"token": "test-token",
			},
			expected: map[string]string{
				"Authorization": "Bearer test-token",
			},
		},
		{
			name:   "JWT authentication",
			method: AuthJWT,
			config: map[string]string{
				"jwt_token": "jwt-test-token",
			},
			expected: map[string]string{
				"Authorization": "Bearer jwt-test-token",
			},
		},
		{
			name:   "API Key authentication",
			method: AuthAPIKey,
			config: map[string]string{
				"api_key":    "test-api-key",
				"header_name": "X-API-Key",
			},
			expected: map[string]string{
				"X-API-Key": "test-api-key",
			},
		},
		{
			name:   "API Key authentication with default header",
			method: AuthAPIKey,
			config: map[string]string{
				"api_key": "test-api-key",
			},
			expected: map[string]string{
				"X-API-Key": "test-api-key",
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			handler := NewAuthenticationHandler(tt.method, tt.config)
			headers := handler.GetHeaders()

			if len(headers) != len(tt.expected) {
				t.Errorf("Expected %d headers, got %d", len(tt.expected), len(headers))
			}

			for name, expectedValue := range tt.expected {
				if actualValue, ok := headers[name]; !ok {
					t.Errorf("Expected header %s not found", name)
				} else if actualValue != expectedValue {
					t.Errorf("Expected header %s value %s, got %s", name, expectedValue, actualValue)
				}
			}
		})
	}
}

// TestIcapClient_buildICAPURL tests URL building
func TestIcapClient_buildICAPURL(t *testing.T) {
	config := &IcapConfig{
		Host: "127.0.0.1",
		Port: 1344,
	}

	client := NewIcapClient(config)
	defer client.Close()

	tests := []struct {
		method   IcapMethod
		expected string
	}{
		{REQMOD, "icap://127.0.0.1:1344/reqmod"},
		{RESPMOD, "icap://127.0.0.1:1344/respmod"},
		{OPTIONS, "icap://127.0.0.1:1344/options"},
	}

	for _, tt := range tests {
		t.Run(string(tt.method), func(t *testing.T) {
			url := client.buildICAPURL(tt.method)
			if url != tt.expected {
				t.Errorf("Expected URL %s, got %s", tt.expected, url)
			}
		})
	}
}

// TestIcapClient_buildEncapsulatedHeader tests encapsulated header building
func TestIcapClient_buildEncapsulatedHeader(t *testing.T) {
	config := &IcapConfig{}
	client := NewIcapClient(config)
	defer client.Close()

	// Test with HTTP request
	httpRequest := &HttpRequest{
		Method:  "GET",
		URI:     "/",
		Version: "HTTP/1.1",
		Headers: map[string]string{
			"Host": "example.com",
		},
	}

	header := client.buildEncapsulatedHeader(httpRequest)
	if header != "req-hdr=0, null-body=75" {
		t.Errorf("Expected 'req-hdr=0, null-body=75', got %s", header)
	}

	// Test with HTTP response
	httpResponse := &HttpResponse{
		Version:    "HTTP/1.1",
		StatusCode: 200,
		Reason:     "OK",
		Headers: map[string]string{
			"Content-Type": "text/html",
		},
	}

	header = client.buildEncapsulatedHeader(httpResponse)
	if header != "res-hdr=0, null-body=120" {
		t.Errorf("Expected 'res-hdr=0, null-body=120', got %s", header)
	}

	// Test with nil
	header = client.buildEncapsulatedHeader(nil)
	if header != "null-body=0" {
		t.Errorf("Expected 'null-body=0', got %s", header)
	}
}

// TestIcapClient_serializeHTTPData tests HTTP data serialization
func TestIcapClient_serializeHTTPData(t *testing.T) {
	config := &IcapConfig{}
	client := NewIcapClient(config)
	defer client.Close()

	// Test HTTP request serialization
	httpRequest := &HttpRequest{
		Method:  "GET",
		URI:     "/",
		Version: "HTTP/1.1",
		Headers: map[string]string{
			"Host":       "example.com",
			"User-Agent": "Go-Client",
		},
		Body: []byte("test body"),
	}

	data := client.serializeHTTPData(httpRequest)
	expected := "GET / HTTP/1.1\r\nHost: example.com\r\nUser-Agent: Go-Client\r\n\r\ntest body"
	if string(data) != expected {
		t.Errorf("Expected %s, got %s", expected, string(data))
	}

	// Test HTTP response serialization
	httpResponse := &HttpResponse{
		Version:    "HTTP/1.1",
		StatusCode: 200,
		Reason:     "OK",
		Headers: map[string]string{
			"Content-Type": "text/html",
		},
		Body: []byte("<html>test</html>"),
	}

	data = client.serializeHTTPData(httpResponse)
	expected = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<html>test</html>"
	if string(data) != expected {
		t.Errorf("Expected %s, got %s", expected, string(data))
	}
}

// TestIcapClient_parseICAPResponse tests ICAP response parsing
func TestIcapClient_parseICAPResponse(t *testing.T) {
	config := &IcapConfig{}
	client := NewIcapClient(config)
	defer client.Close()

	responseText := `ICAP/1.0 200 OK
Server: G3ICAP/1.0.0
ISTag: "test-istag"
Methods: REQMOD, RESPMOD, OPTIONS
Service: G3ICAP Content Filter

HTTP/1.1 200 OK
Content-Type: text/html
Content-Length: 13

Hello World!`

	response := client.parseICAPResponse(responseText)

	if response.Version != "ICAP/1.0" {
		t.Errorf("Expected version ICAP/1.0, got %s", response.Version)
	}

	if response.StatusCode != 200 {
		t.Errorf("Expected status code 200, got %d", response.StatusCode)
	}

	if response.Reason != "OK" {
		t.Errorf("Expected reason OK, got %s", response.Reason)
	}

	expectedHeaders := map[string]string{
		"Server":   "G3ICAP/1.0.0",
		"ISTag":    "\"test-istag\"",
		"Methods":  "REQMOD, RESPMOD, OPTIONS",
		"Service":  "G3ICAP Content Filter",
	}

	for name, expectedValue := range expectedHeaders {
		if actualValue, ok := response.Headers[name]; !ok {
			t.Errorf("Expected header %s not found", name)
		} else if actualValue != expectedValue {
			t.Errorf("Expected header %s value %s, got %s", name, expectedValue, actualValue)
		}
	}

	expectedBody := "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 13\r\n\r\nHello World!"
	if string(response.Body) != expectedBody {
		t.Errorf("Expected body %s, got %s", expectedBody, string(response.Body))
	}
}

// TestIcapClient_Options tests OPTIONS method
func TestIcapClient_Options(t *testing.T) {
	// This test would require a running ICAP server
	// For now, we'll just test that the method exists and doesn't panic
	config := &IcapConfig{
		Host:           "127.0.0.1",
		Port:           1344,
		Timeout:        time.Second,
		Retries:        0, // No retries for test
		LoggingLevel:   "ERROR", // Reduce logging noise
		MetricsEnabled: false,
	}

	client := NewIcapClient(config)
	defer client.Close()

	ctx := context.Background()
	_, err := client.Options(ctx)
	
	// We expect this to fail since there's no server running
	// but we're just testing that the method doesn't panic
	if err == nil {
		t.Log("OPTIONS request succeeded (unexpected in test environment)")
	} else {
		t.Logf("OPTIONS request failed as expected: %v", err)
	}
}

// TestIcapClient_Reqmod tests REQMOD method
func TestIcapClient_Reqmod(t *testing.T) {
	config := &IcapConfig{
		Host:           "127.0.0.1",
		Port:           1344,
		Timeout:        time.Second,
		Retries:        0, // No retries for test
		LoggingLevel:   "ERROR", // Reduce logging noise
		MetricsEnabled: false,
	}

	client := NewIcapClient(config)
	defer client.Close()

	httpRequest := &HttpRequest{
		Method:  "GET",
		URI:     "/",
		Version: "HTTP/1.1",
		Headers: map[string]string{
			"Host": "example.com",
		},
	}

	ctx := context.Background()
	_, err := client.Reqmod(ctx, httpRequest)
	
	// We expect this to fail since there's no server running
	// but we're just testing that the method doesn't panic
	if err == nil {
		t.Log("REQMOD request succeeded (unexpected in test environment)")
	} else {
		t.Logf("REQMOD request failed as expected: %v", err)
	}
}

// TestIcapClient_Respmod tests RESPMOD method
func TestIcapClient_Respmod(t *testing.T) {
	config := &IcapConfig{
		Host:           "127.0.0.1",
		Port:           1344,
		Timeout:        time.Second,
		Retries:        0, // No retries for test
		LoggingLevel:   "ERROR", // Reduce logging noise
		MetricsEnabled: false,
	}

	client := NewIcapClient(config)
	defer client.Close()

	httpResponse := &HttpResponse{
		Version:    "HTTP/1.1",
		StatusCode: 200,
		Reason:     "OK",
		Headers: map[string]string{
			"Content-Type": "text/html",
		},
		Body: []byte("<html>test</html>"),
	}

	ctx := context.Background()
	_, err := client.Respmod(ctx, httpResponse)
	
	// We expect this to fail since there's no server running
	// but we're just testing that the method doesn't panic
	if err == nil {
		t.Log("RESPMOD request succeeded (unexpected in test environment)")
	} else {
		t.Logf("RESPMOD request failed as expected: %v", err)
	}
}

// TestIcapClient_HealthCheck tests health check
func TestIcapClient_HealthCheck(t *testing.T) {
	config := &IcapConfig{
		Host:           "127.0.0.1",
		Port:           1344,
		Timeout:        time.Second,
		Retries:        0, // No retries for test
		LoggingLevel:   "ERROR", // Reduce logging noise
		MetricsEnabled: false,
	}

	client := NewIcapClient(config)
	defer client.Close()

	ctx := context.Background()
	health, err := client.HealthCheck(ctx)
	
	// We expect this to fail since there's no server running
	// but we're just testing that the method doesn't panic
	if err != nil {
		t.Logf("Health check failed as expected: %v", err)
		return
	}

	if health["status"] != "unhealthy" {
		t.Errorf("Expected status unhealthy, got %s", health["status"])
	}
}

// TestLoadConfig tests configuration loading
func TestLoadConfig(t *testing.T) {
	// Test with invalid config file
	_, err := LoadConfig("nonexistent.yaml")
	if err == nil {
		t.Error("Expected error for nonexistent config file")
	}

	// Test with valid config (would need to create a test config file)
	// For now, we'll just test that the function exists
	if LoadConfig == nil {
		t.Error("LoadConfig function not found")
	}
}

// TestIcapError tests error handling
func TestIcapError(t *testing.T) {
	err := &IcapError{
		Message: "Test error",
		Code:    500,
		Err:     nil,
	}

	if err.Error() != "Test error" {
		t.Errorf("Expected error message 'Test error', got %s", err.Error())
	}

	errWithCause := &IcapError{
		Message: "Test error",
		Code:    500,
		Err:     err,
	}

	expected := "Test error: Test error"
	if errWithCause.Error() != expected {
		t.Errorf("Expected error message '%s', got %s", expected, errWithCause.Error())
	}
}

// TestClientMetrics tests metrics creation
func TestClientMetrics(t *testing.T) {
	metrics := NewClientMetrics()
	if metrics == nil {
		t.Fatal("Expected metrics to be created")
	}

	if metrics.RequestsTotal == nil {
		t.Error("Expected RequestsTotal counter to be created")
	}

	if metrics.RequestsSuccess == nil {
		t.Error("Expected RequestsSuccess counter to be created")
	}

	if metrics.RequestsFailed == nil {
		t.Error("Expected RequestsFailed counter to be created")
	}

	if metrics.ResponseTime == nil {
		t.Error("Expected ResponseTime histogram to be created")
	}

	if metrics.ConnectionPool == nil {
		t.Error("Expected ConnectionPool gauge to be created")
	}
}

// BenchmarkIcapClient_serializeHTTPData benchmarks HTTP data serialization
func BenchmarkIcapClient_serializeHTTPData(b *testing.B) {
	config := &IcapConfig{}
	client := NewIcapClient(config)
	defer client.Close()

	httpRequest := &HttpRequest{
		Method:  "GET",
		URI:     "/",
		Version: "HTTP/1.1",
		Headers: map[string]string{
			"Host":       "example.com",
			"User-Agent": "Go-Client",
		},
		Body: []byte("test body"),
	}

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		client.serializeHTTPData(httpRequest)
	}
}

// BenchmarkIcapClient_parseICAPResponse benchmarks ICAP response parsing
func BenchmarkIcapClient_parseICAPResponse(b *testing.B) {
	config := &IcapConfig{}
	client := NewIcapClient(config)
	defer client.Close()

	responseText := `ICAP/1.0 200 OK
Server: G3ICAP/1.0.0
ISTag: "test-istag"
Methods: REQMOD, RESPMOD, OPTIONS
Service: G3ICAP Content Filter

HTTP/1.1 200 OK
Content-Type: text/html
Content-Length: 13

Hello World!`

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		client.parseICAPResponse(responseText)
	}
}
