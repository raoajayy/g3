#!/usr/bin/env python3
"""
G3ICAP Python Client

A comprehensive Python client for interacting with G3ICAP servers,
supporting REQMOD, RESPMOD, and OPTIONS methods with full authentication
and error handling capabilities.

Author: G3ICAP Team
License: Apache 2.0
Version: 1.0.0
"""

import asyncio
import json
import logging
import time
from dataclasses import dataclass
from enum import Enum
from typing import Dict, List, Optional, Union, Any
from urllib.parse import urlparse, urlunparse

import aiohttp
import httpx
import structlog
import yaml
from cryptography.fernet import Fernet
from pydantic import BaseModel, Field


# Configure structured logging
structlog.configure(
    processors=[
        structlog.stdlib.filter_by_level,
        structlog.stdlib.add_logger_name,
        structlog.stdlib.add_log_level,
        structlog.stdlib.PositionalArgumentsFormatter(),
        structlog.processors.TimeStamper(fmt="iso"),
        structlog.processors.StackInfoRenderer(),
        structlog.processors.format_exc_info,
        structlog.processors.UnicodeDecoder(),
        structlog.processors.JSONRenderer()
    ],
    context_class=dict,
    logger_factory=structlog.stdlib.LoggerFactory(),
    wrapper_class=structlog.stdlib.BoundLogger,
    cache_logger_on_first_use=True,
)

logger = structlog.get_logger(__name__)


class IcapMethod(Enum):
    """ICAP method enumeration."""
    REQMOD = "REQMOD"
    RESPMOD = "RESPMOD"
    OPTIONS = "OPTIONS"


class IcapResponseCode(Enum):
    """ICAP response code enumeration."""
    CONTINUE = 100
    OK = 200
    NO_CONTENT = 204
    BAD_REQUEST = 400
    NOT_FOUND = 404
    METHOD_NOT_ALLOWED = 405
    REQUEST_TIMEOUT = 408
    REQUEST_ENTITY_TOO_LARGE = 413
    INTERNAL_SERVER_ERROR = 500
    NOT_IMPLEMENTED = 501
    BAD_GATEWAY = 502
    SERVICE_UNAVAILABLE = 503
    ICAP_VERSION_NOT_SUPPORTED = 505


class AuthenticationMethod(Enum):
    """Authentication method enumeration."""
    NONE = "none"
    BASIC = "basic"
    BEARER = "bearer"
    JWT = "jwt"
    API_KEY = "api_key"


@dataclass
class IcapConfig:
    """ICAP client configuration."""
    host: str = "127.0.0.1"
    port: int = 1344
    timeout: int = 30
    retries: int = 3
    retry_delay: float = 1.0
    max_retry_delay: float = 60.0
    backoff_factor: float = 2.0
    connection_pool_size: int = 10
    keep_alive: bool = True
    verify_ssl: bool = True
    authentication: Optional[Dict[str, Any]] = None
    logging_level: str = "INFO"
    metrics_enabled: bool = True


class HttpRequest(BaseModel):
    """HTTP request model."""
    method: str = Field(..., description="HTTP method")
    uri: str = Field(..., description="Request URI")
    version: str = Field(default="HTTP/1.1", description="HTTP version")
    headers: Dict[str, str] = Field(default_factory=dict, description="HTTP headers")
    body: Optional[bytes] = Field(default=None, description="Request body")


class HttpResponse(BaseModel):
    """HTTP response model."""
    version: str = Field(..., description="HTTP version")
    status_code: int = Field(..., description="Status code")
    reason: str = Field(..., description="Status reason")
    headers: Dict[str, str] = Field(default_factory=dict, description="HTTP headers")
    body: Optional[bytes] = Field(default=None, description="Response body")


class IcapResponse(BaseModel):
    """ICAP response model."""
    version: str = Field(..., description="ICAP version")
    status_code: int = Field(..., description="Status code")
    reason: str = Field(..., description="Status reason")
    headers: Dict[str, str] = Field(default_factory=dict, description="ICAP headers")
    body: Optional[bytes] = Field(default=None, description="Response body")
    http_request: Optional[HttpRequest] = Field(default=None, description="Modified HTTP request")
    http_response: Optional[HttpResponse] = Field(default=None, description="Modified HTTP response")


class IcapError(Exception):
    """Base ICAP client error."""
    pass


class IcapConnectionError(IcapError):
    """ICAP connection error."""
    pass


class IcapAuthenticationError(IcapError):
    """ICAP authentication error."""
    pass


