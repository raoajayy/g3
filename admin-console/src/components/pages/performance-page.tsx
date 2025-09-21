'use client';

import { useState, useEffect, useRef } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { TrendingUp, Zap, Clock, Target, RefreshCw, Activity, Server, Network, Database, AlertTriangle, CheckCircle, XCircle, BarChart3 } from 'lucide-react';

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

export function PerformancePage() {
  const [metrics, setMetrics] = useState<Metric[]>([]);
  const [loading, setLoading] = useState(true);
  const [lastUpdated, setLastUpdated] = useState<Date | null>(null);
  const [error, setError] = useState<string | null>(null);
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
    const interval = setInterval(fetchMetrics, 5000); // Refresh every 5 seconds
    return () => clearInterval(interval);
  }, []);

  // Helper function to get the latest value from a metric
  const getLatestValue = (metric: Metric): number => {
    if (metric.values && metric.values.length > 0) {
      return metric.values[metric.values.length - 1].value;
    }
    return 0;
  };

  // Calculate performance metrics
  const totalRequests = getLatestValue(metrics.find(m => m.name === 'server_connection_total') || { values: [] } as Metric);
  const activeConnections = getLatestValue(metrics.find(m => m.name === 'active_connections') || { values: [] } as Metric);
  const serverTaskTotal = getLatestValue(metrics.find(m => m.name === 'server_task_total') || { values: [] } as Metric);
  const escaperSuccess = getLatestValue(metrics.find(m => m.name === 'escaper_connection_establish') || { values: [] } as Metric);
  const escaperAttempts = getLatestValue(metrics.find(m => m.name === 'escaper_connection_attempt') || { values: [] } as Metric);
  const dnsQueries = getLatestValue(metrics.find(m => m.name === 'resolver_query_total') || { values: [] } as Metric);
  const dnsCacheHits = getLatestValue(metrics.find(m => m.name === 'resolver_query_cached') || { values: [] } as Metric);
  const tokioAliveTasks = getLatestValue(metrics.find(m => m.name === 'runtime_tokio_alive_tasks') || { values: [] } as Metric);
  const tokioQueueDepth = getLatestValue(metrics.find(m => m.name === 'runtime_tokio_global_queue_depth') || { values: [] } as Metric);

  // Calculate derived metrics
  const throughput = totalRequests / (24 * 60 * 60); // Approximate RPS
  const successRate = escaperAttempts > 0 ? (escaperSuccess / escaperAttempts) * 100 : 0;
  const dnsCacheHitRate = dnsQueries > 0 ? (dnsCacheHits / dnsQueries) * 100 : 0;
  const efficiency = activeConnections > 0 ? (serverTaskTotal / activeConnections) * 100 : 0;
  const avgResponseTime = 125; // Mock value for now

  return (
    <div className="p-6 bg-gray-50 min-h-screen">
      <div className="max-w-7xl mx-auto">
        <div className="mb-8">
          <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
            <div>
              <h1 className="text-3xl font-bold text-gray-900 mb-2">Performance Monitoring</h1>
              <p className="text-gray-600 text-lg">
                Real-time system performance analysis and bottleneck identification
              </p>
            </div>
            <div className="flex items-center space-x-4">
              <div className="flex items-center space-x-2 px-3 py-2 bg-green-100 text-green-800 rounded-lg">
                <div className="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
                <span className="text-sm font-medium">Live Monitoring</span>
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

        {/* Performance KPIs */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
          <Card className="shadow-lg border-0 bg-gradient-to-br from-yellow-50 to-yellow-100 hover:shadow-xl transition-shadow">
            <CardContent className="p-6">
              <div className="flex items-center justify-between">
                <div className="flex-1">
                  <p className="text-sm font-medium text-yellow-700">Throughput</p>
                  <p className="text-3xl font-bold text-yellow-900">{throughput.toFixed(1)}</p>
                  <p className="text-xs text-yellow-600 mt-1">requests/second</p>
                  <div className="mt-2 text-xs text-yellow-500">
                    Total: {totalRequests.toLocaleString()}
                  </div>
                </div>
                <div className="p-3 bg-yellow-200 rounded-full">
                  <Zap className="w-6 h-6 text-yellow-700" />
                </div>
              </div>
            </CardContent>
          </Card>

          <Card className="shadow-lg border-0 bg-gradient-to-br from-blue-50 to-blue-100 hover:shadow-xl transition-shadow">
            <CardContent className="p-6">
              <div className="flex items-center justify-between">
                <div className="flex-1">
                  <p className="text-sm font-medium text-blue-900">Response Time</p>
                  <p className="text-3xl font-bold text-blue-900">{avgResponseTime}ms</p>
                  <p className="text-xs text-blue-600 mt-1">average latency</p>
                  <div className="mt-2 text-xs text-blue-500">
                    Active: {activeConnections} connections
                  </div>
                </div>
                <div className="p-3 bg-blue-200 rounded-full">
                  <Clock className="w-6 h-6 text-blue-900" />
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
                  <Target className="w-6 h-6 text-green-700" />
                </div>
              </div>
            </CardContent>
          </Card>

          <Card className="shadow-lg border-0 bg-gradient-to-br from-purple-50 to-purple-100 hover:shadow-xl transition-shadow">
            <CardContent className="p-6">
              <div className="flex items-center justify-between">
                <div className="flex-1">
                  <p className="text-sm font-medium text-purple-700">Efficiency</p>
                  <p className="text-3xl font-bold text-purple-900">{efficiency.toFixed(1)}%</p>
                  <p className="text-xs text-purple-600 mt-1">resource utilization</p>
                  <div className="mt-2 text-xs text-purple-500">
                    Tasks: {serverTaskTotal.toLocaleString()}
                  </div>
                </div>
                <div className="p-3 bg-purple-200 rounded-full">
                  <TrendingUp className="w-6 h-6 text-purple-700" />
                </div>
              </div>
            </CardContent>
          </Card>
        </div>

        {/* System Health Status */}
        <div className="mb-8">
          <Card className="shadow-lg">
            <CardHeader className="bg-gradient-to-r from-gray-50 to-gray-100 border-b border-gray-200">
              <CardTitle className="text-xl font-bold text-gray-900 flex items-center space-x-2">
                <Server className="w-6 h-6 text-gray-600" />
                <span>System Health Status</span>
              </CardTitle>
              <CardDescription className="text-gray-600 font-medium">
                Real-time health indicators for all system components
              </CardDescription>
            </CardHeader>
            <CardContent className="p-6">
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                <div className="flex items-center justify-between p-4 bg-gray-50 rounded-lg">
                  <div className="flex items-center space-x-3">
                    <div className={`w-3 h-3 rounded-full ${activeConnections > 0 ? 'bg-green-500' : 'bg-red-500'}`}></div>
                    <span className="font-medium text-gray-700">G3Proxy Server</span>
                  </div>
                  <div className="text-right">
                    <div className="text-sm font-semibold text-gray-900">{activeConnections} active</div>
                    <div className="text-xs text-gray-500">{serverTaskTotal} tasks</div>
                  </div>
                </div>
                
                <div className="flex items-center justify-between p-4 bg-gray-50 rounded-lg">
                  <div className="flex items-center space-x-3">
                    <div className={`w-3 h-3 rounded-full ${dnsCacheHitRate > 50 ? 'bg-green-500' : 'bg-yellow-500'}`}></div>
                    <span className="font-medium text-gray-700">DNS Resolver</span>
                  </div>
                  <div className="text-right">
                    <div className="text-sm font-semibold text-gray-900">{dnsCacheHitRate.toFixed(1)}% cache hit</div>
                    <div className="text-xs text-gray-500">{dnsQueries} queries</div>
                  </div>
                </div>
                
                <div className="flex items-center justify-between p-4 bg-gray-50 rounded-lg">
                  <div className="flex items-center space-x-3">
                    <div className={`w-3 h-3 rounded-full ${successRate > 80 ? 'bg-green-500' : 'bg-yellow-500'}`}></div>
                    <span className="font-medium text-gray-700">Escaper Service</span>
                  </div>
                  <div className="text-right">
                    <div className="text-sm font-semibold text-gray-900">{successRate.toFixed(1)}% success</div>
                    <div className="text-xs text-gray-500">{escaperAttempts} attempts</div>
                  </div>
                </div>
                
                <div className="flex items-center justify-between p-4 bg-gray-50 rounded-lg">
                  <div className="flex items-center space-x-3">
                    <div className={`w-3 h-3 rounded-full ${tokioAliveTasks > 0 ? 'bg-green-500' : 'bg-yellow-500'}`}></div>
                    <span className="font-medium text-gray-700">Runtime Tasks</span>
                  </div>
                  <div className="text-right">
                    <div className="text-sm font-semibold text-gray-900">{tokioAliveTasks} alive</div>
                    <div className="text-xs text-gray-500">Queue: {tokioQueueDepth}</div>
                  </div>
                </div>
              </div>
            </CardContent>
          </Card>
        </div>

        {/* Performance Charts */}
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
                <Activity className="w-6 h-6 text-green-600" />
                <span>Resource Utilization</span>
              </CardTitle>
              <CardDescription className="text-gray-600 font-medium">
                System resource usage and efficiency metrics
              </CardDescription>
            </CardHeader>
            <CardContent className="p-6">
              {loading ? (
                <div className="flex items-center justify-center h-64">
                  <div className="flex items-center space-x-2">
                    <RefreshCw className="w-5 h-5 animate-spin text-blue-600" />
                    <span className="text-gray-600">Loading resource data...</span>
                  </div>
                </div>
              ) : (
                <div className="h-64">
                  <canvas
                    ref={(el) => { chartRefs.current['resource-utilization'] = el; }}
                    width={400}
                    height={200}
                    className="w-full h-full"
                  />
                </div>
              )}
            </CardContent>
          </Card>
        </div>

        {/* Performance Alerts */}
        <Card className="shadow-lg">
          <CardHeader className="bg-gradient-to-r from-orange-50 to-orange-100 border-b border-gray-200">
            <CardTitle className="text-xl font-bold text-gray-900 flex items-center space-x-2">
              <AlertTriangle className="w-6 h-6 text-orange-600" />
              <span>Performance Alerts</span>
            </CardTitle>
            <CardDescription className="text-gray-600 font-medium">
              Critical performance issues and recommendations
            </CardDescription>
          </CardHeader>
          <CardContent className="p-6">
            <div className="space-y-4">
              {successRate < 80 && (
                <div className="flex items-center space-x-3 p-4 bg-red-50 border border-red-200 rounded-lg">
                  <XCircle className="w-5 h-5 text-red-500" />
                  <div>
                    <div className="text-sm font-medium text-red-800">Low Success Rate</div>
                    <div className="text-xs text-red-600">Escaper success rate is {successRate.toFixed(1)}%, below recommended 80%</div>
                  </div>
                </div>
              )}
              
              {dnsCacheHitRate < 50 && (
                <div className="flex items-center space-x-3 p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
                  <AlertTriangle className="w-5 h-5 text-yellow-500" />
                  <div>
                    <div className="text-sm font-medium text-yellow-800">Low DNS Cache Hit Rate</div>
                    <div className="text-xs text-yellow-600">DNS cache hit rate is {dnsCacheHitRate.toFixed(1)}%, consider optimizing DNS configuration</div>
                  </div>
                </div>
              )}
              
              {activeConnections === 0 && (
                <div className="flex items-center space-x-3 p-4 bg-red-50 border border-red-200 rounded-lg">
                  <XCircle className="w-5 h-5 text-red-500" />
                  <div>
                    <div className="text-sm font-medium text-red-800">No Active Connections</div>
                    <div className="text-xs text-red-600">G3Proxy is not receiving any traffic, check configuration and network connectivity</div>
                  </div>
                </div>
              )}
              
              {tokioQueueDepth > 100 && (
                <div className="flex items-center space-x-3 p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
                  <AlertTriangle className="w-5 h-5 text-yellow-500" />
                  <div>
                    <div className="text-sm font-medium text-yellow-800">High Queue Depth</div>
                    <div className="text-xs text-yellow-600">Tokio queue depth is {tokioQueueDepth}, consider scaling resources</div>
                  </div>
                </div>
              )}
              
              {successRate >= 80 && dnsCacheHitRate >= 50 && activeConnections > 0 && tokioQueueDepth <= 100 && (
                <div className="flex items-center space-x-3 p-4 bg-green-50 border border-green-200 rounded-lg">
                  <CheckCircle className="w-5 h-5 text-green-500" />
                  <div>
                    <div className="text-sm font-medium text-green-800">All Systems Performing Well</div>
                    <div className="text-xs text-green-600">No performance alerts at this time</div>
                  </div>
                </div>
              )}
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
