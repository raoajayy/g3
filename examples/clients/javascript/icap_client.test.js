const {
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
} = require('./icap_client');

// Mock axios for testing
jest.mock('axios');
const axios = require('axios');

describe('IcapClient', () => {
    let config;
    let client;

    beforeEach(() => {
        config = new IcapConfig({
            host: '127.0.0.1',
            port: 1344,
            timeout: 1000,
            retries: 0,
            loggingLevel: 'error',
            metricsEnabled: false
        });
        client = new IcapClient(config);
    });

    afterEach(() => {
        if (client) {
            client.close();
        }
    });

    describe('constructor', () => {
        test('should create client with default config', () => {
            const defaultConfig = new IcapConfig();
            const defaultClient = new IcapClient(defaultConfig);
            expect(defaultClient).toBeDefined();
            expect(defaultClient.config.host).toBe('127.0.0.1');
            expect(defaultClient.config.port).toBe(1344);
            defaultClient.close();
        });

        test('should create client with custom config', () => {
            expect(client).toBeDefined();
            expect(client.config.host).toBe('127.0.0.1');
            expect(client.config.port).toBe(1344);
        });

        test('should create client with authentication', () => {
            const authConfig = new IcapConfig({
                authentication: {
                    method: 'basic',
                    username: 'testuser',
                    password: 'testpass'
                }
            });
            const authClient = new IcapClient(authConfig);
            expect(authClient.authHandler).toBeDefined();
            expect(authClient.authHandler.method).toBe('basic');
            authClient.close();
        });
    });

    describe('buildICAPURL', () => {
        test('should build REQMOD URL', () => {
            const url = client.buildICAPURL(ICAP_METHODS.REQMOD);
            expect(url).toBe('icap://127.0.0.1:1344/reqmod');
        });

        test('should build RESPMOD URL', () => {
            const url = client.buildICAPURL(ICAP_METHODS.RESPMOD);
            expect(url).toBe('icap://127.0.0.1:1344/respmod');
        });

        test('should build OPTIONS URL', () => {
            const url = client.buildICAPURL(ICAP_METHODS.OPTIONS);
            expect(url).toBe('icap://127.0.0.1:1344/options');
        });

        test('should throw error for unsupported method', () => {
            expect(() => {
                client.buildICAPURL('INVALID');
            }).toThrow('Unsupported ICAP method: INVALID');
        });
    });

    describe('buildEncapsulatedHeader', () => {
        test('should build header for HTTP request', () => {
            const httpRequest = new HttpRequest();
            const header = client.buildEncapsulatedHeader(httpRequest);
            expect(header).toBe('req-hdr=0, null-body=75');
        });

        test('should build header for HTTP response', () => {
            const httpResponse = new HttpResponse();
            const header = client.buildEncapsulatedHeader(httpResponse);
            expect(header).toBe('res-hdr=0, null-body=120');
        });

        test('should build header for null data', () => {
            const header = client.buildEncapsulatedHeader(null);
            expect(header).toBe('null-body=0');
        });
    });

    describe('serializeHTTPData', () => {
        test('should serialize HTTP request', () => {
            const httpRequest = new HttpRequest({
                method: 'GET',
                uri: '/',
                version: 'HTTP/1.1',
                headers: {
                    'Host': 'example.com',
                    'User-Agent': 'JS-Client'
                },
                body: Buffer.from('test body')
            });

            const data = client.serializeHTTPData(httpRequest);
            const expected = 'GET / HTTP/1.1\r\nHost: example.com\r\nUser-Agent: JS-Client\r\n\r\ntest body';
            expect(data.toString()).toBe(expected);
        });

        test('should serialize HTTP response', () => {
            const httpResponse = new HttpResponse({
                version: 'HTTP/1.1',
                statusCode: 200,
                reason: 'OK',
                headers: {
                    'Content-Type': 'text/html'
                },
                body: Buffer.from('<html>test</html>')
            });

            const data = client.serializeHTTPData(httpResponse);
            const expected = 'HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<html>test</html>';
            expect(data.toString()).toBe(expected);
        });
    });

    describe('parseICAPResponse', () => {
        test('should parse ICAP response', () => {
            const responseText = `ICAP/1.0 200 OK
Server: G3ICAP/1.0.0
ISTag: "test-istag"
Methods: REQMOD, RESPMOD, OPTIONS
Service: G3ICAP Content Filter

HTTP/1.1 200 OK
Content-Type: text/html
Content-Length: 13

Hello World!`;

            const response = client.parseICAPResponse(responseText);

            expect(response.version).toBe('ICAP/1.0');
            expect(response.statusCode).toBe(200);
            expect(response.reason).toBe('OK');
            expect(response.headers.Server).toBe('G3ICAP/1.0.0');
            expect(response.headers.ISTag).toBe('"test-istag"');
            expect(response.headers.Methods).toBe('REQMOD, RESPMOD, OPTIONS');
            expect(response.headers.Service).toBe('G3ICAP Content Filter');
        });
    });

    describe('makeRequest', () => {
        beforeEach(() => {
            axios.create.mockReturnValue({
                request: jest.fn()
            });
        });

        test('should make successful request', async () => {
            const mockResponse = {
                status: 200,
                data: `ICAP/1.0 200 OK
Server: G3ICAP/1.0.0

`
            };

            client.httpClient.request.mockResolvedValue(mockResponse);

            const response = await client.makeRequest(ICAP_METHODS.OPTIONS);

            expect(response).toBeDefined();
            expect(response.statusCode).toBe(200);
            expect(client.httpClient.request).toHaveBeenCalledWith({
                method: ICAP_METHODS.OPTIONS,
                url: 'icap://127.0.0.1:1344/options',
                headers: expect.objectContaining({
                    'Host': '127.0.0.1:1344',
                    'User-Agent': 'G3ICAP-JS-Client/1.0.0',
                    'Allow': '204'
                }),
                data: Buffer.alloc(0)
            });
        });

        test('should retry on failure', async () => {
            const mockError = new Error('Network error');
            client.httpClient.request.mockRejectedValue(mockError);

            await expect(client.makeRequest(ICAP_METHODS.OPTIONS)).rejects.toThrow('All retry attempts failed');
            expect(client.httpClient.request).toHaveBeenCalledTimes(1); // No retries in test config
        });
    });

    describe('reqmod', () => {
        test('should send REQMOD request', async () => {
            const mockResponse = {
                status: 200,
                data: `ICAP/1.0 200 OK
Server: G3ICAP/1.0.0

`
            };

            client.httpClient.request.mockResolvedValue(mockResponse);

            const httpRequest = new HttpRequest({
                method: 'GET',
                uri: '/',
                version: 'HTTP/1.1',
                headers: { 'Host': 'example.com' }
            });

            const response = await client.reqmod(httpRequest);

            expect(response).toBeDefined();
            expect(response.statusCode).toBe(200);
        });
    });

    describe('respmod', () => {
        test('should send RESPMOD request', async () => {
            const mockResponse = {
                status: 200,
                data: `ICAP/1.0 200 OK
Server: G3ICAP/1.0.0

`
            };

            client.httpClient.request.mockResolvedValue(mockResponse);

            const httpResponse = new HttpResponse({
                version: 'HTTP/1.1',
                statusCode: 200,
                reason: 'OK',
                headers: { 'Content-Type': 'text/html' }
            });

            const response = await client.respmod(httpResponse);

            expect(response).toBeDefined();
            expect(response.statusCode).toBe(200);
        });
    });

    describe('options', () => {
        test('should send OPTIONS request', async () => {
            const mockResponse = {
                status: 200,
                data: `ICAP/1.0 200 OK
Server: G3ICAP/1.0.0

`
            };

            client.httpClient.request.mockResolvedValue(mockResponse);

            const response = await client.options();

            expect(response).toBeDefined();
            expect(response.statusCode).toBe(200);
        });
    });

    describe('healthCheck', () => {
        test('should return healthy status', async () => {
            const mockResponse = {
                status: 200,
                data: `ICAP/1.0 200 OK
Server: G3ICAP/1.0.0
Methods: REQMOD, RESPMOD, OPTIONS
Service: G3ICAP Content Filter
ISTag: "test-istag"

`
            };

            client.httpClient.request.mockResolvedValue(mockResponse);

            const health = await client.healthCheck();

            expect(health.status).toBe('healthy');
            expect(health.statusCode).toBe(200);
            expect(health.methods).toEqual(['REQMOD', 'RESPMOD', 'OPTIONS']);
            expect(health.version).toBe('G3ICAP Content Filter');
            expect(health.istag).toBe('"test-istag"');
        });

        test('should return unhealthy status on error', async () => {
            const mockError = new Error('Network error');
            client.httpClient.request.mockRejectedValue(mockError);

            const health = await client.healthCheck();

            expect(health.status).toBe('unhealthy');
            expect(health.error).toBe('Network error');
        });
    });
});

