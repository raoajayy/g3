'use client';

import { Key, Users, Shield, Clock, CheckCircle, XCircle } from 'lucide-react';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';

interface AccessRule {
  id: string;
  name: string;
  user: string;
  resource: string;
  permission: 'read' | 'write' | 'admin';
  status: 'active' | 'expired' | 'pending';
  expires: string;
  created: string;
}

const mockAccessRules: AccessRule[] = [
  {
    id: '1',
    name: 'Admin Dashboard Access',
    user: 'admin@arcus.com',
    resource: 'Dashboard',
    permission: 'admin',
    status: 'active',
    expires: '2024-12-31',
    created: '2024-01-15 10:00:00'
  },
  {
    id: '2',
    name: 'Metrics Read Access',
    user: 'analyst@arcus.com',
    resource: 'Metrics API',
    permission: 'read',
    status: 'active',
    expires: '2024-06-30',
    created: '2024-01-15 11:00:00'
  },
  {
    id: '3',
    name: 'Policy Management',
    user: 'security@arcus.com',
    resource: 'Policy Engine',
    permission: 'write',
    status: 'active',
    expires: '2024-08-15',
    created: '2024-01-15 12:00:00'
  },
  {
    id: '4',
    name: 'Log Access',
    user: 'auditor@arcus.com',
    resource: 'System Logs',
    permission: 'read',
    status: 'expired',
    expires: '2024-01-10',
    created: '2024-01-01 09:00:00'
  },
  {
    id: '5',
    name: 'User Management',
    user: 'hr@arcus.com',
    resource: 'User Directory',
    permission: 'write',
    status: 'pending',
    expires: '2024-12-31',
    created: '2024-01-15 14:00:00'
  }
];

export function AccessControlPage() {
  const activeRules = mockAccessRules.filter(rule => rule.status === 'active');
  const expiredRules = mockAccessRules.filter(rule => rule.status === 'expired');
  const pendingRules = mockAccessRules.filter(rule => rule.status === 'pending');

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'active': return <CheckCircle className="w-4 h-4 text-green-500" />;
      case 'expired': return <XCircle className="w-4 h-4 text-red-500" />;
      case 'pending': return <Clock className="w-4 h-4 text-yellow-500" />;
      default: return <Shield className="w-4 h-4 text-gray-500" />;
    }
  };

  const getPermissionColor = (permission: string) => {
    switch (permission) {
      case 'admin': return 'bg-red-100 text-red-800';
      case 'write': return 'bg-yellow-100 text-yellow-800';
      case 'read': return 'bg-blue-100 text-blue-800';
      default: return 'bg-gray-100 text-gray-800';
    }
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">Access Control</h1>
          <p className="text-gray-600 mt-2">Manage user permissions and resource access</p>
        </div>
        <div className="flex items-center space-x-3">
          <button className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors">
            <Key className="w-4 h-4 mr-2 inline" />
            Grant Access
          </button>
        </div>
      </div>

      {/* Summary Cards */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
        <Card className="p-6">
          <div className="flex items-center">
            <div className="p-2 bg-green-100 rounded-lg">
              <CheckCircle className="w-6 h-6 text-green-600" />
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-600">Active Rules</p>
              <p className="text-2xl font-bold text-gray-900">{activeRules.length}</p>
            </div>
          </div>
        </Card>

        <Card className="p-6">
          <div className="flex items-center">
            <div className="p-2 bg-yellow-100 rounded-lg">
              <Clock className="w-6 h-6 text-yellow-600" />
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-600">Pending</p>
              <p className="text-2xl font-bold text-yellow-600">{pendingRules.length}</p>
            </div>
          </div>
        </Card>

        <Card className="p-6">
          <div className="flex items-center">
            <div className="p-2 bg-red-100 rounded-lg">
              <XCircle className="w-6 h-6 text-red-600" />
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-600">Expired</p>
              <p className="text-2xl font-bold text-red-600">{expiredRules.length}</p>
            </div>
          </div>
        </Card>

        <Card className="p-6">
          <div className="flex items-center">
            <div className="p-2 bg-blue-100 rounded-lg">
              <Users className="w-6 h-6 text-blue-600" />
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-600">Total Rules</p>
              <p className="text-2xl font-bold text-gray-900">{mockAccessRules.length}</p>
            </div>
          </div>
        </Card>
      </div>

      {/* Access Rules Table */}
      <Card>
        <div className="p-6 border-b border-gray-200">
          <h2 className="text-lg font-semibold text-gray-900">Access Control Rules</h2>
          <p className="text-sm text-gray-600 mt-1">Manage user permissions and resource access policies</p>
        </div>
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Rule Name
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  User
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Resource
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Permission
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Status
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Expires
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Actions
                </th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {mockAccessRules.map((rule) => (
                <tr key={rule.id} className="hover:bg-gray-50">
                  <td className="px-6 py-4">
                    <div className="text-sm font-medium text-gray-900">{rule.name}</div>
                    <div className="text-sm text-gray-500">Created: {rule.created}</div>
                  </td>
                  <td className="px-6 py-4 text-sm text-gray-900">{rule.user}</td>
                  <td className="px-6 py-4 text-sm text-gray-900">{rule.resource}</td>
                  <td className="px-6 py-4">
                    <Badge className={getPermissionColor(rule.permission)}>
                      {rule.permission.toUpperCase()}
                    </Badge>
                  </td>
                  <td className="px-6 py-4">
                    <div className="flex items-center">
                      {getStatusIcon(rule.status)}
                      <span className="ml-2 text-sm text-gray-900 capitalize">{rule.status}</span>
                    </div>
                  </td>
                  <td className="px-6 py-4 text-sm text-gray-900">{rule.expires}</td>
                  <td className="px-6 py-4">
                    <div className="flex space-x-2">
                      <button className="text-blue-600 hover:text-blue-900 text-sm font-medium">
                        Edit
                      </button>
                      <button className="text-red-600 hover:text-red-900 text-sm font-medium">
                        Revoke
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
