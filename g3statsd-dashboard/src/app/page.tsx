'use client';

import { useState, useEffect } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { MetricChart } from '@/components/metric-chart';
import { MetricCard } from '@/components/metric-card';
import { RefreshCw, Activity, TrendingUp, Clock, Database } from 'lucide-react';

interface MetricValue {
  value: number;
  timestamp: number;
}

interface Metric {
  name: string;
  type: string;
  tags: Record<string, string>;
  values: MetricValue[];
}

interface MetricsResponse {
  metrics: Metric[];
  total_count: number;
}

export default function Dashboard() {
  const [metrics, setMetrics] = useState<Metric[]>([]);
  const [loading, setLoading] = useState(true);
  const [lastUpdated, setLastUpdated] = useState<Date | null>(null);

  const fetchMetrics = async () => {
    try {
      const response = await fetch('http://localhost:3001/metrics');
      const data: MetricsResponse = await response.json();
      setMetrics(data.metrics);
      setLastUpdated(new Date());
    } catch (error) {
      console.error('Failed to fetch metrics:', error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchMetrics();
    const interval = setInterval(fetchMetrics, 5000); // Refresh every 5 seconds
    return () => clearInterval(interval);
  }, []);

  const counterMetrics = metrics.filter(m => m.type === 'counter');
  const gaugeMetrics = metrics.filter(m => m.type === 'gauge');

  const totalRequests = counterMetrics
    .filter(m => m.name.includes('requests'))
    .reduce((sum, m) => sum + (m.values[0]?.value || 0), 0);

  const activeConnections = gaugeMetrics
    .find(m => m.name.includes('active_connections'))
    ?.values[0]?.value || 0;

  const avgResponseTime = gaugeMetrics
    .filter(m => m.name.includes('response_time'))
    .reduce((sum, m) => sum + (m.values[0]?.value || 0), 0) / 
    gaugeMetrics.filter(m => m.name.includes('response_time')).length || 0;

  return (
    <div className="min-h-screen bg-gray-50 p-6">
      <div className="max-w-7xl mx-auto">
        {/* Header */}
        <div className="mb-8">
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-3xl font-bold text-gray-900">G3StatsD Dashboard</h1>
              <p className="text-gray-600 mt-2">
                Real-time metrics and statistics from G3StatsD
              </p>
            </div>
            <div className="flex items-center space-x-4">
              <button
                onClick={fetchMetrics}
                disabled={loading}
                className="flex items-center space-x-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50"
              >
                <RefreshCw className={`w-4 h-4 ${loading ? 'animate-spin' : ''}`} />
                <span>Refresh</span>
              </button>
              {lastUpdated && (
                <div className="text-sm text-gray-500">
                  Last updated: {lastUpdated.toLocaleTimeString()}
                </div>
              )}
            </div>
          </div>
        </div>

        {/* Overview Cards */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
          <MetricCard
            title="Total Requests"
            value={totalRequests.toLocaleString()}
            icon={<Activity className="w-5 h-5" />}
            trend="+12%"
            trendUp={true}
          />
          <MetricCard
            title="Active Connections"
            value={activeConnections.toString()}
            icon={<Database className="w-5 h-5" />}
            trend="+5%"
            trendUp={true}
          />
          <MetricCard
            title="Avg Response Time"
            value={`${avgResponseTime.toFixed(1)}ms`}
            icon={<Clock className="w-5 h-5" />}
            trend="-8%"
            trendUp={false}
          />
          <MetricCard
            title="Total Metrics"
            value={metrics.length.toString()}
            icon={<TrendingUp className="w-5 h-5" />}
            trend="+3"
            trendUp={true}
          />
        </div>

        {/* Charts Section */}
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
          <Card>
            <CardHeader>
              <CardTitle>Request Rate Over Time</CardTitle>
              <CardDescription>
                Counter metrics showing request patterns
              </CardDescription>
            </CardHeader>
            <CardContent>
              <MetricChart
                metrics={counterMetrics}
                height={300}
                showLegend={true}
              />
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>System Metrics</CardTitle>
              <CardDescription>
                Gauge metrics showing current system state
              </CardDescription>
            </CardHeader>
            <CardContent>
              <MetricChart
                metrics={gaugeMetrics}
                height={300}
                showLegend={true}
              />
            </CardContent>
          </Card>
        </div>

        {/* Detailed Metrics Table */}
        <Card>
          <CardHeader>
            <CardTitle>All Metrics</CardTitle>
            <CardDescription>
              Complete list of all available metrics
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="overflow-x-auto">
              <table className="w-full text-sm">
                <thead>
                  <tr className="border-b">
                    <th className="text-left p-3">Name</th>
                    <th className="text-left p-3">Type</th>
                    <th className="text-left p-3">Value</th>
                    <th className="text-left p-3">Tags</th>
                    <th className="text-left p-3">Last Updated</th>
                  </tr>
                </thead>
                <tbody>
                  {metrics.map((metric, index) => (
                    <tr key={index} className="border-b hover:bg-gray-50">
                      <td className="p-3 font-mono text-sm">{metric.name}</td>
                      <td className="p-3">
                        <span className={`px-2 py-1 rounded text-xs ${
                          metric.type === 'counter' 
                            ? 'bg-blue-100 text-blue-800' 
                            : 'bg-green-100 text-green-800'
                        }`}>
                          {metric.type}
                        </span>
                      </td>
                      <td className="p-3 font-mono">
                        {metric.values[0]?.value.toFixed(2) || 'N/A'}
                      </td>
                      <td className="p-3">
                        <div className="flex flex-wrap gap-1">
                          {Object.entries(metric.tags).map(([key, value]) => (
                            <span
                              key={key}
                              className="px-2 py-1 bg-gray-100 text-gray-700 rounded text-xs"
                            >
                              {key}: {value}
                            </span>
                          ))}
                        </div>
                      </td>
                      <td className="p-3 text-gray-500">
                        {metric.values[0]?.timestamp 
                          ? new Date(metric.values[0].timestamp * 1000).toLocaleString()
                          : 'N/A'
                        }
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}