describe('AuthenticationHandler', () => {
    test('should handle no authentication', () => {
        const handler = new AuthenticationHandler(AUTH_METHODS.NONE);
        const headers = handler.getHeaders();
        expect(headers).toEqual({});
    });

    test('should handle basic authentication', () => {
        const handler = new AuthenticationHandler(AUTH_METHODS.BASIC, {
            username: 'testuser',
            password: 'testpass'
        });
        const headers = handler.getHeaders();
        expect(headers.Authorization).toBe('Basic dGVzdHVzZXI6dGVzdHBhc3M=');
    });

    test('should handle bearer authentication', () => {
        const handler = new AuthenticationHandler(AUTH_METHODS.BEARER, {
            token: 'test-token'
        });
        const headers = handler.getHeaders();
        expect(headers.Authorization).toBe('Bearer test-token');
    });

    test('should handle JWT authentication', () => {
        const handler = new AuthenticationHandler(AUTH_METHODS.JWT, {
            jwt_token: 'jwt-test-token'
        });
        const headers = handler.getHeaders();
        expect(headers.Authorization).toBe('Bearer jwt-test-token');
    });

    test('should handle API key authentication', () => {
        const handler = new AuthenticationHandler(AUTH_METHODS.API_KEY, {
            api_key: 'test-api-key',
            header_name: 'X-API-Key'
        });
        const headers = handler.getHeaders();
        expect(headers['X-API-Key']).toBe('test-api-key');
    });

    test('should handle API key authentication with default header', () => {
        const handler = new AuthenticationHandler(AUTH_METHODS.API_KEY, {
            api_key: 'test-api-key'
        });
        const headers = handler.getHeaders();
        expect(headers['X-API-Key']).toBe('test-api-key');
    });
});

