#!/usr/bin/env python3
"""
G3ICAP Python Client Tests

Comprehensive test suite for the G3ICAP Python client.

Author: G3ICAP Team
License: Apache 2.0
Version: 1.0.0
"""

import asyncio
import pytest
import json
from unittest.mock import AsyncMock, MagicMock, patch
from icap_client import (
    IcapClient, IcapConfig, IcapMethod, IcapResponse, IcapError,
    HttpRequest, HttpResponse, AuthenticationMethod
)


class TestIcapConfig:
    """Test IcapConfig class."""
    
    def test_default_config(self):
        """Test default configuration."""
        config = IcapConfig()
        assert config.host == "127.0.0.1"
        assert config.port == 1344
        assert config.timeout == 30
        assert config.retries == 3
        assert config.connection_pool_size == 10
        assert config.keep_alive is True
        assert config.verify_ssl is True
        assert config.authentication is None
        assert config.logging_level == "INFO"
        assert config.metrics_enabled is True
    
    def test_custom_config(self):
        """Test custom configuration."""
        config = IcapConfig(
            host="example.com",
            port=8080,
            timeout=60,
            retries=5,
            authentication={"method": "basic", "username": "user", "password": "pass"}
        )
        assert config.host == "example.com"
        assert config.port == 8080
        assert config.timeout == 60
        assert config.retries == 5
        assert config.authentication["method"] == "basic"


class TestHttpRequest:
    """Test HttpRequest model."""
    
    def test_http_request_creation(self):
        """Test HTTP request creation."""
        request = HttpRequest(
            method="GET",
            uri="/",
            version="HTTP/1.1",
            headers={"Host": "example.com"},
            body=b"test body"
        )
        assert request.method == "GET"
        assert request.uri == "/"
        assert request.version == "HTTP/1.1"
        assert request.headers["Host"] == "example.com"
        assert request.body == b"test body"
    
    def test_http_request_defaults(self):
        """Test HTTP request defaults."""
        request = HttpRequest(method="POST", uri="/api")
        assert request.method == "POST"
        assert request.uri == "/api"
        assert request.version == "HTTP/1.1"
        assert request.headers == {}
        assert request.body is None


class TestHttpResponse:
    """Test HttpResponse model."""
    
    def test_http_response_creation(self):
        """Test HTTP response creation."""
        response = HttpResponse(
            version="HTTP/1.1",
            status_code=200,
            reason="OK",
            headers={"Content-Type": "text/html"},
            body=b"<html></html>"
        )
        assert response.version == "HTTP/1.1"
        assert response.status_code == 200
        assert response.reason == "OK"
        assert response.headers["Content-Type"] == "text/html"
        assert response.body == b"<html></html>"


