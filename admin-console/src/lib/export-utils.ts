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

export const exportToCSV = (metrics: Metric[], filename?: string) => {
  const csvContent = [
    ['Metric Name', 'Category', 'Type', 'Current Value', 'Status', 'Last Updated', 'Tags'],
    ...metrics.map(metric => {
      const latestValue = metric.values?.[0]?.value || 0;
      const status = latestValue === 0 ? 'No Data' : latestValue < 10 ? 'Low Activity' : 'Active';
      const lastUpdated = metric.values?.[0]?.timestamp 
        ? new Date(metric.values[0].timestamp).toISOString() 
        : 'N/A';
      const tags = Object.entries(metric.tags || {}).map(([key, value]) => `${key}:${value}`).join(';');
      
      return [
        metric.name,
        (metric as any).category || 'unknown',
        metric.type,
        latestValue.toString(),
        status,
        lastUpdated,
        tags
      ];
    })
  ].map(row => row.join(',')).join('\n');
  
  const blob = new Blob([csvContent], { type: 'text/csv' });
  const url = window.URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = filename || `g3proxy-metrics-${new Date().toISOString().split('T')[0]}.csv`;
  a.click();
  window.URL.revokeObjectURL(url);
};

export const exportToJSON = (metrics: Metric[], filename?: string) => {
  const jsonData = {
    exportDate: new Date().toISOString(),
    totalMetrics: metrics.length,
    metrics: metrics.map(metric => ({
      name: metric.name,
      category: (metric as any).category || 'unknown',
      type: metric.type,
      currentValue: metric.values?.[0]?.value || 0,
      lastUpdated: metric.values?.[0]?.timestamp 
        ? new Date(metric.values[0].timestamp).toISOString() 
        : null,
      tags: metric.tags || {},
      values: metric.values || []
    }))
  };
  
  const blob = new Blob([JSON.stringify(jsonData, null, 2)], { type: 'application/json' });
  const url = window.URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = filename || `g3proxy-metrics-${new Date().toISOString().split('T')[0]}.json`;
  a.click();
  window.URL.revokeObjectURL(url);
};

export const exportToPDF = async (metrics: Metric[], filename?: string) => {
  // This would require a PDF library like jsPDF
  // For now, we'll create a simple HTML report that can be printed to PDF
  const htmlContent = `
    <!DOCTYPE html>
    <html>
    <head>
      <title>G3Proxy Metrics Report</title>
      <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        .header { text-align: center; margin-bottom: 30px; }
        .summary { display: grid; grid-template-columns: repeat(4, 1fr); gap: 20px; margin-bottom: 30px; }
        .summary-card { border: 1px solid #ddd; padding: 15px; text-align: center; }
        .metrics-table { width: 100%; border-collapse: collapse; }
        .metrics-table th, .metrics-table td { border: 1px solid #ddd; padding: 8px; text-align: left; }
        .metrics-table th { background-color: #f5f5f5; }
        .status-active { color: green; }
        .status-low { color: orange; }
        .status-no-data { color: red; }
      </style>
    </head>
    <body>
      <div class="header">
        <h1>G3Proxy Metrics Report</h1>
        <p>Generated on ${new Date().toLocaleString()}</p>
      </div>
      
      <div class="summary">
        <div class="summary-card">
          <h3>Total Metrics</h3>
          <p>${metrics.length}</p>
        </div>
        <div class="summary-card">
          <h3>Active Metrics</h3>
          <p>${metrics.filter(m => (m.values?.[0]?.value || 0) > 0).length}</p>
        </div>
        <div class="summary-card">
          <h3>Counters</h3>
          <p>${metrics.filter(m => m.type === 'counter').length}</p>
        </div>
        <div class="summary-card">
          <h3>Gauges</h3>
          <p>${metrics.filter(m => m.type === 'gauge').length}</p>
        </div>
      </div>
      
      <table class="metrics-table">
        <thead>
          <tr>
            <th>Metric Name</th>
            <th>Category</th>
            <th>Type</th>
            <th>Current Value</th>
            <th>Status</th>
            <th>Last Updated</th>
          </tr>
        </thead>
        <tbody>
          ${metrics.map(metric => {
            const latestValue = metric.values?.[0]?.value || 0;
            const status = latestValue === 0 ? 'No Data' : latestValue < 10 ? 'Low Activity' : 'Active';
            const statusClass = latestValue === 0 ? 'status-no-data' : latestValue < 10 ? 'status-low' : 'status-active';
            const lastUpdated = metric.values?.[0]?.timestamp 
              ? new Date(metric.values[0].timestamp).toLocaleString() 
              : 'N/A';
            
            return `
              <tr>
                <td>${metric.name}</td>
                <td>${(metric as any).category || 'unknown'}</td>
                <td>${metric.type}</td>
                <td>${latestValue.toLocaleString()}</td>
                <td class="${statusClass}">${status}</td>
                <td>${lastUpdated}</td>
              </tr>
            `;
          }).join('')}
        </tbody>
      </table>
    </body>
    </html>
  `;
  
  const blob = new Blob([htmlContent], { type: 'text/html' });
  const url = window.URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = filename || `g3proxy-metrics-report-${new Date().toISOString().split('T')[0]}.html`;
  a.click();
  window.URL.revokeObjectURL(url);
};

export const generateReport = (metrics: Metric[]) => {
  const totalMetrics = metrics.length;
  const activeMetrics = metrics.filter(m => (m.values?.[0]?.value || 0) > 0).length;
  const counterMetrics = metrics.filter(m => m.type === 'counter').length;
  const gaugeMetrics = metrics.filter(m => m.type === 'gauge').length;
  const histogramMetrics = metrics.filter(m => m.type === 'histogram').length;
  
  const categories = [...new Set(metrics.map(m => (m as any).category).filter(Boolean))];
  const categoryStats = categories.map(category => {
    const categoryMetrics = metrics.filter(m => (m as any).category === category);
    const totalValue = categoryMetrics.reduce((sum, m) => sum + (m.values?.[0]?.value || 0), 0);
    return {
      category,
      count: categoryMetrics.length,
      totalValue,
      avgValue: totalValue / categoryMetrics.length
    };
  });
  
  return {
    summary: {
      totalMetrics,
      activeMetrics,
      inactiveMetrics: totalMetrics - activeMetrics,
      counterMetrics,
      gaugeMetrics,
      histogramMetrics
    },
    categories: categoryStats,
    topMetrics: metrics
      .map(m => ({
        name: m.name,
        value: m.values?.[0]?.value || 0,
        category: (m as any).category || 'unknown'
      }))
      .sort((a, b) => b.value - a.value)
      .slice(0, 10),
    generatedAt: new Date().toISOString()
  };
};