describe('IcapConfig', () => {
    test('should create config with defaults', () => {
        const config = new IcapConfig();
        expect(config.host).toBe('127.0.0.1');
        expect(config.port).toBe(1344);
        expect(config.timeout).toBe(30000);
        expect(config.retries).toBe(3);
        expect(config.keepAlive).toBe(true);
        expect(config.verifySSL).toBe(true);
        expect(config.loggingLevel).toBe('info');
        expect(config.metricsEnabled).toBe(true);
    });

    test('should create config with custom values', () => {
        const config = new IcapConfig({
            host: 'example.com',
            port: 8080,
            timeout: 60000,
            retries: 5,
            keepAlive: false,
            verifySSL: false,
            loggingLevel: 'debug',
            metricsEnabled: false
        });

        expect(config.host).toBe('example.com');
        expect(config.port).toBe(8080);
        expect(config.timeout).toBe(60000);
        expect(config.retries).toBe(5);
        expect(config.keepAlive).toBe(false);
        expect(config.verifySSL).toBe(false);
        expect(config.loggingLevel).toBe('debug');
        expect(config.metricsEnabled).toBe(false);
    });

    test('should load config from file', () => {
        // Mock fs.readFileSync
        const fs = require('fs');
        const originalReadFileSync = fs.readFileSync;
        fs.readFileSync = jest.fn().mockReturnValue(`
host: example.com
port: 8080
timeout: 60000
`);

        const config = IcapConfig.fromFile('test.yaml');
        expect(config.host).toBe('example.com');
        expect(config.port).toBe(8080);
        expect(config.timeout).toBe(60000);

        // Restore original function
        fs.readFileSync = originalReadFileSync;
    });

    test('should throw error for invalid config file', () => {
        const fs = require('fs');
        const originalReadFileSync = fs.readFileSync;
        fs.readFileSync = jest.fn().mockImplementation(() => {
            throw new Error('File not found');
        });

        expect(() => {
            IcapConfig.fromFile('nonexistent.yaml');
        }).toThrow('Failed to load config from nonexistent.yaml: File not found');

        // Restore original function
        fs.readFileSync = originalReadFileSync;
    });
});

describe('HttpRequest', () => {
    test('should create request with defaults', () => {
        const request = new HttpRequest();
        expect(request.method).toBe('GET');
        expect(request.uri).toBe('/');
        expect(request.version).toBe('HTTP/1.1');
        expect(request.headers).toEqual({});
        expect(request.body).toEqual(Buffer.alloc(0));
    });

    test('should create request with custom values', () => {
        const request = new HttpRequest({
            method: 'POST',
            uri: '/api/test',
            version: 'HTTP/2.0',
            headers: { 'Content-Type': 'application/json' },
            body: Buffer.from('{"test": "data"}')
        });

        expect(request.method).toBe('POST');
        expect(request.uri).toBe('/api/test');
        expect(request.version).toBe('HTTP/2.0');
        expect(request.headers).toEqual({ 'Content-Type': 'application/json' });
        expect(request.body).toEqual(Buffer.from('{"test": "data"}'));
    });
});

