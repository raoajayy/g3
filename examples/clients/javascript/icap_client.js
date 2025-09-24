#!/usr/bin/env node

/**
 * G3ICAP JavaScript Client
 * 
 * A comprehensive JavaScript/Node.js client for interacting with G3ICAP servers,
 * supporting REQMOD, RESPMOD, and OPTIONS methods with full authentication
 * and error handling capabilities.
 * 
 * @author G3ICAP Team
 * @license Apache-2.0
 * @version 1.0.0
 */

const axios = require('axios');
const winston = require('winston');
const promClient = require('prom-client');
const yargs = require('yargs/yargs');
const { hideBin } = require('yargs/helpers');
const fs = require('fs');
const yaml = require('yaml');

// ICAP Methods
const ICAP_METHODS = {
    REQMOD: 'REQMOD',
    RESPMOD: 'RESPMOD',
    OPTIONS: 'OPTIONS'
};

// ICAP Response Codes
const ICAP_RESPONSE_CODES = {
    CONTINUE: 100,
    OK: 200,
    NO_CONTENT: 204,
    BAD_REQUEST: 400,
    NOT_FOUND: 404,
    METHOD_NOT_ALLOWED: 405,
    REQUEST_TIMEOUT: 408,
    REQUEST_ENTITY_TOO_LARGE: 413,
    INTERNAL_SERVER_ERROR: 500,
    NOT_IMPLEMENTED: 501,
    BAD_GATEWAY: 502,
    SERVICE_UNAVAILABLE: 503,
    ICAP_VERSION_NOT_SUPPORTED: 505
};

// Authentication Methods
const AUTH_METHODS = {
    NONE: 'none',
    BASIC: 'basic',
    BEARER: 'bearer',
    JWT: 'jwt',
    API_KEY: 'api_key'
};

/**
 * ICAP Client Configuration
 */
class IcapConfig {
    constructor(options = {}) {
        this.host = options.host || '127.0.0.1';
        this.port = options.port || 1344;
        this.timeout = options.timeout || 30000;
        this.retries = options.retries || 3;
        this.retryDelay = options.retryDelay || 1000;
        this.maxRetryDelay = options.maxRetryDelay || 60000;
        this.backoffFactor = options.backoffFactor || 2.0;
        this.connectionPoolSize = options.connectionPoolSize || 10;
        this.keepAlive = options.keepAlive !== false;
        this.verifySSL = options.verifySSL !== false;
        this.authentication = options.authentication || null;
        this.loggingLevel = options.loggingLevel || 'info';
        this.metricsEnabled = options.metricsEnabled !== false;
    }

    static fromFile(filePath) {
        try {
            const fileContent = fs.readFileSync(filePath, 'utf8');
            const configData = yaml.parse(fileContent);
            return new IcapConfig(configData);
        } catch (error) {
            throw new Error(`Failed to load config from ${filePath}: ${error.message}`);
        }
    }
}

/**
 * HTTP Request representation
 */
class HttpRequest {
    constructor(options = {}) {
        this.method = options.method || 'GET';
        this.uri = options.uri || '/';
        this.version = options.version || 'HTTP/1.1';
        this.headers = options.headers || {};
        this.body = options.body || Buffer.alloc(0);
    }
}

/**
 * HTTP Response representation
 */
class HttpResponse {
    constructor(options = {}) {
        this.version = options.version || 'HTTP/1.1';
        this.statusCode = options.statusCode || 200;
        this.reason = options.reason || 'OK';
        this.headers = options.headers || {};
        this.body = options.body || Buffer.alloc(0);
    }
}

/**
 * ICAP Response representation
 */
class IcapResponse {
    constructor(options = {}) {
        this.version = options.version || 'ICAP/1.0';
        this.statusCode = options.statusCode || 200;
        this.reason = options.reason || 'OK';
        this.headers = options.headers || {};
        this.body = options.body || Buffer.alloc(0);
        this.httpRequest = options.httpRequest || null;
        this.httpResponse = options.httpResponse || null;
    }
}

/**
 * ICAP Client Error
 */
class IcapError extends Error {
    constructor(message, code = 0, cause = null) {
        super(message);
        this.name = 'IcapError';
        this.code = code;
        this.cause = cause;
    }
}

/**
 * Authentication Handler
 */
class AuthenticationHandler {
    constructor(method, config = {}) {
        this.method = method;
        this.config = config;
    }

