'use client';

import React, { useState, useEffect, useCallback } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Badge } from '@/components/ui/badge';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription, DialogFooter } from '@/components/ui/dialog';
import { LoadingOverlay, LoadingButton } from '@/components/ui/loading';
import { PolicyEditor } from '@/components/policy-editor';
import { 
  Plus, 
  Search, 
  Filter, 
  MoreVertical, 
  Edit, 
  Trash2, 
  Play, 
  Pause,
  Shield,
  AlertTriangle,
  CheckCircle,
  Clock,
  FileText,
  Eye,
  Settings,
  Globe,
  Users,
  Copy,
  Download,
  Upload,
  RefreshCw,
  X,
  Check,
  AlertCircle
} from 'lucide-react';

interface Policy {
  id: string;
  name: string;
  description: string;
  status: 'active' | 'inactive' | 'draft';
  priority: 'critical' | 'high' | 'medium' | 'low';
  lastModified: string;
  createdBy: string;
  type: 'url-filtering' | 'content-security' | 'traffic-control' | 'https-inspection';
  enabled: boolean;
}

interface NotificationState {
  type: 'success' | 'error' | 'warning' | 'info';
  message: string;
  show: boolean;
}

const mockPolicies: Policy[] = [
  {
    id: '1',
    name: 'Block Malware Sites',
    description: 'Blocks access to known malware and phishing sites',
    status: 'active',
    priority: 'critical',
    lastModified: '2024-01-15T10:30:00Z',
    createdBy: 'admin@company.com',
    type: 'url-filtering',
    enabled: true
  },
  {
    id: '2',
    name: 'Social Media Warning',
    description: 'Shows warning page for social media sites during work hours',
    status: 'active',
    priority: 'medium',
    lastModified: '2024-01-14T14:20:00Z',
    createdBy: 'security@company.com',
    type: 'url-filtering',
    enabled: true
  },
  {
    id: '3',
    name: 'HTTPS Inspection',
    description: 'Inspects HTTPS traffic for security threats',
    status: 'active',
    priority: 'high',
    lastModified: '2024-01-13T09:15:00Z',
    createdBy: 'admin@company.com',
    type: 'https-inspection',
    enabled: true
  },
  {
    id: '4',
    name: 'Bandwidth Limits',
    description: 'Enforces bandwidth limits per user and group',
    status: 'draft',
    priority: 'low',
    lastModified: '2024-01-12T16:45:00Z',
    createdBy: 'admin@company.com',
    type: 'traffic-control',
    enabled: false
  },
  {
    id: '5',
    name: 'Data Loss Prevention',
    description: 'Scans content for sensitive data patterns',
    status: 'inactive',
    priority: 'high',
    lastModified: '2024-01-11T11:30:00Z',
    createdBy: 'security@company.com',
    type: 'content-security',
    enabled: false
  }
];

const statusConfig = {
  active: { icon: CheckCircle, color: 'text-green-600', bg: 'bg-green-50' },
  inactive: { icon: Pause, color: 'text-gray-600', bg: 'bg-gray-50' },
  draft: { icon: Clock, color: 'text-yellow-600', bg: 'bg-yellow-50' }
};

const priorityConfig = {
  critical: { color: 'text-red-600', bg: 'bg-red-50' },
  high: { color: 'text-orange-600', bg: 'bg-orange-50' },
  medium: { color: 'text-blue-600', bg: 'bg-blue-50' },
  low: { color: 'text-gray-600', bg: 'bg-gray-50' }
};

const typeConfig = {
  'url-filtering': { icon: Globe, color: 'text-blue-600' },
  'content-security': { icon: AlertTriangle, color: 'text-red-600' },
  'traffic-control': { icon: Clock, color: 'text-green-600' },
  'https-inspection': { icon: Settings, color: 'text-purple-600' }
};