describe('HttpResponse', () => {
    test('should create response with defaults', () => {
        const response = new HttpResponse();
        expect(response.version).toBe('HTTP/1.1');
        expect(response.statusCode).toBe(200);
        expect(response.reason).toBe('OK');
        expect(response.headers).toEqual({});
        expect(response.body).toEqual(Buffer.alloc(0));
    });

    test('should create response with custom values', () => {
        const response = new HttpResponse({
            version: 'HTTP/2.0',
            statusCode: 404,
            reason: 'Not Found',
            headers: { 'Content-Type': 'text/html' },
            body: Buffer.from('<html>Not Found</html>')
        });

        expect(response.version).toBe('HTTP/2.0');
        expect(response.statusCode).toBe(404);
        expect(response.reason).toBe('Not Found');
        expect(response.headers).toEqual({ 'Content-Type': 'text/html' });
        expect(response.body).toEqual(Buffer.from('<html>Not Found</html>'));
    });
});

describe('IcapResponse', () => {
    test('should create response with defaults', () => {
        const response = new IcapResponse();
        expect(response.version).toBe('ICAP/1.0');
        expect(response.statusCode).toBe(200);
        expect(response.reason).toBe('OK');
        expect(response.headers).toEqual({});
        expect(response.body).toEqual(Buffer.alloc(0));
        expect(response.httpRequest).toBeNull();
        expect(response.httpResponse).toBeNull();
    });

    test('should create response with custom values', () => {
        const httpRequest = new HttpRequest();
        const httpResponse = new HttpResponse();
        const response = new IcapResponse({
            version: 'ICAP/1.0',
            statusCode: 204,
            reason: 'No Content',
            headers: { 'Server': 'G3ICAP/1.0.0' },
            body: Buffer.from('test'),
            httpRequest,
            httpResponse
        });

        expect(response.version).toBe('ICAP/1.0');
        expect(response.statusCode).toBe(204);
        expect(response.reason).toBe('No Content');
        expect(response.headers).toEqual({ 'Server': 'G3ICAP/1.0.0' });
        expect(response.body).toEqual(Buffer.from('test'));
        expect(response.httpRequest).toBe(httpRequest);
        expect(response.httpResponse).toBe(httpResponse);
    });
});

describe('IcapError', () => {
    test('should create error with message only', () => {
        const error = new IcapError('Test error');
        expect(error.message).toBe('Test error');
        expect(error.name).toBe('IcapError');
        expect(error.code).toBe(0);
        expect(error.cause).toBeNull();
    });

    test('should create error with message and code', () => {
        const error = new IcapError('Test error', 500);
        expect(error.message).toBe('Test error');
        expect(error.code).toBe(500);
        expect(error.cause).toBeNull();
    });

    test('should create error with message, code, and cause', () => {
        const cause = new Error('Original error');
        const error = new IcapError('Test error', 500, cause);
        expect(error.message).toBe('Test error');
        expect(error.code).toBe(500);
        expect(error.cause).toBe(cause);
    });
});

describe('ClientMetrics', () => {
    test('should create metrics', () => {
        const metrics = new ClientMetrics();
        expect(metrics.requestsTotal).toBeDefined();
        expect(metrics.requestsSuccess).toBeDefined();
        expect(metrics.requestsFailed).toBeDefined();
        expect(metrics.responseTime).toBeDefined();
        expect(metrics.connectionPool).toBeDefined();
    });
});

describe('Constants', () => {
    test('should have correct ICAP methods', () => {
        expect(ICAP_METHODS.REQMOD).toBe('REQMOD');
        expect(ICAP_METHODS.RESPMOD).toBe('RESPMOD');
        expect(ICAP_METHODS.OPTIONS).toBe('OPTIONS');
    });

    test('should have correct ICAP response codes', () => {
        expect(ICAP_RESPONSE_CODES.CONTINUE).toBe(100);
        expect(ICAP_RESPONSE_CODES.OK).toBe(200);
        expect(ICAP_RESPONSE_CODES.NO_CONTENT).toBe(204);
        expect(ICAP_RESPONSE_CODES.BAD_REQUEST).toBe(400);
        expect(ICAP_RESPONSE_CODES.NOT_FOUND).toBe(404);
        expect(ICAP_RESPONSE_CODES.INTERNAL_SERVER_ERROR).toBe(500);
    });

    test('should have correct authentication methods', () => {
        expect(AUTH_METHODS.NONE).toBe('none');
        expect(AUTH_METHODS.BASIC).toBe('basic');
        expect(AUTH_METHODS.BEARER).toBe('bearer');
        expect(AUTH_METHODS.JWT).toBe('jwt');
        expect(AUTH_METHODS.API_KEY).toBe('api_key');
    });
});
