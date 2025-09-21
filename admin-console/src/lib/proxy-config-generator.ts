/**
 * Proxy Configuration Generator
 * Converts policy configurations to G3Proxy YAML configurations
 */

export interface ProxyConfig {
  runtime: {
    thread_number: number;
  };
  log: string;
  auditor?: Array<{
    name: string;
    protocol_inspection: Record<string, any>;
    tls_cert_generator: Record<string, any>;
    tls_ticketer: Record<string, any>;
    tls_stream_dump: Record<string, any>;
    task_audit_ratio?: number;
  }>;
  server: Array<{
    name: string;
    escaper: string;
    auditor?: string;
    type: 'http_proxy' | 'socks_proxy';
    listen: {
      address: string;
    };
    tls_client?: Record<string, any>;
  }>;
  resolver: Array<{
    name: string;
    type: string;
  }>;
  escaper: Array<{
    name: string;
    type: string;
    resolver?: string;
    exact_match?: Array<{
      next: string;
      hosts: string[];
    }>;
    default_next?: string;
    url_filtering?: {
      block_categories: string[];
      warn_categories: string[];
      custom_rules: Array<{
        name: string;
        pattern: string;
        action: 'block' | 'warn' | 'allow' | 'inspect';
        rule_type: 'wildcard' | 'regex' | 'exact' | 'domain' | 'suffix';
      }>;
    };
  }>;
}

export interface PolicyFormData {
  name: string;
  description: string;
  status: 'active' | 'inactive';
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
      pattern: string;
      action: 'block' | 'warn' | 'allow' | 'inspect';
      ruleType: 'wildcard' | 'regex' | 'exact' | 'domain' | 'suffix';
      message?: string;
    }>;
  };
  contentSecurity: {
    malwareScanning: {
      enabled: boolean;
      action: 'block' | 'warn' | 'allow';
    };
    dataLossPrevention: {
      enabled: boolean;
      scanUploads: boolean;
      scanDownloads: boolean;
    };
  };
  trafficControl: {
    bandwidthLimit: number;
    connectionLimit: number;
    rateLimit: number;
  };
  httpsInspection: {
    enabled: boolean;
    action: 'block' | 'warn' | 'allow';
  };
}

export class ProxyConfigGenerator {
  private baseConfig: Partial<ProxyConfig>;

  constructor() {
    this.baseConfig = {
      runtime: { thread_number: 4 },
      log: 'stdout',
      auditor: [{
        name: 'default',
        protocol_inspection: {},
        tls_cert_generator: {},
        tls_ticketer: {},
        tls_stream_dump: {},
        task_audit_ratio: 1.0
      }],
      server: [
        {
          name: 'http',
          escaper: 'policy_escaper',
          auditor: 'default',
          type: 'http_proxy',
          listen: { address: '0.0.0.0:3128' },
          tls_client: {}
        },
        {
          name: 'socks',
          escaper: 'policy_escaper',
          auditor: 'default',
          type: 'socks_proxy',
          listen: { address: '0.0.0.0:1080' }
        }
      ],
      resolver: [{
        name: 'default',
        type: 'c-ares'
      }]
    };
  }

