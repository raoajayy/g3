'use client';

import React, { useState, useEffect } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Badge } from '@/components/ui/badge';
import { 
  Settings, 
  TestTube, 
  CheckCircle, 
  XCircle,
  AlertTriangle,
  RefreshCw,
  Plus,
  Edit,
  Trash2,
  Key,
  Globe,
  Clock
} from 'lucide-react';

interface CategoryProvider {
  name: string;
  baseUrl: string;
  apiKey?: string;
  rateLimit: number;
  freeTier: boolean;
  categories: string[];
  enabled: boolean;
}

interface TestResult {
  provider: string;
  success: boolean;
  result?: any;
  error?: string;
  responseTime: number;
}

export function ProviderManager() {
  const [providers, setProviders] = useState<CategoryProvider[]>([]);
  const [loading, setLoading] = useState(false);
  const [testing, setTesting] = useState<string | null>(null);
  const [testResults, setTestResults] = useState<TestResult[]>([]);
  const [showAddModal, setShowAddModal] = useState(false);
  const [editingProvider, setEditingProvider] = useState<CategoryProvider | null>(null);

  useEffect(() => {
    loadProviders();
  }, []);

  const loadProviders = async () => {
    setLoading(true);
    try {
      const response = await fetch('/api/categories/providers');
      const data = await response.json();
      if (data.success) {
        setProviders(data.data);
      }
    } catch (error) {
      console.error('Error loading providers:', error);
    } finally {
      setLoading(false);
    }
  };

  const testProvider = async (provider: CategoryProvider) => {
    setTesting(provider.name);
    const startTime = Date.now();
    
    try {
      const response = await fetch('/api/categories/test-provider', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          providerName: provider.name,
          testUrl: 'https://example.com'
        })
      });
      
      const data = await response.json();
      const responseTime = Date.now() - startTime;
      
      const result: TestResult = {
        provider: provider.name,
        success: data.success,
        result: data.data,
        error: data.error,
        responseTime
      };
      
      setTestResults(prev => [result, ...prev.slice(0, 9)]); // Keep last 10 results
    } catch (error) {
      const result: TestResult = {
        provider: provider.name,
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error',
        responseTime: Date.now() - startTime
      };
      setTestResults(prev => [result, ...prev.slice(0, 9)]);
    } finally {
      setTesting(null);
    }
  };

  const toggleProvider = async (provider: CategoryProvider) => {
    try {
      const response = await fetch('/api/categories/providers', {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name: provider.name,
          updates: { enabled: !provider.enabled }
        })
      });
      
      if (response.ok) {
        loadProviders();
      }
    } catch (error) {
      console.error('Error toggling provider:', error);
    }
  };

  const deleteProvider = async (provider: CategoryProvider) => {
    if (!confirm(`Are you sure you want to delete ${provider.name}?`)) return;
    
    try {
      const response = await fetch(`/api/categories/providers?name=${provider.name}`, {
        method: 'DELETE'
      });
      
      if (response.ok) {
        loadProviders();
      }
    } catch (error) {
      console.error('Error deleting provider:', error);
    }
  };

  const getStatusIcon = (provider: CategoryProvider) => {
    if (testing === provider.name) {
      return <RefreshCw className="w-4 h-4 animate-spin text-blue-500" />;
    }
    return provider.enabled ? 
      <CheckCircle className="w-4 h-4 text-green-500" /> : 
      <XCircle className="w-4 h-4 text-gray-400" />;
  };

  const getStatusColor = (provider: CategoryProvider) => {
    return provider.enabled ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-800';
  };

  const getTestResultIcon = (result: TestResult) => {
    return result.success ? 
      <CheckCircle className="w-4 h-4 text-green-500" /> : 
      <XCircle className="w-4 h-4 text-red-500" />;
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold text-gray-900">External Provider Management</h2>
          <p className="text-gray-600">Manage URL categorization data sources</p>
        </div>
        <div className="flex items-center gap-2">
          <Button variant="outline" onClick={loadProviders} disabled={loading}>
            <RefreshCw className={`w-4 h-4 mr-2 ${loading ? 'animate-spin' : ''}`} />
            Refresh
          </Button>
          <Button onClick={() => setShowAddModal(true)}>
            <Plus className="w-4 h-4 mr-2" />
            Add Provider
          </Button>
        </div>
      </div>

      {/* Test Results */}
      {testResults.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <TestTube className="w-5 h-5" />
              Recent Test Results
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-2">
              {testResults.slice(0, 5).map((result, index) => (
                <div key={index} className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
                  <div className="flex items-center gap-3">
                    {getTestResultIcon(result)}
                    <div>
                      <p className="font-medium">{result.provider}</p>
                      <p className="text-sm text-gray-600">
                        {result.success ? 'Test passed' : result.error}
                      </p>
                    </div>
                  </div>
                  <div className="text-sm text-gray-500">
                    {result.responseTime}ms
                  </div>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      )}

      {/* Providers List */}
      <Card>
        <CardHeader>
          <CardTitle>Data Providers ({providers.length})</CardTitle>
        </CardHeader>
        <CardContent>
          {loading ? (
            <div className="text-center py-8">
              <RefreshCw className="w-8 h-8 animate-spin mx-auto text-gray-400" />
              <p className="text-gray-600 mt-2">Loading providers...</p>
            </div>
          ) : providers.length === 0 ? (
            <div className="text-center py-8 text-gray-500">
              No providers configured.
            </div>
          ) : (
            <div className="space-y-4">
              {providers.map((provider) => (
                <div key={provider.name} className="border border-gray-200 rounded-lg p-4">
                  <div className="flex items-start justify-between">
                    <div className="flex-1">
                      <div className="flex items-center gap-3 mb-2">
                        <h3 className="font-medium text-gray-900">{provider.name}</h3>
                        <Badge className={getStatusColor(provider)}>
                          {provider.enabled ? 'Enabled' : 'Disabled'}
                        </Badge>
                        <Badge variant="outline">
                          {provider.freeTier ? 'Free' : 'Paid'}
                        </Badge>
                        {getStatusIcon(provider)}
                      </div>
                      
                      <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm text-gray-600">
                        <div className="flex items-center gap-2">
                          <Globe className="w-4 h-4" />
                          <span>{provider.baseUrl}</span>
                        </div>
                        <div className="flex items-center gap-2">
                          <Clock className="w-4 h-4" />
                          <span>{provider.rateLimit} req/min</span>
                        </div>
                        <div className="flex items-center gap-2">
                          <Settings className="w-4 h-4" />
                          <span>{provider.categories.length} categories</span>
                        </div>
                      </div>
                      
                      {provider.apiKey && (
                        <div className="mt-2 flex items-center gap-2 text-sm text-gray-500">
                          <Key className="w-4 h-4" />
                          <span>API Key configured</span>
                        </div>
                      )}
                    </div>
                    
                    <div className="flex items-center gap-1 ml-4">
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => testProvider(provider)}
                        disabled={testing === provider.name}
                      >
                        <TestTube className="w-4 h-4" />
                      </Button>
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => setEditingProvider(provider)}
                      >
                        <Edit className="w-4 h-4" />
                      </Button>
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => toggleProvider(provider)}
                      >
                        {provider.enabled ? 'Disable' : 'Enable'}
                      </Button>
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => deleteProvider(provider)}
                        className="text-red-600 hover:text-red-800"
                      >
                        <Trash2 className="w-4 h-4" />
                      </Button>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
