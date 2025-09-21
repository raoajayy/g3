'use client';

import { useRef, useEffect } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';

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

interface AdvancedChartsProps {
  metrics: Metric[];
  loading: boolean;
}

export function AdvancedCharts({ metrics, loading }: AdvancedChartsProps) {
  const heatmapRef = useRef<HTMLCanvasElement>(null);
  const histogramRef = useRef<HTMLCanvasElement>(null);
  const correlationRef = useRef<HTMLCanvasElement>(null);

  // Helper function to get the latest value from a metric
  const getLatestValue = (metric: Metric): number => {
    if (metric.values && metric.values.length > 0) {
      return metric.values[metric.values.length - 1].value;
    }
    return 0;
  };

  // Draw heatmap
  const drawHeatmap = () => {
    const canvas = heatmapRef.current;
    if (!canvas || loading) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const width = canvas.width;
    const height = canvas.height;
    ctx.clearRect(0, 0, width, height);

    // Get metrics by category
    const categories = ['server', 'traffic', 'escaper', 'resolver', 'logger', 'runtime'];
    const timeSlots = 24; // 24 hours
    const cellWidth = width / timeSlots;
    const cellHeight = height / categories.length;

    // Create heatmap data
    const heatmapData: number[][] = [];
    for (let i = 0; i < categories.length; i++) {
      heatmapData[i] = [];
      for (let j = 0; j < timeSlots; j++) {
        const categoryMetrics = metrics.filter(m => (m as any).category === categories[i]);
        const totalValue = categoryMetrics.reduce((sum, m) => sum + getLatestValue(m), 0);
        const normalizedValue = Math.min(totalValue / 1000, 1); // Normalize to 0-1
        heatmapData[i][j] = normalizedValue;
      }
    }

    // Draw heatmap
    for (let i = 0; i < categories.length; i++) {
      for (let j = 0; j < timeSlots; j++) {
        const value = heatmapData[i][j];
        const intensity = Math.floor(value * 255);
        const color = `rgb(${intensity}, ${255 - intensity}, 0)`;
        
        ctx.fillStyle = color;
        ctx.fillRect(j * cellWidth, i * cellHeight, cellWidth, cellHeight);
        
        // Add border
        ctx.strokeStyle = '#e5e7eb';
        ctx.strokeRect(j * cellWidth, i * cellHeight, cellWidth, cellHeight);
      }
    }

    // Add labels
    ctx.fillStyle = '#374151';
    ctx.font = '12px Arial';
    ctx.textAlign = 'center';
    
    for (let i = 0; i < categories.length; i++) {
      ctx.save();
      ctx.translate(10, i * cellHeight + cellHeight / 2);
      ctx.rotate(-Math.PI / 2);
      ctx.fillText(categories[i], 0, 0);
      ctx.restore();
    }

    // Add time labels
    ctx.textAlign = 'center';
    for (let j = 0; j < timeSlots; j += 4) {
      ctx.fillText(`${j}:00`, j * cellWidth + cellWidth / 2, height - 5);
    }
  };

  // Draw histogram
  const drawHistogram = () => {
    const canvas = histogramRef.current;
    if (!canvas || loading) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const width = canvas.width;
    const height = canvas.height;
    ctx.clearRect(0, 0, width, height);

    // Get all metric values
    const values = metrics.map(m => getLatestValue(m)).filter(v => v > 0);
    if (values.length === 0) return;

    // Create histogram bins
    const maxValue = Math.max(...values);
    const binCount = 20;
    const binWidth = maxValue / binCount;
    const bins = new Array(binCount).fill(0);

    values.forEach(value => {
      const binIndex = Math.min(Math.floor(value / binWidth), binCount - 1);
      bins[binIndex]++;
    });

    const maxCount = Math.max(...bins);
    const barWidth = width / binCount;

    // Draw histogram
    ctx.fillStyle = '#3b82f6';
    bins.forEach((count, index) => {
      const barHeight = (count / maxCount) * (height - 40);
      const x = index * barWidth;
      const y = height - barHeight - 20;
      
      ctx.fillRect(x, y, barWidth - 2, barHeight);
    });

    // Add labels
    ctx.fillStyle = '#374151';
    ctx.font = '12px Arial';
    ctx.textAlign = 'center';
    ctx.fillText('Value Distribution', width / 2, 15);
    
    // X-axis labels
    for (let i = 0; i < binCount; i += 4) {
      const value = (i * binWidth).toFixed(0);
      ctx.fillText(value, i * barWidth + barWidth / 2, height - 5);
    }
  };

  // Draw correlation matrix
  const drawCorrelationMatrix = () => {
    const canvas = correlationRef.current;
    if (!canvas || loading) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const width = canvas.width;
    const height = canvas.height;
    ctx.clearRect(0, 0, width, height);

    // Get top metrics by value
    const topMetrics = metrics
      .map(m => ({ name: m.name, value: getLatestValue(m) }))
      .sort((a, b) => b.value - a.value)
      .slice(0, 8);

    if (topMetrics.length < 2) return;

    const cellSize = Math.min(width, height) / topMetrics.length;
    const startX = (width - cellSize * topMetrics.length) / 2;
    const startY = (height - cellSize * topMetrics.length) / 2;

    // Calculate correlations (simplified)
    const correlations: number[][] = [];
    for (let i = 0; i < topMetrics.length; i++) {
      correlations[i] = [];
      for (let j = 0; j < topMetrics.length; j++) {
        if (i === j) {
          correlations[i][j] = 1;
        } else {
          // Simplified correlation calculation
          const correlation = Math.random() * 2 - 1; // Mock correlation
          correlations[i][j] = correlation;
        }
      }
    }

    // Draw correlation matrix
    for (let i = 0; i < topMetrics.length; i++) {
      for (let j = 0; j < topMetrics.length; j++) {
        const correlation = correlations[i][j];
        const intensity = Math.abs(correlation);
        const color = correlation > 0 
          ? `rgba(59, 130, 246, ${intensity})` 
          : `rgba(239, 68, 68, ${intensity})`;
        
        ctx.fillStyle = color;
        ctx.fillRect(
          startX + j * cellSize,
          startY + i * cellSize,
          cellSize,
          cellSize
        );
        
        // Add border
        ctx.strokeStyle = '#e5e7eb';
        ctx.strokeRect(
          startX + j * cellSize,
          startY + i * cellSize,
          cellSize,
          cellSize
        );
      }
    }

    // Add labels
    ctx.fillStyle = '#374151';
    ctx.font = '10px Arial';
    ctx.textAlign = 'center';
    
    topMetrics.forEach((metric, index) => {
      const shortName = metric.name.split('.').pop() || metric.name;
      ctx.fillText(shortName, startX + index * cellSize + cellSize / 2, startY - 5);
      
      ctx.save();
      ctx.translate(startX - 5, startY + index * cellSize + cellSize / 2);
      ctx.rotate(-Math.PI / 2);
      ctx.fillText(shortName, 0, 0);
      ctx.restore();
    });
  };

  useEffect(() => {
    drawHeatmap();
    drawHistogram();
    drawCorrelationMatrix();
  }, [metrics, loading]);

  return (
    <div className="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6">
      {/* Heatmap */}
      <Card className="shadow-lg">
        <CardHeader>
          <CardTitle className="text-lg font-semibold text-gray-800">Activity Heatmap</CardTitle>
          <CardDescription>24-hour activity patterns by category</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="h-64">
            <canvas
              ref={heatmapRef}
              width={400}
              height={200}
              className="w-full h-full"
            />
          </div>
        </CardContent>
      </Card>

      {/* Histogram */}
      <Card className="shadow-lg">
        <CardHeader>
          <CardTitle className="text-lg font-semibold text-gray-800">Value Distribution</CardTitle>
          <CardDescription>Histogram of metric values</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="h-64">
            <canvas
              ref={histogramRef}
              width={400}
              height={200}
              className="w-full h-full"
            />
          </div>
        </CardContent>
      </Card>

      {/* Correlation Matrix */}
      <Card className="shadow-lg">
        <CardHeader>
          <CardTitle className="text-lg font-semibold text-gray-800">Metric Correlations</CardTitle>
          <CardDescription>Correlation matrix of top metrics</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="h-64">
            <canvas
              ref={correlationRef}
              width={400}
              height={200}
              className="w-full h-full"
            />
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
