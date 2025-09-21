'use client';

import React, { useState, useEffect } from 'react';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Badge } from '@/components/ui/badge';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { 
  Play, 
  Square, 
  RefreshCw, 
  CheckCircle, 
  XCircle, 
  AlertTriangle, 
  Eye,
  Activity,
  Server,
  Clock,
  Zap
} from 'lucide-react';
import { HotReloadManager, ReloadStatus, ProxyStatus, PolicyChangeEvent } from '@/lib/hot-reload-manager';
import { PolicyFormData } from '@/lib/proxy-config-generator';

interface ProxyIntegrationTesterProps {
  policies: PolicyFormData[];
  onPolicyChange?: (policies: PolicyFormData[]) => void;
}

export function ProxyIntegrationTester({ policies, onPolicyChange }: ProxyIntegrationTesterProps) {
  const [hotReloadManager] = useState(() => new HotReloadManager());
  const [reloadStatus, setReloadStatus] = useState<ReloadStatus | null>(null);
  const [proxyStatus, setProxyStatus] = useState<ProxyStatus | null>(null);
  const [testUrl, setTestUrl] = useState('https://example.com');
  const [testResults, setTestResults] = useState<any[]>([]);
  const [isTesting, setIsTesting] = useState(false);
  const [isMonitoring, setIsMonitoring] = useState(false);
  const [logs, setLogs] = useState<string[]>([]);

  useEffect(() => {
    // Initialize hot reload manager
    hotReloadManager.initialize().then(() => {
      setReloadStatus(hotReloadManager.getReloadStatus());
      setProxyStatus(hotReloadManager.getProxyStatus());
    });

    // Set up event listeners
    hotReloadManager.on('reloadStarted', (data) => {
      setReloadStatus(data.status);
      addLog(`Reload started: ${data.status.message}`);
    });

    hotReloadManager.on('reloadCompleted', (data) => {
      setReloadStatus(data.status);
      addLog(`Reload completed: ${data.status.message}`);
    });

    hotReloadManager.on('reloadFailed', (data) => {
      setReloadStatus(data.status);
      addLog(`Reload failed: ${data.status.message}`);
    });

    hotReloadManager.on('healthCheck', (data) => {
      setProxyStatus(data.status);
    });

    // Start health monitoring
    hotReloadManager.startHealthMonitoring(10000);

    return () => {
      hotReloadManager.destroy();
    };
  }, []);

  const addLog = (message: string) => {
    const timestamp = new Date().toLocaleTimeString();
    setLogs(prev => [...prev.slice(-49), `[${timestamp}] ${message}`]);
  };

  const handleApplyPolicies = async () => {
    try {
      addLog('Loading policies from API...');
      
      // First, load policies from the API
      const response = await fetch('/api/policies');
      if (!response.ok) {
        throw new Error('Failed to load policies');
      }
      
      const data = await response.json();
      const loadedPolicies = data.policies || [];
      
      addLog(`Loaded ${loadedPolicies.length} policies from API`);
      addLog('Applying policy changes...');
      
      const status = await hotReloadManager.applyPolicyChanges(loadedPolicies);
      setReloadStatus(status);
      
      if (status.status === 'success') {
        addLog(`Policies applied successfully (v${status.configVersion})`);
        // Update the policies in the parent component
        if (onPolicyChange) {
          onPolicyChange(loadedPolicies);
        }
      } else {
        addLog(`Policy application failed: ${status.message}`);
      }
    } catch (error) {
      addLog(`Error applying policies: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  };

  const handleTestUrl = async () => {
    if (!testUrl.trim()) return;

    setIsTesting(true);
    addLog(`Testing URL: ${testUrl}`);

    try {
      const results = await Promise.all(
        policies.map(async (policy) => {
          const result = await hotReloadManager.testPolicy(policy, testUrl);
          return {
            policyName: policy.name,
            ...result
          };
        })
      );

      setTestResults(results);
      addLog(`Test completed for ${results.length} policies`);
    } catch (error) {
      addLog(`Test failed: ${error instanceof Error ? error.message : 'Unknown error'}`);
    } finally {
      setIsTesting(false);
    }
  };

  const handleProxyAction = async (action: 'start' | 'stop' | 'restart' | 'reload') => {
    try {
      addLog(`Executing proxy ${action}...`);
      
      const response = await fetch('/api/proxy/status', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ action })
      });

      if (response.ok) {
        const result = await response.json();
        setProxyStatus(result.status);
        addLog(`Proxy ${action} completed successfully`);
      } else {
        addLog(`Proxy ${action} failed`);
      }
    } catch (error) {
      addLog(`Error executing proxy ${action}: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'success':
        return <CheckCircle className="w-4 h-4 text-green-500" />;
      case 'error':
        return <XCircle className="w-4 h-4 text-red-500" />;
      case 'reloading':
        return <RefreshCw className="w-4 h-4 text-blue-500 animate-spin" />;
      default:
        return <Clock className="w-4 h-4 text-gray-500" />;
    }
  };

  const getHealthColor = (health: string) => {
    switch (health) {
      case 'healthy':
        return 'text-green-600';
      case 'degraded':
        return 'text-yellow-600';
      case 'unhealthy':
        return 'text-red-600';
      default:
        return 'text-gray-600';
    }
  };

  const getActionIcon = (action: string) => {
    switch (action) {
      case 'allow':
        return <CheckCircle className="w-4 h-4 text-green-500" />;
      case 'block':
        return <XCircle className="w-4 h-4 text-red-500" />;
      case 'warn':
        return <AlertTriangle className="w-4 h-4 text-yellow-500" />;
      case 'inspect':
        return <Eye className="w-4 h-4 text-blue-500" />;
      default:
        return <Clock className="w-4 h-4 text-gray-500" />;
    }
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold text-gray-900">Proxy Integration Tester</h2>
          <p className="text-gray-600">Test policies against proxy configuration with real-time monitoring</p>
        </div>
        <div className="flex items-center gap-2">
          <Button
            onClick={() => setIsMonitoring(!isMonitoring)}
            variant={isMonitoring ? "default" : "outline"}
            className="flex items-center gap-2"
          >
            <Activity className="w-4 h-4" />
            {isMonitoring ? 'Stop Monitoring' : 'Start Monitoring'}
          </Button>
        </div>
      </div>

      {/* Status Cards */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        {/* Reload Status */}
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium flex items-center gap-2">
              <RefreshCw className="w-4 h-4" />
              Reload Status
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="flex items-center gap-2">
              {reloadStatus && getStatusIcon(reloadStatus.status)}
              <span className="text-sm font-medium">
                {reloadStatus?.status || 'Unknown'}
              </span>
            </div>
            <p className="text-xs text-gray-600 mt-1">
              {reloadStatus?.message || 'No status available'}
            </p>
            <p className="text-xs text-gray-500 mt-1">
              v{reloadStatus?.configVersion || 0}
            </p>
          </CardContent>
        </Card>

        {/* Proxy Status */}
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium flex items-center gap-2">
              <Server className="w-4 h-4" />
              Proxy Status
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="flex items-center gap-2">
              <div className={`w-2 h-2 rounded-full ${
                proxyStatus?.isRunning ? 'bg-green-500' : 'bg-red-500'
              }`} />
              <span className="text-sm font-medium">
                {proxyStatus?.isRunning ? 'Running' : 'Stopped'}
              </span>
            </div>
            <p className={`text-xs mt-1 ${getHealthColor(proxyStatus?.health || 'unknown')}`}>
              {proxyStatus?.health || 'Unknown'}
            </p>
            <p className="text-xs text-gray-500 mt-1">
              Port: {proxyStatus?.port || 'N/A'}
            </p>
          </CardContent>
        </Card>

        {/* Test Results */}
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium flex items-center gap-2">
              <Zap className="w-4 h-4" />
              Test Results
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-gray-900">
              {testResults.length}
            </div>
            <p className="text-xs text-gray-600">
              {testResults.filter(r => r.action === 'block').length} blocked
            </p>
            <p className="text-xs text-gray-500">
              {testResults.filter(r => r.action === 'allow').length} allowed
            </p>
          </CardContent>
        </Card>
      </div>

      {/* Controls */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Policy Controls */}
        <Card>
          <CardHeader>
            <CardTitle className="text-lg">Policy Controls</CardTitle>
            <CardDescription>
              Apply policy changes to proxy configuration
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <Button
              onClick={handleApplyPolicies}
              disabled={reloadStatus?.status === 'reloading'}
              className="w-full"
            >
              <RefreshCw className={`w-4 h-4 mr-2 ${reloadStatus?.status === 'reloading' ? 'animate-spin' : ''}`} />
              Apply Policy Changes
            </Button>
            
            <div className="text-sm text-gray-600">
              <p>Active Policies: {policies.filter(p => p.status === 'active').length}</p>
              <p>Total Policies: {policies.length}</p>
            </div>
          </CardContent>
        </Card>

        {/* URL Testing */}
        <Card>
          <CardHeader>
            <CardTitle className="text-lg">URL Testing</CardTitle>
            <CardDescription>
              Test policies against specific URLs
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="flex gap-2">
              <Input
                value={testUrl}
                onChange={(e) => setTestUrl(e.target.value)}
                placeholder="Enter URL to test..."
                className="flex-1"
              />
              <Button
                onClick={handleTestUrl}
                disabled={isTesting || !testUrl.trim()}
              >
                <Play className="w-4 h-4" />
              </Button>
            </div>
            
            {isTesting && (
              <div className="flex items-center gap-2 text-sm text-blue-600">
                <RefreshCw className="w-4 h-4 animate-spin" />
                Testing URL...
              </div>
            )}
          </CardContent>
        </Card>
      </div>

      {/* Test Results */}
      {testResults.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle className="text-lg">Test Results for: {testUrl}</CardTitle>
            <CardDescription>
              Policy evaluation results for the tested URL
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-3">
              {testResults.map((result, index) => (
                <div key={index} className="flex items-center justify-between p-3 border rounded-lg">
                  <div className="flex items-center gap-3">
                    {getActionIcon(result.action || 'block')}
                    <div>
                      <p className="font-medium">{result.policyName || 'Unknown Policy'}</p>
                      <p className="text-sm text-gray-600">{result.reason || 'No reason provided'}</p>
                    </div>
                  </div>
                  <div className="flex items-center gap-2">
                    <Badge variant={
                      result.action === 'allow' ? 'default' :
                      result.action === 'block' ? 'destructive' :
                      result.action === 'warn' ? 'secondary' : 'outline'
                    }>
                      {result.action || 'block'}
                    </Badge>
                    {result.matchedRules && Array.isArray(result.matchedRules) && result.matchedRules.length > 0 && (
                      <Badge variant="outline">
                        {result.matchedRules.length} rules
                      </Badge>
                    )}
                  </div>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      )}

      {/* Proxy Controls */}
      <Card>
        <CardHeader>
          <CardTitle className="text-lg">Proxy Controls</CardTitle>
          <CardDescription>
            Manage proxy server operations
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="flex gap-2">
            <Button
              onClick={() => handleProxyAction('start')}
              variant="outline"
              size="sm"
            >
              <Play className="w-4 h-4 mr-2" />
              Start
            </Button>
            <Button
              onClick={() => handleProxyAction('stop')}
              variant="outline"
              size="sm"
            >
              <Square className="w-4 h-4 mr-2" />
              Stop
            </Button>
            <Button
              onClick={() => handleProxyAction('restart')}
              variant="outline"
              size="sm"
            >
              <RefreshCw className="w-4 h-4 mr-2" />
              Restart
            </Button>
            <Button
              onClick={() => handleProxyAction('reload')}
              variant="outline"
              size="sm"
            >
              <RefreshCw className="w-4 h-4 mr-2" />
              Reload Config
            </Button>
          </div>
        </CardContent>
      </Card>

      {/* Activity Logs */}
      <Card>
        <CardHeader>
          <CardTitle className="text-lg">Activity Logs</CardTitle>
          <CardDescription>
            Real-time monitoring and operation logs
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="bg-gray-900 text-green-400 p-4 rounded-lg font-mono text-sm max-h-64 overflow-y-auto">
            {logs.length === 0 ? (
              <div className="text-gray-500">No logs available</div>
            ) : (
              logs.map((log, index) => (
                <div key={index} className="mb-1">{log}</div>
              ))
            )}
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