class TestIcapClient:
    """Test IcapClient class."""
    
    @pytest.fixture
    def config(self):
        """Test configuration."""
        return IcapConfig(host="127.0.0.1", port=1344, timeout=5)
    
    @pytest.fixture
    def client(self, config):
        """Test client."""
        return IcapClient(config)
    
    def test_client_initialization(self, config):
        """Test client initialization."""
        client = IcapClient(config)
        assert client.config == config
        assert client.session is None
        assert client.auth_handler is None
        assert client.metrics["requests_total"] == 0
    
    def test_client_with_authentication(self):
        """Test client with authentication."""
        config = IcapConfig(
            authentication={
                "method": "basic",
                "username": "user",
                "password": "pass"
            }
        )
        client = IcapClient(config)
        assert client.auth_handler is not None
        assert client.auth_handler.method == AuthenticationMethod.BASIC
    
    @pytest.mark.asyncio
    async def test_connect(self, client):
        """Test client connection."""
        with patch('aiohttp.ClientSession') as mock_session:
            mock_session.return_value = AsyncMock()
            await client.connect()
            assert client.session is not None
            mock_session.assert_called_once()
    
    @pytest.mark.asyncio
    async def test_close(self, client):
        """Test client disconnection."""
        client.session = AsyncMock()
        await client.close()
        client.session.close.assert_called_once()
    
    def test_build_icap_url(self, client):
        """Test ICAP URL building."""
        reqmod_url = client._build_icap_url(IcapMethod.REQMOD)
        assert reqmod_url == "icap://127.0.0.1:1344/reqmod"
        
        respmod_url = client._build_icap_url(IcapMethod.RESPMOD)
        assert respmod_url == "icap://127.0.0.1:1344/respmod"
        
        options_url = client._build_icap_url(IcapMethod.OPTIONS)
        assert options_url == "icap://127.0.0.1:1344/options"
    
    def test_build_encapsulated_header(self, client):
        """Test encapsulated header building."""
        http_request = HttpRequest(method="GET", uri="/")
        header = client._build_encapsulated_header(http_request)
        assert header == "req-hdr=0, null-body=75"
        
        http_response = HttpResponse(version="HTTP/1.1", status_code=200, reason="OK")
        header = client._build_encapsulated_header(http_response)
        assert header == "res-hdr=0, null-body=120"
    
    def test_serialize_http_data(self, client):
        """Test HTTP data serialization."""
        http_request = HttpRequest(
            method="GET",
            uri="/",
            headers={"Host": "example.com"},
            body=b"test body"
        )
        
        serialized = client._serialize_http_data(http_request)
        assert b"GET / HTTP/1.1" in serialized
        assert b"Host: example.com" in serialized
        assert b"test body" in serialized
    
    def test_parse_icap_response(self, client):
        """Test ICAP response parsing."""
        response_text = """ICAP/1.0 204 No Content
ISTag: W3E4R7U9
Service: G3ICAP
Methods: REQMOD, RESPMOD, OPTIONS

"""
        
        response = client._parse_icap_response(response_text)
        assert response.version == "ICAP/1.0"
        assert response.status_code == 204
        assert response.reason == "No Content"
        assert response.headers["ISTag"] == "W3E4R7U9"
        assert response.headers["Service"] == "G3ICAP"
        assert response.headers["Methods"] == "REQMOD, RESPMOD, OPTIONS"
    
    @pytest.mark.asyncio
    async def test_make_request_success(self, client):
        """Test successful request."""
        mock_response = AsyncMock()
        mock_response.status = 204
        mock_response.text = AsyncMock(return_value="ICAP/1.0 204 No Content\r\n\r\n")
        
        client.session = AsyncMock()
        client.session.request.return_value.__aenter__.return_value = mock_response
        
        http_request = HttpRequest(method="GET", uri="/")
        response = await client._make_request(IcapMethod.REQMOD, http_request)
        
        assert response.status_code == 204
        assert response.reason == "No Content"
        assert client.metrics["requests_total"] == 1
        assert client.metrics["requests_success"] == 1
    
    @pytest.mark.asyncio
    async def test_make_request_retry(self, client):
        """Test request retry logic."""
        client.config.retries = 2
        
        mock_response = AsyncMock()
        mock_response.status = 500
        mock_response.text = AsyncMock(return_value="ICAP/1.0 500 Internal Server Error\r\n\r\n")
        
        client.session = AsyncMock()
        client.session.request.return_value.__aenter__.return_value = mock_response
        
        http_request = HttpRequest(method="GET", uri="/")
        
        with pytest.raises(IcapError):
            await client._make_request(IcapMethod.REQMOD, http_request)
        
        # Should have retried 3 times (1 initial + 2 retries)
        assert client.session.request.call_count == 3
        assert client.metrics["requests_total"] == 3
        assert client.metrics["requests_failed"] == 3
    
    @pytest.mark.asyncio
    async def test_reqmod(self, client):
        """Test REQMOD method."""
        mock_response = AsyncMock()
        mock_response.status = 204
        mock_response.text = AsyncMock(return_value="ICAP/1.0 204 No Content\r\n\r\n")
        
        client.session = AsyncMock()
        client.session.request.return_value.__aenter__.return_value = mock_response
        
        http_request = HttpRequest(method="GET", uri="/")
        response = await client.reqmod(http_request)
        
        assert response.status_code == 204
        client.session.request.assert_called_once()
    
    @pytest.mark.asyncio
    async def test_respmod(self, client):
        """Test RESPMOD method."""
        mock_response = AsyncMock()
        mock_response.status = 204
        mock_response.text = AsyncMock(return_value="ICAP/1.0 204 No Content\r\n\r\n")
        
        client.session = AsyncMock()
        client.session.request.return_value.__aenter__.return_value = mock_response
        
        http_response = HttpResponse(version="HTTP/1.1", status_code=200, reason="OK")
        response = await client.respmod(http_response)
        
        assert response.status_code == 204
        client.session.request.assert_called_once()
    
    @pytest.mark.asyncio
    async def test_options(self, client):
        """Test OPTIONS method."""
        mock_response = AsyncMock()
        mock_response.status = 200
        mock_response.text = AsyncMock(return_value="""ICAP/1.0 200 OK
ISTag: W3E4R7U9
Service: G3ICAP
Methods: REQMOD, RESPMOD, OPTIONS

""")
        
        client.session = AsyncMock()
        client.session.request.return_value.__aenter__.return_value = mock_response
        
        response = await client.options()
        
        assert response.status_code == 200
        assert response.headers["ISTag"] == "W3E4R7U9"
        client.session.request.assert_called_once()
    
    @pytest.mark.asyncio
    async def test_health_check_success(self, client):
        """Test successful health check."""
        mock_response = AsyncMock()
        mock_response.status = 200
        mock_response.text = AsyncMock(return_value="""ICAP/1.0 200 OK
ISTag: W3E4R7U9
Service: G3ICAP
Methods: REQMOD, RESPMOD, OPTIONS

""")
        
        client.session = AsyncMock()
        client.session.request.return_value.__aenter__.return_value = mock_response
        
        health = await client.health_check()
        
        assert health["status"] == "healthy"
        assert health["status_code"] == 200
        assert health["version"] == "G3ICAP"
        assert "REQMOD" in health["methods"]
    
    @pytest.mark.asyncio
    async def test_health_check_failure(self, client):
        """Test failed health check."""
        client.session = AsyncMock()
        client.session.request.side_effect = Exception("Connection failed")
        
        health = await client.health_check()
        
        assert health["status"] == "unhealthy"
        assert "error" in health
    
    def test_get_metrics(self, client):
        """Test metrics retrieval."""
        client.metrics["requests_total"] = 100
        client.metrics["requests_success"] = 95
        client.metrics["requests_failed"] = 5
        
        metrics = client.get_metrics()
        
        assert metrics["requests_total"] == 100
        assert metrics["requests_success"] == 95
        assert metrics["requests_failed"] == 5
        assert "connection_pool_size" in metrics
        assert "timeout" in metrics


