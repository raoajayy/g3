package main

/*
G3ICAP Go Client

A comprehensive Go client for interacting with G3ICAP servers,
supporting REQMOD, RESPMOD, and OPTIONS methods with full authentication
and error handling capabilities.

Author: G3ICAP Team
License: Apache 2.0
Version: 1.0.0
*/

import (
	"bytes"
	"context"
	"crypto/tls"
	"encoding/base64"
	"fmt"
	"io"
	"net"
	"net/http"
	"strconv"
	"strings"
	"time"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
	"github.com/sirupsen/logrus"
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

// IcapMethod represents ICAP methods
type IcapMethod string

const (
	REQMOD  IcapMethod = "REQMOD"
	RESPMOD IcapMethod = "RESPMOD"
	OPTIONS IcapMethod = "OPTIONS"
)

// IcapResponseCode represents ICAP response codes
type IcapResponseCode int

const (
	Continue                    IcapResponseCode = 100
	OK                         IcapResponseCode = 200
	NoContent                  IcapResponseCode = 204
	BadRequest                 IcapResponseCode = 400
	NotFound                   IcapResponseCode = 404
	MethodNotAllowed           IcapResponseCode = 405
	RequestTimeout             IcapResponseCode = 408
	RequestEntityTooLarge      IcapResponseCode = 413
	InternalServerError        IcapResponseCode = 500
	NotImplemented             IcapResponseCode = 501
	BadGateway                 IcapResponseCode = 502
	ServiceUnavailable         IcapResponseCode = 503
	IcapVersionNotSupported    IcapResponseCode = 505
)

// AuthenticationMethod represents authentication methods
type AuthenticationMethod string

const (
	AuthNone    AuthenticationMethod = "none"
	AuthBasic   AuthenticationMethod = "basic"
	AuthBearer  AuthenticationMethod = "bearer"
	AuthJWT     AuthenticationMethod = "jwt"
	AuthAPIKey  AuthenticationMethod = "api_key"
)

// IcapConfig represents ICAP client configuration
type IcapConfig struct {
	Host               string            `yaml:"host" json:"host"`
	Port               int               `yaml:"port" json:"port"`
	Timeout            time.Duration     `yaml:"timeout" json:"timeout"`
	Retries            int               `yaml:"retries" json:"retries"`
	RetryDelay         time.Duration     `yaml:"retry_delay" json:"retry_delay"`
	MaxRetryDelay      time.Duration     `yaml:"max_retry_delay" json:"max_retry_delay"`
	BackoffFactor      float64           `yaml:"backoff_factor" json:"backoff_factor"`
	ConnectionPoolSize int               `yaml:"connection_pool_size" json:"connection_pool_size"`
	KeepAlive          bool              `yaml:"keep_alive" json:"keep_alive"`
	VerifySSL          bool              `yaml:"verify_ssl" json:"verify_ssl"`
	Authentication     map[string]string `yaml:"authentication" json:"authentication"`
	LoggingLevel       string            `yaml:"logging_level" json:"logging_level"`
	MetricsEnabled     bool              `yaml:"metrics_enabled" json:"metrics_enabled"`
}

// HttpRequest represents an HTTP request
type HttpRequest struct {
	Method  string            `yaml:"method" json:"method"`
	URI     string            `yaml:"uri" json:"uri"`
	Version string            `yaml:"version" json:"version"`
	Headers map[string]string `yaml:"headers" json:"headers"`
	Body    []byte            `yaml:"body" json:"body"`
}

// HttpResponse represents an HTTP response
type HttpResponse struct {
	Version    string            `yaml:"version" json:"version"`
	StatusCode int               `yaml:"status_code" json:"status_code"`
	Reason     string            `yaml:"reason" json:"reason"`
	Headers    map[string]string `yaml:"headers" json:"headers"`
	Body       []byte            `yaml:"body" json:"body"`
}

// IcapResponse represents an ICAP response
type IcapResponse struct {
	Version    string            `yaml:"version" json:"version"`
	StatusCode int               `yaml:"status_code" json:"status_code"`
	Reason     string            `yaml:"reason" json:"reason"`
	Headers    map[string]string `yaml:"headers" json:"headers"`
	Body       []byte            `yaml:"body" json:"body"`
	HttpRequest  *HttpRequest  `yaml:"http_request,omitempty" json:"http_request,omitempty"`
	HttpResponse *HttpResponse `yaml:"http_response,omitempty" json:"http_response,omitempty"`
}

// IcapError represents ICAP client errors
type IcapError struct {
	Message string
	Code    int
	Err     error
}

func (e *IcapError) Error() string {
	if e.Err != nil {
		return fmt.Sprintf("%s: %v", e.Message, e.Err)
	}
	return e.Message
}

// AuthenticationHandler handles different authentication methods
type AuthenticationHandler struct {
	method AuthenticationMethod
	config map[string]string
}

// NewAuthenticationHandler creates a new authentication handler
func NewAuthenticationHandler(method AuthenticationMethod, config map[string]string) *AuthenticationHandler {
	return &AuthenticationHandler{
		method: method,
		config: config,
	}
}

// GetHeaders returns authentication headers
func (h *AuthenticationHandler) GetHeaders() map[string]string {
	headers := make(map[string]string)

	switch h.method {
	case AuthNone:
		// No authentication
	case AuthBasic:
		username := h.config["username"]
		password := h.config["password"]
		credentials := base64.StdEncoding.EncodeToString([]byte(username + ":" + password))
		headers["Authorization"] = "Basic " + credentials
	case AuthBearer:
		token := h.config["token"]
		headers["Authorization"] = "Bearer " + token
	case AuthJWT:
		token := h.config["jwt_token"]
		headers["Authorization"] = "Bearer " + token
	case AuthAPIKey:
		apiKey := h.config["api_key"]
		headerName := h.config["header_name"]
		if headerName == "" {
			headerName = "X-API-Key"
		}
		headers[headerName] = apiKey
	}

	return headers
}

// IcapClient represents the ICAP client
type IcapClient struct {
	config        *IcapConfig
	logger        *logrus.Logger
	httpClient    *http.Client
	authHandler   *AuthenticationHandler
	metrics       *ClientMetrics
}

// ClientMetrics represents client metrics
type ClientMetrics struct {
	RequestsTotal     prometheus.Counter
	RequestsSuccess   prometheus.Counter
	RequestsFailed    prometheus.Counter
	ResponseTime      prometheus.Histogram
	ConnectionPool    prometheus.Gauge
}

// NewClientMetrics creates new client metrics
func NewClientMetrics() *ClientMetrics {
	return &ClientMetrics{
		RequestsTotal: promauto.NewCounter(prometheus.CounterOpts{
			Name: "icap_client_requests_total",
			Help: "Total number of ICAP requests",
		}),
		RequestsSuccess: promauto.NewCounter(prometheus.CounterOpts{
			Name: "icap_client_requests_success_total",
			Help: "Total number of successful ICAP requests",
		}),
		RequestsFailed: promauto.NewCounter(prometheus.CounterOpts{
			Name: "icap_client_requests_failed_total",
			Help: "Total number of failed ICAP requests",
		}),
		ResponseTime: promauto.NewHistogram(prometheus.HistogramOpts{
			Name:    "icap_client_response_time_seconds",
			Help:    "ICAP client response time in seconds",
			Buckets: prometheus.DefBuckets,
		}),
		ConnectionPool: promauto.NewGauge(prometheus.GaugeOpts{
			Name: "icap_client_connection_pool_size",
			Help: "ICAP client connection pool size",
		}),
	}
}

// NewIcapClient creates a new ICAP client
func NewIcapClient(config *IcapConfig) *IcapClient {
	logger := logrus.New()
	logger.SetLevel(getLogLevel(config.LoggingLevel))

	// Setup authentication
	var authHandler *AuthenticationHandler
	if config.Authentication != nil {
		method := AuthenticationMethod(config.Authentication["method"])
		authHandler = NewAuthenticationHandler(method, config.Authentication)
	}

	// Setup HTTP client
	transport := &http.Transport{
		MaxIdleConns:        config.ConnectionPoolSize,
		MaxIdleConnsPerHost: config.ConnectionPoolSize,
		IdleConnTimeout:     config.Timeout,
		DisableKeepAlives:   !config.KeepAlive,
		TLSClientConfig: &tls.Config{
			InsecureSkipVerify: !config.VerifySSL,
		},
		DialContext: (&net.Dialer{
			Timeout:   config.Timeout,
			KeepAlive: config.Timeout,
		}).DialContext,
	}

	httpClient := &http.Client{
		Transport: transport,
		Timeout:   config.Timeout,
	}

	// Setup metrics
	var metrics *ClientMetrics
	if config.MetricsEnabled {
		metrics = NewClientMetrics()
		metrics.ConnectionPool.Set(float64(config.ConnectionPoolSize))
	}

	return &IcapClient{
		config:      config,
		logger:      logger,
		httpClient:  httpClient,
		authHandler: authHandler,
		metrics:     metrics,
	}
}

// getLogLevel converts string to logrus level
func getLogLevel(level string) logrus.Level {
	switch strings.ToUpper(level) {
	case "DEBUG":
		return logrus.DebugLevel
	case "INFO":
		return logrus.InfoLevel
	case "WARN", "WARNING":
		return logrus.WarnLevel
	case "ERROR":
		return logrus.ErrorLevel
	case "FATAL":
		return logrus.FatalLevel
	default:
		return logrus.InfoLevel
	}
}

// buildICAPURL builds ICAP URL for method
func (c *IcapClient) buildICAPURL(method IcapMethod) string {
	var path string
	switch method {
	case REQMOD:
		path = "/reqmod"
	case RESPMOD:
		path = "/respmod"
	case OPTIONS:
		path = "/options"
	}
	return fmt.Sprintf("icap://%s:%d%s", c.config.Host, c.config.Port, path)
}

// buildEncapsulatedHeader builds Encapsulated header for ICAP request
func (c *IcapClient) buildEncapsulatedHeader(httpData interface{}) string {
	switch httpData.(type) {
	case *HttpRequest:
		return "req-hdr=0, null-body=75"
	case *HttpResponse:
		return "res-hdr=0, null-body=120"
	default:
		return "null-body=0"
	}
}

// serializeHTTPData serializes HTTP data for ICAP body
func (c *IcapClient) serializeHTTPData(httpData interface{}) []byte {
	var lines []string

	switch data := httpData.(type) {
	case *HttpRequest:
		lines = append(lines, fmt.Sprintf("%s %s %s", data.Method, data.URI, data.Version))
		for name, value := range data.Headers {
			lines = append(lines, fmt.Sprintf("%s: %s", name, value))
		}
		lines = append(lines, "") // Empty line
		if len(data.Body) > 0 {
			lines = append(lines, string(data.Body))
		}
	case *HttpResponse:
		lines = append(lines, fmt.Sprintf("%s %d %s", data.Version, data.StatusCode, data.Reason))
		for name, value := range data.Headers {
			lines = append(lines, fmt.Sprintf("%s: %s", name, value))
		}
		lines = append(lines, "") // Empty line
		if len(data.Body) > 0 {
			lines = append(lines, string(data.Body))
		}
	}

	return []byte(strings.Join(lines, "\r\n"))
}

// parseICAPResponse parses ICAP response
func (c *IcapClient) parseICAPResponse(responseText string) *IcapResponse {
	lines := strings.Split(responseText, "\r\n")

	// Parse status line
	statusLine := lines[0]
	parts := strings.SplitN(statusLine, " ", 3)
	version := parts[0]
	statusCode, _ := strconv.Atoi(parts[1])
	reason := parts[2]

	// Parse headers
	headers := make(map[string]string)
	bodyStart := 0

	for i, line := range lines[1:] {
		if line == "" {
			bodyStart = i + 2
			break
		}
		if strings.Contains(line, ":") {
			parts := strings.SplitN(line, ":", 2)
			name := strings.TrimSpace(parts[0])
			value := strings.TrimSpace(parts[1])
			headers[name] = value
		}
	}

	// Parse body
	var body []byte
	if bodyStart < len(lines) {
		bodyText := strings.Join(lines[bodyStart:], "\r\n")
		if strings.TrimSpace(bodyText) != "" {
			body = []byte(bodyText)
		}
	}

	return &IcapResponse{
		Version:    version,
		StatusCode: statusCode,
		Reason:     reason,
		Headers:    headers,
		Body:       body,
	}
}

// makeRequest makes ICAP request with retry logic
func (c *IcapClient) makeRequest(ctx context.Context, method IcapMethod, httpData interface{}) (*IcapResponse, error) {
	url := c.buildICAPURL(method)

	// Build headers
	headers := make(map[string]string)
	headers["Host"] = fmt.Sprintf("%s:%d", c.config.Host, c.config.Port)
	headers["User-Agent"] = "G3ICAP-Go-Client/1.0.0"
	headers["Allow"] = "204"

	if httpData != nil {
		headers["Encapsulated"] = c.buildEncapsulatedHeader(httpData)
	}

	// Add authentication headers
	if c.authHandler != nil {
		authHeaders := c.authHandler.GetHeaders()
		for name, value := range authHeaders {
			headers[name] = value
		}
	}

	// Build body
	var body []byte
	if httpData != nil {
		body = c.serializeHTTPData(httpData)
	}

	// Retry logic
	var lastErr error
	for attempt := 0; attempt <= c.config.Retries; attempt++ {
		startTime := time.Now()

		// Create request
		req, err := http.NewRequestWithContext(ctx, string(method), url, bytes.NewReader(body))
		if err != nil {
			lastErr = &IcapError{Message: "Failed to create request", Err: err}
			continue
		}

		// Set headers
		for name, value := range headers {
			req.Header.Set(name, value)
		}

		// Make request
		resp, err := c.httpClient.Do(req)
		if err != nil {
			lastErr = &IcapError{Message: "Request failed", Err: err}
			c.logger.WithError(err).WithField("attempt", attempt+1).Warn("Request failed")
			continue
		}

		// Read response
		responseBody, err := io.ReadAll(resp.Body)
		resp.Body.Close()
		if err != nil {
			lastErr = &IcapError{Message: "Failed to read response", Err: err}
			continue
		}

		responseTime := time.Since(startTime)

		// Update metrics
		if c.metrics != nil {
			c.metrics.RequestsTotal.Inc()
			c.metrics.ResponseTime.Observe(responseTime.Seconds())
			if resp.StatusCode < 400 {
				c.metrics.RequestsSuccess.Inc()
			} else {
				c.metrics.RequestsFailed.Inc()
			}
		}

		// Parse response
		icapResponse := c.parseICAPResponse(string(responseBody))

		c.logger.WithFields(logrus.Fields{
			"method":       method,
			"status_code":  icapResponse.StatusCode,
			"response_time": responseTime,
			"attempt":      attempt + 1,
		}).Info("ICAP request completed")

		return icapResponse, nil
	}

	// All retries failed
	if c.metrics != nil {
		c.metrics.RequestsFailed.Inc()
	}
	return nil, lastErr
}

// Reqmod sends REQMOD request
func (c *IcapClient) Reqmod(ctx context.Context, httpRequest *HttpRequest) (*IcapResponse, error) {
	c.logger.WithField("uri", httpRequest.URI).Info("Sending REQMOD request")

	response, err := c.makeRequest(ctx, REQMOD, httpRequest)
	if err != nil {
		c.logger.WithError(err).Error("REQMOD request failed")
		return nil, err
	}

	return response, nil
}

// Respmod sends RESPMOD request
func (c *IcapClient) Respmod(ctx context.Context, httpResponse *HttpResponse) (*IcapResponse, error) {
	c.logger.WithField("status_code", httpResponse.StatusCode).Info("Sending RESPMOD request")

	response, err := c.makeRequest(ctx, RESPMOD, httpResponse)
	if err != nil {
		c.logger.WithError(err).Error("RESPMOD request failed")
		return nil, err
	}

	return response, nil
}

// Options sends OPTIONS request
func (c *IcapClient) Options(ctx context.Context) (*IcapResponse, error) {
	c.logger.Info("Sending OPTIONS request")

	response, err := c.makeRequest(ctx, OPTIONS, nil)
	if err != nil {
		c.logger.WithError(err).Error("OPTIONS request failed")
		return nil, err
	}

	return response, nil
}

// HealthCheck checks server health
func (c *IcapClient) HealthCheck(ctx context.Context) (map[string]interface{}, error) {
	response, err := c.Options(ctx)
	if err != nil {
		return map[string]interface{}{
			"status": "unhealthy",
			"error":  err.Error(),
		}, nil
	}

	status := "healthy"
	if response.StatusCode >= 400 {
		status = "unhealthy"
	}

	methods := []string{}
	if methodsStr, ok := response.Headers["Methods"]; ok {
		methods = strings.Split(methodsStr, ",")
		for i, method := range methods {
			methods[i] = strings.TrimSpace(method)
		}
	}

	return map[string]interface{}{
		"status":      status,
		"status_code": response.StatusCode,
		"version":     response.Headers["Service"],
		"methods":     methods,
		"istag":       response.Headers["ISTag"],
	}, nil
}

// Close closes the client
func (c *IcapClient) Close() {
	if c.httpClient != nil {
		c.httpClient.CloseIdleConnections()
	}
	c.logger.Info("ICAP client closed")
}

// LoadConfig loads configuration from file
func LoadConfig(configPath string) (*IcapConfig, error) {
	viper.SetConfigFile(configPath)
	viper.SetConfigType("yaml")

	// Set defaults
	viper.SetDefault("host", "127.0.0.1")
	viper.SetDefault("port", 1344)
	viper.SetDefault("timeout", "30s")
	viper.SetDefault("retries", 3)
	viper.SetDefault("retry_delay", "1s")
	viper.SetDefault("max_retry_delay", "60s")
	viper.SetDefault("backoff_factor", 2.0)
	viper.SetDefault("connection_pool_size", 10)
	viper.SetDefault("keep_alive", true)
	viper.SetDefault("verify_ssl", true)
	viper.SetDefault("logging_level", "INFO")
	viper.SetDefault("metrics_enabled", true)

	if err := viper.ReadInConfig(); err != nil {
		return nil, fmt.Errorf("failed to read config file: %w", err)
	}

	var config IcapConfig
	if err := viper.Unmarshal(&config); err != nil {
		return nil, fmt.Errorf("failed to unmarshal config: %w", err)
	}

	return &config, nil
}

// main function and CLI
func main() {
	var rootCmd = &cobra.Command{
		Use:   "icap-client",
		Short: "G3ICAP Go Client",
		Long:  "A comprehensive Go client for interacting with G3ICAP servers",
	}

	var configPath string
	var host string
	var port int
	var method string
	var verbose bool

	rootCmd.PersistentFlags().StringVar(&configPath, "config", "", "Configuration file path")
	rootCmd.PersistentFlags().StringVar(&host, "host", "127.0.0.1", "ICAP server host")
	rootCmd.PersistentFlags().IntVar(&port, "port", 1344, "ICAP server port")
	rootCmd.PersistentFlags().StringVar(&method, "method", "options", "ICAP method (reqmod, respmod, options)")
	rootCmd.PersistentFlags().BoolVar(&verbose, "verbose", false, "Verbose logging")

	rootCmd.RunE = func(cmd *cobra.Command, args []string) error {
		// Load configuration
		var config *IcapConfig
		var err error

		if configPath != "" {
			config, err = LoadConfig(configPath)
			if err != nil {
				return fmt.Errorf("failed to load config: %w", err)
			}
		} else {
			config = &IcapConfig{
				Host:           host,
				Port:           port,
				Timeout:        30 * time.Second,
				Retries:        3,
				RetryDelay:     time.Second,
				MaxRetryDelay:  60 * time.Second,
				BackoffFactor:  2.0,
				ConnectionPoolSize: 10,
				KeepAlive:      true,
				VerifySSL:      true,
				LoggingLevel:   "INFO",
				MetricsEnabled: true,
			}
		}

		if verbose {
			config.LoggingLevel = "DEBUG"
		}

		// Create client
		client := NewIcapClient(config)
		defer client.Close()

		ctx := context.Background()

		// Execute method
		switch method {
		case "options":
			response, err := client.Options(ctx)
			if err != nil {
				return fmt.Errorf("OPTIONS request failed: %w", err)
			}
			fmt.Printf("OPTIONS Response: %d %s\n", response.StatusCode, response.Reason)
			fmt.Printf("Headers: %+v\n", response.Headers)

		case "reqmod":
			httpRequest := &HttpRequest{
				Method:  "GET",
				URI:     "/",
				Version: "HTTP/1.1",
				Headers: map[string]string{
					"Host":       "example.com",
					"User-Agent": "Go-Client",
				},
			}

			response, err := client.Reqmod(ctx, httpRequest)
			if err != nil {
				return fmt.Errorf("REQMOD request failed: %w", err)
			}
			fmt.Printf("REQMOD Response: %d %s\n", response.StatusCode, response.Reason)

		case "respmod":
			httpResponse := &HttpResponse{
				Version:    "HTTP/1.1",
				StatusCode: 200,
				Reason:     "OK",
				Headers: map[string]string{
					"Content-Type": "text/html",
				},
				Body: []byte("<html><body>Hello World</body></html>"),
			}

			response, err := client.Respmod(ctx, httpResponse)
			if err != nil {
				return fmt.Errorf("RESPMOD request failed: %w", err)
			}
			fmt.Printf("RESPMOD Response: %d %s\n", response.StatusCode, response.Reason)
		}

		// Health check
		health, err := client.HealthCheck(ctx)
		if err != nil {
			return fmt.Errorf("health check failed: %w", err)
		}
		fmt.Printf("Health Check: %+v\n", health)

		return nil
	}

	if err := rootCmd.Execute(); err != nil {
		fmt.Fprintf(rootCmd.OutOrStderr(), "Error: %v\n", err)
	}
}
