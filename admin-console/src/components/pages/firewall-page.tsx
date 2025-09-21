'use client';

import { Lock, Plus, Search, Filter, Shield, AlertTriangle } from 'lucide-react';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';

interface FirewallRule {
  id: string;
  name: string;
  action: 'allow' | 'deny';
  protocol: string;
  source: string;
  destination: string;
  port: string;
  status: 'active' | 'inactive';
  priority: number;
  created: string;
}

const mockRules: FirewallRule[] = [
  {
    id: '1',
    name: 'Allow HTTP Traffic',
    action: 'allow',
    protocol: 'TCP',
    source: '0.0.0.0/0',
    destination: '192.168.1.0/24',
    port: '80',
    status: 'active',
    priority: 1,
    created: '2024-01-15 10:30:00'
  },
  {
    id: '2',
    name: 'Allow HTTPS Traffic',
    action: 'allow',
    protocol: 'TCP',
    source: '0.0.0.0/0',
    destination: '192.168.1.0/24',
    port: '443',
    status: 'active',
    priority: 2,
    created: '2024-01-15 10:31:00'
  },
  {
    id: '3',
    name: 'Block Malicious IPs',
    action: 'deny',
    protocol: 'ALL',
    source: '10.0.0.0/8',
    destination: '0.0.0.0/0',
    port: 'ANY',
    status: 'active',
    priority: 3,
    created: '2024-01-15 11:00:00'
  },
  {
    id: '4',
    name: 'Allow SSH Admin',
    action: 'allow',
    protocol: 'TCP',
    source: '192.168.1.100',
    destination: '192.168.1.1',
    port: '22',
    status: 'active',
    priority: 4,
    created: '2024-01-15 11:15:00'
  },
  {
    id: '5',
    name: 'Block FTP',
    action: 'deny',
    protocol: 'TCP',
    source: '0.0.0.0/0',
    destination: '0.0.0.0/0',
    port: '21',
    status: 'inactive',
    priority: 5,
    created: '2024-01-15 12:00:00'
  }
];

export function FirewallPage() {
  const activeRules = mockRules.filter(rule => rule.status === 'active');
  const allowRules = mockRules.filter(rule => rule.action === 'allow');
  const denyRules = mockRules.filter(rule => rule.action === 'deny');

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">Firewall Rules</h1>
          <p className="text-gray-600 mt-2">Manage network security and access control rules</p>
        </div>
        <div className="flex items-center space-x-3">
          <Button variant="outline">
            <Filter className="w-4 h-4 mr-2" />
            Filter
          </Button>
          <Button>
            <Plus className="w-4 h-4 mr-2" />
            Add Rule
          </Button>
        </div>
      </div>

      {/* Summary Cards */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
        <Card className="p-6">
          <div className="flex items-center">
            <div className="p-2 bg-green-100 rounded-lg">
              <Shield className="w-6 h-6 text-green-600" />
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-600">Active Rules</p>
              <p className="text-2xl font-bold text-gray-900">{activeRules.length}</p>
            </div>
          </div>
        </Card>

        <Card className="p-6">
          <div className="flex items-center">
            <div className="p-2 bg-blue-100 rounded-lg">
              <Lock className="w-6 h-6 text-blue-600" />
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-600">Allow Rules</p>
              <p className="text-2xl font-bold text-blue-600">{allowRules.length}</p>
            </div>
          </div>
        </Card>

        <Card className="p-6">
          <div className="flex items-center">
            <div className="p-2 bg-red-100 rounded-lg">
              <AlertTriangle className="w-6 h-6 text-red-600" />
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-600">Deny Rules</p>
              <p className="text-2xl font-bold text-red-600">{denyRules.length}</p>
            </div>
          </div>
        </Card>

        <Card className="p-6">
          <div className="flex items-center">
            <div className="p-2 bg-gray-100 rounded-lg">
              <Lock className="w-6 h-6 text-gray-600" />
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-600">Total Rules</p>
              <p className="text-2xl font-bold text-gray-900">{mockRules.length}</p>
            </div>
          </div>
        </Card>
      </div>

      {/* Search and Filters */}
      <Card className="p-6">
        <div className="flex items-center space-x-4">
          <div className="flex-1">
            <div className="relative">
              <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 w-4 h-4" />
              <Input
                type="text"
                placeholder="Search firewall rules..."
                className="pl-10"
              />
            </div>
          </div>
          <Button variant="outline">Protocol: All</Button>
          <Button variant="outline">Status: All</Button>
          <Button variant="outline">Action: All</Button>
        </div>
      </Card>

      {/* Rules Table */}
      <Card>
        <div className="p-6 border-b border-gray-200">
          <h2 className="text-lg font-semibold text-gray-900">Firewall Rules</h2>
          <p className="text-sm text-gray-600 mt-1">Configure network access and security policies</p>
        </div>
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Rule Name
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Action
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Protocol
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Source
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Destination
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Port
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Status
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Priority
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Actions
                </th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {mockRules.map((rule) => (
                <tr key={rule.id} className="hover:bg-gray-50">
                  <td className="px-6 py-4">
                    <div className="text-sm font-medium text-gray-900">{rule.name}</div>
                    <div className="text-sm text-gray-500">Created: {rule.created}</div>
                  </td>
                  <td className="px-6 py-4">
                    <Badge className={rule.action === 'allow' ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'}>
                      {rule.action.toUpperCase()}
                    </Badge>
                  </td>
                  <td className="px-6 py-4 text-sm text-gray-900">{rule.protocol}</td>
                  <td className="px-6 py-4 text-sm text-gray-900 font-mono">{rule.source}</td>
                  <td className="px-6 py-4 text-sm text-gray-900 font-mono">{rule.destination}</td>
                  <td className="px-6 py-4 text-sm text-gray-900">{rule.port}</td>
                  <td className="px-6 py-4">
                    <Badge className={rule.status === 'active' ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-800'}>
                      {rule.status.toUpperCase()}
                    </Badge>
                  </td>
                  <td className="px-6 py-4 text-sm text-gray-900">{rule.priority}</td>
                  <td className="px-6 py-4">
                    <div className="flex space-x-2">
                      <button className="text-blue-600 hover:text-blue-900 text-sm font-medium">
                        Edit
                      </button>
                      <button className="text-red-600 hover:text-red-900 text-sm font-medium">
                        Delete
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
