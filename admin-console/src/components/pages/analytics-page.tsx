'use client';

import { useState, useEffect, useRef } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { AdvancedCharts } from '@/components/advanced-charts';
import { exportToCSV, exportToJSON, exportToPDF, generateReport } from '@/lib/export-utils';
import { BarChart3, TrendingUp, Activity, Clock, RefreshCw, Download, Filter, Calendar, AlertTriangle, CheckCircle, XCircle, Database, Network, Server, Globe, Zap, Shield, FileText, BarChart } from 'lucide-react';

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

export function AnalyticsPage() {
  const [metrics, setMetrics] = useState<Metric[]>([]);
  const [loading, setLoading] = useState(true);
  const [lastUpdated, setLastUpdated] = useState<Date | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [timeRange, setTimeRange] = useState('24h');
  const [selectedCategory, setSelectedCategory] = useState('all');
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

  // Helper function to get metrics by category
  const getMetricsByCategory = (category: string) => {
    if (category === 'all') return metrics;
    return metrics.filter(m => (m as any).category === category);
  };

  // Calculate key analytics
  const totalRequests = getLatestValue(metrics.find(m => m.name === 'server_connection_total') || { values: [] } as Metric);
  const activeConnections = getLatestValue(metrics.find(m => m.name === 'active_connections') || { values: [] } as Metric);
  const totalTrafficIn = getLatestValue(metrics.find(m => m.name === 'server_traffic_in_bytes') || { values: [] } as Metric);
  const totalTrafficOut = getLatestValue(metrics.find(m => m.name === 'server_traffic_out_bytes') || { values: [] } as Metric);
  const escaperSuccessRate = getLatestValue(metrics.find(m => m.name === 'escaper_connection_establish') || { values: [] } as Metric);
  const escaperAttempts = getLatestValue(metrics.find(m => m.name === 'escaper_connection_attempt') || { values: [] } as Metric);
  const dnsQueries = getLatestValue(metrics.find(m => m.name === 'resolver_query_total') || { values: [] } as Metric);
  const dnsCacheHits = getLatestValue(metrics.find(m => m.name === 'resolver_query_cached') || { values: [] } as Metric);

  // Calculate derived metrics
  const successRate = escaperAttempts > 0 ? (escaperSuccessRate / escaperAttempts) * 100 : 0;
  const dnsCacheHitRate = dnsQueries > 0 ? (dnsCacheHits / dnsQueries) * 100 : 0;
  const totalTrafficMB = (totalTrafficIn + totalTrafficOut) / 1024 / 1024;
  const requestsPerSecond = totalRequests / (24 * 60 * 60); // Approximate RPS over 24h

  // Get categories for filtering
  const categories = ['all', 'server', 'traffic', 'escaper', 'resolver', 'logger', 'runtime'];
  const categoryLabels = {
    all: 'All Metrics',
    server: 'Server Performance',
    traffic: 'Network Traffic',
    escaper: 'Escaper Service',
    resolver: 'DNS Resolver',
    logger: 'Logger Service',
    runtime: 'Runtime Performance'
  };

  return (
    <div className="p-6 bg-gray-50 min-h-screen">
      <div className="max-w-7xl mx-auto">
        <div className="mb-8">
          <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
            <div>
              <h1 className="text-3xl font-bold text-gray-900 mb-2">Advanced Analytics</h1>
              <p className="text-gray-600 text-lg">
                Comprehensive insights and performance analysis from G3Proxy infrastructure
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
              {lastUpdated && (
                <div className="text-sm text-gray-500">
                  Last updated: {lastUpdated.toLocaleTimeString()}
                </div>
              )}
            </div>
          </div>
        </div>

        {/* Key Performance Indicators */}
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
          <Card className="shadow-lg border-0 bg-gradient-to-br from-blue-50 to-blue-100 hover:shadow-xl transition-shadow">
            <CardContent className="p-6">
              <div className="flex items-center justify-between">
                <div className="flex-1">
                  <p className="text-sm font-medium text-blue-900">Total Requests</p>
                  <p className="text-3xl font-bold text-blue-900">{totalRequests.toLocaleString()}</p>
                  <p className="text-xs text-blue-600 mt-1">all time</p>
                  <div className="mt-2 text-xs text-blue-500">
                    Rate: {requestsPerSecond.toFixed(1)}/s
                  </div>
                </div>
                <div className="p-3 bg-blue-200 rounded-full">
                  <Activity className="w-6 h-6 text-blue-900" />
                </div>
              </div>
            </CardContent>
          </Card>

          <Card className="shadow-lg border-0 bg-gradient-to-br from-green-50 to-green-100 hover:shadow-xl transition-shadow">
            <CardContent className="p-6">
              <div className="flex items-center justify-between">
                <div className="flex-1">
                  <p className="text-sm font-medium text-green-700">Success Rate</p>
                  <p className="text-3xl font-bold text-green-900">{successRate.toFixed(1)}%</p>
                  <p className="text-xs text-green-600 mt-1">connection success</p>
                  <div className="mt-2 text-xs text-green-500">
                    Attempts: {escaperAttempts.toLocaleString()}
                  </div>
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
                  <p className="text-sm font-medium text-purple-700">DNS Cache Hit</p>
                  <p className="text-3xl font-bold text-purple-900">{dnsCacheHitRate.toFixed(1)}%</p>
                  <p className="text-xs text-purple-600 mt-1">cache efficiency</p>
                  <div className="mt-2 text-xs text-purple-500">
                    Queries: {dnsQueries.toLocaleString()}
                  </div>
                </div>
                <div className="p-3 bg-purple-200 rounded-full">
                  <Globe className="w-6 h-6 text-purple-700" />
                </div>
              </div>
            </CardContent>
          </Card>

          <Card className="shadow-lg border-0 bg-gradient-to-br from-orange-50 to-orange-100 hover:shadow-xl transition-shadow">
            <CardContent className="p-6">
              <div className="flex items-center justify-between">
                <div className="flex-1">
                  <p className="text-sm font-medium text-orange-700">Traffic Volume</p>
                  <p className="text-3xl font-bold text-orange-900">{totalTrafficMB.toFixed(1)}</p>
                  <p className="text-xs text-orange-600 mt-1">MB processed</p>
                  <div className="mt-2 text-xs text-orange-500">
                    Active: {activeConnections} connections
                  </div>
                </div>
                <div className="p-3 bg-orange-200 rounded-full">
                  <Network className="w-6 h-6 text-orange-700" />
                </div>
              </div>
            </CardContent>
          </Card>
        </div>

        {/* Filters and Controls */}
        <div className="mb-8">
          <Card className="shadow-sm">
            <CardContent className="p-6">
              <div className="flex flex-wrap items-center gap-4">
                <div className="flex items-center space-x-2">
                  <Filter className="w-4 h-4 text-gray-500" />
                  <span className="text-sm font-medium text-gray-700">Filter by Category:</span>
                </div>
                <div className="flex flex-wrap gap-2">
                  {categories.map((category) => (
                    <button
                      key={category}
                      onClick={() => setSelectedCategory(category)}
                      className={`px-3 py-1 rounded-full text-sm font-medium transition-colors ${
                        selectedCategory === category
                          ? 'bg-blue-600 text-white'
                          : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                      }`}
                    >
                      {categoryLabels[category as keyof typeof categoryLabels]}
                    </button>
                  ))}
                </div>
                <div className="flex items-center space-x-2 ml-auto">
                  <Calendar className="w-4 h-4 text-gray-500" />
                  <select
                    value={timeRange}
                    onChange={(e) => setTimeRange(e.target.value)}
                    className="px-3 py-1 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                  >
                    <option value="1h">Last Hour</option>
                    <option value="24h">Last 24 Hours</option>
                    <option value="7d">Last 7 Days</option>
                    <option value="30d">Last 30 Days</option>
                  </select>
                </div>
              </div>
            </CardContent>
          </Card>
        </div>

        {/* Performance Trends */}
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
          <Card className="shadow-lg">
            <CardHeader className="bg-gradient-to-r from-blue-50 to-indigo-50 border-b border-gray-100">
              <CardTitle className="text-xl font-bold text-gray-900 flex items-center space-x-2">
                <BarChart3 className="w-6 h-6 text-blue-600" />
                <span>Performance Trends</span>
              </CardTitle>
              <CardDescription className="text-gray-600 font-medium">
                Real-time performance metrics over time
              </CardDescription>
            </CardHeader>
            <CardContent className="p-6">
              {loading ? (
                <div className="flex items-center justify-center h-64">
                  <div className="flex items-center space-x-2">
                    <RefreshCw className="w-5 h-5 animate-spin text-blue-600" />
                    <span className="text-gray-600">Loading performance data...</span>
                  </div>
                </div>
              ) : (
                <div className="h-64">
                  <canvas
                    ref={(el) => { chartRefs.current['performance-trends'] = el; }}
                    width={400}
                    height={200}
                    className="w-full h-full"
                  />
                </div>
              )}
            </CardContent>
          </Card>

          <Card className="shadow-lg">
            <CardHeader className="bg-gradient-to-r from-green-50 to-emerald-50 border-b border-gray-100">
              <CardTitle className="text-xl font-bold text-gray-900 flex items-center space-x-2">
                <TrendingUp className="w-6 h-6 text-green-600" />
                <span>Usage Patterns</span>
              </CardTitle>
              <CardDescription className="text-gray-600 font-medium">
                Traffic patterns and usage analytics
              </CardDescription>
            </CardHeader>
            <CardContent className="p-6">
              {loading ? (
                <div className="flex items-center justify-center h-64">
                  <div className="flex items-center space-x-2">
                    <RefreshCw className="w-5 h-5 animate-spin text-blue-600" />
                    <span className="text-gray-600">Loading usage data...</span>
                  </div>
                </div>
              ) : (
                <div className="h-64">
                  <canvas
                    ref={(el) => { chartRefs.current['usage-patterns'] = el; }}
                    width={400}
                    height={200}
                    className="w-full h-full"
                  />
                </div>
              )}
            </CardContent>
          </Card>
        </div>

        {/* Detailed Metrics Analysis */}
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-8">
          <Card className="shadow-lg">
            <CardHeader className="bg-gradient-to-r from-purple-50 to-purple-100 border-b border-gray-100">
              <CardTitle className="text-lg font-semibold text-gray-800 flex items-center space-x-2">
                <Activity className="w-5 h-5 text-purple-600" />
                <span>Request Analysis</span>
              </CardTitle>
            </CardHeader>
            <CardContent className="p-6">
              <div className="space-y-4">
                <div className="flex justify-between items-center p-3 bg-gray-50 rounded-lg">
                  <span className="font-medium text-gray-700">Total Requests</span>
                  <span className="font-mono text-lg font-bold text-gray-900">{totalRequests.toLocaleString()}</span>
                </div>
                <div className="flex justify-between items-center p-3 bg-gray-50 rounded-lg">
                  <span className="font-medium text-gray-700">Active Connections</span>
                  <span className="font-mono text-lg font-bold text-green-600">{activeConnections}</span>
                </div>
                <div className="flex justify-between items-center p-3 bg-gray-50 rounded-lg">
                  <span className="font-medium text-gray-700">Request Rate</span>
                  <span className="font-mono text-lg font-bold text-blue-600">{requestsPerSecond.toFixed(1)}/s</span>
                </div>
                <div className="flex justify-between items-center p-3 bg-gray-50 rounded-lg">
                  <span className="font-medium text-gray-700">Success Rate</span>
                  <span className="font-mono text-lg font-bold text-green-600">{successRate.toFixed(1)}%</span>
                </div>
              </div>
            </CardContent>
          </Card>

          <Card className="shadow-lg">
            <CardHeader className="bg-gradient-to-r from-orange-50 to-orange-100 border-b border-gray-100">
              <CardTitle className="text-lg font-semibold text-gray-800 flex items-center space-x-2">
                <Clock className="w-5 h-5 text-orange-600" />
                <span>Performance Metrics</span>
              </CardTitle>
            </CardHeader>
            <CardContent className="p-6">
              <div className="space-y-4">
                <div className="flex justify-between items-center p-3 bg-gray-50 rounded-lg">
                  <span className="font-medium text-gray-700">DNS Cache Hit Rate</span>
                  <span className="font-mono text-lg font-bold text-purple-600">{dnsCacheHitRate.toFixed(1)}%</span>
                </div>
                <div className="flex justify-between items-center p-3 bg-gray-50 rounded-lg">
                  <span className="font-medium text-gray-700">DNS Queries</span>
                  <span className="font-mono text-lg font-bold text-blue-600">{dnsQueries.toLocaleString()}</span>
                </div>
                <div className="flex justify-between items-center p-3 bg-gray-50 rounded-lg">
                  <span className="font-medium text-gray-700">Cache Hits</span>
                  <span className="font-mono text-lg font-bold text-green-600">{dnsCacheHits.toLocaleString()}</span>
                </div>
                <div className="flex justify-between items-center p-3 bg-gray-50 rounded-lg">
                  <span className="font-medium text-gray-700">Escaper Attempts</span>
                  <span className="font-mono text-lg font-bold text-orange-600">{escaperAttempts.toLocaleString()}</span>
                </div>
              </div>
            </CardContent>
          </Card>

          <Card className="shadow-lg">
            <CardHeader className="bg-gradient-to-r from-blue-50 to-blue-100 border-b border-gray-100">
              <CardTitle className="text-lg font-semibold text-gray-800 flex items-center space-x-2">
                <Network className="w-5 h-5 text-blue-600" />
                <span>Network Traffic</span>
              </CardTitle>
            </CardHeader>
            <CardContent className="p-6">
              <div className="space-y-4">
                <div className="flex justify-between items-center p-3 bg-gray-50 rounded-lg">
                  <span className="font-medium text-gray-700">Total Traffic</span>
                  <span className="font-mono text-lg font-bold text-gray-900">{totalTrafficMB.toFixed(1)} MB</span>
                </div>
                <div className="flex justify-between items-center p-3 bg-gray-50 rounded-lg">
                  <span className="font-medium text-gray-700">Inbound</span>
                  <span className="font-mono text-lg font-bold text-green-600">{(totalTrafficIn / 1024 / 1024).toFixed(1)} MB</span>
                </div>
                <div className="flex justify-between items-center p-3 bg-gray-50 rounded-lg">
                  <span className="font-medium text-gray-700">Outbound</span>
                  <span className="font-mono text-lg font-bold text-blue-600">{(totalTrafficOut / 1024 / 1024).toFixed(1)} MB</span>
                </div>
                <div className="flex justify-between items-center p-3 bg-gray-50 rounded-lg">
                  <span className="font-medium text-gray-700">Traffic Rate</span>
                  <span className="font-mono text-lg font-bold text-purple-600">{(totalTrafficMB / 24).toFixed(1)} MB/h</span>
                </div>
              </div>
            </CardContent>
          </Card>
        </div>

        {/* Advanced Charts */}
        <div className="mb-8">
          <Card className="shadow-lg">
            <CardHeader className="bg-gradient-to-r from-purple-50 to-purple-100 border-b border-gray-200">
              <CardTitle className="text-xl font-bold text-gray-900 flex items-center space-x-2">
                <BarChart className="w-6 h-6 text-purple-600" />
                <span>Advanced Analytics</span>
              </CardTitle>
              <CardDescription className="text-gray-600 font-medium">
                Advanced visualizations including heatmaps, histograms, and correlation analysis
              </CardDescription>
            </CardHeader>
            <CardContent className="p-6">
              <AdvancedCharts metrics={metrics} loading={loading} />
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
                <div className="relative group">
                  <button className="flex items-center space-x-2 px-3 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors">
                    <Download className="w-4 h-4" />
                    <span className="text-sm">Export</span>
                  </button>
                  <div className="absolute right-0 mt-2 w-48 bg-white rounded-md shadow-lg z-10 opacity-0 group-hover:opacity-100 transition-opacity">
                    <div className="py-1">
                      <button
                        onClick={() => exportToCSV(metrics)}
                        className="flex items-center space-x-2 w-full px-4 py-2 text-sm text-gray-700 hover:bg-gray-100"
                      >
                        <FileText className="w-4 h-4" />
                        <span>Export CSV</span>
                      </button>
                      <button
                        onClick={() => exportToJSON(metrics)}
                        className="flex items-center space-x-2 w-full px-4 py-2 text-sm text-gray-700 hover:bg-gray-100"
                      >
                        <FileText className="w-4 h-4" />
                        <span>Export JSON</span>
                      </button>
                      <button
                        onClick={() => exportToPDF(metrics)}
                        className="flex items-center space-x-2 w-full px-4 py-2 text-sm text-gray-700 hover:bg-gray-100"
                      >
                        <FileText className="w-4 h-4" />
                        <span>Export PDF</span>
                      </button>
                    </div>
                  </div>
                </div>
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
                      <th className="text-left p-4 font-semibold text-gray-700">Trend</th>
                      <th className="text-left p-4 font-semibold text-gray-700">Status</th>
                    </tr>
                  </thead>
                  <tbody>
                    {getMetricsByCategory(selectedCategory).map((metric, index) => {
                      const latestValue = getLatestValue(metric);
                      const isHealthy = latestValue > 0;
                      const trend = Math.random() > 0.5 ? 'up' : 'down'; // Mock trend for now
                      
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
                            <div className="flex items-center space-x-1">
                              {trend === 'up' ? (
                                <TrendingUp className="w-4 h-4 text-green-500" />
                              ) : (
                                <TrendingUp className="w-4 h-4 text-red-500 rotate-180" />
                              )}
                              <span className={`text-xs ${trend === 'up' ? 'text-green-600' : 'text-red-600'}`}>
                                {trend === 'up' ? '+' : '-'}{Math.random() * 10 + 1}%
                              </span>
                            </div>
                          </td>
                          <td className="p-4">
                            <div className="flex items-center space-x-2">
                              {isHealthy ? (
                                <CheckCircle className="w-4 h-4 text-green-500" />
                              ) : (
                                <XCircle className="w-4 h-4 text-red-500" />
                              )}
                              <span className={`text-xs font-medium ${isHealthy ? 'text-green-600' : 'text-red-600'}`}>
                                {isHealthy ? 'Healthy' : 'No Data'}
                              </span>
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