class TestAuthenticationHandler:
    """Test AuthenticationHandler class."""
    
    def test_none_authentication(self):
        """Test no authentication."""
        from icap_client import AuthenticationHandler
        handler = AuthenticationHandler(AuthenticationMethod.NONE, {})
        headers = handler.get_headers()
        assert headers == {}
    
    def test_basic_authentication(self):
        """Test basic authentication."""
        from icap_client import AuthenticationHandler
        handler = AuthenticationHandler(
            AuthenticationMethod.BASIC,
            {"username": "user", "password": "pass"}
        )
        headers = handler.get_headers()
        assert "Authorization" in headers
        assert headers["Authorization"].startswith("Basic ")
    
    def test_bearer_authentication(self):
        """Test bearer authentication."""
        from icap_client import AuthenticationHandler
        handler = AuthenticationHandler(
            AuthenticationMethod.BEARER,
            {"token": "test-token"}
        )
        headers = handler.get_headers()
        assert headers["Authorization"] == "Bearer test-token"
    
    def test_api_key_authentication(self):
        """Test API key authentication."""
        from icap_client import AuthenticationHandler
        handler = AuthenticationHandler(
            AuthenticationMethod.API_KEY,
            {"api_key": "test-key", "header_name": "X-API-Key"}
        )
        headers = handler.get_headers()
        assert headers["X-API-Key"] == "test-key"


@pytest.mark.asyncio
async def test_context_manager():
    """Test client as context manager."""
    config = IcapConfig(host="127.0.0.1", port=1344)
    
    with patch('aiohttp.ClientSession') as mock_session:
        mock_session.return_value = AsyncMock()
        
        async with IcapClient(config) as client:
            assert client.session is not None
        
        # Session should be closed
        mock_session.return_value.close.assert_called_once()


@pytest.mark.asyncio
async def test_integration_example():
    """Test integration example."""
    config = IcapConfig(host="127.0.0.1", port=1344, timeout=5)
    
    with patch('aiohttp.ClientSession') as mock_session:
        mock_response = AsyncMock()
        mock_response.status = 204
        mock_response.text = AsyncMock(return_value="ICAP/1.0 204 No Content\r\n\r\n")
        
        mock_session.return_value = AsyncMock()
        mock_session.return_value.request.return_value.__aenter__.return_value = mock_response
        
        async with IcapClient(config) as client:
            # Test REQMOD
            http_request = HttpRequest(
                method="GET",
                uri="/",
                headers={"Host": "example.com"}
            )
            response = await client.reqmod(http_request)
            assert response.status_code == 204
            
            # Test RESPMOD
            http_response = HttpResponse(
                version="HTTP/1.1",
                status_code=200,
                reason="OK",
                headers={"Content-Type": "text/html"},
                body=b"<html>Hello</html>"
            )
            response = await client.respmod(http_response)
            assert response.status_code == 204
            
            # Test OPTIONS
            response = await client.options()
            assert response.status_code == 204


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
