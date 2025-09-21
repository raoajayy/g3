'use client';

import React from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { PolicyValidator, ValidationResult } from '@/lib/policy-validation';
import { 
  CheckCircle, 
  AlertTriangle, 
  Info, 
  XCircle,
  Shield,
  Globe,
  Clock,
  Settings,
  Users,
  Network
} from 'lucide-react';

interface PolicyPreviewProps {
  policy: any;
  onClose: () => void;
  onSave?: () => void;
  showActions?: boolean;
}

export function PolicyPreview({ policy, onClose, onSave, showActions = true }: PolicyPreviewProps) {
  const validationResult: ValidationResult = PolicyValidator.validate(policy);

  const getSeverityIcon = (severity: string) => {
    switch (severity) {
      case 'error':
        return <XCircle className="w-4 h-4 text-red-500" />;
      case 'warning':
        return <AlertTriangle className="w-4 h-4 text-yellow-500" />;
      case 'info':
        return <Info className="w-4 h-4 text-blue-500" />;
      default:
        return <CheckCircle className="w-4 h-4 text-green-500" />;
    }
  };

  const getSeverityColor = (severity: string) => {
    switch (severity) {
      case 'error':
        return 'bg-red-50 border-red-200 text-red-800';
      case 'warning':
        return 'bg-yellow-50 border-yellow-200 text-yellow-800';
      case 'info':
        return 'bg-blue-50 border-blue-200 text-blue-800';
      default:
        return 'bg-green-50 border-green-200 text-green-800';
    }
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold text-gray-900">Policy Preview</h2>
          <p className="text-gray-600">Review policy configuration before saving</p>
        </div>
        {showActions && (
          <div className="flex gap-2">
            <Button variant="outline" onClick={onClose}>
              Cancel
            </Button>
            {onSave && (
              <Button 
                onClick={onSave} 
                disabled={!validationResult.isValid}
                className="bg-blue-600 text-white hover:bg-blue-700"
              >
                Save Policy
              </Button>
            )}
          </div>
        )}
      </div>

      {/* Validation Results */}
      {(validationResult.errors.length > 0 || validationResult.warnings.length > 0 || validationResult.info.length > 0) && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Shield className="w-5 h-5" />
              Validation Results
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            {/* Errors */}
            {validationResult.errors.length > 0 && (
              <div className="space-y-2">
                <h4 className="font-medium text-red-800 flex items-center gap-2">
                  <XCircle className="w-4 h-4" />
                  Errors ({validationResult.errors.length})
                </h4>
                {validationResult.errors.map((error, index) => (
                  <div key={index} className={`p-3 rounded-md border ${getSeverityColor(error.severity)}`}>
                    <div className="flex items-start gap-2">
                      {getSeverityIcon(error.severity)}
                      <div>
                        <p className="font-medium">{error.field}</p>
                        <p className="text-sm">{error.message}</p>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            )}

            {/* Warnings */}
            {validationResult.warnings.length > 0 && (
              <div className="space-y-2">
                <h4 className="font-medium text-yellow-800 flex items-center gap-2">
                  <AlertTriangle className="w-4 h-4" />
                  Warnings ({validationResult.warnings.length})
                </h4>
                {validationResult.warnings.map((warning, index) => (
                  <div key={index} className={`p-3 rounded-md border ${getSeverityColor(warning.severity)}`}>
                    <div className="flex items-start gap-2">
                      {getSeverityIcon(warning.severity)}
                      <div>
                        <p className="font-medium">{warning.field}</p>
                        <p className="text-sm">{warning.message}</p>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            )}

            {/* Info */}
            {validationResult.info.length > 0 && (
              <div className="space-y-2">
                <h4 className="font-medium text-blue-800 flex items-center gap-2">
                  <Info className="w-4 h-4" />
                  Information ({validationResult.info.length})
                </h4>
                {validationResult.info.map((info, index) => (
                  <div key={index} className={`p-3 rounded-md border ${getSeverityColor(info.severity)}`}>
                    <div className="flex items-start gap-2">
                      {getSeverityIcon(info.severity)}
                      <div>
                        <p className="font-medium">{info.field}</p>
                        <p className="text-sm">{info.message}</p>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </CardContent>
        </Card>
      )}

      {/* Policy Summary */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* Basic Information */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Shield className="w-5 h-5" />
              Basic Information
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-3">
            <div>
              <label className="text-sm font-medium text-gray-600">Name</label>
              <p className="text-gray-900">{policy.name || 'Not specified'}</p>
            </div>
            <div>
              <label className="text-sm font-medium text-gray-600">Description</label>
              <p className="text-gray-900">{policy.description || 'No description'}</p>
            </div>
            <div>
              <label className="text-sm font-medium text-gray-600">Priority</label>
              <Badge className={`${
                policy.priority === 'critical' ? 'bg-red-100 text-red-800' :
                policy.priority === 'high' ? 'bg-orange-100 text-orange-800' :
                policy.priority === 'medium' ? 'bg-blue-100 text-blue-800' :
                'bg-gray-100 text-gray-800'
              }`}>
                {policy.priority || 'Not specified'}
              </Badge>
            </div>
            <div>
              <label className="text-sm font-medium text-gray-600">Status</label>
              <Badge className={policy.enabled ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-800'}>
                {policy.enabled ? 'Enabled' : 'Disabled'}
              </Badge>
            </div>
          </CardContent>
        </Card>

        {/* Targets */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Users className="w-5 h-5" />
              Targets
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-3">
            <div>
              <label className="text-sm font-medium text-gray-600">User Groups</label>
              <div className="flex flex-wrap gap-1 mt-1">
                {policy.targets?.userGroups?.length > 0 ? (
                  policy.targets.userGroups.map((group: string, index: number) => (
                    <Badge key={index} variant="secondary">{group}</Badge>
                  ))
                ) : (
                  <span className="text-gray-500 text-sm">None specified</span>
                )}
              </div>
            </div>
            <div>
              <label className="text-sm font-medium text-gray-600">Users</label>
              <div className="flex flex-wrap gap-1 mt-1">
                {policy.targets?.users?.length > 0 ? (
                  policy.targets.users.map((user: string, index: number) => (
                    <Badge key={index} variant="secondary">{user}</Badge>
                  ))
                ) : (
                  <span className="text-gray-500 text-sm">None specified</span>
                )}
              </div>
            </div>
            <div>
              <label className="text-sm font-medium text-gray-600">Source Networks</label>
              <div className="flex flex-wrap gap-1 mt-1">
                {policy.targets?.sourceNetworks?.length > 0 ? (
                  policy.targets.sourceNetworks.map((network: string, index: number) => (
                    <Badge key={index} variant="outline">{network}</Badge>
                  ))
                ) : (
                  <span className="text-gray-500 text-sm">None specified</span>
                )}
              </div>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* URL Filtering */}
      {policy.urlFiltering && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Globe className="w-5 h-5" />
              URL Filtering
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div>
                <label className="text-sm font-medium text-gray-600">Block Categories</label>
                <div className="flex flex-wrap gap-1 mt-1">
                  {policy.urlFiltering.categories?.block?.length > 0 ? (
                    policy.urlFiltering.categories.block.map((category: string, index: number) => (
                      <Badge key={index} variant="destructive">{category}</Badge>
                    ))
                  ) : (
                    <span className="text-gray-500 text-sm">None</span>
                  )}
                </div>
              </div>
              <div>
                <label className="text-sm font-medium text-gray-600">Warn Categories</label>
                <div className="flex flex-wrap gap-1 mt-1">
                  {policy.urlFiltering.categories?.warn?.length > 0 ? (
                    policy.urlFiltering.categories.warn.map((category: string, index: number) => (
                      <Badge key={index} className="bg-yellow-100 text-yellow-800">{category}</Badge>
                    ))
                  ) : (
                    <span className="text-gray-500 text-sm">None</span>
                  )}
                </div>
              </div>
              <div>
                <label className="text-sm font-medium text-gray-600">Allow Categories</label>
                <div className="flex flex-wrap gap-1 mt-1">
                  {policy.urlFiltering.categories?.allow?.length > 0 ? (
                    policy.urlFiltering.categories.allow.map((category: string, index: number) => (
                      <Badge key={index} className="bg-green-100 text-green-800">{category}</Badge>
                    ))
                  ) : (
                    <span className="text-gray-500 text-sm">None</span>
                  )}
                </div>
              </div>
            </div>
            
            {policy.urlFiltering.customRules?.length > 0 && (
              <div>
                <label className="text-sm font-medium text-gray-600">Custom Rules</label>
                <div className="space-y-2 mt-1">
                  {policy.urlFiltering.customRules.map((rule: any, index: number) => (
                    <div key={index} className="p-3 bg-gray-50 rounded-md">
                      <div className="flex items-center justify-between">
                        <span className="font-medium">{rule.name}</span>
                        <Badge variant="outline">{rule.action}</Badge>
                      </div>
                      <p className="text-sm text-gray-600 mt-1">{rule.pattern}</p>
                    </div>
                  ))}
                </div>
              </div>
            )}
          </CardContent>
        </Card>
      )}

      {/* Traffic Control */}
      {policy.trafficControl && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Clock className="w-5 h-5" />
              Traffic Control
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label className="text-sm font-medium text-gray-600">Bandwidth Limits</label>
                <div className="mt-1 space-y-1">
                  <p className="text-sm">Per User: {policy.trafficControl.bandwidthLimits?.perUser || 'Not set'}</p>
                  <p className="text-sm">Total: {policy.trafficControl.bandwidthLimits?.total || 'Not set'}</p>
                </div>
              </div>
              <div>
                <label className="text-sm font-medium text-gray-600">Data Quotas</label>
                <div className="mt-1 space-y-1">
                  <p className="text-sm">Daily: {policy.trafficControl.quotas?.dailyDataPerUser || 'Not set'}</p>
                  <p className="text-sm">Monthly: {policy.trafficControl.quotas?.monthlyDataPerUser || 'Not set'}</p>
                </div>
              </div>
            </div>
          </CardContent>
        </Card>
      )}

      {/* HTTPS Inspection */}
      {policy.httpsInspection && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Settings className="w-5 h-5" />
              HTTPS Inspection
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="flex items-center gap-2">
              <Badge className={policy.httpsInspection.enabled ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-800'}>
                {policy.httpsInspection.enabled ? 'Enabled' : 'Disabled'}
              </Badge>
              <span className="text-sm text-gray-600">Mode: {policy.httpsInspection.mode || 'Not specified'}</span>
            </div>
            
            {policy.httpsInspection.bypassDomains?.length > 0 && (
              <div>
                <label className="text-sm font-medium text-gray-600">Bypass Domains</label>
                <div className="flex flex-wrap gap-1 mt-1">
                  {policy.httpsInspection.bypassDomains.map((domain: string, index: number) => (
                    <Badge key={index} variant="outline">{domain}</Badge>
                  ))}
                </div>
              </div>
            )}
            
            {policy.httpsInspection.inspectDomains?.length > 0 && (
              <div>
                <label className="text-sm font-medium text-gray-600">Inspect Domains</label>
                <div className="flex flex-wrap gap-1 mt-1">
                  {policy.httpsInspection.inspectDomains.map((domain: string, index: number) => (
                    <Badge key={index} className="bg-blue-100 text-blue-800">{domain}</Badge>
                  ))}
                </div>
              </div>
            )}
          </CardContent>
        </Card>
      )}
    </div>
  );
}
