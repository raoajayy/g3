'use client';

import { Eye, AlertTriangle, Shield, Clock, CheckCircle, XCircle } from 'lucide-react';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';

interface Threat {
  id: string;
  type: string;
  severity: 'low' | 'medium' | 'high' | 'critical';
  description: string;
  source: string;
  status: 'detected' | 'investigating' | 'resolved' | 'false_positive';
  timestamp: string;
  affectedSystems: string[];
}

const mockThreats: Threat[] = [
  {
    id: '1',
    type: 'Malware Detection',
    severity: 'high',
    description: 'Suspicious executable detected in network traffic',
    source: '192.168.1.50',
    status: 'investigating',
    timestamp: '2024-01-15 14:30:25',
    affectedSystems: ['g3proxy-server-1', 'g3statsd-daemon']
  },
  {
    id: '2',
    type: 'Brute Force Attack',
    severity: 'medium',
    description: 'Multiple failed login attempts from single IP',
    source: '10.0.0.15',
    status: 'detected',
    timestamp: '2024-01-15 13:45:12',
    affectedSystems: ['auth-service']
  },
  {
    id: '3',
    type: 'Data Exfiltration',
    severity: 'critical',
    description: 'Large data transfer to external IP detected',
    source: '192.168.1.25',
    status: 'resolved',
    timestamp: '2024-01-15 12:15:30',
    affectedSystems: ['database-server']
  },
  {
    id: '4',
    type: 'Port Scanning',
    severity: 'low',
    description: 'Systematic port scanning detected',
    source: '203.0.113.10',
    status: 'false_positive',
    timestamp: '2024-01-15 11:20:45',
    affectedSystems: ['firewall']
  },
  {
    id: '5',
    type: 'SQL Injection Attempt',
    severity: 'high',
    description: 'SQL injection pattern detected in web requests',
    source: '198.51.100.5',
    status: 'investigating',
    timestamp: '2024-01-15 10:05:18',
    affectedSystems: ['web-application']
  }
];

export function ThreatsPage() {
  const activeThreats = mockThreats.filter(threat => threat.status === 'detected' || threat.status === 'investigating');
  const criticalThreats = mockThreats.filter(threat => threat.severity === 'critical');
  const resolvedThreats = mockThreats.filter(threat => threat.status === 'resolved');

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
      case 'detected': return <AlertTriangle className="w-4 h-4 text-red-500" />;
      case 'investigating': return <Clock className="w-4 h-4 text-yellow-500" />;
      case 'resolved': return <CheckCircle className="w-4 h-4 text-green-500" />;
      case 'false_positive': return <XCircle className="w-4 h-4 text-gray-500" />;
      default: return <Eye className="w-4 h-4 text-gray-500" />;
    }
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">Threat Detection</h1>
          <p className="text-gray-600 mt-2">Monitor and respond to security threats and attacks</p>
        </div>
        <div className="flex items-center space-x-3">
          <button className="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors">
            <Shield className="w-4 h-4 mr-2 inline" />
            Respond to All
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
              <p className="text-sm font-medium text-gray-600">Active Threats</p>
              <p className="text-2xl font-bold text-gray-900">{activeThreats.length}</p>
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
              <p className="text-2xl font-bold text-red-600">{criticalThreats.length}</p>
            </div>
          </div>
        </Card>

        <Card className="p-6">
          <div className="flex items-center">
            <div className="p-2 bg-green-100 rounded-lg">
              <CheckCircle className="w-6 h-6 text-green-600" />
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-600">Resolved</p>
              <p className="text-2xl font-bold text-green-600">{resolvedThreats.length}</p>
            </div>
          </div>
        </Card>

        <Card className="p-6">
          <div className="flex items-center">
            <div className="p-2 bg-blue-100 rounded-lg">
              <Eye className="w-6 h-6 text-blue-600" />
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-600">Total Threats</p>
              <p className="text-2xl font-bold text-gray-900">{mockThreats.length}</p>
            </div>
          </div>
        </Card>
      </div>

      {/* Threats Table */}
      <Card>
        <div className="p-6 border-b border-gray-200">
          <h2 className="text-lg font-semibold text-gray-900">Security Threats</h2>
          <p className="text-sm text-gray-600 mt-1">Monitor and respond to detected security threats</p>
        </div>
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Threat Type
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Severity
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Description
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Source
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Status
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Affected Systems
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
              {mockThreats.map((threat) => (
                <tr key={threat.id} className="hover:bg-gray-50">
                  <td className="px-6 py-4">
                    <div className="text-sm font-medium text-gray-900">{threat.type}</div>
                  </td>
                  <td className="px-6 py-4">
                    <Badge className={getSeverityColor(threat.severity)}>
                      {threat.severity.toUpperCase()}
                    </Badge>
                  </td>
                  <td className="px-6 py-4">
                    <div className="text-sm text-gray-900 max-w-xs truncate">{threat.description}</div>
                  </td>
                  <td className="px-6 py-4 text-sm text-gray-900 font-mono">{threat.source}</td>
                  <td className="px-6 py-4">
                    <div className="flex items-center">
                      {getStatusIcon(threat.status)}
                      <span className="ml-2 text-sm text-gray-900 capitalize">{threat.status.replace('_', ' ')}</span>
                    </div>
                  </td>
                  <td className="px-6 py-4">
                    <div className="text-sm text-gray-900">
                      {threat.affectedSystems.join(', ')}
                    </div>
                  </td>
                  <td className="px-6 py-4 text-sm text-gray-500">{threat.timestamp}</td>
                  <td className="px-6 py-4">
                    <div className="flex space-x-2">
                      <button className="text-blue-600 hover:text-blue-900 text-sm font-medium">
                        Investigate
                      </button>
                      <button className="text-green-600 hover:text-green-900 text-sm font-medium">
                        Resolve
                      </button>
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