  /**
   * Convert API policy format to PolicyFormData format
   */
  private convertApiPolicyToFormData(apiPolicy: any): PolicyFormData {
    return {
      name: apiPolicy.name || 'Unknown Policy',
      description: apiPolicy.description || '',
      status: apiPolicy.status === 'active' ? 'active' : 'inactive',
      targets: {
        userGroups: apiPolicy.targets?.userGroups || ['all'],
        users: apiPolicy.targets?.users || [],
        sourceNetworks: apiPolicy.targets?.sourceNetworks || []
      },
      urlFiltering: {
        categories: {
          block: apiPolicy.urlFiltering?.categories?.block || [],
          warn: apiPolicy.urlFiltering?.categories?.warn || [],
          allow: apiPolicy.urlFiltering?.categories?.allow || []
        },
        customRules: apiPolicy.urlFiltering?.customRules || []
      },
      contentSecurity: {
        malwareScanning: {
          enabled: apiPolicy.contentSecurity?.malwareScanning?.enabled || false,
          action: apiPolicy.contentSecurity?.malwareScanning?.action || 'allow'
        },
        dataLossPrevention: {
          enabled: apiPolicy.contentSecurity?.dataLossPrevention?.enabled || false,
          scanUploads: apiPolicy.contentSecurity?.dataLossPrevention?.scanUploads || false,
          scanDownloads: apiPolicy.contentSecurity?.dataLossPrevention?.scanDownloads || false
        }
      },
      trafficControl: {
        bandwidthLimit: apiPolicy.trafficControl?.bandwidthLimit || 0,
        connectionLimit: apiPolicy.trafficControl?.connectionLimit || 0,
        rateLimit: apiPolicy.trafficControl?.rateLimit || 0
      },
      httpsInspection: {
        enabled: apiPolicy.httpsInspection?.enabled || false,
        action: apiPolicy.httpsInspection?.action || 'allow'
      }
    };
  }

  /**
   * Generate proxy configuration from policies
   */
  generateConfig(policies: any[]): ProxyConfig {
    // Convert API policies to PolicyFormData format if needed
    const convertedPolicies = policies.map(policy => {
      // Check if it's already in PolicyFormData format
      if (policy.urlFiltering && policy.targets && policy.contentSecurity) {
        return policy as PolicyFormData;
      }
      // Convert from API format
      return this.convertApiPolicyToFormData(policy);
    });
    const activePolicies = convertedPolicies.filter(p => p.status === 'active');
    
    if (activePolicies.length === 0) {
      return this.generateDefaultConfig();
    }

    // Group policies by target scope
    const globalPolicies = activePolicies.filter(p => 
      p.targets.userGroups.length === 0 && 
      p.targets.users.length === 0 && 
      p.targets.sourceNetworks.length === 0
    );

    const targetedPolicies = activePolicies.filter(p => 
      p.targets.userGroups.length > 0 || 
      p.targets.users.length > 0 || 
      p.targets.sourceNetworks.length > 0
    );

    // Generate URL filtering rules
    const urlFilteringRules = this.generateUrlFilteringRules(activePolicies);
    
    // Generate escapers based on policies
    const escapers = this.generateEscapers(globalPolicies, targetedPolicies, urlFilteringRules);

    return {
      ...this.baseConfig,
      escaper: escapers
    } as ProxyConfig;
  }

  /**
   * Generate default configuration when no policies are active
   */
  private generateDefaultConfig(): ProxyConfig {
    return {
      ...this.baseConfig,
      escaper: [{
        name: 'policy_escaper',
        type: 'direct_fixed',
        resolver: 'default'
      }]
    } as ProxyConfig;
  }

  /**
   * Generate URL filtering rules from policies
   */
  private generateUrlFilteringRules(policies: PolicyFormData[]) {
    const blockCategories = new Set<string>();
    const warnCategories = new Set<string>();
    const allowCategories = new Set<string>();
    const customRules: Array<{
      name: string;
      pattern: string;
      action: 'block' | 'warn' | 'allow' | 'inspect';
      rule_type: 'wildcard' | 'regex' | 'exact' | 'domain' | 'suffix';
    }> = [];

    policies.forEach(policy => {
      // Collect categories
      policy.urlFiltering.categories.block.forEach(cat => blockCategories.add(cat));
      policy.urlFiltering.categories.warn.forEach(cat => warnCategories.add(cat));
      policy.urlFiltering.categories.allow.forEach(cat => allowCategories.add(cat));

      // Collect custom rules
      policy.urlFiltering.customRules.forEach(rule => {
        customRules.push({
          name: `${policy.name}_${rule.name}`,
          pattern: rule.pattern,
          action: rule.action,
          rule_type: rule.ruleType
        });
      });
    });

    return {
      block_categories: Array.from(blockCategories),
      warn_categories: Array.from(warnCategories),
      allow_categories: Array.from(allowCategories),
      custom_rules: customRules
    };
  }