export function PoliciesPageEnhanced() {
  const [policies, setPolicies] = useState<Policy[]>(mockPolicies);
  const [searchTerm, setSearchTerm] = useState('');
  const [statusFilter, setStatusFilter] = useState<string>('all');
  const [typeFilter, setTypeFilter] = useState<string>('all');
  const [priorityFilter, setPriorityFilter] = useState<string>('all');
  const [selectedPolicy, setSelectedPolicy] = useState<Policy | null>(null);
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [showEditModal, setShowEditModal] = useState(false);
  const [showViewModal, setShowViewModal] = useState(false);
  const [showDeleteDialog, setShowDeleteDialog] = useState(false);
  const [policyToDelete, setPolicyToDelete] = useState<Policy | null>(null);
  const [loading, setLoading] = useState(false);
  const [actionLoading, setActionLoading] = useState<string | null>(null);
  const [selectedPolicies, setSelectedPolicies] = useState<Set<string>>(new Set());
  const [notification, setNotification] = useState<NotificationState>({
    type: 'info',
    message: '',
    show: false
  });
  const [currentPage, setCurrentPage] = useState(1);
  const [itemsPerPage] = useState(10);

  // Load policies from API
  useEffect(() => {
    loadPolicies();
  }, []);

  const showNotification = useCallback((type: NotificationState['type'], message: string) => {
    setNotification({ type, message, show: true });
    setTimeout(() => {
      setNotification(prev => ({ ...prev, show: false }));
    }, 5000);
  }, []);

  const loadPolicies = async () => {
    setLoading(true);
    try {
      const response = await fetch('/api/policies');
      if (response.ok) {
        const data = await response.json();
        setPolicies(data.policies || []);
        showNotification('success', 'Policies loaded successfully');
      } else {
        throw new Error('Failed to load policies');
      }
    } catch (error) {
      console.error('Failed to load policies:', error);
      showNotification('error', 'Failed to load policies');
    } finally {
      setLoading(false);
    }
  };

  const filteredPolicies = policies.filter(policy => {
    const matchesSearch = policy.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
                         policy.description.toLowerCase().includes(searchTerm.toLowerCase());
    const matchesStatus = statusFilter === 'all' || policy.status === statusFilter;
    const matchesType = typeFilter === 'all' || policy.type === typeFilter;
    const matchesPriority = priorityFilter === 'all' || policy.priority === priorityFilter;
    
    return matchesSearch && matchesStatus && matchesType && matchesPriority;
  });

  const paginatedPolicies = filteredPolicies.slice(
    (currentPage - 1) * itemsPerPage,
    currentPage * itemsPerPage
  );

  const totalPages = Math.ceil(filteredPolicies.length / itemsPerPage);

  const handleStatusToggle = async (policyId: string) => {
    setActionLoading(policyId);
    const policy = policies.find(p => p.id === policyId);
    if (!policy) return;

    const newStatus = policy.status === 'active' ? 'inactive' : 'active';
    
    try {
      const response = await fetch(`/api/policies/${policyId}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ ...policy, status: newStatus })
      });

      if (response.ok) {
        setPolicies(prev => prev.map(p => 
          p.id === policyId ? { ...p, status: newStatus } : p
        ));
        showNotification('success', `Policy ${newStatus === 'active' ? 'activated' : 'deactivated'} successfully`);
      } else {
        throw new Error('Failed to update policy status');
      }
    } catch (error) {
      console.error('Failed to update policy status:', error);
      showNotification('error', 'Failed to update policy status');
    } finally {
      setActionLoading(null);
    }
  };

  const handleDeletePolicy = async (policy: Policy) => {
    setPolicyToDelete(policy);
    setShowDeleteDialog(true);
  };

  const confirmDeletePolicy = async () => {
    if (!policyToDelete) return;

    setActionLoading(policyToDelete.id);
    try {
      const response = await fetch(`/api/policies/${policyToDelete.id}`, {
        method: 'DELETE'
      });

      if (response.ok) {
        setPolicies(prev => prev.filter(p => p.id !== policyToDelete.id));
        showNotification('success', 'Policy deleted successfully');
        setShowDeleteDialog(false);
        setPolicyToDelete(null);
      } else {
        throw new Error('Failed to delete policy');
      }
    } catch (error) {
      console.error('Failed to delete policy:', error);
      showNotification('error', 'Failed to delete policy');
    } finally {
      setActionLoading(null);
    }
  };

  const handleSavePolicy = async (policyData: any) => {
    setActionLoading('save');
    try {
      const url = selectedPolicy ? `/api/policies/${selectedPolicy.id}` : '/api/policies';
      const method = selectedPolicy ? 'PUT' : 'POST';
      
      const response = await fetch(url, {
        method,
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(policyData)
      });

      if (response.ok) {
        await loadPolicies();
        setShowCreateModal(false);
        setShowEditModal(false);
        setSelectedPolicy(null);
        showNotification('success', `Policy ${selectedPolicy ? 'updated' : 'created'} successfully`);
      } else {
        throw new Error(`Failed to ${selectedPolicy ? 'update' : 'create'} policy`);
      }
    } catch (error) {
      console.error('Failed to save policy:', error);
      showNotification('error', `Failed to ${selectedPolicy ? 'update' : 'create'} policy`);
    } finally {
      setActionLoading(null);
    }
  };

  const handleEditPolicy = (policy: Policy) => {
    setSelectedPolicy(policy);
    setShowEditModal(true);
  };

  const handleViewPolicy = (policy: Policy) => {
    setSelectedPolicy(policy);
    setShowViewModal(true);
  };

  const handleCreatePolicy = () => {
    setSelectedPolicy(null);
    setShowCreateModal(true);
  };

  const handleCopyPolicy = (policy: Policy) => {
    const copiedPolicy = {
      ...policy,
      id: '',
      name: `${policy.name} (Copy)`,
      status: 'draft' as const,
      lastModified: new Date().toISOString(),
      createdBy: 'current-user@company.com'
    };
    setSelectedPolicy(copiedPolicy);
    setShowEditModal(true);
    showNotification('info', 'Policy copied for editing');
  };

  const handleBulkAction = async (action: 'activate' | 'deactivate' | 'delete') => {
    if (selectedPolicies.size === 0) return;

    setActionLoading('bulk');
    try {
      const promises = Array.from(selectedPolicies).map(async (policyId) => {
        const policy = policies.find(p => p.id === policyId);
        if (!policy) return;

        if (action === 'delete') {
          return fetch(`/api/policies/${policyId}`, { method: 'DELETE' });
        } else {
          const newStatus = action === 'activate' ? 'active' : 'inactive';
          return fetch(`/api/policies/${policyId}`, {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ ...policy, status: newStatus })
          });
        }
      });

      await Promise.all(promises);
      await loadPolicies();
      setSelectedPolicies(new Set());
      showNotification('success', `Bulk ${action} completed successfully`);
    } catch (error) {
      console.error(`Failed to perform bulk ${action}:`, error);
      showNotification('error', `Failed to perform bulk ${action}`);
    } finally {
      setActionLoading(null);
    }
  };

  const handleSelectPolicy = (policyId: string) => {
    setSelectedPolicies(prev => {
      const newSet = new Set(prev);
      if (newSet.has(policyId)) {
        newSet.delete(policyId);
      } else {
        newSet.add(policyId);
      }
      return newSet;
    });
  };

  const handleSelectAll = () => {
    if (selectedPolicies.size === paginatedPolicies.length) {
      setSelectedPolicies(new Set());
    } else {
      setSelectedPolicies(new Set(paginatedPolicies.map(p => p.id)));
    }
  };

  const closeModals = () => {
    setShowCreateModal(false);
    setShowEditModal(false);
    setShowViewModal(false);
    setShowDeleteDialog(false);
    setSelectedPolicy(null);
    setPolicyToDelete(null);
  };

  return (
    <div className="space-y-6">
      {/* Notification */}
      {notification.show && (
        <Alert variant={notification.type === 'error' ? 'destructive' : notification.type}>
          <AlertCircle className="h-4 w-4" />
          <AlertDescription>{notification.message}</AlertDescription>
        </Alert>
      )}

      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">Security Policies</h1>
          <p className="text-gray-600">Manage web security policies and rules</p>
        </div>
        <div className="flex gap-2">
          <Button variant="outline" onClick={loadPolicies} disabled={loading}>
            <RefreshCw className={`w-4 h-4 mr-2 ${loading ? 'animate-spin' : ''}`} />
            Refresh
          </Button>
          <Button onClick={handleCreatePolicy} className="flex items-center gap-2">
            <Plus className="w-4 h-4" />
            Create Policy
          </Button>
        </div>
      </div>

      {/* Stats Cards */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
        <Card>
          <CardContent className="p-6">
            <div className="flex items-center">
              <Shield className="h-8 w-8 text-blue-600" />
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-600">Total Policies</p>
                <p className="text-2xl font-bold text-gray-900">{policies.length}</p>
              </div>
            </div>
          </CardContent>
        </Card>
        
        <Card>
          <CardContent className="p-6">
            <div className="flex items-center">
              <CheckCircle className="h-8 w-8 text-green-600" />
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-600">Active</p>
                <p className="text-2xl font-bold text-gray-900">
                  {policies.filter(p => p.status === 'active').length}
                </p>
              </div>
            </div>
          </CardContent>
        </Card>
        
        <Card>
          <CardContent className="p-6">
            <div className="flex items-center">
              <Clock className="h-8 w-8 text-yellow-600" />
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-600">Draft</p>
                <p className="text-2xl font-bold text-gray-900">
                  {policies.filter(p => p.status === 'draft').length}
                </p>
              </div>
            </div>
          </CardContent>
        </Card>
        
        <Card>
          <CardContent className="p-6">
            <div className="flex items-center">
              <AlertTriangle className="h-8 w-8 text-red-600" />
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-600">Critical</p>
                <p className="text-2xl font-bold text-gray-900">
                  {policies.filter(p => p.priority === 'critical').length}
                </p>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Filters and Bulk Actions */}
      <Card>
        <CardContent className="p-6">
          <div className="flex flex-col lg:flex-row gap-4">
            <div className="flex-1">
              <div className="relative">
                <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 w-4 h-4" />
                <Input
                  placeholder="Search policies..."
                  value={searchTerm}
                  onChange={(e) => setSearchTerm(e.target.value)}
                  className="pl-10"
                />
              </div>
            </div>
            
            <div className="flex gap-4">
              <select
                value={statusFilter}
                onChange={(e) => setStatusFilter(e.target.value)}
                className="px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option value="all">All Status</option>
                <option value="active">Active</option>
                <option value="inactive">Inactive</option>
                <option value="draft">Draft</option>
              </select>
              
              <select
                value={typeFilter}
                onChange={(e) => setTypeFilter(e.target.value)}
                className="px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option value="all">All Types</option>
                <option value="url-filtering">URL Filtering</option>
                <option value="content-security">Content Security</option>
                <option value="traffic-control">Traffic Control</option>
                <option value="https-inspection">HTTPS Inspection</option>
              </select>
              
              <select
                value={priorityFilter}
                onChange={(e) => setPriorityFilter(e.target.value)}
                className="px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option value="all">All Priorities</option>
                <option value="critical">Critical</option>
                <option value="high">High</option>
                <option value="medium">Medium</option>
                <option value="low">Low</option>
              </select>
            </div>
          </div>

          {/* Bulk Actions */}
          {selectedPolicies.size > 0 && (
            <div className="mt-4 p-4 bg-blue-50 rounded-lg border border-blue-200">
              <div className="flex items-center justify-between">
                <span className="text-sm font-medium text-blue-800">
                  {selectedPolicies.size} policy(ies) selected
                </span>
                <div className="flex gap-2">
                  <Button
                    size="sm"
                    variant="outline"
                    onClick={() => handleBulkAction('activate')}
                    disabled={actionLoading === 'bulk'}
                  >
                    <Play className="w-4 h-4 mr-1" />
                    Activate
                  </Button>
                  <Button
                    size="sm"
                    variant="outline"
                    onClick={() => handleBulkAction('deactivate')}
                    disabled={actionLoading === 'bulk'}
                  >
                    <Pause className="w-4 h-4 mr-1" />
                    Deactivate
                  </Button>
                  <Button
                    size="sm"
                    variant="destructive"
                    onClick={() => handleBulkAction('delete')}
                    disabled={actionLoading === 'bulk'}
                  >
                    <Trash2 className="w-4 h-4 mr-1" />
                    Delete
                  </Button>
                  <Button
                    size="sm"
                    variant="ghost"
                    onClick={() => setSelectedPolicies(new Set())}
                  >
                    <X className="w-4 h-4" />
                  </Button>
                </div>
              </div>
            </div>
          )}
        </CardContent>
      </Card>

      {/* Policies Table */}
      <LoadingOverlay isLoading={loading} message="Loading policies...">
        <Card>
          <CardHeader>
            <CardTitle>Policy Management</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="overflow-x-auto">
              <table className="w-full">
                <thead>
                  <tr className="border-b border-gray-200">
                    <th className="text-left py-3 px-4 font-medium text-gray-600">
                      <input
                        type="checkbox"
                        checked={selectedPolicies.size === paginatedPolicies.length && paginatedPolicies.length > 0}
                        onChange={handleSelectAll}
                        className="rounded border-gray-300"
                      />
                    </th>
                    <th className="text-left py-3 px-4 font-medium text-gray-600">Policy</th>
                    <th className="text-left py-3 px-4 font-medium text-gray-600">Type</th>
                    <th className="text-left py-3 px-4 font-medium text-gray-600">Status</th>
                    <th className="text-left py-3 px-4 font-medium text-gray-600">Priority</th>
                    <th className="text-left py-3 px-4 font-medium text-gray-600">Last Modified</th>
                    <th className="text-left py-3 px-4 font-medium text-gray-600">Actions</th>
                  </tr>
                </thead>
                <tbody>
                  {paginatedPolicies.map((policy) => {
                    const StatusIcon = statusConfig[policy.status].icon;
                    const TypeIcon = typeConfig[policy.type].icon;
                    
                    return (
                      <tr key={policy.id} className="border-b border-gray-100 hover:bg-gray-50">
                        <td className="py-4 px-4">
                          <input
                            type="checkbox"
                            checked={selectedPolicies.has(policy.id)}
                            onChange={() => handleSelectPolicy(policy.id)}
                            className="rounded border-gray-300"
                          />
                        </td>
                        <td className="py-4 px-4">
                          <div>
                            <div className="font-medium text-gray-900">{policy.name}</div>
                            <div className="text-sm text-gray-500">{policy.description}</div>
                          </div>
                        </td>
                        <td className="py-4 px-4">
                          <div className="flex items-center gap-2">
                            <TypeIcon className={typeConfig[policy.type].color} />
                            <span className="text-sm text-gray-900 capitalize">
                              {policy.type.replace('-', ' ')}
                            </span>
                          </div>
                        </td>
                        <td className="py-4 px-4">
                          <div className="flex items-center gap-2">
                            <StatusIcon className={statusConfig[policy.status].color} />
                            <Badge className={`${statusConfig[policy.status].bg} ${statusConfig[policy.status].color}`}>
                              {policy.status}
                            </Badge>
                          </div>
                        </td>
                        <td className="py-4 px-4">
                          <Badge className={`${priorityConfig[policy.priority].bg} ${priorityConfig[policy.priority].color}`}>
                            {policy.priority}
                          </Badge>
                        </td>
                        <td className="py-4 px-4 text-sm text-gray-600">
                          {new Date(policy.lastModified).toLocaleDateString()}
                        </td>
                        <td className="py-4 px-4">
                          <div className="flex items-center gap-1">
                            <LoadingButton
                              size="sm"
                              variant="ghost"
                              loading={actionLoading === policy.id}
                              onClick={() => handleStatusToggle(policy.id)}
                              className={policy.status === 'active' ? 'text-green-600' : 'text-gray-400'}
                            >
                              {policy.status === 'active' ? <Pause className="w-4 h-4" /> : <Play className="w-4 h-4" />}
                            </LoadingButton>
                            <Button
                              variant="ghost"
                              size="sm"
                              onClick={() => handleViewPolicy(policy)}
                            >
                              <Eye className="w-4 h-4" />
                            </Button>
                            <Button
                              variant="ghost"
                              size="sm"
                              onClick={() => handleEditPolicy(policy)}
                            >
                              <Edit className="w-4 h-4" />
                            </Button>
                            <Button
                              variant="ghost"
                              size="sm"
                              onClick={() => handleCopyPolicy(policy)}
                            >
                              <Copy className="w-4 h-4" />
                            </Button>
                            <Button
                              variant="ghost"
                              size="sm"
                              onClick={() => handleDeletePolicy(policy)}
                              className="text-red-600 hover:text-red-800"
                            >
                              <Trash2 className="w-4 h-4" />
                            </Button>
                          </div>
                        </td>
                      </tr>
                    );
                  })}
                </tbody>
              </table>
            </div>
            
            {filteredPolicies.length === 0 && (
              <div className="text-center py-8 text-gray-500">
                {loading ? 'Loading policies...' : 'No policies found matching your criteria.'}
              </div>
            )}

            {/* Pagination */}
            {totalPages > 1 && (
              <div className="flex items-center justify-between mt-4">
                <div className="text-sm text-gray-700">
                  Showing {((currentPage - 1) * itemsPerPage) + 1} to {Math.min(currentPage * itemsPerPage, filteredPolicies.length)} of {filteredPolicies.length} policies
                </div>
                <div className="flex gap-2">
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => setCurrentPage(prev => Math.max(prev - 1, 1))}
                    disabled={currentPage === 1}
                  >
                    Previous
                  </Button>
                  <span className="px-3 py-1 text-sm">
                    Page {currentPage} of {totalPages}
                  </span>
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => setCurrentPage(prev => Math.min(prev + 1, totalPages))}
                    disabled={currentPage === totalPages}
                  >
                    Next
                  </Button>
                </div>
              </div>
            )}
          </CardContent>
        </Card>
      </LoadingOverlay>

      {/* Modals */}
      <Dialog open={showCreateModal} onOpenChange={setShowCreateModal}>
        <DialogContent>
          <PolicyEditor
            onSave={handleSavePolicy}
            onCancel={closeModals}
            mode="create"
          />
        </DialogContent>
      </Dialog>

      <Dialog open={showEditModal} onOpenChange={setShowEditModal}>
        <DialogContent>
          <PolicyEditor
            policy={selectedPolicy}
            onSave={handleSavePolicy}
            onCancel={closeModals}
            mode="edit"
          />
        </DialogContent>
      </Dialog>

      <Dialog open={showViewModal} onOpenChange={setShowViewModal}>
        <DialogContent>
          <PolicyEditor
            policy={selectedPolicy}
            onSave={handleSavePolicy}
            onCancel={closeModals}
            mode="view"
          />
        </DialogContent>
      </Dialog>

      {/* Delete Confirmation Dialog */}
      <Dialog open={showDeleteDialog} onOpenChange={setShowDeleteDialog}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Delete Policy</DialogTitle>
            <DialogDescription>
              Are you sure you want to delete "{policyToDelete?.name}"? This action cannot be undone.
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button variant="outline" onClick={() => setShowDeleteDialog(false)}>
              Cancel
            </Button>
            <LoadingButton
              variant="destructive"
              loading={actionLoading === policyToDelete?.id}
              loadingText="Deleting..."
              onClick={confirmDeletePolicy}
            >
              Delete Policy
            </LoadingButton>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