    getHeaders() {
        const headers = {};

        switch (this.method) {
            case AUTH_METHODS.NONE:
                // No authentication
                break;
            case AUTH_METHODS.BASIC:
                const username = this.config.username;
                const password = this.config.password;
                const credentials = Buffer.from(`${username}:${password}`).toString('base64');
                headers['Authorization'] = `Basic ${credentials}`;
                break;
            case AUTH_METHODS.BEARER:
                const token = this.config.token;
                headers['Authorization'] = `Bearer ${token}`;
                break;
            case AUTH_METHODS.JWT:
                const jwtToken = this.config.jwt_token;
                headers['Authorization'] = `Bearer ${jwtToken}`;
                break;
            case AUTH_METHODS.API_KEY:
                const apiKey = this.config.api_key;
                const headerName = this.config.header_name || 'X-API-Key';
                headers[headerName] = apiKey;
                break;
        }

        return headers;
    }
}

/**
 * Client Metrics
 */
class ClientMetrics {
    constructor() {
        this.requestsTotal = new promClient.Counter({
            name: 'icap_client_requests_total',
            help: 'Total number of ICAP requests'
        });

        this.requestsSuccess = new promClient.Counter({
            name: 'icap_client_requests_success_total',
            help: 'Total number of successful ICAP requests'
        });

        this.requestsFailed = new promClient.Counter({
            name: 'icap_client_requests_failed_total',
            help: 'Total number of failed ICAP requests'
        });

        this.responseTime = new promClient.Histogram({
            name: 'icap_client_response_time_seconds',
            help: 'ICAP client response time in seconds',
            buckets: [0.1, 0.5, 1, 2, 5, 10, 30, 60]
        });

        this.connectionPool = new promClient.Gauge({
            name: 'icap_client_connection_pool_size',
            help: 'ICAP client connection pool size'
        });
    }
}

/**
 * ICAP Client
 */
class IcapClient {
    constructor(config) {
        this.config = config;
        this.logger = this.createLogger();
        this.httpClient = this.createHttpClient();
        this.authHandler = this.createAuthHandler();
        this.metrics = config.metricsEnabled ? new ClientMetrics() : null;

        if (this.metrics) {
            this.metrics.connectionPool.set(config.connectionPoolSize);
        }
    }

    createLogger() {
        const level = this.config.loggingLevel.toLowerCase();
        return winston.createLogger({
            level: level,
            format: winston.format.combine(
                winston.format.timestamp(),
                winston.format.errors({ stack: true }),
                winston.format.json()
            ),
            transports: [
                new winston.transports.Console({
                    format: winston.format.combine(
                        winston.format.colorize(),
                        winston.format.simple()
                    )
                })
            ]
        });
    }

    createHttpClient() {
        const axiosConfig = {
            timeout: this.config.timeout,
            maxRedirects: 0,
            validateStatus: () => true, // Accept all status codes
            httpsAgent: new (require('https').Agent)({
                rejectUnauthorized: this.config.verifySSL,
                keepAlive: this.config.keepAlive
            }),
            httpAgent: new (require('http').Agent)({
                keepAlive: this.config.keepAlive
            })
        };

        return axios.create(axiosConfig);
    }

    createAuthHandler() {
        if (!this.config.authentication) {
            return null;
        }

        const method = this.config.authentication.method;
        return new AuthenticationHandler(method, this.config.authentication);
    }

    buildICAPURL(method) {
        let path;
        switch (method) {
            case ICAP_METHODS.REQMOD:
                path = '/reqmod';
                break;
            case ICAP_METHODS.RESPMOD:
                path = '/respmod';
                break;
            case ICAP_METHODS.OPTIONS:
                path = '/options';
                break;
            default:
                throw new IcapError(`Unsupported ICAP method: ${method}`);
        }

        return `icap://${this.config.host}:${this.config.port}${path}`;
    }

    buildEncapsulatedHeader(httpData) {
        if (httpData instanceof HttpRequest) {
            return 'req-hdr=0, null-body=75';
        } else if (httpData instanceof HttpResponse) {
            return 'res-hdr=0, null-body=120';
        } else {
            return 'null-body=0';
        }
    }

