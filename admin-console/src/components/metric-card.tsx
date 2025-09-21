import { ReactNode } from 'react';
import { TrendingUp, TrendingDown } from 'lucide-react';

interface MetricCardProps {
  title: string;
  value: string;
  icon: ReactNode;
  trend?: string;
  trendUp?: boolean;
  status?: 'healthy' | 'warning' | 'error';
  subtitle?: string;
  loading?: boolean;
}

export function MetricCard({ 
  title, 
  value, 
  icon, 
  trend, 
  trendUp, 
  status = 'healthy',
  subtitle,
  loading = false 
}: MetricCardProps) {
  const getStatusColor = (status: string) => {
    switch (status) {
      case 'healthy':
        return 'border-green-200 bg-green-50';
      case 'warning':
        return 'border-yellow-200 bg-yellow-50';
      case 'error':
        return 'border-red-200 bg-red-50';
      default:
        return 'border-gray-200 bg-white';
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'healthy':
        return '✅';
      case 'warning':
        return '⚠️';
      case 'error':
        return '❌';
      default:
        return '';
    }
  };

  if (loading) {
    return (
      <div className="bg-white rounded-xl shadow-sm border border-gray-100 p-6 animate-pulse">
        <div className="flex items-center justify-between">
          <div className="flex-1">
            <div className="h-4 bg-gray-200 rounded w-3/4 mb-2"></div>
            <div className="h-8 bg-gray-200 rounded w-1/2 mb-2"></div>
            {subtitle && <div className="h-3 bg-gray-200 rounded w-1/3"></div>}
          </div>
          <div className="w-12 h-12 bg-gray-200 rounded-lg"></div>
        </div>
      </div>
    );
  }

  return (
    <div className={`bg-white rounded-xl shadow-sm border p-6 hover:shadow-md transition-all duration-200 ${getStatusColor(status)}`}>
      <div className="flex items-start justify-between mb-4">
        <div className="flex-1">
          <div className="flex items-center space-x-2 mb-1">
            <p className="text-sm font-medium text-gray-600">{title || 'Unknown'}</p>
            {status !== 'healthy' && (
              <span className="text-xs" title={`Status: ${status}`}>
                {getStatusIcon(status)}
              </span>
            )}
          </div>
          <p className="text-3xl font-bold text-gray-900 mb-1">{value || '0'}</p>
          {subtitle && (
            <p className="text-xs text-gray-500">{subtitle}</p>
          )}
        </div>
        <div className="flex items-center justify-center w-12 h-12 bg-blue-50 rounded-lg flex-shrink-0">
          {icon}
        </div>
      </div>
      
      {trend && (
        <div className="flex items-center justify-between">
          <div className={`flex items-center px-2.5 py-1 rounded-full text-xs font-medium ${
            trendUp 
              ? 'bg-green-100 text-green-700' 
              : 'bg-red-100 text-red-700'
          }`}>
            {trendUp ? (
              <TrendingUp className="w-3 h-3 mr-1" />
            ) : (
              <TrendingDown className="w-3 h-3 mr-1" />
            )}
            {trend}
          </div>
          <span className="text-xs text-gray-500">from last hour</span>
        </div>
      )}
    </div>
  );
}
