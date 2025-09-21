'use client';

import React, { useState, useEffect } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Badge } from '@/components/ui/badge';
import { 
  Save, 
  Play, 
  AlertTriangle, 
  CheckCircle, 
  Code, 
  Eye,
  Settings,
  Shield,
  Users,
  Globe,
  Clock,
  FileText
} from 'lucide-react';

interface PolicyEditorProps {
  policy?: any;
  onSave: (policy: any) => void;
  onCancel: () => void;
  mode: 'create' | 'edit' | 'view';
}

interface PolicyFormData {
  name: string;
  description: string;
  priority: string;
  enabled: boolean;
  targets: {
    userGroups: string[];
    users: string[];
    sourceNetworks: string[];
  };
  urlFiltering: {
    categories: {
      block: string[];
      warn: string[];
      allow: string[];
    };
    customRules: Array<{
      name: string;
      action: string;
      pattern: string;
      ruleType: string;
      message: string;
      priority: number;
    }>;
  };
  contentSecurity: {
    malwareScanning: {
      enabled: boolean;
      icapServer: string;
      action: string;
      timeout: string;
    };
    dataLossPrevention: {
      enabled: boolean;
      scanUploads: boolean;
      scanDownloads: boolean;
      sensitiveDataPatterns: Array<{
        name: string;
        pattern: string;
        action: string;
      }>;
    };
  };
  trafficControl: {
    bandwidthLimits: {
      perUser: string;
      total: string;
    };
    quotas: {
      dailyDataPerUser: string;
      monthlyDataPerUser: string;
    };
  };
  httpsInspection: {
    enabled: boolean;
    mode: string;
    certificateGeneration: string;
    bypassDomains: string[];
    inspectDomains: string[];
  };
}

const defaultPolicy: PolicyFormData = {
  name: '',
  description: '',
  priority: 'medium',
  enabled: true,
  targets: {
    userGroups: [],
    users: [],
    sourceNetworks: [],
  },
  urlFiltering: {
    categories: {
      block: [],
      warn: [],
      allow: [],
    },
    customRules: [],
  },
  contentSecurity: {
    malwareScanning: {
      enabled: false,
      icapServer: '',
      action: 'block',
      timeout: '30s',
    },
    dataLossPrevention: {
      enabled: false,
      scanUploads: false,
      scanDownloads: false,
      sensitiveDataPatterns: [],
    },
  },
  trafficControl: {
    bandwidthLimits: {
      perUser: '',
      total: '',
    },
    quotas: {
      dailyDataPerUser: '',
      monthlyDataPerUser: '',
    },
  },
  httpsInspection: {
    enabled: false,
    mode: 'selective',
    certificateGeneration: 'automatic',
    bypassDomains: [],
    inspectDomains: [],
  },
};

const categoryOptions = [
  'gambling', 'adult-content', 'malware', 'phishing', 'peer-to-peer',
  'social-media', 'streaming', 'gaming', 'business-tools', 'productivity',
  'news', 'education'
];

const priorityOptions = [
  { value: 'critical', label: 'Critical', color: 'bg-red-100 text-red-800' },
  { value: 'high', label: 'High', color: 'bg-orange-100 text-orange-800' },
  { value: 'medium', label: 'Medium', color: 'bg-blue-100 text-blue-800' },
  { value: 'low', label: 'Low', color: 'bg-gray-100 text-gray-800' },
];

