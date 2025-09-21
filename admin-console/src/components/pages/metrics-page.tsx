'use client';

import { useState, useEffect, useRef } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Activity, Database, RefreshCw, Filter, Search, Download, TrendingUp, AlertTriangle, CheckCircle, XCircle, BarChart3, Clock, Network, Server, Globe, Zap, Shield } from 'lucide-react';

interface MetricValue {
  value: number;
  timestamp: number;
}

interface Metric {
  name: string;
  type: 'counter' | 'gauge' | 'histogram';
  tags: Record<string, string>;
  category?: string;
  values: MetricValue[];
}

interface MetricsResponse {
  metrics: Metric[];
  total_count: number;
}

export function MetricsPage() {
  const [metrics, setMetrics] = useState<Metric[]>([]);
  const [loading, setLoading] = useState(true);
  const [lastUpdated, setLastUpdated] = useState<Date | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [selectedType, setSelectedType] = useState('all');
  const [selectedCategory, setSelectedCategory] = useState('all');
  const [sortBy, setSortBy] = useState('name');
  const [sortOrder, setSortOrder] = useState<'asc' | 'desc'>('asc');
  const chartRefs = useRef<{ [key: string]: HTMLCanvasElement | null }>({});

  const fetchMetrics = async () => {
    try {
      setError(null);
      const response = await fetch('/api/metrics');
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      const data: MetricsResponse = await response.json();
      setMetrics(data.metrics || []);
      setLastUpdated(new Date());
    } catch (error) {
      console.error('Failed to fetch metrics:', error);
      setError(error instanceof Error ? error.message : 'Failed to fetch metrics');
      setMetrics([]);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchMetrics();
    const interval = setInterval(fetchMetrics, 10000); // Refresh every 10 seconds
    return () => clearInterval(interval);
  }, []);

  // Helper function to get the latest value from a metric
  const getLatestValue = (metric: Metric): number => {
    if (metric.values && metric.values.length > 0) {
      return metric.values[metric.values.length - 1].value;
    }
    return 0;
  };

  // Filter and sort metrics
  const filteredMetrics = metrics
    .filter(metric => {
      const matchesSearch = metric.name.toLowerCase().includes(searchTerm.toLowerCase());
      const matchesType = selectedType === 'all' || metric.type === selectedType;
      const matchesCategory = selectedCategory === 'all' || (metric as any).category === selectedCategory;
      return matchesSearch && matchesType && matchesCategory;
    })
    .sort((a, b) => {
      let aValue, bValue;
      switch (sortBy) {
        case 'name':
          aValue = a.name;
          bValue = b.name;
          break;
        case 'value':
          aValue = getLatestValue(a);
          bValue = getLatestValue(b);
          break;
        case 'type':
          aValue = a.type;
          bValue = b.type;
          break;
        case 'category':
          aValue = (a as any).category || 'unknown';
          bValue = (b as any).category || 'unknown';
          break;
        default:
          aValue = a.name;
          bValue = b.name;
      }
      
      if (sortOrder === 'asc') {
        return aValue < bValue ? -1 : aValue > bValue ? 1 : 0;
      } else {
        return aValue > bValue ? -1 : aValue < bValue ? 1 : 0;
      }
    });

  // Get unique categories and types
  const categories = ['all', ...Array.from(new Set(metrics.map(m => (m as any).category).filter(Boolean)))];
  const types = ['all', ...Array.from(new Set(metrics.map(m => m.type)))];

  const categoryLabels = {
    all: 'All Categories',
    server: 'Server Performance',
    traffic: 'Network Traffic',
    escaper: 'Escaper Service',
    resolver: 'DNS Resolver',
    logger: 'Logger Service',
    runtime: 'Runtime Performance'
  };

  // Calculate summary statistics
  const totalMetrics = metrics.length;
  const healthyMetrics = metrics.filter(m => getLatestValue(m) > 0).length;
  const counterMetrics = metrics.filter(m => m.type === 'counter').length;
  const gaugeMetrics = metrics.filter(m => m.type === 'gauge').length;
  const histogramMetrics = metrics.filter(m => m.type === 'histogram').length;

  const getStatusColor = (value: number) => {
    if (value === 0) return 'text-red-600 bg-red-100';
    if (value < 10) return 'text-yellow-600 bg-yellow-100';
    return 'text-green-600 bg-green-100';
  };

  const getStatusIcon = (value: number) => {
    if (value === 0) return <XCircle className="w-4 h-4" />;
    if (value < 10) return <AlertTriangle className="w-4 h-4" />;
    return <CheckCircle className="w-4 h-4" />;
  };

  const getStatusText = (value: number) => {
    if (value === 0) return 'No Data';
    if (value < 10) return 'Low Activity';
    return 'Active';
  };

  const exportMetrics = () => {
    const csvContent = [
      ['Metric Name', 'Category', 'Type', 'Current Value', 'Status', 'Last Updated'],
      ...filteredMetrics.map(metric => [
        metric.name,
        (metric as any).category || 'unknown',
        metric.type,
        getLatestValue(metric).toString(),
        getStatusText(getLatestValue(metric)),
        metric.values?.[0]?.timestamp ? new Date(metric.values[0].timestamp).toISOString() : 'N/A'
      ])
    ].map(row => row.join(',')).join('\n');
    
    const blob = new Blob([csvContent], { type: 'text/csv' });
    const url = window.URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `g3proxy-metrics-${new Date().toISOString().split('T')[0]}.csv`;
    a.click();
    window.URL.revokeObjectURL(url);
  };

  return (
    <div className="p-6 bg-gray-50 min-h-screen">
      <div className="max-w-7xl mx-auto">
        <div className="mb-8">
          <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
            <div>
              <h1 className="text-3xl font-bold text-gray-900 mb-2">System Metrics</h1>
              <p className="text-gray-600 text-lg">
                Comprehensive monitoring and analysis of G3Proxy performance metrics
              </p>
            </div>
            <div className="flex items-center space-x-4">
              <div className="flex items-center space-x-2 px-3 py-2 bg-green-100 text-green-800 rounded-lg">
                <div className="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
                <span className="text-sm font-medium">Live Data</span>
              </div>
              <button
                onClick={fetchMetrics}
                disabled={loading}
                className="flex items-center space-x-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 transition-colors"
              >
                <RefreshCw className={`w-4 h-4 ${loading ? 'animate-spin' : ''}`} />
                <span>{loading ? 'Loading...' : 'Refresh'}</span>
              </button>
              <button
                onClick={exportMetrics}
                className="flex items-center space-x-2 px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors"
              >
                <Download className="w-4 h-4" />
                <span>Export CSV</span>
              </button>
              {lastUpdated && (
                <div className="text-sm text-gray-500">
                  Last updated: {lastUpdated.toLocaleTimeString()}
                </div>
              )}
            </div>
          </div>
        </div>

        {/* Summary Statistics */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
          <Card className="shadow-lg border-0 bg-gradient-to-br from-blue-50 to-blue-100 hover:shadow-xl transition-shadow">
            <CardContent className="p-6">
              <div className="flex items-center justify-between">
                <div className="flex-1">
                  <p className="text-sm font-medium text-blue-900">Total Metrics</p>
                  <p className="text-3xl font-bold text-blue-900">{totalMetrics}</p>
                  <p className="text-xs text-blue-600 mt-1">monitored</p>
                </div>
                <div className="p-3 bg-blue-200 rounded-full">
                  <Database className="w-6 h-6 text-blue-900" />
                </div>
                </div>
              </CardContent>
            </Card>

          <Card className="shadow-lg border-0 bg-gradient-to-br from-green-50 to-green-100 hover:shadow-xl transition-shadow">
            <CardContent className="p-6">
              <div className="flex items-center justify-between">
                <div className="flex-1">
                  <p className="text-sm font-medium text-green-700">Active Metrics</p>
                  <p className="text-3xl font-bold text-green-900">{healthyMetrics}</p>
                  <p className="text-xs text-green-600 mt-1">with data</p>
        </div>
                <div className="p-3 bg-green-200 rounded-full">
                  <CheckCircle className="w-6 h-6 text-green-700" />
                </div>
              </div>
            </CardContent>
          </Card>

          <Card className="shadow-lg border-0 bg-gradient-to-br from-purple-50 to-purple-100 hover:shadow-xl transition-shadow">
            <CardContent className="p-6">
              <div className="flex items-center justify-between">
                <div className="flex-1">
                  <p className="text-sm font-medium text-purple-700">Counters</p>
                  <p className="text-3xl font-bold text-purple-900">{counterMetrics}</p>
                  <p className="text-xs text-purple-600 mt-1">cumulative</p>
                </div>
                <div className="p-3 bg-purple-200 rounded-full">
                  <TrendingUp className="w-6 h-6 text-purple-700" />
                </div>
              </div>
            </CardContent>
          </Card>

          <Card className="shadow-lg border-0 bg-gradient-to-br from-orange-50 to-orange-100 hover:shadow-xl transition-shadow">
            <CardContent className="p-6">
              <div className="flex items-center justify-between">
                <div className="flex-1">
                  <p className="text-sm font-medium text-orange-700">Gauges</p>
                  <p className="text-3xl font-bold text-orange-900">{gaugeMetrics}</p>
                  <p className="text-xs text-orange-600 mt-1">current values</p>
                </div>
                <div className="p-3 bg-orange-200 rounded-full">
                  <BarChart3 className="w-6 h-6 text-orange-700" />
                </div>
              </div>
            </CardContent>
          </Card>
        </div>

        {/* Filters and Search */}
        <div className="mb-8">
          <Card className="shadow-sm">
            <CardContent className="p-6">
              <div className="flex flex-wrap items-center gap-4">
                <div className="flex items-center space-x-2">
                  <Search className="w-4 h-4 text-gray-500" />
                  <input
                    type="text"
                    placeholder="Search metrics..."
                    value={searchTerm}
                    onChange={(e) => setSearchTerm(e.target.value)}
                    className="px-3 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 w-64"
                  />
                </div>
                
                <div className="flex items-center space-x-2">
                  <Filter className="w-4 h-4 text-gray-500" />
                  <select
                    value={selectedType}
                    onChange={(e) => setSelectedType(e.target.value)}
                    className="px-3 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                  >
                    <option value="all">All Types</option>
                    {types.map(type => (
                      <option key={type} value={type}>{type.charAt(0).toUpperCase() + type.slice(1)}</option>
                    ))}
                  </select>
                </div>
                
                <div className="flex items-center space-x-2">
                  <select
                    value={selectedCategory}
                    onChange={(e) => setSelectedCategory(e.target.value)}
                    className="px-3 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                  >
                    {categories.map(category => (
                      <option key={category} value={category}>
                        {categoryLabels[category as keyof typeof categoryLabels] || category}
                      </option>
                    ))}
                  </select>
                </div>
                
                <div className="flex items-center space-x-2">
                  <select
                    value={sortBy}
                    onChange={(e) => setSortBy(e.target.value)}
                    className="px-3 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                  >
                    <option value="name">Sort by Name</option>
                    <option value="value">Sort by Value</option>
                    <option value="type">Sort by Type</option>
                    <option value="category">Sort by Category</option>
                  </select>
                </div>
                
                <button
                  onClick={() => setSortOrder(sortOrder === 'asc' ? 'desc' : 'asc')}
                  className="flex items-center space-x-1 px-3 py-2 bg-gray-100 text-gray-700 rounded-md hover:bg-gray-200 transition-colors"
                >
                  <TrendingUp className={`w-4 h-4 ${sortOrder === 'desc' ? 'rotate-180' : ''}`} />
                  <span className="text-sm">{sortOrder === 'asc' ? 'Ascending' : 'Descending'}</span>
                </button>
                
                <div className="ml-auto text-sm text-gray-500">
                  Showing {filteredMetrics.length} of {totalMetrics} metrics
                </div>
              </div>
            </CardContent>
          </Card>
        </div>

        {/* Metrics Table */}
        <Card className="shadow-lg">
          <CardHeader className="bg-gradient-to-r from-gray-50 to-gray-100 border-b border-gray-200">
            <div className="flex items-center justify-between">
              <div>
                <CardTitle className="text-xl font-bold text-gray-900 flex items-center space-x-2">
                  <Database className="w-6 h-6 text-gray-600" />
                  <span>Detailed Metrics Analysis</span>
                </CardTitle>
                <CardDescription className="text-gray-600 font-medium">
                  Comprehensive view of all available metrics with real-time data
                </CardDescription>
              </div>
              <div className="flex items-center space-x-2">
                <button
                  onClick={exportMetrics}
                  className="flex items-center space-x-2 px-3 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors"
                >
                  <Download className="w-4 h-4" />
                  <span className="text-sm">Export CSV</span>
                </button>
              </div>
            </div>
          </CardHeader>
          <CardContent className="p-6">
            {loading ? (
              <div className="flex items-center justify-center py-12">
                <div className="flex items-center space-x-2">
                  <RefreshCw className="w-5 h-5 animate-spin text-blue-600" />
                  <span className="text-gray-600">Loading metrics...</span>
                </div>
              </div>
            ) : (
              <div className="overflow-x-auto">
                <table className="w-full text-sm">
                  <thead>
                    <tr className="border-b border-gray-200">
                      <th className="text-left p-4 font-semibold text-gray-700">Metric Name</th>
                      <th className="text-left p-4 font-semibold text-gray-700">Category</th>
                      <th className="text-left p-4 font-semibold text-gray-700">Type</th>
                      <th className="text-left p-4 font-semibold text-gray-700">Current Value</th>
                      <th className="text-left p-4 font-semibold text-gray-700">Status</th>
                      <th className="text-left p-4 font-semibold text-gray-700">Last Updated</th>
                      <th className="text-left p-4 font-semibold text-gray-700">Actions</th>
                    </tr>
                  </thead>
                  <tbody>
                    {filteredMetrics.map((metric, index) => {
                      const latestValue = getLatestValue(metric);
                      const status = getStatusText(latestValue);
                      const statusColor = getStatusColor(latestValue);
                      
                      return (
                        <tr key={index} className="border-b border-gray-100 hover:bg-gray-50 transition-colors">
                          <td className="p-4 font-mono text-sm text-gray-900">{metric.name}</td>
                          <td className="p-4">
                            <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
                              {(metric as any).category || 'unknown'}
                            </span>
                          </td>
                          <td className="p-4">
                            <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
                              metric.type === 'counter' ? 'bg-green-100 text-green-800' : 
                              metric.type === 'gauge' ? 'bg-blue-100 text-blue-800' : 
                              'bg-purple-100 text-purple-800'
                            }`}>
                              {metric.type}
                            </span>
                          </td>
                          <td className="p-4 font-mono text-sm font-semibold text-gray-900">
                            {latestValue.toLocaleString()}
                          </td>
                          <td className="p-4">
                            <div className="flex items-center space-x-2">
                              {getStatusIcon(latestValue)}
                              <span className={`text-xs font-medium ${statusColor}`}>
                                {status}
                              </span>
                            </div>
                          </td>
                          <td className="p-4 text-gray-500 text-sm">
                            {metric.values?.[0]?.timestamp 
                              ? new Date(metric.values[0].timestamp).toLocaleString()
                              : 'N/A'
                            }
                          </td>
                          <td className="p-4">
                            <div className="flex items-center space-x-2">
                              <button className="text-blue-600 hover:text-blue-800 text-xs">
                                View Details
                              </button>
                              <button className="text-green-600 hover:text-green-800 text-xs">
                                Export
                              </button>
                            </div>
                          </td>
                        </tr>
                      );
                    })}
                  </tbody>
                </table>
              </div>
            )}
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
