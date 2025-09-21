'use client';

import { useState, useEffect } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { MetricChart } from '@/components/metric-chart';
import { MetricCard } from '@/components/metric-card';
import { MetricsEmptyState } from '@/components/empty-state';
import { RefreshCw, Activity, TrendingUp, Clock, Database } from 'lucide-react';

interface MetricValue {
  value: number;
  timestamp: number;
}

interface Metric {
  name: string;
  value: number;
  tags: Record<string, string>;
  type?: string;
  values?: MetricValue[];
}

interface MetricsResponse {
  metrics: Metric[];
  total_count: number;
}

export function DashboardPage() {
  const [metrics, setMetrics] = useState<Metric[]>([]);
  const [loading, setLoading] = useState(true);
  const [lastUpdated, setLastUpdated] = useState<Date | null>(null);
  const [error, setError] = useState<string | null>(null);

  const fetchMetrics = async () => {
    try {
      setError(null);
      console.log('Fetching metrics from /api/metrics...');
      const response = await fetch('/api/metrics');
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      const data: MetricsResponse = await response.json();
      console.log('Received metrics data:', data);
      setMetrics(data.metrics || []);
      setLastUpdated(new Date());
      console.log('Metrics set successfully, count:', data.metrics?.length || 0);
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
    const interval = setInterval(fetchMetrics, 5000);
    return () => clearInterval(interval);
  }, []);

  // Helper function to get the latest value from a metric
  const getLatestValue = (metric: Metric): number => {
    if (metric.values && metric.values.length > 0) {
      return metric.values[metric.values.length - 1].value;
    }
    return metric.value || 0;
  };

  // Calculate metrics from the available data
  console.log('Calculating metrics from:', metrics.length, 'metrics');
  
  const totalRequests = metrics
    .filter(m => m.name.includes('requests_per_second'))
    .reduce((sum, m) => sum + getLatestValue(m), 0);

  const activeConnections = metrics
    .filter(m => m.name.includes('active_connections'))
    .reduce((sum, m) => sum + getLatestValue(m), 0);

  const responseTimeMetrics = metrics.filter(m => m.name.includes('response_time'));
  const avgResponseTime = responseTimeMetrics.length > 0 
    ? responseTimeMetrics.reduce((sum, m) => sum + getLatestValue(m), 0) / responseTimeMetrics.length
    : 0;

  const errorRate = metrics
    .filter(m => m.name.includes('error_rate'))
    .reduce((sum, m) => sum + getLatestValue(m), 0);

  const bytesTransferred = metrics
    .filter(m => m.name.includes('bytes_transferred'))
    .reduce((sum, m) => sum + getLatestValue(m), 0);

  console.log('Calculated values:', {
    totalRequests,
    activeConnections,
    avgResponseTime,
    errorRate,
    bytesTransferred
  });

  return (
    <div className="p-6">
      <div className="max-w-7xl mx-auto">
        {/* Header */}
        <div className="mb-8">
          <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
            <div>
              <h1 className="text-3xl font-bold text-gray-900 mb-2">Dashboard</h1>
              <p className="text-gray-600 text-sm sm:text-base">
                Real-time metrics and statistics from G3StatsD
              </p>
            </div>
            <div className="flex items-center space-x-4">
              <button
                onClick={fetchMetrics}
                disabled={loading}
                className="flex items-center space-x-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                aria-label={loading ? 'Refreshing metrics' : 'Refresh metrics'}
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
          
          {/* Error Display */}
          {error && (
            <div className="mt-4 p-4 bg-red-50 border border-red-200 rounded-lg" role="alert">
              <div className="flex items-center">
                <div className="text-red-600 mr-2" aria-hidden="true">⚠️</div>
                <div className="text-red-800">
                  <strong>Error:</strong> {error}
                </div>
              </div>
            </div>
          )}
        </div>

        {/* Overview Cards */}
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 md:gap-6 mb-8">
          {loading ? (
            <>
              {[1, 2, 3, 4].map((i) => (
                <div key={i} className="bg-white rounded-xl shadow-sm border border-gray-100 p-6 animate-pulse">
                  <div className="flex items-center justify-between">
                    <div className="flex-1">
                      <div className="h-4 bg-gray-200 rounded w-3/4 mb-2"></div>
                      <div className="h-8 bg-gray-200 rounded w-1/2"></div>
                    </div>
                    <div className="w-12 h-12 bg-gray-200 rounded-lg"></div>
                  </div>
                </div>
              ))}
            </>
          ) : (
            <>
              <MetricCard
                title="Total Requests"
                value={totalRequests.toLocaleString()}
                icon={<Activity className="w-5 h-5 text-blue-600" />}
                trend="+12%"
                trendUp={true}
              />
              <MetricCard
                title="Active Connections"
                value={activeConnections.toString()}
                icon={<Database className="w-5 h-5 text-green-600" />}
                trend="+5%"
                trendUp={true}
              />
              <MetricCard
                title="Avg Response Time"
                value={`${avgResponseTime.toFixed(1)}ms`}
                icon={<Clock className="w-5 h-5 text-orange-600" />}
                trend="-8%"
                trendUp={false}
              />
              <MetricCard
                title="Error Rate"
                value={`${(errorRate * 100).toFixed(2)}%`}
                icon={<TrendingUp className="w-5 h-5 text-red-600" />}
                trend={errorRate < 0.02 ? "-5%" : "+2%"}
                trendUp={errorRate < 0.02}
                status={errorRate < 0.02 ? 'healthy' : errorRate < 0.05 ? 'warning' : 'error'}
              />
            </>
          )}
        </div>

        {/* Charts Section */}
        <div className="grid grid-cols-1 xl:grid-cols-2 gap-4 md:gap-6 mb-8">
          <Card className="shadow-sm border border-gray-200">
            <CardHeader className="bg-gradient-to-r from-blue-50 to-indigo-50 border-b border-gray-100">
              <CardTitle className="text-xl font-bold text-gray-900 flex items-center space-x-2">
                <Activity className="w-6 h-6 text-blue-600" />
                <span>Request Rate Over Time</span>
              </CardTitle>
              <CardDescription className="text-gray-600 font-medium">
                Counter metrics showing request patterns and trends
              </CardDescription>
            </CardHeader>
            <CardContent className="p-6">
              {loading ? (
                <div className="flex items-center justify-center h-64">
                  <div className="flex items-center space-x-2">
                    <RefreshCw className="w-5 h-5 animate-spin text-blue-600" />
                    <span className="text-gray-600">Loading chart data...</span>
                  </div>
                </div>
              ) : (
                <MetricChart
                  metrics={metrics.filter(m => m.name.includes('requests_per_second'))}
                  height={300}
                  showLegend={true}
                />
              )}
            </CardContent>
          </Card>

          <Card className="shadow-sm border border-gray-200">
            <CardHeader className="bg-gradient-to-r from-green-50 to-emerald-50 border-b border-gray-100">
              <CardTitle className="text-xl font-bold text-gray-900 flex items-center space-x-2">
                <Database className="w-6 h-6 text-green-600" />
                <span>System Metrics</span>
              </CardTitle>
              <CardDescription className="text-gray-600 font-medium">
                Gauge metrics showing current system state and performance
              </CardDescription>
            </CardHeader>
            <CardContent className="p-6">
              {loading ? (
                <div className="flex items-center justify-center h-64">
                  <div className="flex items-center space-x-2">
                    <RefreshCw className="w-5 h-5 animate-spin text-blue-600" />
                    <span className="text-gray-600">Loading chart data...</span>
                  </div>
                </div>
              ) : (
                <MetricChart
                  metrics={metrics.filter(m => m.name.includes('active_connections') || m.name.includes('response_time'))}
                  height={300}
                  showLegend={true}
                />
              )}
            </CardContent>
          </Card>
        </div>

        {/* System Health Overview */}
        {!loading && (
          <Card className="shadow-lg mb-8">
            <CardHeader className="bg-gradient-to-r from-gray-50 to-gray-100 border-b border-gray-200">
              <CardTitle className="text-xl font-bold text-gray-900 flex items-center space-x-2">
                <Activity className="w-6 h-6 text-gray-600" />
                <span>System Health Overview</span>
              </CardTitle>
              <CardDescription className="text-gray-600 font-medium">
                Real-time system status and performance indicators
              </CardDescription>
            </CardHeader>
            <CardContent className="p-6">
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                <div className="flex items-center justify-between p-4 bg-gray-50 rounded-lg">
                  <div className="flex items-center space-x-3">
                    <div className={`w-3 h-3 rounded-full ${activeConnections > 0 ? 'bg-green-500' : 'bg-red-500'}`}></div>
                    <span className="font-medium text-gray-700">Server Status</span>
                  </div>
                  <div className="text-right">
                    <div className="text-sm font-semibold text-gray-900">{activeConnections > 0 ? 'Active' : 'Inactive'}</div>
                    <div className="text-xs text-gray-500">{activeConnections} connections</div>
                  </div>
                </div>
                
                <div className="flex items-center justify-between p-4 bg-gray-50 rounded-lg">
                  <div className="flex items-center space-x-3">
                    <div className={`w-3 h-3 rounded-full ${errorRate < 0.05 ? 'bg-green-500' : 'bg-yellow-500'}`}></div>
                    <span className="font-medium text-gray-700">Error Rate</span>
                  </div>
                  <div className="text-right">
                    <div className="text-sm font-semibold text-gray-900">{(errorRate * 100).toFixed(2)}%</div>
                    <div className="text-xs text-gray-500">success rate</div>
                  </div>
                </div>
                
                <div className="flex items-center justify-between p-4 bg-gray-50 rounded-lg">
                  <div className="flex items-center space-x-3">
                    <div className={`w-3 h-3 rounded-full ${avgResponseTime < 200 ? 'bg-green-500' : 'bg-yellow-500'}`}></div>
                    <span className="font-medium text-gray-700">Response Time</span>
                  </div>
                  <div className="text-right">
                    <div className="text-sm font-semibold text-gray-900">{avgResponseTime.toFixed(1)}ms</div>
                    <div className="text-xs text-gray-500">average</div>
                  </div>
                </div>
                
                <div className="flex items-center justify-between p-4 bg-gray-50 rounded-lg">
                  <div className="flex items-center space-x-3">
                    <div className={`w-3 h-3 rounded-full ${bytesTransferred > 0 ? 'bg-green-500' : 'bg-gray-500'}`}></div>
                    <span className="font-medium text-gray-700">Data Transfer</span>
                  </div>
                  <div className="text-right">
                    <div className="text-sm font-semibold text-gray-900">{(bytesTransferred / 1024 / 1024).toFixed(1)}MB</div>
                    <div className="text-xs text-gray-500">processed</div>
                  </div>
                </div>
              </div>
            </CardContent>
          </Card>
        )}

        {/* Detailed Metrics Table */}
        <Card>
          <CardHeader>
            <CardTitle>All Metrics</CardTitle>
            <CardDescription>
              Complete list of all available metrics
            </CardDescription>
          </CardHeader>
          <CardContent>
            {loading ? (
              <div className="flex items-center justify-center py-12">
                <div className="flex items-center space-x-2">
                  <RefreshCw className="w-5 h-5 animate-spin text-blue-600" />
                  <span className="text-gray-600">Loading metrics...</span>
                </div>
              </div>
            ) : metrics.length === 0 ? (
              <MetricsEmptyState />
            ) : (
              <div className="overflow-x-auto">
                <table className="w-full text-sm">
                  <thead>
                    <tr className="border-b border-gray-200">
                      <th className="text-left p-4 font-semibold text-gray-700">Name</th>
                      <th className="text-left p-4 font-semibold text-gray-700">Type</th>
                      <th className="text-left p-4 font-semibold text-gray-700">Value</th>
                      <th className="text-left p-4 font-semibold text-gray-700">Tags</th>
                      <th className="text-left p-4 font-semibold text-gray-700">Last Updated</th>
                    </tr>
                  </thead>
                  <tbody>
                    {metrics.map((metric, index) => (
                      <tr key={index} className="border-b border-gray-100 hover:bg-gray-50 transition-colors">
                        <td className="p-4 font-mono text-sm text-gray-900">{metric.name}</td>
                        <td className="p-4">
                          <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
                            metric.name.includes('total') || metric.name.includes('bytes') || metric.name.includes('accepted')
                              ? 'bg-blue-100 text-blue-800' 
                              : 'bg-green-100 text-green-800'
                          }`}>
                            {metric.name.includes('total') || metric.name.includes('bytes') || metric.name.includes('accepted') ? 'counter' : 'gauge'}
                          </span>
                        </td>
                        <td className="p-4 font-mono text-sm font-semibold text-gray-900">
                          {getLatestValue(metric).toFixed(2)}
                        </td>
                        <td className="p-4">
                          <div className="flex flex-wrap gap-1">
                            {Object.entries(metric.tags || {}).map(([key, value]) => (
                              <span
                                key={key}
                                className="inline-flex items-center px-2 py-1 bg-gray-100 text-gray-700 rounded-md text-xs"
                              >
                                {key}: {value}
                              </span>
                            ))}
                          </div>
                        </td>
                        <td className="p-4 text-gray-500 text-sm">
                          {metric.values?.[0]?.timestamp 
                            ? new Date(metric.values[0].timestamp * 1000).toLocaleString()
                            : new Date().toLocaleString()
                          }
                        </td>
                      </tr>
                    ))}
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
