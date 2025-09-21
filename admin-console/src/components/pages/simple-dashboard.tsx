'use client';

import { useState, useEffect, useRef } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { RefreshCw, Activity, Database, Clock, TrendingUp, Server, Download, Upload, Users, Zap } from 'lucide-react';

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

export function Dashboard() {
  const [metrics, setMetrics] = useState<Metric[]>([]);
  const [loading, setLoading] = useState(true);
  const [lastUpdated, setLastUpdated] = useState<Date | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [chartData, setChartData] = useState<any>({});
  const chartRefs = useRef<{ [key: string]: HTMLCanvasElement | null }>({});

  const fetchMetrics = async () => {
    try {
      setError(null);
      console.log('🔄 Fetching metrics from /api/metrics...');
      const response = await fetch('/api/metrics');
      console.log('📡 Response status:', response.status);
      
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      
      const data: MetricsResponse = await response.json();
      console.log('📊 Received metrics data:', data);
      console.log('📈 Metrics count:', data.metrics?.length || 0);
      console.log('🔄 Source:', data.source);
      
      setMetrics(data.metrics || []);
      setLastUpdated(new Date());
      setLoading(false);
      
      console.log('✅ Metrics state updated successfully');
      console.log('📊 Current metrics state:', data.metrics);
    } catch (error) {
      console.error('❌ Failed to fetch metrics:', error);
      setError(error instanceof Error ? error.message : 'Failed to fetch metrics');
      setMetrics([]);
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchMetrics();
    const interval = setInterval(fetchMetrics, 10000); // Refresh every 10 seconds
    return () => clearInterval(interval);
  }, []);

  // Draw charts when metrics data changes
  useEffect(() => {
    if (metrics.length > 0) {
      // Draw charts for each metric
      metrics.forEach(metric => {
        const chartData = getChartData(metric);
        if (chartData.labels.length > 0) {
          const canvasId = `chart-${metric.name}`;
          const colors: { [key: string]: string } = {
            'requests_per_second': '#3b82f6',
            'response_time_seconds': '#ef4444',
            'active_connections': '#10b981',
            'server_connection_total': '#8b5cf6',
            'server_traffic_in_bytes': '#f59e0b',
            'server_traffic_out_bytes': '#06b6d4'
          };
          drawChart(canvasId, chartData, metric.name.replace(/_/g, ' ').toUpperCase(), colors[metric.name] || '#6b7280');
        }
      });
    }
  }, [metrics]);

  // Helper function to get the latest value from a metric
  const getLatestValue = (metric: Metric): number => {
    if (metric.values && metric.values.length > 0) {
      return metric.values[metric.values.length - 1].value;
    }
    return metric.value || 0;
  };

  // Helper function to get chart data for a metric
  const getChartData = (metric: Metric) => {
    if (!metric.values || metric.values.length === 0) {
      return { labels: [], values: [] };
    }
    
    const sortedValues = [...metric.values].sort((a, b) => a.timestamp - b.timestamp);
    return {
      labels: sortedValues.map(v => new Date(v.timestamp).toLocaleTimeString()),
      values: sortedValues.map(v => v.value)
    };
  };

  // Draw simple chart
  const drawChart = (canvasId: string, data: any, label: string, color: string) => {
    const canvas = chartRefs.current[canvasId];
    if (!canvas || !data.labels.length) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Clear canvas
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    const padding = 20;
    const chartWidth = canvas.width - 2 * padding;
    const chartHeight = canvas.height - 2 * padding;

    if (data.values.length === 0) return;

    const maxValue = Math.max(...data.values);
    const minValue = Math.min(...data.values);
    const valueRange = maxValue - minValue || 1;

    // Draw axes
    ctx.strokeStyle = '#e5e7eb';
    ctx.lineWidth = 1;
    ctx.beginPath();
    ctx.moveTo(padding, padding);
    ctx.lineTo(padding, canvas.height - padding);
    ctx.lineTo(canvas.width - padding, canvas.height - padding);
    ctx.stroke();

    // Draw line chart
    ctx.strokeStyle = color;
    ctx.lineWidth = 2;
    ctx.beginPath();

    data.values.forEach((value: number, index: number) => {
      const x = padding + (index / (data.values.length - 1)) * chartWidth;
      const y = canvas.height - padding - ((value - minValue) / valueRange) * chartHeight;
      
      if (index === 0) {
        ctx.moveTo(x, y);
      } else {
        ctx.lineTo(x, y);
      }
    });

    ctx.stroke();

    // Draw points
    ctx.fillStyle = color;
    data.values.forEach((value: number, index: number) => {
      const x = padding + (index / (data.values.length - 1)) * chartWidth;
      const y = canvas.height - padding - ((value - minValue) / valueRange) * chartHeight;
      
      ctx.beginPath();
      ctx.arc(x, y, 3, 0, 2 * Math.PI);
      ctx.fill();
    });

    // Draw title
    ctx.fillStyle = '#374151';
    ctx.font = '12px Arial';
    ctx.textAlign = 'center';
    ctx.fillText(label, canvas.width / 2, 15);
  };

  // Calculate metrics from the available data
  console.log('🧮 Calculating metrics from:', metrics.length, 'metrics');
  console.log('📊 Available metric names:', metrics.map(m => m.name));
  
  // Get current values from InfluxDB v3 metrics
  const requestsPerSecond = metrics
    .find(m => m.name === 'requests_per_second')
    ?.values?.[0]?.value || 0;

  const activeConnections = metrics
    .find(m => m.name === 'active_connections')
    ?.values?.[0]?.value || 0;

  const avgResponseTime = metrics
    .find(m => m.name === 'response_time_seconds')
    ?.values?.[0]?.value || 0;

  const totalConnections = metrics
    .find(m => m.name === 'server_connection_total')
    ?.values?.[0]?.value || 0;

  const trafficInBytes = metrics
    .find(m => m.name === 'server_traffic_in_bytes')
    ?.values?.[0]?.value || 0;

  const trafficOutBytes = metrics
    .find(m => m.name === 'server_traffic_out_bytes')
    ?.values?.[0]?.value || 0;

  console.log('📊 Calculated values:', {
    requestsPerSecond,
    activeConnections,
    avgResponseTime,
    totalConnections,
    trafficInBytes,
    trafficOutBytes,
    metricsCount: metrics.length
  });

  return (
    <div className="p-6 bg-gray-50 min-h-screen">
      <div className="max-w-7xl mx-auto">
        {/* Header */}
        <div className="mb-8">
          <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
            <div>
              <h1 className="text-3xl font-bold text-gray-900 mb-2">G3Proxy Dashboard</h1>
              <p className="text-gray-600 text-sm sm:text-base">
                Real-time metrics from G3Proxy via G3StatsD → InfluxDB v3
              </p>
            </div>
            <div className="flex items-center space-x-4">
              <div className="flex items-center space-x-2 px-3 py-2 bg-green-100 text-green-800 rounded-lg">
                <div className="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
                <span className="text-sm font-medium">Live</span>
              </div>
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
          <Card className="shadow-sm border border-gray-200">
            <CardContent className="p-6">
              <div className="flex items-center justify-between">
                <div className="flex-1">
                  <p className="text-sm font-medium text-gray-600">Requests/Second</p>
                  <p className="text-2xl font-bold text-gray-900">{requestsPerSecond.toFixed(1)}</p>
                </div>
                <Activity className="w-8 h-8 text-blue-600" />
              </div>
            </CardContent>
          </Card>

          <Card className="shadow-sm border border-gray-200">
            <CardContent className="p-6">
              <div className="flex items-center justify-between">
                <div className="flex-1">
                  <p className="text-sm font-medium text-gray-600">Active Connections</p>
                  <p className="text-2xl font-bold text-gray-900">{activeConnections}</p>
                </div>
                <Database className="w-8 h-8 text-green-600" />
              </div>
            </CardContent>
          </Card>

          <Card className="shadow-sm border border-gray-200">
            <CardContent className="p-6">
              <div className="flex items-center justify-between">
                <div className="flex-1">
                  <p className="text-sm font-medium text-gray-600">Response Time</p>
                  <p className="text-2xl font-bold text-gray-900">{(avgResponseTime * 1000).toFixed(1)}ms</p>
                </div>
                <Clock className="w-8 h-8 text-orange-600" />
              </div>
            </CardContent>
          </Card>

          <Card className="shadow-sm border border-gray-200">
            <CardContent className="p-6">
              <div className="flex items-center justify-between">
                <div className="flex-1">
                  <p className="text-sm font-medium text-gray-600">Total Connections</p>
                  <p className="text-2xl font-bold text-gray-900">{totalConnections}</p>
                </div>
                <TrendingUp className="w-8 h-8 text-purple-600" />
              </div>
            </CardContent>
          </Card>
        </div>

        {/* Traffic Cards */}
        <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 md:gap-6 mb-8">
          <Card className="shadow-sm border border-gray-200">
            <CardContent className="p-6">
              <div className="flex items-center justify-between">
                <div className="flex-1">
                  <p className="text-sm font-medium text-gray-600">Traffic In</p>
                  <p className="text-2xl font-bold text-gray-900">{(trafficInBytes / 1024 / 1024).toFixed(2)} MB</p>
                </div>
                <TrendingUp className="w-8 h-8 text-green-600" />
              </div>
            </CardContent>
          </Card>

          <Card className="shadow-sm border border-gray-200">
            <CardContent className="p-6">
              <div className="flex items-center justify-between">
                <div className="flex-1">
                  <p className="text-sm font-medium text-gray-600">Traffic Out</p>
                  <p className="text-2xl font-bold text-gray-900">{(trafficOutBytes / 1024 / 1024).toFixed(2)} MB</p>
                </div>
                <TrendingUp className="w-8 h-8 text-blue-600" />
              </div>
            </CardContent>
          </Card>
        </div>

        {/* Metrics Table */}
        <Card className="shadow-sm border border-gray-200">
          <CardHeader>
            <CardTitle className="text-xl font-semibold text-gray-900">All Metrics</CardTitle>
            <CardDescription>
              Complete list of all available metrics ({metrics.length} total)
            </CardDescription>
          </CardHeader>
          <CardContent className="p-6 pt-0">
            {loading ? (
              <div className="flex items-center justify-center py-12">
                <div className="flex items-center space-x-2">
                  <RefreshCw className="w-5 h-5 animate-spin text-blue-600" />
                  <span className="text-gray-600">Loading metrics...</span>
                </div>
              </div>
            ) : metrics.length === 0 ? (
              <div className="text-center py-12">
                <p className="text-gray-500">No metrics available</p>
              </div>
            ) : (
              <div className="overflow-x-auto">
                <table className="w-full text-sm">
                  <thead>
                    <tr className="border-b border-gray-200">
                      <th className="text-left p-4 font-semibold text-gray-700">Name</th>
                      <th className="text-left p-4 font-semibold text-gray-700">Type</th>
                      <th className="text-left p-4 font-semibold text-gray-700">Latest Value</th>
                      <th className="text-left p-4 font-semibold text-gray-700">Tags</th>
                    </tr>
                  </thead>
                  <tbody>
                    {metrics.map((metric, index) => (
                      <tr key={index} className="border-b border-gray-100 hover:bg-gray-50 transition-colors">
                        <td className="p-4 font-mono text-sm text-gray-900">{metric.name}</td>
                        <td className="p-4">
                          <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
                            metric.type === 'counter' ? 'bg-blue-100 text-blue-800' : 'bg-green-100 text-green-800'
                          }`}>
                            {metric.type || 'unknown'}
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
                                className="inline-flex items-center px-2 py-1 rounded text-xs font-medium bg-gray-100 text-gray-800"
                              >
                                {key}: {value}
                              </span>
                            ))}
                          </div>
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