    serializeHTTPData(httpData) {
        const lines = [];

        if (httpData instanceof HttpRequest) {
            lines.push(`${httpData.method} ${httpData.uri} ${httpData.version}`);
            for (const [name, value] of Object.entries(httpData.headers)) {
                lines.push(`${name}: ${value}`);
            }
            lines.push(''); // Empty line
            if (httpData.body && httpData.body.length > 0) {
                lines.push(httpData.body.toString());
            }
        } else if (httpData instanceof HttpResponse) {
            lines.push(`${httpData.version} ${httpData.statusCode} ${httpData.reason}`);
            for (const [name, value] of Object.entries(httpData.headers)) {
                lines.push(`${name}: ${value}`);
            }
            lines.push(''); // Empty line
            if (httpData.body && httpData.body.length > 0) {
                lines.push(httpData.body.toString());
            }
        }

        return Buffer.from(lines.join('\r\n'));
    }

    parseICAPResponse(responseText) {
        const lines = responseText.split('\r\n');

        // Parse status line
        const statusLine = lines[0];
        const statusParts = statusLine.split(' ', 3);
        const version = statusParts[0];
        const statusCode = parseInt(statusParts[1], 10);
        const reason = statusParts[2];

        // Parse headers
        const headers = {};
        let bodyStart = 0;

        for (let i = 1; i < lines.length; i++) {
            const line = lines[i];
            if (line === '') {
                bodyStart = i + 1;
                break;
            }
            if (line.includes(':')) {
                const headerParts = line.split(':', 2);
                const name = headerParts[0].trim();
                const value = headerParts[1].trim();
                headers[name] = value;
            }
        }

        // Parse body
        let body = Buffer.alloc(0);
        if (bodyStart < lines.length) {
            const bodyText = lines.slice(bodyStart).join('\r\n');
            if (bodyText.trim() !== '') {
                body = Buffer.from(bodyText);
            }
        }

        return new IcapResponse({
            version,
            statusCode,
            reason,
            headers,
            body
        });
    }

    async makeRequest(method, httpData = null) {
        const url = this.buildICAPURL(method);

        // Build headers
        const headers = {
            'Host': `${this.config.host}:${this.config.port}`,
            'User-Agent': 'G3ICAP-JS-Client/1.0.0',
            'Allow': '204'
        };

        if (httpData) {
            headers['Encapsulated'] = this.buildEncapsulatedHeader(httpData);
        }

        // Add authentication headers
        if (this.authHandler) {
            const authHeaders = this.authHandler.getHeaders();
            Object.assign(headers, authHeaders);
        }

        // Build body
        let body = Buffer.alloc(0);
        if (httpData) {
            body = this.serializeHTTPData(httpData);
        }

        // Retry logic
        let lastError;
        for (let attempt = 0; attempt <= this.config.retries; attempt++) {
            const startTime = Date.now();

            try {
                const response = await this.httpClient.request({
                    method: method,
                    url: url,
                    headers: headers,
                    data: body
                });

                const responseTime = (Date.now() - startTime) / 1000;

                // Update metrics
                if (this.metrics) {
                    this.metrics.requestsTotal.inc();
                    this.metrics.responseTime.observe(responseTime);
                    if (response.status < 400) {
                        this.metrics.requestsSuccess.inc();
                    } else {
                        this.metrics.requestsFailed.inc();
                    }
                }

                // Parse response
                const icapResponse = this.parseICAPResponse(response.data);

                this.logger.info('ICAP request completed', {
                    method,
                    statusCode: icapResponse.statusCode,
                    responseTime,
                    attempt: attempt + 1
                });

                return icapResponse;

            } catch (error) {
                lastError = error;
                this.logger.warn('Request failed', {
                    error: error.message,
                    attempt: attempt + 1
                });

                if (attempt < this.config.retries) {
                    const delay = Math.min(
                        this.config.retryDelay * Math.pow(this.config.backoffFactor, attempt),
                        this.config.maxRetryDelay
                    );
                    await new Promise(resolve => setTimeout(resolve, delay));
                }
            }
        }

        // All retries failed
        if (this.metrics) {
            this.metrics.requestsFailed.inc();
        }

        throw new IcapError('All retry attempts failed', 0, lastError);
    }

    async reqmod(httpRequest) {
        this.logger.info('Sending REQMOD request', { uri: httpRequest.uri });

        try {
            const response = await this.makeRequest(ICAP_METHODS.REQMOD, httpRequest);
            return response;
        } catch (error) {
            this.logger.error('REQMOD request failed', { error: error.message });
            throw error;
        }
    }

