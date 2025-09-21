'use client';

import React, { useState, useEffect } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Badge } from '@/components/ui/badge';
import { PolicyEditor } from '@/components/policy-editor';
import { PolicyPreview } from '@/components/policy-preview';
import { PolicyTester } from '@/components/policy-tester';
import { policyTemplates, getTemplateById } from '@/lib/policy-templates';
import { PolicyValidator } from '@/lib/policy-validation';
import { PolicyImporterExporter } from '@/lib/policy-import-export';
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
  TestTube,
  Layout,
  EyeIcon
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

export function PoliciesPage() {
  const [policies, setPolicies] = useState<Policy[]>(mockPolicies);
  const [searchTerm, setSearchTerm] = useState('');
  const [statusFilter, setStatusFilter] = useState<string>('all');
  const [typeFilter, setTypeFilter] = useState<string>('all');
  const [priorityFilter, setPriorityFilter] = useState<string>('all');
  const [selectedPolicy, setSelectedPolicy] = useState<Policy | null>(null);
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [showEditModal, setShowEditModal] = useState(false);
  const [showViewModal, setShowViewModal] = useState(false);
  const [showPreviewModal, setShowPreviewModal] = useState(false);
  const [showTesterModal, setShowTesterModal] = useState(false);
  const [showTemplatesModal, setShowTemplatesModal] = useState(false);
  const [showImportModal, setShowImportModal] = useState(false);
  const [loading, setLoading] = useState(false);
  const [importResult, setImportResult] = useState<any>(null);

  // Load policies from API
  useEffect(() => {
    loadPolicies();
  }, []);

  const loadPolicies = async () => {
    setLoading(true);
    try {
      const response = await fetch('/api/policies');
      if (response.ok) {
        const data = await response.json();
        setPolicies(data.policies || []);
      }
    } catch (error) {
      console.error('Failed to load policies:', error);
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

  const handleStatusToggle = async (policyId: string) => {
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
      }
    } catch (error) {
      console.error('Failed to update policy status:', error);
    }
  };

  const handleDeletePolicy = async (policyId: string) => {
    if (!confirm('Are you sure you want to delete this policy?')) return;

    try {
      const response = await fetch(`/api/policies/${policyId}`, {
        method: 'DELETE'
      });

      if (response.ok) {
        setPolicies(prev => prev.filter(p => p.id !== policyId));
      }
    } catch (error) {
      console.error('Failed to delete policy:', error);
    }
  };

  const handleSavePolicy = async (policyData: any) => {
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
      }
    } catch (error) {
      console.error('Failed to save policy:', error);
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

  const closeModals = () => {
    setShowCreateModal(false);
    setShowEditModal(false);
    setShowViewModal(false);
    setShowPreviewModal(false);
    setShowTesterModal(false);
    setShowTemplatesModal(false);
    setShowImportModal(false);
    setSelectedPolicy(null);
    setImportResult(null);
  };

  // Policy duplication
  const handleDuplicatePolicy = (policy: Policy) => {
    const duplicatedPolicy = {
      ...policy,
      id: Date.now().toString(),
      name: `${policy.name} (Copy)`,
      status: 'draft' as const,
      lastModified: new Date().toISOString(),
      createdBy: 'admin@company.com'
    };
    setPolicies(prev => [duplicatedPolicy, ...prev]);
  };

  // Policy testing
  const handleTestPolicy = (policy: Policy) => {
    setSelectedPolicy(policy);
    setShowTesterModal(true);
  };

  // Policy preview
  const handlePreviewPolicy = (policy: Policy) => {
    setSelectedPolicy(policy);
    setShowPreviewModal(true);
  };

  // Template selection
  const handleSelectTemplate = (templateId: string) => {
    const template = getTemplateById(templateId);
    if (template) {
      const newPolicy = {
        ...template.policy,
        id: Date.now().toString(),
        status: 'draft' as const,
        lastModified: new Date().toISOString(),
        createdBy: 'admin@company.com'
      };
      setSelectedPolicy(newPolicy);
      setShowTemplatesModal(false);
      setShowCreateModal(true);
    }
  };

  // Import/Export functions
  const handleExportPolicies = () => {
    const exportData = PolicyImporterExporter.exportToJSON(policies);
    PolicyImporterExporter.downloadFile(exportData, 'policies-export.json', 'application/json');
  };

  const handleExportCSV = () => {
    const csvData = PolicyImporterExporter.exportToCSV(policies);
    PolicyImporterExporter.downloadFile(csvData, 'policies-export.csv', 'text/csv');
  };

  const handleImportFile = async (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;

    const text = await file.text();
    let result;

    if (file.name.endsWith('.json')) {
      result = await PolicyImporterExporter.importFromJSON(text);
    } else if (file.name.endsWith('.csv')) {
      result = await PolicyImporterExporter.importFromCSV(text);
    } else {
      setImportResult({
        success: false,
        imported: 0,
        failed: 0,
        errors: ['Unsupported file format. Please use JSON or CSV.'],
        warnings: []
      });
      return;
    }

    setImportResult(result);
    if (result.success && result.imported > 0) {
      // In a real app, you would save the imported policies to the backend
      console.log('Imported policies:', result.imported);
    }
  };

  // If we're in create or edit mode, show the policy editor as full page
  if (showCreateModal || showEditModal) {
    return (
      <div className="space-y-6">
        {/* Header with back button */}
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-4">
            <Button 
              variant="outline" 
              onClick={closeModals}
              className="flex items-center gap-2"
            >
              ← Back to Policies
            </Button>
            <div>
              <h1 className="text-3xl font-bold text-gray-900">
                {showCreateModal ? 'Create Policy' : 'Edit Policy'}
              </h1>
              <p className="text-gray-600">
                {showCreateModal ? 'Create a new security policy' : 'Modify the security policy'}
              </p>
            </div>
          </div>
        </div>

        {/* Policy Editor */}
        <PolicyEditor
          policy={selectedPolicy}
          onSave={handleSavePolicy}
          onCancel={closeModals}
          mode={showCreateModal ? 'create' : 'edit'}
        />
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">Security Policies</h1>
          <p className="text-gray-600">Manage web security policies and rules</p>
        </div>
        <div className="flex items-center gap-2">
          <Button 
            variant="outline" 
            onClick={() => setShowTemplatesModal(true)}
            className="flex items-center gap-2"
          >
            <Layout className="w-4 h-4" />
            Templates
          </Button>
          <Button 
            variant="outline" 
            onClick={() => setShowImportModal(true)}
            className="flex items-center gap-2"
          >
            <Upload className="w-4 h-4" />
            Import
          </Button>
          <Button 
            variant="outline" 
            onClick={handleExportPolicies}
            className="flex items-center gap-2"
          >
            <Download className="w-4 h-4" />
            Export
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

      {/* Filters */}
      <Card>
        <CardContent className="p-6">
          <div className="flex flex-col md:flex-row gap-4">
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
        </CardContent>
      </Card>

      {/* Policies Table */}
      <Card>
        <CardHeader>
          <CardTitle>Policy Management</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="border-b border-gray-200">
                  <th className="text-left py-3 px-4 font-medium text-gray-600">Policy</th>
                  <th className="text-left py-3 px-4 font-medium text-gray-600">Type</th>
                  <th className="text-left py-3 px-4 font-medium text-gray-600">Status</th>
                  <th className="text-left py-3 px-4 font-medium text-gray-600">Priority</th>
                  <th className="text-left py-3 px-4 font-medium text-gray-600">Last Modified</th>
                  <th className="text-left py-3 px-4 font-medium text-gray-600">Actions</th>
                </tr>
              </thead>
              <tbody>
                {filteredPolicies.map((policy) => {
                  const StatusIcon = statusConfig[policy.status].icon;
                  const TypeIcon = typeConfig[policy.type].icon;
                  
                  return (
                    <tr key={policy.id} className="border-b border-gray-100 hover:bg-gray-50">
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
                        <div className="flex items-center gap-2">
                          <Button
                            variant="ghost"
                            size="sm"
                            onClick={() => handleStatusToggle(policy.id)}
                            className={policy.status === 'active' ? 'text-green-600' : 'text-gray-400'}
                          >
                            {policy.status === 'active' ? <Pause className="w-4 h-4" /> : <Play className="w-4 h-4" />}
                          </Button>
                          <Button
                            variant="ghost"
                            size="sm"
                            onClick={() => handleViewPolicy(policy)}
                            title="View Policy"
                          >
                            <Eye className="w-4 h-4" />
                          </Button>
                          <Button
                            variant="ghost"
                            size="sm"
                            onClick={() => handlePreviewPolicy(policy)}
                            title="Preview Policy"
                          >
                            <EyeIcon className="w-4 h-4" />
                          </Button>
                          <Button
                            variant="ghost"
                            size="sm"
                            onClick={() => handleTestPolicy(policy)}
                            title="Test Policy"
                          >
                            <TestTube className="w-4 h-4" />
                          </Button>
                          <Button
                            variant="ghost"
                            size="sm"
                            onClick={() => handleEditPolicy(policy)}
                            title="Edit Policy"
                          >
                            <Edit className="w-4 h-4" />
                          </Button>
                          <Button
                            variant="ghost"
                            size="sm"
                            onClick={() => handleDuplicatePolicy(policy)}
                            title="Duplicate Policy"
                          >
                            <Copy className="w-4 h-4" />
                          </Button>
                          <Button
                            variant="ghost"
                            size="sm"
                            onClick={() => handleDeletePolicy(policy.id)}
                            className="text-red-600 hover:text-red-800"
                            title="Delete Policy"
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
        </CardContent>
      </Card>

      {/* View Modal - Keep only view as modal */}
      {showViewModal && selectedPolicy && (
        <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center p-4">
          <div className="bg-white rounded-lg max-w-6xl w-full max-h-[90vh] overflow-y-auto">
            <PolicyEditor
              policy={selectedPolicy}
              onSave={handleSavePolicy}
              onCancel={closeModals}
              mode="view"
            />
          </div>
        </div>
      )}

      {/* Policy Preview Modal */}
      {showPreviewModal && selectedPolicy && (
        <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center p-4">
          <div className="bg-white rounded-lg max-w-6xl w-full max-h-[90vh] overflow-y-auto">
            <PolicyPreview
              policy={selectedPolicy}
              onClose={closeModals}
            />
          </div>
        </div>
      )}

      {/* Policy Tester Modal */}
      {showTesterModal && selectedPolicy && (
        <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center p-4">
          <div className="bg-white rounded-lg max-w-6xl w-full max-h-[90vh] overflow-y-auto">
            <PolicyTester
              policy={selectedPolicy}
              onClose={closeModals}
            />
          </div>
        </div>
      )}

      {/* Templates Modal */}
      {showTemplatesModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center p-4">
          <div className="bg-white rounded-lg max-w-4xl w-full max-h-[90vh] overflow-y-auto">
            <div className="p-6">
              <div className="flex items-center justify-between mb-6">
                <h2 className="text-2xl font-bold text-gray-900">Policy Templates</h2>
                <Button variant="outline" onClick={closeModals}>
                  Close
                </Button>
              </div>
              
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {policyTemplates.map((template) => (
                  <Card key={template.id} className="cursor-pointer hover:shadow-md transition-shadow">
                    <CardHeader>
                      <CardTitle className="flex items-center gap-2">
                        <span className="text-2xl">{template.icon}</span>
                        {template.name}
                      </CardTitle>
                    </CardHeader>
                    <CardContent>
                      <p className="text-gray-600 mb-4">{template.description}</p>
                      <Badge variant="outline">{template.category}</Badge>
                      <div className="mt-4">
                        <Button 
                          onClick={() => handleSelectTemplate(template.id)}
                          className="w-full"
                        >
                          Use Template
                        </Button>
                      </div>
                    </CardContent>
                  </Card>
                ))}
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Import Modal */}
      {showImportModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center p-4">
          <div className="bg-white rounded-lg max-w-2xl w-full">
            <div className="p-6">
              <div className="flex items-center justify-between mb-6">
                <h2 className="text-2xl font-bold text-gray-900">Import Policies</h2>
                <Button variant="outline" onClick={closeModals}>
                  Close
                </Button>
              </div>
              
              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Select File (JSON or CSV)
                  </label>
                  <input
                    type="file"
                    accept=".json,.csv"
                    onChange={handleImportFile}
                    className="block w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-full file:border-0 file:text-sm file:font-semibold file:bg-blue-50 file:text-blue-900 hover:file:bg-blue-100"
                  />
                </div>
                
                {importResult && (
                  <div className={`p-4 rounded-md ${
                    importResult.success ? 'bg-green-50 border border-green-200' : 'bg-red-50 border border-red-200'
                  }`}>
                    <h3 className={`font-medium ${
                      importResult.success ? 'text-green-800' : 'text-red-800'
                    }`}>
                      Import {importResult.success ? 'Successful' : 'Failed'}
                    </h3>
                    <p className="text-sm mt-1">
                      Imported: {importResult.imported} | Failed: {importResult.failed}
                    </p>
                    {importResult.errors.length > 0 && (
                      <div className="mt-2">
                        <p className="text-sm font-medium text-red-800">Errors:</p>
                        <ul className="text-sm text-red-700 list-disc list-inside">
                          {importResult.errors.map((error: string, index: number) => (
                            <li key={index}>{error}</li>
                          ))}
                        </ul>
                      </div>
                    )}
                    {importResult.warnings.length > 0 && (
                      <div className="mt-2">
                        <p className="text-sm font-medium text-yellow-800">Warnings:</p>
                        <ul className="text-sm text-yellow-700 list-disc list-inside">
                          {importResult.warnings.map((warning: string, index: number) => (
                            <li key={index}>{warning}</li>
                          ))}
                        </ul>
                      </div>
                    )}
                  </div>
                )}
                
                <div className="flex gap-2">
                  <Button 
                    variant="outline" 
                    onClick={handleExportPolicies}
                    className="flex items-center gap-2"
                  >
                    <Download className="w-4 h-4" />
                    Export JSON
                  </Button>
                  <Button 
                    variant="outline" 
                    onClick={handleExportCSV}
                    className="flex items-center gap-2"
                  >
                    <Download className="w-4 h-4" />
                    Export CSV
                  </Button>
                </div>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}