  /**
   * Generate escapers based on policies
   */
  private generateEscapers(
    globalPolicies: PolicyFormData[], 
    targetedPolicies: PolicyFormData[],
    urlFilteringRules: any
  ) {
    const escapers: any[] = [];

    // Main policy escaper with URL filtering
    escapers.push({
      name: 'policy_escaper',
      type: 'route_upstream',
      url_filtering: urlFilteringRules,
      default_next: 'direct'
    });

    // Direct escaper for allowed traffic
    escapers.push({
      name: 'direct',
      type: 'direct_fixed',
      resolver: 'default'
    });

    // Blocked escaper for denied traffic
    escapers.push({
      name: 'blocked',
      type: 'blocked',
      resolver: 'default'
    });

    // Generate targeted escapers for specific user groups/networks
    targetedPolicies.forEach(policy => {
      const escaperName = `policy_${policy.name.toLowerCase().replace(/\s+/g, '_')}`;
      
      escapers.push({
        name: escaperName,
        type: 'route_upstream',
        url_filtering: this.generateUrlFilteringRules([policy]),
        default_next: 'direct'
      });
    });

    return escapers;
  }

  /**
   * Generate configuration for specific policy
   */
  generatePolicyConfig(policy: PolicyFormData): Partial<ProxyConfig> {
    const urlFilteringRules = this.generateUrlFilteringRules([policy]);
    
    return {
      escaper: [{
        name: `policy_${policy.name.toLowerCase().replace(/\s+/g, '_')}`,
        type: 'route_upstream',
        url_filtering: urlFilteringRules,
        default_next: 'direct'
      }]
    };
  }

  /**
   * Validate proxy configuration
   */
  validateConfig(config: ProxyConfig): { isValid: boolean; errors: string[] } {
    const errors: string[] = [];

    // Validate runtime configuration
    if (!config.runtime || config.runtime.thread_number < 1) {
      errors.push('Invalid runtime configuration: thread_number must be >= 1');
    }

    // Validate server configuration
    if (!config.server || config.server.length === 0) {
      errors.push('At least one server must be configured');
    }

    // Validate resolver configuration
    if (!config.resolver || config.resolver.length === 0) {
      errors.push('At least one resolver must be configured');
    }

    // Validate escaper configuration
    if (!config.escaper || config.escaper.length === 0) {
      errors.push('At least one escaper must be configured');
    }

    // Check for circular references in escapers
    const escaperNames = new Set(config.escaper.map(e => e.name));
    config.escaper.forEach(escaper => {
      if (escaper.default_next && !escaperNames.has(escaper.default_next)) {
        errors.push(`Escaper '${escaper.name}' references unknown escaper '${escaper.default_next}'`);
      }
    });

    return {
      isValid: errors.length === 0,
      errors
    };
  }

  /**
   * Convert configuration to YAML string
   */
  toYaml(config: ProxyConfig): string {
    // This would use a YAML library in a real implementation
    // For now, return a JSON representation
    return JSON.stringify(config, null, 2);
  }

  /**
   * Generate test configuration for policy testing
   */
  generateTestConfig(policy: PolicyFormData, testUrl: string): Partial<ProxyConfig> {
    return {
      escaper: [{
        name: 'test_escaper',
        type: 'route_upstream',
        url_filtering: {
          block_categories: policy.urlFiltering?.categories?.block || [],
          warn_categories: policy.urlFiltering?.categories?.warn || [],
          allow_categories: policy.urlFiltering?.categories?.allow || [],
          custom_rules: (policy.urlFiltering?.customRules || []).map(rule => ({
            name: rule.name,
            pattern: rule.pattern,
            action: rule.action,
            rule_type: rule.ruleType
          }))
        },
        default_next: 'direct'
      }]
    };
  }
}
