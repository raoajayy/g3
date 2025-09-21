import { NextResponse } from 'next/server';

// Real G3Proxy metrics from StatsD/Prometheus
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

// Cache for metrics to avoid hitting external APIs too frequently
let metricsCache: Metric[] = [];
let lastFetch = 0;
const CACHE_DURATION = 5000; // 5 seconds

async function fetchRealMetrics(): Promise<Metric[]> {
  const now = Date.now();
  
  // Return cached data if still fresh
  if (now - lastFetch < CACHE_DURATION && metricsCache.length > 0) {
    return metricsCache;
  }

  try {
    // Fetch from InfluxDB v3 using SQL queries
    console.log('🔄 Fetching metrics from InfluxDB v3...');
    
    const metrics = await fetchInfluxDBv3Metrics();
    
    if (metrics.length > 0) {
      metricsCache = metrics;
      lastFetch = now;
      console.log(`✅ Retrieved ${metrics.length} metrics from InfluxDB v3`);
      return metrics;
    }
  } catch (error) {
    console.log('❌ InfluxDB v3 not available:', error);
  }

  // NO FALLBACK - Only real data allowed
  throw new Error('No real G3Proxy metrics available from InfluxDB v3');
}

async function fetchInfluxDBv3Metrics(): Promise<Metric[]> {
  const metrics: Metric[] = [];
  
  try {
    // Define comprehensive G3Proxy metrics
    const metricQueries = [
      // Server Metrics
      {
        name: 'active_connections',
        query: 'SELECT time, value FROM "g3proxy.server.task.alive" ORDER BY time DESC LIMIT 10',
        type: 'gauge' as const,
        category: 'server'
      },
      {
        name: 'server_connection_total',
        query: 'SELECT time, count FROM "g3proxy.server.connection.total" ORDER BY time DESC LIMIT 10',
        type: 'counter' as const,
        category: 'server'
      },
      {
        name: 'server_task_total',
        query: 'SELECT time, count FROM "g3proxy.server.task.total" ORDER BY time DESC LIMIT 10',
        type: 'counter' as const,
        category: 'server'
      },
      {
        name: 'server_traffic_in_bytes',
        query: 'SELECT time, count FROM "g3proxy.server.traffic.in.bytes" ORDER BY time DESC LIMIT 10',
        type: 'counter' as const,
        category: 'traffic'
      },
      {
        name: 'server_traffic_out_bytes',
        query: 'SELECT time, count FROM "g3proxy.server.traffic.out.bytes" ORDER BY time DESC LIMIT 10',
        type: 'counter' as const,
        category: 'traffic'
      },
      
      // Escaper Metrics
      {
        name: 'escaper_connection_attempt',
        query: 'SELECT time, count FROM "g3proxy.escaper.connection.attempt" ORDER BY time DESC LIMIT 10',
        type: 'counter' as const,
        category: 'escaper'
      },
      {
        name: 'escaper_connection_establish',
        query: 'SELECT time, count FROM "g3proxy.escaper.connection.establish" ORDER BY time DESC LIMIT 10',
        type: 'counter' as const,
        category: 'escaper'
      },
      {
        name: 'escaper_task_total',
        query: 'SELECT time, count FROM "g3proxy.escaper.task.total" ORDER BY time DESC LIMIT 10',
        type: 'counter' as const,
        category: 'escaper'
      },
      {
        name: 'escaper_tcp_connect_attempt',
        query: 'SELECT time, count FROM "g3proxy.escaper.tcp.connect.attempt" ORDER BY time DESC LIMIT 10',
        type: 'counter' as const,
        category: 'escaper'
      },
      {
        name: 'escaper_tcp_connect_error',
        query: 'SELECT time, count FROM "g3proxy.escaper.tcp.connect.error" ORDER BY time DESC LIMIT 10',
        type: 'counter' as const,
        category: 'escaper'
      },
      {
        name: 'escaper_tcp_connect_establish',
        query: 'SELECT time, count FROM "g3proxy.escaper.tcp.connect.establish" ORDER BY time DESC LIMIT 10',
        type: 'counter' as const,
        category: 'escaper'
      },
      {
        name: 'escaper_tcp_connect_success',
        query: 'SELECT time, count FROM "g3proxy.escaper.tcp.connect.success" ORDER BY time DESC LIMIT 10',
        type: 'counter' as const,
        category: 'escaper'
      },
      {
        name: 'escaper_traffic_in_bytes',
        query: 'SELECT time, count FROM "g3proxy.escaper.traffic.in.bytes" ORDER BY time DESC LIMIT 10',
        type: 'counter' as const,
        category: 'traffic'
      },
      {
        name: 'escaper_traffic_out_bytes',
        query: 'SELECT time, count FROM "g3proxy.escaper.traffic.out.bytes" ORDER BY time DESC LIMIT 10',
        type: 'counter' as const,
        category: 'traffic'
      },
      
      // Listener Metrics
      {
        name: 'listen_accepted',
        query: 'SELECT time, count FROM "g3proxy.listen.accepted" ORDER BY time DESC LIMIT 10',
        type: 'counter' as const,
        category: 'listener'
      },
      {
        name: 'listen_instance_count',
        query: 'SELECT time, value FROM "g3proxy.listen.instance.count" ORDER BY time DESC LIMIT 10',
        type: 'gauge' as const,
        category: 'listener'
      },
      
      // Logger Metrics
      {
        name: 'logger_message_drop',
        query: 'SELECT time, count FROM "g3proxy.logger.message.drop" ORDER BY time DESC LIMIT 10',
        type: 'counter' as const,
        category: 'logger'
      },
      {
        name: 'logger_message_pass',
        query: 'SELECT time, count FROM "g3proxy.logger.message.pass" ORDER BY time DESC LIMIT 10',
        type: 'counter' as const,
        category: 'logger'
      },
      {
        name: 'logger_message_total',
        query: 'SELECT time, count FROM "g3proxy.logger.message.total" ORDER BY time DESC LIMIT 10',
        type: 'counter' as const,
        category: 'logger'
      },
      {
        name: 'logger_traffic_pass',
        query: 'SELECT time, count FROM "g3proxy.logger.traffic.pass" ORDER BY time DESC LIMIT 10',
        type: 'counter' as const,
        category: 'logger'
      },
      
      // Resolver Metrics
      {
        name: 'resolver_memory_cache_capacity',
        query: 'SELECT time, value FROM "g3proxy.resolver.memory.cache.capacity" ORDER BY time DESC LIMIT 10',
        type: 'gauge' as const,
        category: 'resolver'
      },
      {
        name: 'resolver_memory_cache_length',
        query: 'SELECT time, value FROM "g3proxy.resolver.memory.cache.length" ORDER BY time DESC LIMIT 10',
        type: 'gauge' as const,
        category: 'resolver'
      },
      {
        name: 'resolver_memory_doing_capacity',
        query: 'SELECT time, value FROM "g3proxy.resolver.memory.doing.capacity" ORDER BY time DESC LIMIT 10',
        type: 'gauge' as const,
        category: 'resolver'
      },
      {
        name: 'resolver_memory_doing_length',
        query: 'SELECT time, value FROM "g3proxy.resolver.memory.doing.length" ORDER BY time DESC LIMIT 10',
        type: 'gauge' as const,
        category: 'resolver'
      },
      {
        name: 'resolver_memory_trash_capacity',
        query: 'SELECT time, value FROM "g3proxy.resolver.memory.trash.capacity" ORDER BY time DESC LIMIT 10',
        type: 'gauge' as const,
        category: 'resolver'
      },
      {
        name: 'resolver_memory_trash_length',
        query: 'SELECT time, value FROM "g3proxy.resolver.memory.trash.length" ORDER BY time DESC LIMIT 10',
        type: 'gauge' as const,
        category: 'resolver'
      },
      {
        name: 'resolver_query_cached',
        query: 'SELECT time, count FROM "g3proxy.resolver.query.cached" ORDER BY time DESC LIMIT 10',
        type: 'counter' as const,
        category: 'resolver'
      },
      {
        name: 'resolver_query_driver_total',
        query: 'SELECT time, count FROM "g3proxy.resolver.query.driver.total" ORDER BY time DESC LIMIT 10',
        type: 'counter' as const,
        category: 'resolver'
      },
      {
        name: 'resolver_query_server_not_found',
        query: 'SELECT time, count FROM "g3proxy.resolver.query.server.not_found" ORDER BY time DESC LIMIT 10',
        type: 'counter' as const,
        category: 'resolver'
      },
      {
        name: 'resolver_query_total',
        query: 'SELECT time, count FROM "g3proxy.resolver.query.total" ORDER BY time DESC LIMIT 10',
        type: 'counter' as const,
        category: 'resolver'
      },
      {
        name: 'resolver_query_trashed',
        query: 'SELECT time, count FROM "g3proxy.resolver.query.trashed" ORDER BY time DESC LIMIT 10',
        type: 'counter' as const,
        category: 'resolver'
      },
      
      // Runtime Metrics
      {
        name: 'runtime_tokio_alive_tasks',
        query: 'SELECT time, value FROM "g3proxy.runtime.tokio.alive_tasks" ORDER BY time DESC LIMIT 10',
        type: 'gauge' as const,
        category: 'runtime'
      },
      {
        name: 'runtime_tokio_global_queue_depth',
        query: 'SELECT time, value FROM "g3proxy.runtime.tokio.global_queue_depth" ORDER BY time DESC LIMIT 10',
        type: 'gauge' as const,
        category: 'runtime'
      }
    ];

    for (const metricQuery of metricQueries) {
      try {
        const response = await fetch('http://localhost:8182/query', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({
            query: metricQuery.query,
            db: 'arcus'
          }),
          signal: AbortSignal.timeout(5000)
        });

        if (response.ok) {
          const data = await response.json();
          
          // Parse the InfluxDB v3 response format
          if (data && data.results && data.results.length > 0) {
            const result = data.results[0];
            if (result.series && result.series.length > 0) {
              const series = result.series[0];
              const values = series.values || [];
              
              const metricValues: MetricValue[] = values.map((row: any[]) => {
                // Find the value column (could be 'count' or 'value')
                const timeIndex = series.columns.indexOf('time');
                const countIndex = series.columns.indexOf('count');
                const valueIndex = series.columns.indexOf('value');
                
                const metricValue = countIndex !== -1 ? parseFloat(row[countIndex]) || 0 : 
                                 valueIndex !== -1 ? parseFloat(row[valueIndex]) || 0 : 0;
                
                return {
                  value: metricValue,
                  timestamp: new Date(row[timeIndex]).getTime()
                };
              });

              if (metricValues.length > 0) {
                metrics.push({
                  name: metricQuery.name,
                  type: metricQuery.type,
                  tags: { daemon_group: 'g3proxy' },
                  category: metricQuery.category,
                  values: metricValues
                });
              }
            }
          }
        } else {
          console.log(`❌ Failed to fetch ${metricQuery.name}: ${response.status}`);
        }
      } catch (error) {
        console.log(`❌ Error fetching ${metricQuery.name}:`, error);
      }
    }

    return metrics;
  } catch (error) {
    console.error('❌ Error fetching from InfluxDB v3:', error);
    throw error;
  }
}