    async respmod(httpResponse) {
        this.logger.info('Sending RESPMOD request', { statusCode: httpResponse.statusCode });

        try {
            const response = await this.makeRequest(ICAP_METHODS.RESPMOD, httpResponse);
            return response;
        } catch (error) {
            this.logger.error('RESPMOD request failed', { error: error.message });
            throw error;
        }
    }

    async options() {
        this.logger.info('Sending OPTIONS request');

        try {
            const response = await this.makeRequest(ICAP_METHODS.OPTIONS);
            return response;
        } catch (error) {
            this.logger.error('OPTIONS request failed', { error: error.message });
            throw error;
        }
    }

    async healthCheck() {
        try {
            const response = await this.options();
            const status = response.statusCode < 400 ? 'healthy' : 'unhealthy';

            const methods = response.headers.Methods ? 
                response.headers.Methods.split(',').map(m => m.trim()) : [];

            return {
                status,
                statusCode: response.statusCode,
                version: response.headers.Service,
                methods,
                istag: response.headers.ISTag
            };
        } catch (error) {
            return {
                status: 'unhealthy',
                error: error.message
            };
        }
    }

    close() {
        if (this.httpClient) {
            // Close any persistent connections
            this.httpClient.defaults.httpsAgent?.destroy();
            this.httpClient.defaults.httpAgent?.destroy();
        }
        this.logger.info('ICAP client closed');
    }
}

// CLI Interface
async function main() {
    const argv = yargs(hideBin(process.argv))
        .option('config', {
            alias: 'c',
            type: 'string',
            description: 'Configuration file path'
        })
        .option('host', {
            alias: 'h',
            type: 'string',
            default: '127.0.0.1',
            description: 'ICAP server host'
        })
        .option('port', {
            alias: 'p',
            type: 'number',
            default: 1344,
            description: 'ICAP server port'
        })
        .option('method', {
            alias: 'm',
            type: 'string',
            default: 'options',
            choices: ['reqmod', 'respmod', 'options'],
            description: 'ICAP method'
        })
        .option('verbose', {
            alias: 'v',
            type: 'boolean',
            default: false,
            description: 'Verbose logging'
        })
        .help()
        .argv;

    try {
        // Load configuration
        let config;
        if (argv.config) {
            config = IcapConfig.fromFile(argv.config);
        } else {
            config = new IcapConfig({
                host: argv.host,
                port: argv.port,
                loggingLevel: argv.verbose ? 'debug' : 'info',
                metricsEnabled: true
            });
        }

        // Create client
        const client = new IcapClient(config);

        // Execute method
        switch (argv.method) {
            case 'options':
                const optionsResponse = await client.options();
                console.log(`OPTIONS Response: ${optionsResponse.statusCode} ${optionsResponse.reason}`);
                console.log('Headers:', optionsResponse.headers);
                break;

            case 'reqmod':
                const httpRequest = new HttpRequest({
                    method: 'GET',
                    uri: '/',
                    version: 'HTTP/1.1',
                    headers: {
                        'Host': 'example.com',
                        'User-Agent': 'JS-Client'
                    }
                });

                const reqmodResponse = await client.reqmod(httpRequest);
                console.log(`REQMOD Response: ${reqmodResponse.statusCode} ${reqmodResponse.reason}`);
                break;

            case 'respmod':
                const httpResponse = new HttpResponse({
                    version: 'HTTP/1.1',
                    statusCode: 200,
                    reason: 'OK',
                    headers: {
                        'Content-Type': 'text/html'
                    },
                    body: Buffer.from('<html><body>Hello World</body></html>')
                });

                const respmodResponse = await client.respmod(httpResponse);
                console.log(`RESPMOD Response: ${respmodResponse.statusCode} ${respmodResponse.reason}`);
                break;
        }

        // Health check
        const health = await client.healthCheck();
        console.log('Health Check:', health);

        client.close();

    } catch (error) {
        console.error('Error:', error.message);
        process.exit(1);
    }
}

// Export classes for use as a module
module.exports = {
    IcapClient,
    IcapConfig,
    HttpRequest,
    HttpResponse,
    IcapResponse,
    IcapError,
    AuthenticationHandler,
    ClientMetrics,
    ICAP_METHODS,
    ICAP_RESPONSE_CODES,
    AUTH_METHODS
};

// Run CLI if this file is executed directly
if (require.main === module) {
    main().catch(console.error);
}
