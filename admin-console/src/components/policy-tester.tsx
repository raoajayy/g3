'use client';

import React, { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Badge } from '@/components/ui/badge';
import { 
  Play, 
  CheckCircle, 
  XCircle, 
  AlertTriangle,
  Globe,
  User,
  Network,
  Clock,
  Shield
} from 'lucide-react';

interface PolicyTesterProps {
  policy: any;
  onClose: () => void;
}

interface TestResult {
  url: string;
  user: string;
  userGroup: string;
  sourceNetwork: string;
  action: 'allow' | 'block' | 'warn' | 'inspect';
  reason: string;
  processingTime: number;
  matchedRules: string[];
}

export function PolicyTester({ policy, onClose }: PolicyTesterProps) {
  const [testUrl, setTestUrl] = useState('');
  const [testUser, setTestUser] = useState('');
  const [testUserGroup, setTestUserGroup] = useState('');
  const [testSourceNetwork, setTestSourceNetwork] = useState('');
  const [testResults, setTestResults] = useState<TestResult[]>([]);
  const [isRunning, setIsRunning] = useState(false);

  const runTest = async () => {
    if (!testUrl.trim()) return;

    setIsRunning(true);
    
    // Simulate policy evaluation
    const result = await simulatePolicyEvaluation({
      url: testUrl,
      user: testUser || 'test-user',
      userGroup: testUserGroup || 'test-group',
      sourceNetwork: testSourceNetwork || '192.168.1.100'
    });

    setTestResults(prev => [result, ...prev]);
    setIsRunning(false);
  };

  const simulatePolicyEvaluation = async (testCase: any): Promise<TestResult> => {
    // Simulate processing time
    await new Promise(resolve => setTimeout(resolve, Math.random() * 1000 + 500));

    const matchedRules: string[] = [];
    let action: 'allow' | 'block' | 'warn' | 'inspect' = 'allow';
    let reason = 'No matching rules found';

    // Check if user/target matches policy targets
    const userMatches = !policy.targets?.userGroups?.length || 
                       policy.targets.userGroups.includes(testCase.userGroup) ||
                       policy.targets?.users?.includes(testCase.user);
    
    const networkMatches = !policy.targets?.sourceNetworks?.length ||
                          policy.targets.sourceNetworks.some((net: string) => 
                            isIpInNetwork(testCase.sourceNetwork, net)
                          );

    if (!userMatches && !networkMatches) {
      return {
        ...testCase,
        action: 'allow',
        reason: 'User/network not in policy targets',
        processingTime: Math.random() * 100,
        matchedRules: []
      };
    }

    // Get enhanced URL categorization with external data
    let enhancedResult: any = null;
    try {
      const response = await fetch('/api/categories/enhanced-lookup', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ url: testCase.url, useExternal: true })
      });
      const data = await response.json();
      if (data.success) {
        enhancedResult = data.data;
      }
    } catch (error) {
      console.error('Error fetching enhanced URL categories:', error);
    }

    // Check URL filtering rules with enhanced categorization
    if (policy.urlFiltering) {
      // Check categories using enhanced data
      if (enhancedResult) {
        const consensusCategory = enhancedResult.consensusCategory;
        const consensusConfidence = enhancedResult.consensusConfidence;
        const riskScore = enhancedResult.riskScore;
        
        if (policy.urlFiltering.categories?.block?.includes(consensusCategory)) {
          action = 'block';
          reason = `URL category '${consensusCategory}' is blocked (confidence: ${Math.round(consensusConfidence * 100)}%, risk: ${Math.round(riskScore * 100)}%)`;
          matchedRules.push(`Block category: ${consensusCategory}`);
        } else if (policy.urlFiltering.categories?.warn?.includes(consensusCategory)) {
          action = 'warn';
          reason = `URL category '${consensusCategory}' triggers warning (confidence: ${Math.round(consensusConfidence * 100)}%, risk: ${Math.round(riskScore * 100)}%)`;
          matchedRules.push(`Warn category: ${consensusCategory}`);
        } else if (policy.urlFiltering.categories?.allow?.includes(consensusCategory)) {
          action = 'allow';
          reason = `URL category '${consensusCategory}' is explicitly allowed (confidence: ${Math.round(consensusConfidence * 100)}%, risk: ${Math.round(riskScore * 100)}%)`;
          matchedRules.push(`Allow category: ${consensusCategory}`);
        } else {
          // Use fallback categorization
          const urlCategory = getUrlCategory(testCase.url);
          if (policy.urlFiltering.categories?.block?.includes(urlCategory)) {
            action = 'block';
            reason = `URL category '${urlCategory}' is blocked (fallback)`;
            matchedRules.push(`Block category: ${urlCategory} (fallback)`);
          } else if (policy.urlFiltering.categories?.warn?.includes(urlCategory)) {
            action = 'warn';
            reason = `URL category '${urlCategory}' triggers warning (fallback)`;
            matchedRules.push(`Warn category: ${urlCategory} (fallback)`);
          } else if (policy.urlFiltering.categories?.allow?.includes(urlCategory)) {
            action = 'allow';
            reason = `URL category '${urlCategory}' is explicitly allowed (fallback)`;
            matchedRules.push(`Allow category: ${urlCategory} (fallback)`);
          }
        }
      } else {
        // Fallback to basic categorization
        const urlCategory = getUrlCategory(testCase.url);
        if (policy.urlFiltering.categories?.block?.includes(urlCategory)) {
          action = 'block';
          reason = `URL category '${urlCategory}' is blocked (fallback)`;
          matchedRules.push(`Block category: ${urlCategory} (fallback)`);
        } else if (policy.urlFiltering.categories?.warn?.includes(urlCategory)) {
          action = 'warn';
          reason = `URL category '${urlCategory}' triggers warning (fallback)`;
          matchedRules.push(`Warn category: ${urlCategory} (fallback)`);
        } else if (policy.urlFiltering.categories?.allow?.includes(urlCategory)) {
          action = 'allow';
          reason = `URL category '${urlCategory}' is explicitly allowed (fallback)`;
          matchedRules.push(`Allow category: ${urlCategory} (fallback)`);
        }
      }

      // Check custom rules
      if (policy.urlFiltering.customRules) {
        for (const rule of policy.urlFiltering.customRules) {
          if (matchesPattern(testCase.url, rule.pattern, rule.ruleType)) {
            action = rule.action as any;
            reason = rule.message || `Matched custom rule: ${rule.name}`;
            matchedRules.push(`Custom rule: ${rule.name}`);
            break;
          }
        }
      }
    }

    // Check HTTPS inspection
    if (policy.httpsInspection?.enabled) {
      const domain = extractDomain(testCase.url);
      if (policy.httpsInspection.inspectDomains?.some((d: string) => matchesDomain(domain, d))) {
        action = 'inspect';
        reason = 'HTTPS inspection required for this domain';
        matchedRules.push('HTTPS inspection rule');
      } else if (policy.httpsInspection.bypassDomains?.some((d: string) => matchesDomain(domain, d))) {
        action = 'allow';
        reason = 'Domain is in bypass list';
        matchedRules.push('HTTPS bypass rule');
      }
    }

    return {
      ...testCase,
      action,
      reason,
      processingTime: Math.random() * 100 + 50,
      matchedRules
    };
  };

  const getActionIcon = (action: string) => {
    switch (action) {
      case 'allow':
        return <CheckCircle className="w-4 h-4 text-green-500" />;
      case 'block':
        return <XCircle className="w-4 h-4 text-red-500" />;
      case 'warn':
        return <AlertTriangle className="w-4 h-4 text-yellow-500" />;
      case 'inspect':
        return <Shield className="w-4 h-4 text-blue-500" />;
      default:
        return <CheckCircle className="w-4 h-4 text-gray-500" />;
    }
  };

  const getActionColor = (action: string) => {
    switch (action) {
      case 'allow':
        return 'bg-green-100 text-green-800';
      case 'block':
        return 'bg-red-100 text-red-800';
      case 'warn':
        return 'bg-yellow-100 text-yellow-800';
      case 'inspect':
        return 'bg-blue-100 text-blue-800';
      default:
        return 'bg-gray-100 text-gray-800';
    }
  };

  const clearResults = () => {
    setTestResults([]);
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold text-gray-900">Policy Testing</h2>
          <p className="text-gray-600">Test policy behavior with sample requests</p>
        </div>
        <Button variant="outline" onClick={onClose}>
          Close
        </Button>
      </div>

      {/* Test Configuration */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Play className="w-5 h-5" />
            Test Configuration
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Test URL
              </label>
              <Input
                placeholder="https://example.com"
                value={testUrl}
                onChange={(e) => setTestUrl(e.target.value)}
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                User
              </label>
              <Input
                placeholder="user@company.com"
                value={testUser}
                onChange={(e) => setTestUser(e.target.value)}
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                User Group
              </label>
              <Input
                placeholder="employees"
                value={testUserGroup}
                onChange={(e) => setTestUserGroup(e.target.value)}
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Source Network
              </label>
              <Input
                placeholder="192.168.1.100"
                value={testSourceNetwork}
                onChange={(e) => setTestSourceNetwork(e.target.value)}
              />
            </div>
          </div>
          
          <div className="flex gap-2">
            <Button 
              onClick={runTest} 
              disabled={!testUrl.trim() || isRunning}
              className="bg-blue-600 text-white hover:bg-blue-700"
            >
              {isRunning ? 'Testing...' : 'Run Test'}
            </Button>
            {testResults.length > 0 && (
              <Button variant="outline" onClick={clearResults}>
                Clear Results
              </Button>
            )}
          </div>
        </CardContent>
      </Card>

      {/* Test Results */}
      {testResults.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <CheckCircle className="w-5 h-5" />
              Test Results ({testResults.length})
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              {testResults.map((result, index) => (
                <div key={index} className="border border-gray-200 rounded-lg p-4">
                  <div className="flex items-start justify-between mb-3">
                    <div className="flex items-center gap-3">
                      {getActionIcon(result.action)}
                      <div>
                        <p className="font-medium text-gray-900">{result.url}</p>
                        <p className="text-sm text-gray-600">{result.reason}</p>
                      </div>
                    </div>
                    <Badge className={getActionColor(result.action)}>
                      {result.action.toUpperCase()}
                    </Badge>
                  </div>
                  
                  <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
                    <div className="flex items-center gap-2">
                      <User className="w-4 h-4 text-gray-400" />
                      <span className="text-gray-600">User: {result.user}</span>
                    </div>
                    <div className="flex items-center gap-2">
                      <Network className="w-4 h-4 text-gray-400" />
                      <span className="text-gray-600">Group: {result.userGroup}</span>
                    </div>
                    <div className="flex items-center gap-2">
                      <Clock className="w-4 h-4 text-gray-400" />
                      <span className="text-gray-600">{result.processingTime.toFixed(1)}ms</span>
                    </div>
                  </div>
                  
                  {result.matchedRules.length > 0 && (
                    <div className="mt-3">
                      <p className="text-sm font-medium text-gray-700 mb-1">Matched Rules:</p>
                      <div className="flex flex-wrap gap-1">
                        {result.matchedRules.map((rule, ruleIndex) => (
                          <Badge key={ruleIndex} variant="outline" className="text-xs">
                            {rule}
                          </Badge>
                        ))}
                      </div>
                    </div>
                  )}
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      )}

      {/* Quick Test URLs */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Globe className="w-5 h-5" />
            Quick Test URLs
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
            {[
              'https://facebook.com',
              'https://youtube.com',
              'https://malware-site.com',
              'https://business-tool.com',
              'https://bank.com',
              'https://suspicious-domain.com'
            ].map((url) => (
              <Button
                key={url}
                variant="outline"
                size="sm"
                onClick={() => setTestUrl(url)}
                className="justify-start"
              >
                {url}
              </Button>
            ))}
          </div>
        </CardContent>
      </Card>
    </div>
  );
}

// Helper functions
function getUrlCategory(url: string): string {
  const domain = extractDomain(url);
  if (domain.includes('facebook') || domain.includes('twitter')) return 'social-media';
  if (domain.includes('youtube') || domain.includes('netflix')) return 'streaming';
  if (domain.includes('malware') || domain.includes('virus')) return 'malware';
  if (domain.includes('bank') || domain.includes('paypal')) return 'business-tools';
  if (domain.includes('gaming') || domain.includes('steam')) return 'gaming';
  return 'other';
}

function extractDomain(url: string): string {
  try {
    return new URL(url).hostname;
  } catch {
    return url;
  }
}

function matchesPattern(url: string, pattern: string, ruleType: string): boolean {
  const domain = extractDomain(url);
  
  switch (ruleType) {
    case 'wildcard':
      return matchesWildcard(domain, pattern);
    case 'regex':
      try {
        return new RegExp(pattern).test(url);
      } catch {
        return false;
      }
    case 'exact':
      return url === pattern;
    case 'domain':
      return domain === pattern;
    case 'suffix':
      return domain.endsWith(pattern);
    default:
      return false;
  }
}

function matchesWildcard(text: string, pattern: string): boolean {
  const regex = new RegExp('^' + pattern.replace(/\*/g, '.*') + '$');
  return regex.test(text);
}

function matchesDomain(domain: string, pattern: string): boolean {
  if (pattern.startsWith('*.')) {
    const suffix = pattern.substring(2);
    return domain.endsWith(suffix);
  }
  return domain === pattern;
}

function isIpInNetwork(ip: string, network: string): boolean {
  // Simplified IP network matching
  const [networkIp, cidr] = network.split('/');
  const ipParts = ip.split('.').map(Number);
  const networkParts = networkIp.split('.').map(Number);
  
  if (ipParts.length !== 4 || networkParts.length !== 4) return false;
  
  const mask = parseInt(cidr);
  const maskBytes = Math.floor(mask / 8);
  const remainingBits = mask % 8;
  
  for (let i = 0; i < maskBytes; i++) {
    if (ipParts[i] !== networkParts[i]) return false;
  }
  
  if (remainingBits > 0) {
    const maskByte = 0xFF << (8 - remainingBits);
    return (ipParts[maskBytes] & maskByte) === (networkParts[maskBytes] & maskByte);
  }
  
  return true;
}