function parsePrometheusMetrics(data: string): Metric[] {
  const metrics: Metric[] = [];
  const lines = data.split('\n').filter(line => line.trim() && !line.startsWith('#'));
  
  // Track metric types from TYPE comments
  const metricTypes: Record<string, string> = {};
  const typeLines = data.split('\n').filter(line => line.startsWith('# TYPE'));
  for (const typeLine of typeLines) {
    const match = typeLine.match(/# TYPE (\w+) (\w+)/);
    if (match) {
      metricTypes[match[1]] = match[2];
    }
  }
  
  for (const line of lines) {
    if (line.includes('g3proxy_')) {
      const parts = line.split(' ');
      if (parts.length >= 2) {
        const metricLine = parts[0];
        const valueStr = parts[1];
        
        // Handle metrics with and without labels
        let name: string;
        let tags: Record<string, string> = {};
        
        if (metricLine.includes('{')) {
          // Has labels
          const [metricName, labels] = metricLine.split('{');
          name = metricName;
          
          // Parse labels
          const labelStr = labels.replace('}', '');
          if (labelStr) {
            labelStr.split(',').forEach(label => {
              const [key, value] = label.split('=');
              if (key && value) {
                tags[key.trim()] = value.trim().replace(/"/g, '');
              }
            });
          }
        } else {
          // No labels
          name = metricLine;
        }
        
        const cleanName = name.replace('g3proxy_', '').replace(/_/g, '.');
        const value = parseFloat(valueStr || '0');
        const ts = Date.now();
        const metricType = metricTypes[name] || 'gauge';
        
        metrics.push({
          name: cleanName,
          type: metricType,
          tags,
          values: [{ value, timestamp: ts }]
        });
      }
    }
  }
  
  return metrics;
}

async function fetchDirectStatsDMetrics(): Promise<Metric[]> {
  // This would connect directly to StatsD UDP port 8125
  // For now, return empty array as we need to implement UDP client
  // In a real implementation, this would:
  // 1. Connect to UDP port 8125
  // 2. Send StatsD commands to query metrics
  // 3. Parse the response
  return [];
}

function parseStatsDMetrics(data: any): Metric[] {
  // Parse StatsD JSON format
  if (Array.isArray(data)) {
    return data.map((item: any) => ({
      name: item.name || item.metric,
      type: item.type || 'counter',
      tags: item.tags || {},
      values: [{
        value: item.value || 0,
        timestamp: item.timestamp || Date.now()
      }]
    }));
  }
  
  if (typeof data === 'object' && data.metrics) {
    return data.metrics.map((item: any) => ({
      name: item.name || item.metric,
      type: item.type || 'counter',
      tags: item.tags || {},
      values: [{
        value: item.value || 0,
        timestamp: item.timestamp || Date.now()
      }]
    }));
  }
  
  return [];
}

// REMOVED: No mock data generation functions allowed

export async function GET() {
  try {
    console.log('🔄 Fetching REAL G3Proxy metrics...');
    
    const metrics = await fetchRealMetrics();
    
    console.log(`✅ Retrieved ${metrics.length} real metrics from G3Proxy`);
    
    return NextResponse.json({
      metrics,
      total_count: metrics.length,
      source: 'g3proxy_live',
      timestamp: new Date().toISOString()
    });
  } catch (error) {
    console.error('❌ Failed to fetch real G3Proxy metrics:', error);
    
    // NO FALLBACK - Return error if no real data available
    return NextResponse.json(
      { 
        error: 'No real G3Proxy metrics available',
        details: 'Prometheus and StatsD endpoints are not accessible',
        metrics: [],
        total_count: 0,
        source: 'none',
        timestamp: new Date().toISOString()
      },
      { status: 503 }
    );
  }
}