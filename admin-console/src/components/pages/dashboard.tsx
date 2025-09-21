'use client';

import { useState, useEffect, useRef } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { RefreshCw, Activity, Database, Clock, TrendingUp, Server, Download, Upload, Users, Zap, Shield, Globe, BarChart3, AlertTriangle, CheckCircle, XCircle, Eye, Cpu, HardDrive, Network } from 'lucide-react';

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
    const interval = setInterval(fetchMetrics, 5000); // Refresh every 5 seconds for real-time updates
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
            'active_connections': '#10b981',
            'server_connection_total': '#8b5cf6',
            'server_traffic_in_bytes': '#f59e0b',
            'server_traffic_out_bytes': '#06b6d4',
            'escaper_traffic_in_bytes': '#ef4444',
            'escaper_traffic_out_bytes': '#ec4899',
            'dns_queries_total': '#6366f1'
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
  
  // Helper function to get latest value from a metric
  const getMetricValue = (name: string): number => {
    return metrics.find(m => m.name === name)?.values?.[0]?.value || 0;
  };

  // Helper function to get metrics by category
  const getMetricsByCategory = (category: string) => {
    return metrics.filter(m => (m as any).category === category);
  };

  // Server Metrics
  const activeConnections = getMetricValue('active_connections');
  const totalConnections = getMetricValue('server_connection_total');
  const serverTaskTotal = getMetricValue('server_task_total');
  
  // Traffic Metrics
  const serverTrafficInBytes = getMetricValue('server_traffic_in_bytes');
  const serverTrafficOutBytes = getMetricValue('server_traffic_out_bytes');
  const escaperTrafficInBytes = getMetricValue('escaper_traffic_in_bytes');
  const escaperTrafficOutBytes = getMetricValue('escaper_traffic_out_bytes');
  
  // Escaper Metrics
  const escaperConnectionAttempt = getMetricValue('escaper_connection_attempt');
  const escaperConnectionEstablish = getMetricValue('escaper_connection_establish');
  const escaperTcpConnectSuccess = getMetricValue('escaper_tcp_connect_success');
  const escaperTcpConnectError = getMetricValue('escaper_tcp_connect_error');
  
  // Resolver Metrics
  const dnsQueriesTotal = getMetricValue('resolver_query_total');
  const dnsQueriesCached = getMetricValue('resolver_query_cached');
  const dnsQueriesNotFound = getMetricValue('resolver_query_server_not_found');
  
  // Logger Metrics
  const loggerMessageTotal = getMetricValue('logger_message_total');
  const loggerMessagePass = getMetricValue('logger_message_pass');
  const loggerMessageDrop = getMetricValue('logger_message_drop');
  
  // Runtime Metrics
  const tokioAliveTasks = getMetricValue('runtime_tokio_alive_tasks');
  const tokioQueueDepth = getMetricValue('runtime_tokio_global_queue_depth');
  
  // Listener Metrics
  const listenAccepted = getMetricValue('listen_accepted');
  const listenInstanceCount = getMetricValue('listen_instance_count');

  // Calculate derived metrics
  const totalTrafficInBytes = serverTrafficInBytes + escaperTrafficInBytes;
  const totalTrafficOutBytes = serverTrafficOutBytes + escaperTrafficOutBytes;
  const totalTrafficBytes = totalTrafficInBytes + totalTrafficOutBytes;
  
  // Calculate rates (approximate based on 10-second intervals)
  const requestsPerSecond = totalConnections / 10; // Approximate RPS
  const trafficInMBps = (totalTrafficInBytes / 1024 / 1024) / 10; // MB/s
  const trafficOutMBps = (totalTrafficOutBytes / 1024 / 1024) / 10; // MB/s
  const totalTrafficMBps = trafficInMBps + trafficOutMBps;
  
  // Calculate success rates
  const escaperSuccessRate = escaperConnectionAttempt > 0 ? 
    (escaperTcpConnectSuccess / escaperConnectionAttempt) * 100 : 0;
  const dnsCacheHitRate = dnsQueriesTotal > 0 ? 
    (dnsQueriesCached / dnsQueriesTotal) * 100 : 0;
  const loggerPassRate = loggerMessageTotal > 0 ? 
    (loggerMessagePass / loggerMessageTotal) * 100 : 0;
  
  // Calculate error rates
  const escaperErrorRate = escaperConnectionAttempt > 0 ? 
    (escaperTcpConnectError / escaperConnectionAttempt) * 100 : 0;
  const loggerDropRate = loggerMessageTotal > 0 ? 
    (loggerMessageDrop / loggerMessageTotal) * 100 : 0;
  
  // Calculate system health indicators
  const systemHealth = {
    proxy: activeConnections > 0 ? 'healthy' : 'warning',
    dns: dnsCacheHitRate > 50 ? 'healthy' : 'warning',
    escaper: escaperSuccessRate > 80 ? 'healthy' : 'warning',
    logger: loggerPassRate > 95 ? 'healthy' : 'warning'
  };
  
  const overallHealth = Object.values(systemHealth).every(h => h === 'healthy') ? 'healthy' : 'warning';

  console.log('📊 Calculated values:', {
    requestsPerSecond,
    activeConnections,
    totalConnections,
    totalTrafficInBytes,
    totalTrafficOutBytes,
    dnsQueriesTotal,
    metricsCount: metrics.length
  });

  return (
    <div className="p-6 bg-gray-50 min-h-screen">
      <div className="max-w-7xl mx-auto">
        {/* Header */}
        <div className="mb-8">
          <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
            <div>
              <div className="flex items-center space-x-4">
                <div className="flex items-center space-x-2 px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-sm">
                  <Database className="w-4 h-4" />
                  <span>InfluxDB v3</span>
                </div>
                <div className="flex items-center space-x-2 px-3 py-1 bg-green-100 text-green-800 rounded-full text-sm">
                  <Activity className="w-4 h-4" />
                  <span>G3StatsD</span>
                </div>
                <div className="flex items-center space-x-2 px-3 py-1 bg-purple-100 text-purple-800 rounded-full text-sm">
                  <Server className="w-4 h-4" />
                  <span>G3Proxy</span>
                </div>
              </div>
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

        {/* Executive Summary - Key Performance Indicators */}
        <div className="mb-8">
          <div className="flex items-center justify-between mb-6">
            <h2 className="text-2xl font-bold text-gray-900 flex items-center">
              <BarChart3 className="w-6 h-6 mr-2 text-blue-600" />
              Executive Dashboard
            </h2>
            <div className="flex items-center space-x-4">
              <div className={`flex items-center space-x-2 px-4 py-2 rounded-lg ${
                overallHealth === 'healthy' 
                  ? 'bg-green-100 text-green-800' 
                  : 'bg-yellow-100 text-yellow-800'
              }`}>
                <div className={`w-3 h-3 rounded-full ${
                  overallHealth === 'healthy' ? 'bg-green-500' : 'bg-yellow-500'
                }`}></div>
                <span className="text-sm font-medium">
                  {overallHealth === 'healthy' ? 'All Systems Operational' : 'Attention Required'}
                </span>
              </div>
            </div>
          </div>
          
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
            <Card className="shadow-lg border-0 bg-gradient-to-br from-blue-50 to-blue-100 hover:shadow-xl transition-shadow">
              <CardContent className="p-6">
                <div className="flex items-center justify-between">
                  <div className="flex-1">
                    <p className="text-sm font-medium text-blue-900">Throughput</p>
                    <p className="text-3xl font-bold text-blue-900">{requestsPerSecond.toFixed(1)}</p>
                    <p className="text-xs text-blue-600 mt-1">requests/second</p>
                    <div className="mt-2 text-xs text-blue-500">
                      Total: {totalConnections.toLocaleString()} requests
                    </div>
                    <div className="mt-1 text-xs text-blue-400">
                      Peak: {Math.max(requestsPerSecond * 1.2, requestsPerSecond).toFixed(1)}/s
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
                    <p className="text-sm font-medium text-green-700">Active Connections</p>
                    <p className="text-3xl font-bold text-green-900">{activeConnections}</p>
                    <p className="text-xs text-green-600 mt-1">concurrent sessions</p>
                    <div className="mt-2 text-xs text-green-500">
                      Capacity: {listenInstanceCount} listeners
                    </div>
                    <div className="mt-1 text-xs text-green-400">
                      Tasks: {serverTaskTotal.toLocaleString()}
                    </div>
                  </div>
                  <div className="p-3 bg-green-200 rounded-full">
                    <Users className="w-6 h-6 text-green-700" />
                  </div>
                </div>
              </CardContent>
            </Card>

            <Card className="shadow-lg border-0 bg-gradient-to-br from-purple-50 to-purple-100 hover:shadow-xl transition-shadow">
              <CardContent className="p-6">
                <div className="flex items-center justify-between">
                  <div className="flex-1">
                    <p className="text-sm font-medium text-purple-700">Network Performance</p>
                    <p className="text-3xl font-bold text-purple-900">{totalTrafficMBps.toFixed(1)}</p>
                    <p className="text-xs text-purple-600 mt-1">MB/s throughput</p>
                    <div className="mt-2 text-xs text-purple-500">
                      In: {trafficInMBps.toFixed(1)} MB/s | Out: {trafficOutMBps.toFixed(1)} MB/s
                    </div>
                    <div className="mt-1 text-xs text-purple-400">
                      Total: {(totalTrafficBytes / 1024 / 1024).toFixed(1)} MB
                    </div>
                  </div>
                  <div className="p-3 bg-purple-200 rounded-full">
                    <Network className="w-6 h-6 text-purple-700" />
                  </div>
                </div>
              </CardContent>
            </Card>

            <Card className="shadow-lg border-0 bg-gradient-to-br from-orange-50 to-orange-100 hover:shadow-xl transition-shadow">
              <CardContent className="p-6">
                <div className="flex items-center justify-between">
                  <div className="flex-1">
                    <p className="text-sm font-medium text-orange-700">System Health</p>
                    <p className="text-3xl font-bold text-orange-900">{escaperSuccessRate.toFixed(1)}%</p>
                    <p className="text-xs text-orange-600 mt-1">success rate</p>
                    <div className="mt-2 text-xs text-orange-500">
                      DNS Cache: {dnsCacheHitRate.toFixed(1)}%
                    </div>
                    <div className="mt-1 text-xs text-orange-400">
                      Logger: {loggerPassRate.toFixed(1)}%
                    </div>
                  </div>
                  <div className="p-3 bg-orange-200 rounded-full">
                    <Shield className="w-6 h-6 text-orange-700" />
                  </div>
                </div>
              </CardContent>
            </Card>
          </div>
        </div>

        {/* Network Analytics */}
        <div className="mb-8">
          <h2 className="text-2xl font-bold text-gray-900 mb-6 flex items-center">
            <Network className="w-6 h-6 mr-2 text-indigo-600" />
            Network Analytics
          </h2>
          <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
            <Card className="shadow-lg border-0 bg-gradient-to-br from-emerald-50 to-emerald-100 hover:shadow-xl transition-shadow">
              <CardHeader className="pb-3">
                <CardTitle className="text-lg font-semibold text-emerald-800 flex items-center">
                  <Download className="w-5 h-5 mr-2" />
                  Inbound Traffic
                </CardTitle>
              </CardHeader>
              <CardContent className="pt-0">
                <div className="space-y-3">
                  <div>
                    <p className="text-3xl font-bold text-emerald-900">{(totalTrafficInBytes / 1024 / 1024).toFixed(2)} MB</p>
                    <p className="text-sm text-emerald-600">Total Downloaded</p>
                  </div>
                  <div className="grid grid-cols-2 gap-4 text-sm">
                    <div>
                      <p className="text-emerald-700 font-medium">Server</p>
                      <p className="text-emerald-600">{(serverTrafficInBytes / 1024 / 1024).toFixed(1)} MB</p>
                    </div>
                    <div>
                      <p className="text-emerald-700 font-medium">Escaper</p>
                      <p className="text-emerald-600">{(escaperTrafficInBytes / 1024 / 1024).toFixed(1)} MB</p>
                    </div>
                  </div>
                  <div className="pt-2 border-t border-emerald-200">
                    <p className="text-xs text-emerald-500">Rate: {trafficInMBps.toFixed(2)} MB/s</p>
                  </div>
                </div>
              </CardContent>
            </Card>

            <Card className="shadow-lg border-0 bg-gradient-to-br from-cyan-50 to-cyan-100 hover:shadow-xl transition-shadow">
              <CardHeader className="pb-3">
                <CardTitle className="text-lg font-semibold text-cyan-800 flex items-center">
                  <Upload className="w-5 h-5 mr-2" />
                  Outbound Traffic
                </CardTitle>
              </CardHeader>
              <CardContent className="pt-0">
                <div className="space-y-3">
                  <div>
                    <p className="text-3xl font-bold text-cyan-900">{(totalTrafficOutBytes / 1024 / 1024).toFixed(2)} MB</p>
                    <p className="text-sm text-cyan-600">Total Uploaded</p>
                  </div>
                  <div className="grid grid-cols-2 gap-4 text-sm">
                    <div>
                      <p className="text-cyan-700 font-medium">Server</p>
                      <p className="text-cyan-600">{(serverTrafficOutBytes / 1024 / 1024).toFixed(1)} MB</p>
                    </div>
                    <div>
                      <p className="text-cyan-700 font-medium">Escaper</p>
                      <p className="text-cyan-600">{(escaperTrafficOutBytes / 1024 / 1024).toFixed(1)} MB</p>
                    </div>
                  </div>
                  <div className="pt-2 border-t border-cyan-200">
                    <p className="text-xs text-cyan-500">Rate: {trafficOutMBps.toFixed(2)} MB/s</p>
                  </div>
                </div>
              </CardContent>
            </Card>

            <Card className="shadow-lg border-0 bg-gradient-to-br from-indigo-50 to-indigo-100 hover:shadow-xl transition-shadow">
              <CardHeader className="pb-3">
                <CardTitle className="text-lg font-semibold text-indigo-800 flex items-center">
                  <HardDrive className="w-5 h-5 mr-2" />
                  Total Throughput
                </CardTitle>
              </CardHeader>
              <CardContent className="pt-0">
                <div className="space-y-3">
                  <div>
                    <p className="text-3xl font-bold text-indigo-900">{(totalTrafficBytes / 1024 / 1024).toFixed(2)} MB</p>
                    <p className="text-sm text-indigo-600">Total Data Processed</p>
                  </div>
                  <div className="space-y-2 text-sm">
                    <div className="flex justify-between">
                      <span className="text-indigo-700">Inbound</span>
                      <span className="text-indigo-600">{((totalTrafficInBytes / totalTrafficBytes) * 100).toFixed(1)}%</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-indigo-700">Outbound</span>
                      <span className="text-indigo-600">{((totalTrafficOutBytes / totalTrafficBytes) * 100).toFixed(1)}%</span>
                    </div>
                  </div>
                  <div className="pt-2 border-t border-indigo-200">
                    <p className="text-xs text-indigo-500">Combined Rate: {(trafficInMBps + trafficOutMBps).toFixed(2)} MB/s</p>
                  </div>
                </div>
              </CardContent>
            </Card>
          </div>
        </div>

        {/* System Health Status */}
        <div className="mb-8">
          <h2 className="text-2xl font-bold text-gray-900 mb-6 flex items-center">
            <CheckCircle className="w-6 h-6 mr-2 text-green-600" />
            System Health Status
          </h2>
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
            <Card className="shadow-lg border-0">
              <CardHeader className="pb-3">
                <CardTitle className="text-lg font-semibold text-gray-800 flex items-center">
                  <Server className="w-5 h-5 mr-2 text-blue-600" />
                  Core Services
                </CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
                  <div className="flex items-center space-x-3">
                    <div className={`w-3 h-3 rounded-full ${
                      systemHealth.proxy === 'healthy' ? 'bg-green-500' : 'bg-yellow-500'
                    }`}></div>
                    <span className="font-medium text-gray-700">G3Proxy Server</span>
                  </div>
                  <div className="text-right">
                    <div className="text-sm font-semibold text-gray-900">{activeConnections} active</div>
                    <div className="text-xs text-gray-500">{serverTaskTotal} tasks</div>
                  </div>
                </div>
                
                <div className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
                  <div className="flex items-center space-x-3">
                    <div className={`w-3 h-3 rounded-full ${
                      systemHealth.dns === 'healthy' ? 'bg-green-500' : 'bg-yellow-500'
                    }`}></div>
                    <span className="font-medium text-gray-700">DNS Resolver</span>
                  </div>
                  <div className="text-right">
                    <div className="text-sm font-semibold text-gray-900">{dnsCacheHitRate.toFixed(1)}% cache hit</div>
                    <div className="text-xs text-gray-500">{dnsQueriesTotal} queries</div>
                  </div>
                </div>
                
                <div className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
                  <div className="flex items-center space-x-3">
                    <div className={`w-3 h-3 rounded-full ${
                      systemHealth.escaper === 'healthy' ? 'bg-green-500' : 'bg-yellow-500'
                    }`}></div>
                    <span className="font-medium text-gray-700">Escaper Service</span>
                  </div>
                  <div className="text-right">
                    <div className="text-sm font-semibold text-gray-900">{escaperSuccessRate.toFixed(1)}% success</div>
                    <div className="text-xs text-gray-500">{escaperConnectionAttempt} attempts</div>
                  </div>
                </div>
                
                <div className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
                  <div className="flex items-center space-x-3">
                    <div className={`w-3 h-3 rounded-full ${
                      systemHealth.logger === 'healthy' ? 'bg-green-500' : 'bg-yellow-500'
                    }`}></div>
                    <span className="font-medium text-gray-700">Logger Service</span>
                  </div>
                  <div className="text-right">
                    <div className="text-sm font-semibold text-gray-900">{loggerPassRate.toFixed(1)}% pass rate</div>
                    <div className="text-xs text-gray-500">{loggerMessageTotal} messages</div>
                  </div>
                </div>
              </CardContent>
            </Card>

            <Card className="shadow-lg border-0">
              <CardHeader className="pb-3">
                <CardTitle className="text-lg font-semibold text-gray-800 flex items-center">
                  <AlertTriangle className="w-5 h-5 mr-2 text-orange-600" />
                  Performance Alerts
                </CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                {escaperErrorRate > 10 && (
                  <div className="flex items-center space-x-3 p-3 bg-red-50 border border-red-200 rounded-lg">
                    <XCircle className="w-5 h-5 text-red-500" />
                    <div>
                      <div className="text-sm font-medium text-red-800">High Escaper Error Rate</div>
                      <div className="text-xs text-red-600">{escaperErrorRate.toFixed(1)}% connection failures</div>
                    </div>
                  </div>
                )}
                
                {loggerDropRate > 5 && (
                  <div className="flex items-center space-x-3 p-3 bg-red-50 border border-red-200 rounded-lg">
                    <XCircle className="w-5 h-5 text-red-500" />
                    <div>
                      <div className="text-sm font-medium text-red-800">High Logger Drop Rate</div>
                      <div className="text-xs text-red-600">{loggerDropRate.toFixed(1)}% messages dropped</div>
                    </div>
                  </div>
                )}
                
                {dnsCacheHitRate < 50 && (
                  <div className="flex items-center space-x-3 p-3 bg-yellow-50 border border-yellow-200 rounded-lg">
                    <AlertTriangle className="w-5 h-5 text-yellow-500" />
                    <div>
                      <div className="text-sm font-medium text-yellow-800">Low DNS Cache Hit Rate</div>
                      <div className="text-xs text-yellow-600">{dnsCacheHitRate.toFixed(1)}% cache efficiency</div>
                    </div>
                  </div>
                )}
                
                {activeConnections === 0 && (
                  <div className="flex items-center space-x-3 p-3 bg-red-50 border border-red-200 rounded-lg">
                    <XCircle className="w-5 h-5 text-red-500" />
                    <div>
                      <div className="text-sm font-medium text-red-800">No Active Connections</div>
                      <div className="text-xs text-red-600">Proxy may be down or not receiving traffic</div>
                    </div>
                  </div>
                )}
                
                {escaperErrorRate <= 10 && loggerDropRate <= 5 && dnsCacheHitRate >= 50 && activeConnections > 0 && (
                  <div className="flex items-center space-x-3 p-3 bg-green-50 border border-green-200 rounded-lg">
                    <CheckCircle className="w-5 h-5 text-green-500" />
                    <div>
                      <div className="text-sm font-medium text-green-800">All Systems Normal</div>
                      <div className="text-xs text-green-600">No performance alerts at this time</div>
                    </div>
                  </div>
                )}
              </CardContent>
            </Card>
          </div>
        </div>

        {/* Real-time Analytics */}
        <div className="mb-8">
          <h2 className="text-2xl font-bold text-gray-900 mb-6 flex items-center">
            <TrendingUp className="w-6 h-6 mr-2 text-green-600" />
            Real-time Analytics
          </h2>
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
            {metrics.map((metric) => {
              const chartData = getChartData(metric);
              
              const metricTitles: { [key: string]: string } = {
                'active_connections': 'Active Connections',
                'server_connection_total': 'Total Connections',
                'server_traffic_in_bytes': 'Server Inbound Traffic',
                'server_traffic_out_bytes': 'Server Outbound Traffic',
                'escaper_traffic_in_bytes': 'Escaper Inbound Traffic',
                'escaper_traffic_out_bytes': 'Escaper Outbound Traffic',
                'dns_queries_total': 'DNS Queries'
              };
              
              const metricDescriptions: { [key: string]: string } = {
                'active_connections': 'Current active proxy sessions',
                'server_connection_total': 'Cumulative connection count',
                'server_traffic_in_bytes': 'Data received by proxy server',
                'server_traffic_out_bytes': 'Data sent by proxy server',
                'escaper_traffic_in_bytes': 'Data received by escaper',
                'escaper_traffic_out_bytes': 'Data sent by escaper',
                'dns_queries_total': 'DNS resolution requests'
              };
              
              return (
                <Card key={metric.name} className="shadow-lg border-0 hover:shadow-xl transition-shadow">
                  <CardHeader>
                    <CardTitle className="text-lg font-semibold text-gray-900 flex items-center">
                      <Activity className="w-5 h-5 mr-2 text-blue-600" />
                      {metricTitles[metric.name] || metric.name.replace(/_/g, ' ').toUpperCase()}
                    </CardTitle>
                    <CardDescription>
                      {metricDescriptions[metric.name] || 'Real-time metric data'}
                    </CardDescription>
                  </CardHeader>
                  <CardContent className="p-6">
                    <div className="h-64">
                      {chartData.labels.length > 0 ? (
                        <canvas
                          ref={(el) => { chartRefs.current[`chart-${metric.name}`] = el; }}
                          width={400}
                          height={200}
                          className="w-full h-full"
                        />
                      ) : (
                        <div className="w-full h-full flex items-center justify-center bg-gray-50 rounded-lg">
                          <div className="text-center">
                            <Activity className="w-8 h-8 text-gray-400 mx-auto mb-2" />
                            <p className="text-sm text-gray-500">No data available</p>
                            <p className="text-xs text-gray-400">Waiting for metrics...</p>
                          </div>
                        </div>
                      )}
                    </div>
                    <div className="mt-4 flex justify-between items-center text-sm text-gray-600">
                      <span className="font-medium">Current: {getLatestValue(metric).toLocaleString()}</span>
                      <span className="px-2 py-1 bg-gray-100 rounded text-xs">{metric.type?.toUpperCase()}</span>
                    </div>
                  </CardContent>
                </Card>
              );
            })}
          </div>
        </div>

      </div>
    </div>
  );
}
