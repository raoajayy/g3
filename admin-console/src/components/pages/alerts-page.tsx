'use client';

import { AlertTriangle, Bell, Clock, CheckCircle, XCircle } from 'lucide-react';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';

interface Alert {
  id: string;
  title: string;
  description: string;
  severity: 'low' | 'medium' | 'high' | 'critical';
  status: 'active' | 'resolved' | 'acknowledged';
  timestamp: string;
  source: string;
}

const mockAlerts: Alert[] = [
  {
    id: '1',
    title: 'High CPU Usage',
    description: 'CPU usage exceeded 85% for more than 5 minutes',
    severity: 'high',
    status: 'active',
    timestamp: '2024-01-15 14:30:25',
    source: 'g3proxy-server-1'
  },
  {
    id: '2',
    title: 'Memory Leak Detected',
    description: 'Memory usage continuously increasing over 2 hours',
    severity: 'critical',
    status: 'acknowledged',
    timestamp: '2024-01-15 13:45:12',
    source: 'g3statsd-daemon'
  },
  {
    id: '3',
    title: 'Network Latency Spike',
    description: 'Average response time increased by 200%',
    severity: 'medium',
    status: 'resolved',
    timestamp: '2024-01-15 12:15:30',
    source: 'network-monitor'
  },
  {
    id: '4',
    title: 'Failed Authentication Attempts',
    description: 'Multiple failed login attempts detected',
    severity: 'medium',
    status: 'active',
    timestamp: '2024-01-15 11:20:45',
    source: 'auth-service'
  },
  {
    id: '5',
    title: 'Disk Space Low',
    description: 'Available disk space below 10%',
    severity: 'high',
    status: 'resolved',
    timestamp: '2024-01-15 10:05:18',
    source: 'storage-monitor'
  }
];

const getSeverityColor = (severity: string) => {
  switch (severity) {
    case 'critical': return 'bg-red-100 text-red-800 border-red-200';
    case 'high': return 'bg-orange-100 text-orange-800 border-orange-200';
    case 'medium': return 'bg-yellow-100 text-yellow-800 border-yellow-200';
    case 'low': return 'bg-blue-100 text-blue-800 border-blue-200';
    default: return 'bg-gray-100 text-gray-800 border-gray-200';
  }
};

const getStatusIcon = (status: string) => {
  switch (status) {
    case 'active': return <XCircle className="w-4 h-4 text-red-500" />;
    case 'acknowledged': return <Clock className="w-4 h-4 text-yellow-500" />;
    case 'resolved': return <CheckCircle className="w-4 h-4 text-green-500" />;
    default: return <Bell className="w-4 h-4 text-gray-500" />;
  }
};

export function AlertsPage() {
  const activeAlerts = mockAlerts.filter(alert => alert.status === 'active');
  const criticalAlerts = mockAlerts.filter(alert => alert.severity === 'critical');
  const resolvedToday = mockAlerts.filter(alert => alert.status === 'resolved').length;

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">System Alerts</h1>
          <p className="text-gray-600 mt-2">Monitor and manage system alerts and notifications</p>
        </div>
        <div className="flex items-center space-x-3">
          <button className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors">
            <Bell className="w-4 h-4 mr-2 inline" />
            Acknowledge All
          </button>
        </div>
      </div>

      {/* Summary Cards */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
        <Card className="p-6">
          <div className="flex items-center">
            <div className="p-2 bg-red-100 rounded-lg">
              <AlertTriangle className="w-6 h-6 text-red-600" />
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-600">Active Alerts</p>
              <p className="text-2xl font-bold text-gray-900">{activeAlerts.length}</p>
            </div>
          </div>
        </Card>

        <Card className="p-6">
          <div className="flex items-center">
            <div className="p-2 bg-red-100 rounded-lg">
              <XCircle className="w-6 h-6 text-red-600" />
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-600">Critical</p>
              <p className="text-2xl font-bold text-red-600">{criticalAlerts.length}</p>
            </div>
          </div>
        </Card>

        <Card className="p-6">
          <div className="flex items-center">
            <div className="p-2 bg-green-100 rounded-lg">
              <CheckCircle className="w-6 h-6 text-green-600" />
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-600">Resolved Today</p>
              <p className="text-2xl font-bold text-green-600">{resolvedToday}</p>
            </div>
          </div>
        </Card>

        <Card className="p-6">
          <div className="flex items-center">
            <div className="p-2 bg-blue-100 rounded-lg">
              <Bell className="w-6 h-6 text-blue-600" />
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-600">Total Alerts</p>
              <p className="text-2xl font-bold text-gray-900">{mockAlerts.length}</p>
            </div>
          </div>
        </Card>
      </div>

      {/* Alerts Table */}
      <Card>
        <div className="p-6 border-b border-gray-200">
          <h2 className="text-lg font-semibold text-gray-900">Recent Alerts</h2>
          <p className="text-sm text-gray-600 mt-1">All system alerts and their current status</p>
        </div>
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Alert
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Severity
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Status
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Source
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Timestamp
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Actions
                </th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {mockAlerts.map((alert) => (
                <tr key={alert.id} className="hover:bg-gray-50">
                  <td className="px-6 py-4">
                    <div>
                      <div className="text-sm font-medium text-gray-900">{alert.title}</div>
                      <div className="text-sm text-gray-500">{alert.description}</div>
                    </div>
                  </td>
                  <td className="px-6 py-4">
                    <Badge className={getSeverityColor(alert.severity)}>
                      {alert.severity.toUpperCase()}
                    </Badge>
                  </td>
                  <td className="px-6 py-4">
                    <div className="flex items-center">
                      {getStatusIcon(alert.status)}
                      <span className="ml-2 text-sm text-gray-900 capitalize">{alert.status}</span>
                    </div>
                  </td>
                  <td className="px-6 py-4 text-sm text-gray-900">{alert.source}</td>
                  <td className="px-6 py-4 text-sm text-gray-500">{alert.timestamp}</td>
                  <td className="px-6 py-4">
                    <div className="flex space-x-2">
                      <button className="text-blue-600 hover:text-blue-900 text-sm font-medium">
                        View
                      </button>
                      {alert.status === 'active' && (
                        <button className="text-green-600 hover:text-green-900 text-sm font-medium">
                          Acknowledge
                        </button>
                      )}
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </Card>
    </div>
  );
}