class IcapTimeoutError(IcapError):
    """ICAP timeout error."""
    pass


class IcapProtocolError(IcapError):
    """ICAP protocol error."""
    pass


class IcapServerError(IcapError):
    """ICAP server error."""
    pass


class AuthenticationHandler:
    """Authentication handler for different auth methods."""
    
    def __init__(self, method: AuthenticationMethod, config: Dict[str, Any]):
        self.method = method
        self.config = config
    
    def get_headers(self) -> Dict[str, str]:
        """Get authentication headers."""
        if self.method == AuthenticationMethod.NONE:
            return {}
        elif self.method == AuthenticationMethod.BASIC:
            import base64
            username = self.config.get("username", "")
            password = self.config.get("password", "")
            credentials = base64.b64encode(f"{username}:{password}".encode()).decode()
            return {"Authorization": f"Basic {credentials}"}
        elif self.method == AuthenticationMethod.BEARER:
            token = self.config.get("token", "")
            return {"Authorization": f"Bearer {token}"}
        elif self.method == AuthenticationMethod.JWT:
            token = self.config.get("jwt_token", "")
            return {"Authorization": f"Bearer {token}"}
        elif self.method == AuthenticationMethod.API_KEY:
            api_key = self.config.get("api_key", "")
            header_name = self.config.get("header_name", "X-API-Key")
            return {header_name: api_key}
        else:
            return {}


