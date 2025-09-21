export interface PolicyTemplate {
  id: string;
  name: string;
  description: string;
  category: string;
  icon: string;
  policy: any;
}

export const policyTemplates: PolicyTemplate[] = [
  {
    id: 'malware-protection',
    name: 'Malware Protection',
    description: 'Comprehensive protection against malware and phishing sites',
    category: 'Security',
    icon: '🛡️',
    policy: {
      name: 'Malware Protection Policy',
      description: 'Blocks access to known malware and phishing sites with real-time scanning',
      priority: 'critical',
      enabled: true,
      targets: {
        userGroups: ['all-users'],
        users: [],
        sourceNetworks: ['0.0.0.0/0']
      },
      urlFiltering: {
        categories: {
          block: ['malware', 'phishing', 'adult-content'],
          warn: ['social-media', 'streaming'],
          allow: ['business-tools', 'banking', 'education', 'news']
        },
        customRules: [
          {
            name: 'Block Suspicious Domains',
            action: 'block',
            pattern: '*.suspicious-*.com',
            ruleType: 'wildcard',
            message: 'This domain has been flagged as potentially malicious',
            priority: 1
          }
        ]
      },
      contentSecurity: {
        malwareScanning: {
          enabled: true,
          icapServer: 'icap://malware-scanner:1344/scan',
          action: 'block',
          timeout: '30s'
        },
        dataLossPrevention: {
          enabled: true,
          scanUploads: true,
          scanDownloads: true,
          sensitiveDataPatterns: [
            {
              name: 'Credit Card Numbers',
              pattern: '\\b\\d{4}[\\s-]?\\d{4}[\\s-]?\\d{4}[\\s-]?\\d{4}\\b',
              action: 'block'
            }
          ]
        }
      },
      httpsInspection: {
        enabled: true,
        mode: 'selective',
        certificateGeneration: 'automatic',
        bypassDomains: ['*.bank.com', '*.paypal.com'],
        inspectDomains: ['*.suspicious-*.com']
      }
    }
  },
  {
    id: 'social-media-control',
    name: 'Social Media Control',
    description: 'Control access to social media platforms during work hours',
    category: 'Productivity',
    icon: '📱',
    policy: {
      name: 'Social Media Control Policy',
      description: 'Restricts social media access during work hours with warning pages',
      priority: 'medium',
      enabled: true,
      targets: {
        userGroups: ['employees'],
        users: [],
        sourceNetworks: ['10.0.0.0/8']
      },
      urlFiltering: {
        categories: {
          block: [],
          warn: ['social-media', 'streaming'],
          allow: ['business-tools', 'banking', 'education', 'news']
        },
        customRules: [
          {
            name: 'Work Hours Social Media Warning',
            action: 'warn',
            pattern: '*.facebook.com',
            ruleType: 'wildcard',
            message: 'Social media access is restricted during work hours. Continue?',
            priority: 100
          },
          {
            name: 'Work Hours Social Media Warning',
            action: 'warn',
            pattern: '*.twitter.com',
            ruleType: 'wildcard',
            message: 'Social media access is restricted during work hours. Continue?',
            priority: 100
          }
        ]
      }
    }
  },
  {
    id: 'bandwidth-management',
    name: 'Bandwidth Management',
    description: 'Control bandwidth usage and implement data quotas',
    category: 'Network',
    icon: '🌐',
    policy: {
      name: 'Bandwidth Management Policy',
      description: 'Enforces bandwidth limits and data quotas per user and group',
      priority: 'medium',
      enabled: true,
      targets: {
        userGroups: ['all-users'],
        users: [],
        sourceNetworks: ['0.0.0.0/0']
      },
      trafficControl: {
        bandwidthLimits: {
          perUser: '10Mbps',
          total: '100Mbps'
        },
        quotas: {
          dailyDataPerUser: '1GB',
          monthlyDataPerUser: '30GB'
        }
      }
    }
  },
  {
    id: 'data-loss-prevention',
    name: 'Data Loss Prevention',
    description: 'Prevent sensitive data from being uploaded or downloaded',
    category: 'Security',
    icon: '🔒',
    policy: {
      name: 'Data Loss Prevention Policy',
      description: 'Scans and blocks uploads/downloads containing sensitive data',
      priority: 'high',
      enabled: true,
      targets: {
        userGroups: ['all-users'],
        users: [],
        sourceNetworks: ['0.0.0.0/0']
      },
      contentSecurity: {
        dataLossPrevention: {
          enabled: true,
          scanUploads: true,
          scanDownloads: true,
          sensitiveDataPatterns: [
            {
              name: 'Social Security Numbers',
              pattern: '\\b\\d{3}-\\d{2}-\\d{4}\\b',
              action: 'block'
            },
            {
              name: 'Credit Card Numbers',
              pattern: '\\b\\d{4}[\\s-]?\\d{4}[\\s-]?\\d{4}[\\s-]?\\d{4}\\b',
              action: 'block'
            },
            {
              name: 'Email Addresses',
              pattern: '\\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Z|a-z]{2,}\\b',
              action: 'warn'
            }
          ]
        }
      }
    }
  },
  {
    id: 'https-inspection',
    name: 'HTTPS Inspection',
    description: 'Inspect HTTPS traffic for security threats and compliance',
    category: 'Security',
    icon: '🔍',
    policy: {
      name: 'HTTPS Inspection Policy',
      description: 'Inspects HTTPS traffic for malware and policy violations',
      priority: 'high',
      enabled: true,
      targets: {
        userGroups: ['all-users'],
        users: [],
        sourceNetworks: ['0.0.0.0/0']
      },
      httpsInspection: {
        enabled: true,
        mode: 'selective',
        certificateGeneration: 'automatic',
        bypassDomains: ['*.bank.com', '*.paypal.com', '*.apple.com'],
        inspectDomains: ['*.suspicious-*.com', '*.malware-*.com']
      },
      contentSecurity: {
        malwareScanning: {
          enabled: true,
          icapServer: 'icap://malware-scanner:1344/scan',
          action: 'block',
          timeout: '30s'
        }
      }
    }
  },
  {
    id: 'gaming-restriction',
    name: 'Gaming Restriction',
    description: 'Block access to gaming websites and platforms',
    category: 'Productivity',
    icon: '🎮',
    policy: {
      name: 'Gaming Restriction Policy',
      description: 'Blocks access to gaming websites and platforms during work hours',
      priority: 'low',
      enabled: true,
      targets: {
        userGroups: ['employees'],
        users: [],
        sourceNetworks: ['10.0.0.0/8']
      },
      urlFiltering: {
        categories: {
          block: ['gaming'],
          warn: ['streaming'],
          allow: ['business-tools', 'banking', 'education', 'news']
        },
        customRules: [
          {
            name: 'Block Gaming Platforms',
            action: 'block',
            pattern: '*.steam.com',
            ruleType: 'wildcard',
            message: 'Gaming platforms are blocked during work hours',
            priority: 50
          },
          {
            name: 'Block Gaming Platforms',
            action: 'block',
            pattern: '*.epicgames.com',
            ruleType: 'wildcard',
            message: 'Gaming platforms are blocked during work hours',
            priority: 50
          }
        ]
      }
    }
  }
];

export const getTemplateById = (id: string): PolicyTemplate | undefined => {
  return policyTemplates.find(template => template.id === id);
};

export const getTemplatesByCategory = (category: string): PolicyTemplate[] => {
  return policyTemplates.filter(template => template.category === category);
};

export const getTemplateCategories = (): string[] => {
  return [...new Set(policyTemplates.map(template => template.category))];
};