export function PolicyEditor({ policy, onSave, onCancel, mode }: PolicyEditorProps) {
  const [formData, setFormData] = useState<PolicyFormData>(defaultPolicy);
  const [activeTab, setActiveTab] = useState('basic');
  const [validationErrors, setValidationErrors] = useState<string[]>([]);
  const [isValidating, setIsValidating] = useState(false);

  useEffect(() => {
    if (policy) {
      // Transform policy data to match form structure
      const transformedPolicy = {
        name: policy.name || '',
        description: policy.description || '',
        priority: policy.priority || 'medium',
        enabled: policy.enabled !== undefined ? policy.enabled : true,
        targets: {
          userGroups: policy.targets?.userGroups || [],
          users: policy.targets?.users || [],
          sourceNetworks: policy.targets?.sourceNetworks || [],
        },
        urlFiltering: {
          categories: {
            block: policy.urlFiltering?.categories?.block || [],
            warn: policy.urlFiltering?.categories?.warn || [],
            allow: policy.urlFiltering?.categories?.allow || [],
          },
          customRules: policy.urlFiltering?.customRules || [],
        },
        contentSecurity: {
          malwareScanning: {
            enabled: policy.contentSecurity?.malwareScanning?.enabled || false,
            icapServer: policy.contentSecurity?.malwareScanning?.icapServer || '',
            action: policy.contentSecurity?.malwareScanning?.action || 'block',
            timeout: policy.contentSecurity?.malwareScanning?.timeout || '30s',
          },
          dataLossPrevention: {
            enabled: policy.contentSecurity?.dataLossPrevention?.enabled || false,
            scanUploads: policy.contentSecurity?.dataLossPrevention?.scanUploads || false,
            scanDownloads: policy.contentSecurity?.dataLossPrevention?.scanDownloads || false,
            sensitiveDataPatterns: policy.contentSecurity?.dataLossPrevention?.sensitiveDataPatterns || [],
          },
        },
        trafficControl: {
          bandwidthLimits: {
            perUser: policy.trafficControl?.bandwidthLimits?.perUser || '',
            total: policy.trafficControl?.bandwidthLimits?.total || '',
          },
          quotas: {
            dailyDataPerUser: policy.trafficControl?.quotas?.dailyDataPerUser || '',
            monthlyDataPerUser: policy.trafficControl?.quotas?.monthlyDataPerUser || '',
          },
        },
        httpsInspection: {
          enabled: policy.httpsInspection?.enabled || false,
          mode: policy.httpsInspection?.mode || 'selective',
          certificateGeneration: policy.httpsInspection?.certificateGeneration || 'automatic',
          bypassDomains: policy.httpsInspection?.bypassDomains || [],
          inspectDomains: policy.httpsInspection?.inspectDomains || [],
        },
      };
      setFormData(transformedPolicy);
    }
  }, [policy]);

  const validatePolicy = async () => {
    setIsValidating(true);
    const errors: string[] = [];

    if (!formData.name.trim()) {
      errors.push('Policy name is required');
    }

    if ((formData.targets?.userGroups?.length || 0) === 0 && 
        (formData.targets?.users?.length || 0) === 0 && 
        (formData.targets?.sourceNetworks?.length || 0) === 0) {
      errors.push('At least one target (user group, user, or network) is required');
    }

    if (formData.urlFiltering?.customRules?.some(rule => !rule.name || !rule.pattern)) {
      errors.push('All custom rules must have a name and pattern');
    }

    setValidationErrors(errors);
    setIsValidating(false);
    return errors.length === 0;
  };

  const handleSave = async () => {
    const isValid = await validatePolicy();
    if (isValid) {
      onSave(formData);
    }
  };

  const addCustomRule = () => {
    setFormData(prev => ({
      ...prev,
      urlFiltering: {
        ...prev.urlFiltering,
        customRules: [
          ...(prev.urlFiltering?.customRules || []),
          {
            name: '',
            action: 'block',
            pattern: '',
            ruleType: 'wildcard',
            message: '',
            priority: 100,
          }
        ]
      }
    }));
  };

  const removeCustomRule = (index: number) => {
    setFormData(prev => ({
      ...prev,
      urlFiltering: {
        ...prev.urlFiltering,
        customRules: (prev.urlFiltering?.customRules || []).filter((_, i) => i !== index)
      }
    }));
  };

  const updateCustomRule = (index: number, field: string, value: any) => {
    setFormData(prev => ({
      ...prev,
      urlFiltering: {
        ...prev.urlFiltering,
        customRules: (prev.urlFiltering?.customRules || []).map((rule, i) => 
          i === index ? { ...rule, [field]: value } : rule
        )
      }
    }));
  };

  const addCategory = (type: 'block' | 'warn' | 'allow', category: string) => {
    if (!formData.urlFiltering?.categories?.[type]?.includes(category)) {
      setFormData(prev => ({
        ...prev,
        urlFiltering: {
          ...prev.urlFiltering,
          categories: {
            ...prev.urlFiltering?.categories,
            [type]: [...(prev.urlFiltering?.categories?.[type] || []), category]
          }
        }
      }));
    }
  };

  const removeCategory = (type: 'block' | 'warn' | 'allow', category: string) => {
    setFormData(prev => ({
      ...prev,
      urlFiltering: {
        ...prev.urlFiltering,
        categories: {
          ...prev.urlFiltering?.categories,
          [type]: (prev.urlFiltering?.categories?.[type] || []).filter(c => c !== category)
        }
      }
    }));
  };

  const tabs = [
    { id: 'basic', label: 'Basic Info', icon: FileText },
    { id: 'targets', label: 'Targets', icon: Users },
    { id: 'url-filtering', label: 'URL Filtering', icon: Globe },
    { id: 'content-security', label: 'Content Security', icon: Shield },
    { id: 'traffic-control', label: 'Traffic Control', icon: Clock },
    { id: 'https-inspection', label: 'HTTPS Inspection', icon: Settings },
  ];

  return (
    <div className="space-y-6">
      {/* Action Buttons */}
      <div className="flex justify-end gap-2">
        <Button variant="outline" onClick={onCancel}>
          {mode === 'view' ? 'Close' : 'Cancel'}
        </Button>
        {mode !== 'view' && (
          <Button onClick={handleSave} disabled={isValidating}>
            <Save className="w-4 h-4 mr-2" />
            Save Policy
          </Button>
        )}
      </div>

      {/* Validation Errors */}
      {validationErrors.length > 0 && (
        <Card className="border-red-200 bg-red-50">
          <CardContent className="p-4">
            <div className="flex items-center gap-2 text-red-800">
              <AlertTriangle className="w-5 h-5" />
              <span className="font-medium">Validation Errors:</span>
            </div>
            <ul className="mt-2 list-disc list-inside text-sm text-red-700">
              {validationErrors.map((error, index) => (
                <li key={index}>{error}</li>
              ))}
            </ul>
          </CardContent>
        </Card>
      )}

      {/* Tabs */}
      <div className="border-b border-gray-200 bg-white">
        <nav className="-mb-px flex space-x-8 overflow-x-auto">
          {tabs.map((tab) => {
            const Icon = tab.icon;
            return (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id)}
                className={`flex items-center gap-2 py-3 px-1 border-b-2 font-medium text-sm whitespace-nowrap transition-colors ${
                  activeTab === tab.id
                    ? 'border-blue-500 text-blue-600 bg-blue-50'
                    : 'border-transparent text-gray-600 hover:text-gray-900 hover:border-gray-300 hover:bg-gray-50'
                }`}
              >
                <Icon className="w-4 h-4 flex-shrink-0" />
                <span className="text-sm font-medium">{tab.label}</span>
              </button>
            );
          })}
        </nav>
      </div>

      {/* Tab Content */}
      <div className="space-y-6">
        {activeTab === 'basic' && (
          <Card>
            <CardHeader>
              <CardTitle>Basic Information</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Policy Name
                </label>
                <Input
                  value={formData.name}
                  onChange={(e) => setFormData(prev => ({ ...prev, name: e.target.value }))}
                  placeholder="Enter policy name"
                  disabled={mode === 'view'}
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Description
                </label>
                <textarea
                  value={formData.description}
                  onChange={(e) => setFormData(prev => ({ ...prev, description: e.target.value }))}
                  placeholder="Enter policy description"
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  rows={3}
                  disabled={mode === 'view'}
                />
              </div>
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    Priority
                  </label>
                  <select
                    value={formData.priority}
                    onChange={(e) => setFormData(prev => ({ ...prev, priority: e.target.value }))}
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                    disabled={mode === 'view'}
                  >
                    {priorityOptions.map(option => (
                      <option key={option.value} value={option.value}>
                        {option.label}
                      </option>
                    ))}
                  </select>
                </div>
                <div className="flex items-center">
                  <input
                    type="checkbox"
                    id="enabled"
                    checked={formData.enabled}
                    onChange={(e) => setFormData(prev => ({ ...prev, enabled: e.target.checked }))}
                    className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                    disabled={mode === 'view'}
                  />
                  <label htmlFor="enabled" className="ml-2 text-sm font-medium text-gray-700">
                    Policy Enabled
                  </label>
                </div>
              </div>
            </CardContent>
          </Card>
        )}

        {activeTab === 'targets' && (
          <Card>
            <CardHeader>
              <CardTitle>Policy Targets</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  User Groups
                </label>
                <Input
                  placeholder="Enter user groups (comma-separated)"
                  value={formData.targets?.userGroups?.join(', ') || ''}
                  onChange={(e) => setFormData(prev => ({
                    ...prev,
                    targets: {
                      ...prev.targets,
                      userGroups: e.target.value.split(',').map(g => g.trim()).filter(g => g)
                    }
                  }))}
                  disabled={mode === 'view'}
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Users
                </label>
                <Input
                  placeholder="Enter users (comma-separated)"
                  value={formData.targets?.users?.join(', ') || ''}
                  onChange={(e) => setFormData(prev => ({
                    ...prev,
                    targets: {
                      ...prev.targets,
                      users: e.target.value.split(',').map(u => u.trim()).filter(u => u)
                    }
                  }))}
                  disabled={mode === 'view'}
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Source Networks
                </label>
                <Input
                  placeholder="Enter networks (comma-separated, e.g., 10.0.0.0/8)"
                  value={formData.targets?.sourceNetworks?.join(', ') || ''}
                  onChange={(e) => setFormData(prev => ({
                    ...prev,
                    targets: {
                      ...prev.targets,
                      sourceNetworks: e.target.value.split(',').map(n => n.trim()).filter(n => n)
                    }
                  }))}
                  disabled={mode === 'view'}
                />
              </div>
            </CardContent>
          </Card>
        )}

        {activeTab === 'url-filtering' && (
          <Card>
            <CardHeader>
              <CardTitle>URL Filtering</CardTitle>
            </CardHeader>
            <CardContent className="space-y-6">
              {/* Categories */}
              <div>
                <h3 className="text-lg font-medium text-gray-900 mb-3">Categories</h3>
                <div className="grid grid-cols-3 gap-4">
                  {(['block', 'warn', 'allow'] as const).map(type => (
                    <div key={type}>
                      <label className="block text-sm font-medium text-gray-700 mb-2 capitalize">
                        {type} Categories
                      </label>
                      <div className="space-y-2">
                        {formData.urlFiltering?.categories?.[type]?.map(category => (
                          <div key={category} className="flex items-center justify-between bg-gray-50 p-2 rounded">
                            <span className="text-sm">{category}</span>
                            {mode !== 'view' && (
                              <button
                                onClick={() => removeCategory(type, category)}
                                className="text-red-600 hover:text-red-800"
                              >
                                ×
                              </button>
                            )}
                          </div>
                        ))}
                        {mode !== 'view' && (
                          <select
                            onChange={(e) => {
                              if (e.target.value) {
                                addCategory(type, e.target.value);
                                e.target.value = '';
                              }
                            }}
                            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                          >
                            <option value="">Add category...</option>
                            {categoryOptions
                              .filter(cat => !formData.urlFiltering?.categories?.[type]?.includes(cat))
                              .map(category => (
                                <option key={category} value={category}>
                                  {category}
                                </option>
                              ))}
                          </select>
                        )}
                      </div>
                    </div>
                  ))}
                </div>
              </div>

              {/* Custom Rules */}
              <div>
                <div className="flex items-center justify-between mb-3">
                  <h3 className="text-lg font-medium text-gray-900">Custom Rules</h3>
                  {mode !== 'view' && (
                    <Button onClick={addCustomRule} size="sm">
                      Add Rule
                    </Button>
                  )}
                </div>
                <div className="space-y-4">
                  {formData.urlFiltering?.customRules?.map((rule, index) => (
                    <div key={index} className="border border-gray-200 rounded-lg p-4">
                      <div className="grid grid-cols-2 gap-4 mb-4">
                        <div>
                          <label className="block text-sm font-medium text-gray-700 mb-1">
                            Rule Name
                          </label>
                          <Input
                            value={rule.name}
                            onChange={(e) => updateCustomRule(index, 'name', e.target.value)}
                            disabled={mode === 'view'}
                          />
                        </div>
                        <div>
                          <label className="block text-sm font-medium text-gray-700 mb-1">
                            Action
                          </label>
                          <select
                            value={rule.action}
                            onChange={(e) => updateCustomRule(index, 'action', e.target.value)}
                            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                            disabled={mode === 'view'}
                          >
                            <option value="block">Block</option>
                            <option value="allow">Allow</option>
                            <option value="warn">Warn</option>
                            <option value="inspect">Inspect</option>
                          </select>
                        </div>
                      </div>
                      <div className="grid grid-cols-2 gap-4 mb-4">
                        <div>
                          <label className="block text-sm font-medium text-gray-700 mb-1">
                            Pattern
                          </label>
                          <Input
                            value={rule.pattern}
                            onChange={(e) => updateCustomRule(index, 'pattern', e.target.value)}
                            placeholder="*.example.com"
                            disabled={mode === 'view'}
                          />
                        </div>
                        <div>
                          <label className="block text-sm font-medium text-gray-700 mb-1">
                            Rule Type
                          </label>
                          <select
                            value={rule.ruleType}
                            onChange={(e) => updateCustomRule(index, 'ruleType', e.target.value)}
                            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                            disabled={mode === 'view'}
                          >
                            <option value="wildcard">Wildcard</option>
                            <option value="regex">Regex</option>
                            <option value="exact">Exact</option>
                            <option value="domain">Domain</option>
                            <option value="suffix">Suffix</option>
                          </select>
                        </div>
                      </div>
                      <div className="flex items-center justify-between">
                        <div className="flex-1 mr-4">
                          <label className="block text-sm font-medium text-gray-700 mb-1">
                            Message
                          </label>
                          <Input
                            value={rule.message}
                            onChange={(e) => updateCustomRule(index, 'message', e.target.value)}
                            placeholder="Custom message for blocked requests"
                            disabled={mode === 'view'}
                          />
                        </div>
                        {mode !== 'view' && (
                          <Button
                            variant="destructive"
                            size="sm"
                            onClick={() => removeCustomRule(index)}
                          >
                            Remove
                          </Button>
                        )}
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            </CardContent>
          </Card>
        )}

        {/* Additional tabs would be implemented similarly */}
        {activeTab === 'content-security' && (
          <Card>
            <CardHeader>
              <CardTitle>Content Security</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-gray-600">Content security configuration will be implemented here.</p>
            </CardContent>
          </Card>
        )}

        {activeTab === 'traffic-control' && (
          <Card>
            <CardHeader>
              <CardTitle>Traffic Control</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-gray-600">Traffic control configuration will be implemented here.</p>
            </CardContent>
          </Card>
        )}

        {activeTab === 'https-inspection' && (
          <Card>
            <CardHeader>
              <CardTitle>HTTPS Inspection</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-gray-600">HTTPS inspection configuration will be implemented here.</p>
            </CardContent>
          </Card>
        )}
      </div>
    </div>
  );
}