class IcapClient:
    """G3ICAP Python client."""
    
    def __init__(self, config: IcapConfig):
        self.config = config
        self.logger = logger.bind(component="icap_client")
        self.session: Optional[aiohttp.ClientSession] = None
        self.auth_handler: Optional[AuthenticationHandler] = None
        self.metrics = {
            "requests_total": 0,
            "requests_success": 0,
            "requests_failed": 0,
            "response_time_total": 0.0,
            "response_time_avg": 0.0,
        }
        
        # Setup authentication
        if config.authentication:
            auth_method = AuthenticationMethod(config.authentication.get("method", "none"))
            self.auth_handler = AuthenticationHandler(auth_method, config.authentication)
        
        # Setup logging
        logging.basicConfig(level=getattr(logging, config.logging_level.upper()))
    
    async def __aenter__(self):
        """Async context manager entry."""
        await self.connect()
        return self
    
    async def __aexit__(self, exc_type, exc_val, exc_tb):
        """Async context manager exit."""
        await self.close()
    
    async def connect(self):
        """Connect to ICAP server."""
        try:
            connector = aiohttp.TCPConnector(
                limit=self.config.connection_pool_size,
                keepalive_timeout=self.config.timeout if self.config.keep_alive else 0,
                verify_ssl=self.config.verify_ssl,
            )
            
            timeout = aiohttp.ClientTimeout(total=self.config.timeout)
            
            self.session = aiohttp.ClientSession(
                connector=connector,
                timeout=timeout,
                headers=self._get_default_headers(),
            )
            
            self.logger.info("Connected to ICAP server", 
                           host=self.config.host, 
                           port=self.config.port)
            
        except Exception as e:
            self.logger.error("Failed to connect to ICAP server", error=str(e))
            raise IcapConnectionError(f"Failed to connect: {e}")
    
    async def close(self):
        """Close connection to ICAP server."""
        if self.session:
            await self.session.close()
            self.logger.info("Disconnected from ICAP server")
    
    def _get_default_headers(self) -> Dict[str, str]:
        """Get default headers."""
        headers = {
            "Host": f"{self.config.host}:{self.config.port}",
            "User-Agent": "G3ICAP-Python-Client/1.0.0",
        }
        
        if self.auth_handler:
            headers.update(self.auth_handler.get_headers())
        
        return headers
    
    def _build_icap_url(self, method: IcapMethod) -> str:
        """Build ICAP URL for method."""
        method_path = {
            IcapMethod.REQMOD: "/reqmod",
            IcapMethod.RESPMOD: "/respmod",
            IcapMethod.OPTIONS: "/options",
        }
        
        return f"icap://{self.config.host}:{self.config.port}{method_path[method]}"
    
    def _build_encapsulated_header(self, http_data: Union[HttpRequest, HttpResponse]) -> str:
        """Build Encapsulated header for ICAP request."""
        if isinstance(http_data, HttpRequest):
            return "req-hdr=0, null-body=75"
        elif isinstance(http_data, HttpResponse):
            return "res-hdr=0, null-body=120"
        else:
            return "null-body=0"
    
    def _serialize_http_data(self, http_data: Union[HttpRequest, HttpResponse]) -> bytes:
        """Serialize HTTP data for ICAP body."""
        lines = []
        
        if isinstance(http_data, HttpRequest):
            # HTTP request
            lines.append(f"{http_data.method} {http_data.uri} {http_data.version}")
        elif isinstance(http_data, HttpResponse):
            # HTTP response
            lines.append(f"{http_data.version} {http_data.status_code} {http_data.reason}")
        
        # Add headers
        for name, value in http_data.headers.items():
            lines.append(f"{name}: {value}")
        
        # Add empty line
        lines.append("")
        
        # Add body if present
        if http_data.body:
            lines.append(http_data.body.decode('utf-8', errors='ignore'))
        
        return "\r\n".join(lines).encode('utf-8')
    
    def _parse_icap_response(self, response_text: str) -> IcapResponse:
        """Parse ICAP response."""
        lines = response_text.split('\r\n')
        
        # Parse status line
        status_line = lines[0]
        parts = status_line.split(' ', 2)
        version = parts[0]
        status_code = int(parts[1])
        reason = parts[2] if len(parts) > 2 else ""
        
        # Parse headers
        headers = {}
        body_start = 0
        
        for i, line in enumerate(lines[1:], 1):
            if line == "":
                body_start = i + 1
                break
            
            if ':' in line:
                name, value = line.split(':', 1)
                headers[name.strip()] = value.strip()
        
        # Parse body
        body = None
        if body_start < len(lines):
            body_text = '\r\n'.join(lines[body_start:])
            if body_text.strip():
                body = body_text.encode('utf-8')
        
        return IcapResponse(
            version=version,
            status_code=status_code,
            reason=reason,
            headers=headers,
            body=body
        )
    
    async def _make_request(self, method: IcapMethod, http_data: Optional[Union[HttpRequest, HttpResponse]] = None) -> IcapResponse:
        """Make ICAP request with retry logic."""
        url = self._build_icap_url(method)
        
        # Build headers
        headers = self._get_default_headers()
        headers["Allow"] = "204"
        
        if http_data:
            headers["Encapsulated"] = self._build_encapsulated_header(http_data)
        
        # Build body
        body = None
        if http_data:
            body = self._serialize_http_data(http_data)
        
        # Retry logic
        last_exception = None
        
        for attempt in range(self.config.retries + 1):
            try:
                start_time = time.time()
                
                async with self.session.request(
                    method=method.value,
                    url=url,
                    headers=headers,
                    data=body
                ) as response:
                    response_text = await response.text()
                    response_time = time.time() - start_time
                    
                    # Update metrics
                    self.metrics["requests_total"] += 1
                    self.metrics["response_time_total"] += response_time
                    self.metrics["response_time_avg"] = (
                        self.metrics["response_time_total"] / self.metrics["requests_total"]
                    )
                    
                    if response.status < 400:
                        self.metrics["requests_success"] += 1
                    else:
                        self.metrics["requests_failed"] += 1
                    
                    # Parse response
                    icap_response = self._parse_icap_response(response_text)
                    
                    self.logger.info("ICAP request completed",
                                   method=method.value,
                                   status_code=icap_response.status_code,
                                   response_time=response_time,
                                   attempt=attempt + 1)
                    
                    return icap_response
                    
            except asyncio.TimeoutError as e:
                last_exception = IcapTimeoutError(f"Request timeout: {e}")
                self.logger.warning("Request timeout", attempt=attempt + 1, error=str(e))
                
            except aiohttp.ClientError as e:
                last_exception = IcapConnectionError(f"Connection error: {e}")
                self.logger.warning("Connection error", attempt=attempt + 1, error=str(e))
                
            except Exception as e:
                last_exception = IcapError(f"Unexpected error: {e}")
                self.logger.error("Unexpected error", attempt=attempt + 1, error=str(e))
            
            # Wait before retry
            if attempt < self.config.retries:
                delay = min(
                    self.config.retry_delay * (self.config.backoff_factor ** attempt),
                    self.config.max_retry_delay
                )
                self.logger.info("Retrying request", delay=delay, attempt=attempt + 1)
                await asyncio.sleep(delay)
        
        # All retries failed
        self.metrics["requests_failed"] += 1
        raise last_exception
    
    async def reqmod(self, http_request: HttpRequest) -> IcapResponse:
        """Send REQMOD request."""
        self.logger.info("Sending REQMOD request", uri=http_request.uri)
        
        try:
            response = await self._make_request(IcapMethod.REQMOD, http_request)
            
            # Parse modified HTTP request from response if present
            if response.body and "HTTP/" in response.body.decode('utf-8', errors='ignore'):
                # This is a simplified parser - in practice, you'd want more robust parsing
                pass
            
            return response
            
        except Exception as e:
            self.logger.error("REQMOD request failed", error=str(e))
            raise
    
    async def respmod(self, http_response: HttpResponse) -> IcapResponse:
        """Send RESPMOD request."""
        self.logger.info("Sending RESPMOD request", status_code=http_response.status_code)
        
        try:
            response = await self._make_request(IcapMethod.RESPMOD, http_response)
            
            # Parse modified HTTP response from response if present
            if response.body and "HTTP/" in response.body.decode('utf-8', errors='ignore'):
                # This is a simplified parser - in practice, you'd want more robust parsing
                pass
            
            return response
            
        except Exception as e:
            self.logger.error("RESPMOD request failed", error=str(e))
            raise
    
    async def options(self) -> IcapResponse:
        """Send OPTIONS request."""
        self.logger.info("Sending OPTIONS request")
        
        try:
            response = await self._make_request(IcapMethod.OPTIONS)
            return response
            
        except Exception as e:
            self.logger.error("OPTIONS request failed", error=str(e))
            raise
    
    async def health_check(self) -> Dict[str, Any]:
        """Check server health."""
        try:
            response = await self.options()
            
            return {
                "status": "healthy" if response.status_code < 400 else "unhealthy",
                "status_code": response.status_code,
                "version": response.headers.get("Service", "unknown"),
                "methods": response.headers.get("Methods", "").split(",") if response.headers.get("Methods") else [],
                "istag": response.headers.get("ISTag", ""),
            }
            
        except Exception as e:
            self.logger.error("Health check failed", error=str(e))
            return {
                "status": "unhealthy",
                "error": str(e),
            }
    
    def get_metrics(self) -> Dict[str, Any]:
        """Get client metrics."""
        return {
            **self.metrics,
            "connection_pool_size": self.config.connection_pool_size,
            "timeout": self.config.timeout,
            "retries": self.config.retries,
        }


