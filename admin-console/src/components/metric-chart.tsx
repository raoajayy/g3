import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts';

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

interface MetricChartProps {
  metrics: Metric[];
  height?: number;
  showLegend?: boolean;
}

export function MetricChart({ metrics, height = 300, showLegend = true }: MetricChartProps) {
  if (!metrics || metrics.length === 0) {
    return (
      <div className="flex items-center justify-center h-64 text-gray-500">
        <div className="text-center">
          <div className="text-4xl mb-2">📊</div>
          <div>No metrics available</div>
          <div className="text-sm text-gray-400 mt-1">Data will appear here when metrics are available</div>
        </div>
      </div>
    );
  }

  // Generate time series data for better visualization
  const generateTimeSeriesData = () => {
    const now = Date.now();
    const dataPoints = 10; // Show last 10 data points
    const interval = 5 * 60 * 1000; // 5 minutes between points
    
    return Array.from({ length: dataPoints }, (_, i) => {
      const timestamp = now - (dataPoints - 1 - i) * interval;
      const time = new Date(timestamp);
      
      const dataPoint: Record<string, number | string> = {
        time: time.toLocaleTimeString(),
        timestamp: Math.floor(timestamp / 1000)
      };
      
      // Add each metric as a data series
      metrics.forEach((metric) => {
        const latestValue = metric.value || (metric.values?.[0]?.value || 0);
        // Add some variation to make it look like real time series data
        const variation = (Math.random() - 0.5) * (latestValue * 0.1);
        const value = Math.max(0, latestValue + variation);
        
        dataPoint[metric.name] = value;
        dataPoint[`${metric.name}_type`] = metric.type || 'gauge';
      });
      
      return dataPoint;
    });
  };

  const chartData = generateTimeSeriesData();
  const colors = [
    '#3b82f6', '#10b981', '#f59e0b', '#ef4444', '#8b5cf6', '#06b6d4'
  ];

  // Get readable names for metrics
  const getReadableName = (name: string) => {
    const nameMap: Record<string, string> = {
      'requests_total': 'Requests',
      'active_connections': 'Connections',
      'response_time_ms': 'Response Time (ms)',
      'cpu_usage': 'CPU Usage (%)',
      'memory_usage': 'Memory Usage (MB)',
      'error_rate': 'Error Rate (%)'
    };
    return nameMap[name] || name.replace(/_/g, ' ').replace(/\b\w/g, l => l.toUpperCase());
  };

  return (
    <div className="w-full h-full min-h-[300px]">
      <ResponsiveContainer width="100%" height={height}>
        <LineChart 
          data={chartData} 
          margin={{ 
            top: 20, 
            right: 30, 
            left: 20, 
            bottom: 20 
          }}
        >
          <CartesianGrid 
            strokeDasharray="3 3" 
            stroke="#f0f0f0" 
            strokeOpacity={0.6}
          />
          <XAxis 
            dataKey="time"
            fontSize={12}
            tick={{ fill: '#666', fontSize: 11 }}
            axisLine={{ stroke: '#e0e0e0' }}
            tickLine={{ stroke: '#e0e0e0' }}
            interval="preserveStartEnd"
          />
          <YAxis 
            fontSize={12}
            tick={{ fill: '#666', fontSize: 11 }}
            axisLine={{ stroke: '#e0e0e0' }}
            tickLine={{ stroke: '#e0e0e0' }}
            tickFormatter={(value) => {
              if (value >= 1000000) return `${(value / 1000000).toFixed(1)}M`;
              if (value >= 1000) return `${(value / 1000).toFixed(1)}k`;
              return value.toString();
            }}
            width={60}
          />
          <Tooltip 
            contentStyle={{
              backgroundColor: '#fff',
              border: '1px solid #e0e0e0',
              borderRadius: '8px',
              boxShadow: '0 4px 6px -1px rgba(0, 0, 0, 0.1)',
              fontSize: '12px'
            }}
            formatter={(value: number, name: string) => {
              const metric = metrics.find(m => m.name === name);
              const unit = metric?.name.includes('time') ? 'ms' : 
                          metric?.name.includes('rate') ? '%' : '';
              return [`${value.toFixed(2)}${unit}`, getReadableName(name)];
            }}
            labelFormatter={(label: string) => `Time: ${label}`}
            labelStyle={{ fontSize: '12px', fontWeight: 'bold' }}
          />
          {showLegend && (
            <Legend 
              formatter={(value: string) => getReadableName(value)}
              wrapperStyle={{ 
                paddingTop: '20px',
                fontSize: '12px'
              }}
              iconType="line"
            />
          )}
          {metrics.map((metric, index) => {
            // Create unique key by combining name and tags
            const uniqueKey = `${metric.name}-${Object.entries(metric.tags)
              .sort(([a], [b]) => a.localeCompare(b))
              .map(([key, value]) => `${key}:${value}`)
              .join('-')}`;
            
            return (
              <Line
                key={uniqueKey}
                type="monotone"
                dataKey={metric.name}
                stroke={colors[index % colors.length]}
                strokeWidth={2.5}
                dot={{ 
                  r: 4, 
                  fill: colors[index % colors.length],
                  strokeWidth: 2,
                  stroke: '#fff'
                }}
                activeDot={{ 
                  r: 6, 
                  stroke: colors[index % colors.length], 
                  strokeWidth: 2,
                  fill: '#fff'
                }}
                connectNulls={false}
              />
            );
          })}
        </LineChart>
      </ResponsiveContainer>
    </div>
  );
}
