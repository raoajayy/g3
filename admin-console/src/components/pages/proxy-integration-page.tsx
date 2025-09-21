'use client';

import React, { useState, useEffect } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { 
  Server, 
  Activity, 
  Settings, 
  TestTube, 
  Monitor,
  RefreshCw,
  CheckCircle,
  XCircle,
  AlertTriangle
} from 'lucide-react';
import { ProxyIntegrationTester } from '@/components/proxy-integration-tester';
import { PolicyTester } from '@/components/policy-tester';
import { PolicyPreview } from '@/components/policy-preview';
import { HotReloadManager, ReloadStatus, ProxyStatus } from '@/lib/hot-reload-manager';
import { PolicyFormData } from '@/lib/proxy-config-generator';

export function ProxyIntegrationPage() {
  const [policies, setPolicies] = useState<PolicyFormData[]>([]);
  const [selectedPolicy, setSelectedPolicy] = useState<PolicyFormData | null>(null);
  const [hotReloadManager] = useState(() => new HotReloadManager());
  const [reloadStatus, setReloadStatus] = useState<ReloadStatus | null>(null);
  const [proxyStatus, setProxyStatus] = useState<ProxyStatus | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadPolicies();
    initializeHotReload();
  }, []);

  const loadPolicies = async () => {
    try {
      setLoading(true);
      const response = await fetch('/api/policies');
      if (response.ok) {
        const data = await response.json();
        setPolicies(data.policies || []);
      } else {
        throw new Error('Failed to load policies');
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load policies');
    } finally {
      setLoading(false);
    }
  };

  const initializeHotReload = async () => {
    try {
      await hotReloadManager.initialize();
      setReloadStatus(hotReloadManager.getReloadStatus());
      setProxyStatus(hotReloadManager.getProxyStatus());

      // Set up event listeners
      hotReloadManager.on('reloadStarted', (data) => {
        setReloadStatus(data.status);
      });

      hotReloadManager.on('reloadCompleted', (data) => {
        setReloadStatus(data.status);
      });

      hotReloadManager.on('reloadFailed', (data) => {
        setReloadStatus(data.status);
      });

      hotReloadManager.on('healthCheck', (data) => {
        setProxyStatus(data.status);
      });

      // Start health monitoring
      hotReloadManager.startHealthMonitoring(15000);

    } catch (err) {
      console.error('Failed to initialize hot reload manager:', err);
    }
  };

  const handlePolicyChange = (updatedPolicies: PolicyFormData[]) => {
    setPolicies(updatedPolicies);
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
        return <AlertTriangle className="w-4 h-4 text-yellow-500" />;
    }
  };

  const getHealthColor = (health: string) => {
    switch (health) {
      case 'healthy':
        return 'text-green-600 bg-green-50';
      case 'degraded':
        return 'text-yellow-600 bg-yellow-50';
      case 'unhealthy':
        return 'text-red-600 bg-red-50';
      default:
        return 'text-gray-600 bg-gray-50';
    }
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="flex items-center gap-2">
          <RefreshCw className="w-5 h-5 animate-spin" />
          <span>Loading proxy integration...</span>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <Alert className="border-red-200 bg-red-50">
        <XCircle className="h-4 w-4 text-red-600" />
        <AlertDescription className="text-red-800">
          {error}
        </AlertDescription>
      </Alert>
    );
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">Proxy Integration</h1>
          <p className="text-gray-600">Manage policy-to-proxy integration with real-time testing and monitoring</p>
        </div>
        <div className="flex items-center gap-2">
          <Button
            onClick={loadPolicies}
            variant="outline"
            size="sm"
          >
            <RefreshCw className="w-4 h-4 mr-2" />
            Refresh
          </Button>
        </div>
      </div>

      {/* Status Overview */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <Card>
          <CardContent className="p-4">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-gray-600">Proxy Status</p>
                <div className="flex items-center gap-2 mt-1">
                  <div className={`w-2 h-2 rounded-full ${
                    proxyStatus?.isRunning ? 'bg-green-500' : 'bg-red-500'
                  }`} />
                  <span className="text-lg font-semibold">
                    {proxyStatus?.isRunning ? 'Running' : 'Stopped'}
                  </span>
                </div>
              </div>
              <Server className="w-8 h-8 text-gray-400" />
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="p-4">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-gray-600">Health Status</p>
                <Badge className={`mt-1 ${getHealthColor(proxyStatus?.health || 'unknown')}`}>
                  {proxyStatus?.health || 'Unknown'}
                </Badge>
              </div>
              <Activity className="w-8 h-8 text-gray-400" />
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="p-4">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-gray-600">Config Version</p>
                <span className="text-lg font-semibold">
                  v{reloadStatus?.configVersion || 0}
                </span>
              </div>
              <Settings className="w-8 h-8 text-gray-400" />
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="p-4">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-gray-600">Active Policies</p>
                <span className="text-lg font-semibold">
                  {policies.filter(p => p.status === 'active').length}
                </span>
              </div>
              <Monitor className="w-8 h-8 text-gray-400" />
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Main Content Tabs */}
      <Tabs defaultValue="integration" className="space-y-6">
        <TabsList className="grid w-full grid-cols-4">
          <TabsTrigger value="integration" className="flex items-center gap-2">
            <Server className="w-4 h-4" />
            Integration
          </TabsTrigger>
          <TabsTrigger value="testing" className="flex items-center gap-2">
            <TestTube className="w-4 h-4" />
            Testing
          </TabsTrigger>
          <TabsTrigger value="monitoring" className="flex items-center gap-2">
            <Monitor className="w-4 h-4" />
            Monitoring
          </TabsTrigger>
          <TabsTrigger value="preview" className="flex items-center gap-2">
            <Settings className="w-4 h-4" />
            Preview
          </TabsTrigger>
        </TabsList>

        <TabsContent value="integration" className="space-y-6">
          <ProxyIntegrationTester
            policies={policies}
            onPolicyChange={handlePolicyChange}
          />
        </TabsContent>

        <TabsContent value="testing" className="space-y-6">
          <Card>
            <CardHeader>
              <CardTitle>Policy Testing</CardTitle>
              <CardDescription>
                Test individual policies against URLs and scenarios
              </CardDescription>
            </CardHeader>
            <CardContent>
              {selectedPolicy ? (
                <PolicyTester
                  policy={selectedPolicy}
                  onClose={() => setSelectedPolicy(null)}
                />
              ) : (
                <div className="space-y-4">
                  <p className="text-gray-600">Select a policy to test:</p>
                  <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                    {policies.map((policy) => (
                      <Card
                        key={policy.name}
                        className="cursor-pointer hover:shadow-md transition-shadow"
                        onClick={() => setSelectedPolicy(policy)}
                      >
                        <CardContent className="p-4">
                          <div className="flex items-center justify-between mb-2">
                            <h3 className="font-medium">{policy.name}</h3>
                            <Badge variant={policy.status === 'active' ? 'default' : 'secondary'}>
                              {policy.status}
                            </Badge>
                          </div>
                          <p className="text-sm text-gray-600 mb-2">
                            {policy.description}
                          </p>
                          <div className="text-xs text-gray-500">
                            {policy.urlFiltering?.customRules?.length || 0} custom rules
                          </div>
                        </CardContent>
                      </Card>
                    ))}
                  </div>
                </div>
              )}
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="monitoring" className="space-y-6">
          <Card>
            <CardHeader>
              <CardTitle>Real-time Monitoring</CardTitle>
              <CardDescription>
                Monitor proxy performance and policy enforcement
              </CardDescription>
            </CardHeader>
            <CardContent>
              <div className="space-y-4">
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div className="p-4 border rounded-lg">
                    <h4 className="font-medium mb-2">Reload Status</h4>
                    <div className="flex items-center gap-2">
                      {reloadStatus && getStatusIcon(reloadStatus.status)}
                      <span className="text-sm">
                        {reloadStatus?.status || 'Unknown'}
                      </span>
                    </div>
                    <p className="text-xs text-gray-600 mt-1">
                      {reloadStatus?.message || 'No status available'}
                    </p>
                  </div>

                  <div className="p-4 border rounded-lg">
                    <h4 className="font-medium mb-2">Proxy Health</h4>
                    <div className="flex items-center gap-2">
                      <div className={`w-2 h-2 rounded-full ${
                        proxyStatus?.isRunning ? 'bg-green-500' : 'bg-red-500'
                      }`} />
                      <span className="text-sm">
                        {proxyStatus?.isRunning ? 'Running' : 'Stopped'}
                      </span>
                    </div>
                    <p className="text-xs text-gray-600 mt-1">
                      Last reload: {proxyStatus?.lastReload ? 
                        new Date(proxyStatus.lastReload).toLocaleString() : 'Never'
                      }
                    </p>
                  </div>
                </div>

                <div className="p-4 border rounded-lg">
                  <h4 className="font-medium mb-2">Configuration Details</h4>
                  <div className="text-sm text-gray-600 space-y-1">
                    <p>Port: {proxyStatus?.port || 'N/A'}</p>
                    <p>PID: {proxyStatus?.pid || 'N/A'}</p>
                    <p>Uptime: {proxyStatus?.uptime ? `${Math.floor(proxyStatus.uptime / 60)}m` : 'N/A'}</p>
                    <p>Requests Processed: {proxyStatus?.requestsProcessed || 0}</p>
                    <p>Blocked Requests: {proxyStatus?.blockedRequests || 0}</p>
                    <p>Error Rate: {proxyStatus?.errorRate ? `${(proxyStatus.errorRate * 100).toFixed(2)}%` : 'N/A'}</p>
                  </div>
                </div>
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="preview" className="space-y-6">
          <Card>
            <CardHeader>
              <CardTitle>Configuration Preview</CardTitle>
              <CardDescription>
                Preview generated proxy configuration
              </CardDescription>
            </CardHeader>
            <CardContent>
              {selectedPolicy ? (
                <PolicyPreview
                  policy={selectedPolicy}
                  onClose={() => setSelectedPolicy(null)}
                />
              ) : (
                <div className="space-y-4">
                  <p className="text-gray-600">Select a policy to preview:</p>
                  <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                    {policies.map((policy) => (
                      <Card
                        key={policy.name}
                        className="cursor-pointer hover:shadow-md transition-shadow"
                        onClick={() => setSelectedPolicy(policy)}
                      >
                        <CardContent className="p-4">
                          <div className="flex items-center justify-between mb-2">
                            <h3 className="font-medium">{policy.name}</h3>
                            <Badge variant={policy.status === 'active' ? 'default' : 'secondary'}>
                              {policy.status}
                            </Badge>
                          </div>
                          <p className="text-sm text-gray-600">
                            {policy.description}
                          </p>
                        </CardContent>
                      </Card>
                    ))}
                  </div>
                </div>
              )}
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
    </div>
  );
}