# Example usage and CLI
async def main():
    """Main function demonstrating client usage."""
    import argparse
    
    parser = argparse.ArgumentParser(description="G3ICAP Python Client")
    parser.add_argument("--host", default="127.0.0.1", help="ICAP server host")
    parser.add_argument("--port", type=int, default=1344, help="ICAP server port")
    parser.add_argument("--method", choices=["reqmod", "respmod", "options"], 
                       default="options", help="ICAP method")
    parser.add_argument("--config", help="Configuration file path")
    parser.add_argument("--verbose", "-v", action="store_true", help="Verbose logging")
    
    args = parser.parse_args()
    
    # Load configuration
    config = IcapConfig(host=args.host, port=args.port)
    
    if args.config:
        with open(args.config, 'r') as f:
            config_data = yaml.safe_load(f)
            config = IcapConfig(**config_data)
    
    if args.verbose:
        config.logging_level = "DEBUG"
    
    # Create client
    async with IcapClient(config) as client:
        try:
            if args.method == "options":
                response = await client.options()
                print(f"OPTIONS Response: {response.status_code} {response.reason}")
                print(f"Headers: {response.headers}")
                
            elif args.method == "reqmod":
                # Example HTTP request
                http_request = HttpRequest(
                    method="GET",
                    uri="/",
                    headers={"Host": "example.com", "User-Agent": "Python-Client"}
                )
                
                response = await client.reqmod(http_request)
                print(f"REQMOD Response: {response.status_code} {response.reason}")
                
            elif args.method == "respmod":
                # Example HTTP response
                http_response = HttpResponse(
                    version="HTTP/1.1",
                    status_code=200,
                    reason="OK",
                    headers={"Content-Type": "text/html"},
                    body=b"<html><body>Hello World</body></html>"
                )
                
                response = await client.respmod(http_response)
                print(f"RESPMOD Response: {response.status_code} {response.reason}")
            
            # Health check
            health = await client.health_check()
            print(f"Health Check: {health}")
            
            # Metrics
            metrics = client.get_metrics()
            print(f"Metrics: {metrics}")
            
        except Exception as e:
            print(f"Error: {e}")
            return 1
    
    return 0


if __name__ == "__main__":
    import sys
    sys.exit(asyncio.run(main()))
