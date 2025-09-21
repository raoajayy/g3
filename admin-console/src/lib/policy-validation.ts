export interface ValidationError {
  field: string;
  message: string;
  severity: 'error' | 'warning' | 'info';
}

export interface ValidationResult {
  isValid: boolean;
  errors: ValidationError[];
  warnings: ValidationError[];
  info: ValidationError[];
}

export interface PolicyData {
  name: string;
  description: string;
  priority: string;
  enabled: boolean;
  targets: {
    userGroups: string[];
    users: string[];
    sourceNetworks: string[];
  };
  urlFiltering?: {
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
  contentSecurity?: {
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
  trafficControl?: {
    bandwidthLimits: {
      perUser: string;
      total: string;
    };
    quotas: {
      dailyDataPerUser: string;
      monthlyDataPerUser: string;
    };
  };
  httpsInspection?: {
    enabled: boolean;
    mode: string;
    certificateGeneration: string;
    bypassDomains: string[];
    inspectDomains: string[];
  };
}

export class PolicyValidator {
  static validate(policy: PolicyData): ValidationResult {
    const errors: ValidationError[] = [];
    const warnings: ValidationError[] = [];
    const info: ValidationError[] = [];

    // Basic validation
    this.validateBasic(policy, errors, warnings, info);
    
    // Target validation
    this.validateTargets(policy, errors, warnings, info);
    
    // URL filtering validation
    if (policy.urlFiltering) {
      this.validateUrlFiltering(policy.urlFiltering, errors, warnings, info);
    }
    
    // Content security validation
    if (policy.contentSecurity) {
      this.validateContentSecurity(policy.contentSecurity, errors, warnings, info);
    }
    
    // Traffic control validation
    if (policy.trafficControl) {
      this.validateTrafficControl(policy.trafficControl, errors, warnings, info);
    }
    
    // HTTPS inspection validation
    if (policy.httpsInspection) {
      this.validateHttpsInspection(policy.httpsInspection, errors, warnings, info);
    }

    return {
      isValid: errors.length === 0,
      errors,
      warnings,
      info
    };
  }

  private static validateBasic(policy: PolicyData, errors: ValidationError[], warnings: ValidationError[], info: ValidationError[]) {
    if (!policy.name || policy.name.trim().length === 0) {
      errors.push({
        field: 'name',
        message: 'Policy name is required',
        severity: 'error'
      });
    } else if (policy.name.length < 3) {
      warnings.push({
        field: 'name',
        message: 'Policy name should be at least 3 characters long',
        severity: 'warning'
      });
    }

    if (!policy.description || policy.description.trim().length === 0) {
      warnings.push({
        field: 'description',
        message: 'Policy description is recommended',
        severity: 'warning'
      });
    }

    if (!policy.priority) {
      errors.push({
        field: 'priority',
        message: 'Policy priority is required',
        severity: 'error'
      });
    }
  }

  private static validateTargets(policy: PolicyData, errors: ValidationError[], warnings: ValidationError[], info: ValidationError[]) {
    const hasUserGroups = policy.targets.userGroups && policy.targets.userGroups.length > 0;
    const hasUsers = policy.targets.users && policy.targets.users.length > 0;
    const hasNetworks = policy.targets.sourceNetworks && policy.targets.sourceNetworks.length > 0;

    if (!hasUserGroups && !hasUsers && !hasNetworks) {
      errors.push({
        field: 'targets',
        message: 'At least one target (user group, user, or network) must be specified',
        severity: 'error'
      });
    }

    // Validate network formats
    if (hasNetworks) {
      policy.targets.sourceNetworks.forEach((network, index) => {
        if (!this.isValidNetworkFormat(network)) {
          errors.push({
            field: `targets.sourceNetworks[${index}]`,
            message: `Invalid network format: ${network}. Expected format: 192.168.1.0/24 or 10.0.0.0/8`,
            severity: 'error'
          });
        }
      });
    }
  }

  private static validateUrlFiltering(urlFiltering: any, errors: ValidationError[], warnings: ValidationError[], info: ValidationError[]) {
    // Validate custom rules
    if (urlFiltering.customRules) {
      urlFiltering.customRules.forEach((rule: any, index: number) => {
        if (!rule.name || rule.name.trim().length === 0) {
          errors.push({
            field: `urlFiltering.customRules[${index}].name`,
            message: 'Rule name is required',
            severity: 'error'
          });
        }

        if (!rule.pattern || rule.pattern.trim().length === 0) {
          errors.push({
            field: `urlFiltering.customRules[${index}].pattern`,
            message: 'Rule pattern is required',
            severity: 'error'
          });
        } else if (!this.isValidPattern(rule.pattern, rule.ruleType)) {
          errors.push({
            field: `urlFiltering.customRules[${index}].pattern`,
            message: `Invalid pattern format for ${rule.ruleType} rule`,
            severity: 'error'
          });
        }

        if (rule.priority && (rule.priority < 1 || rule.priority > 1000)) {
          warnings.push({
            field: `urlFiltering.customRules[${index}].priority`,
            message: 'Rule priority should be between 1 and 1000',
            severity: 'warning'
          });
        }
      });
    }
  }

  private static validateContentSecurity(contentSecurity: any, errors: ValidationError[], warnings: ValidationError[], info: ValidationError[]) {
    // Validate malware scanning
    if (contentSecurity.malwareScanning?.enabled) {
      if (!contentSecurity.malwareScanning.icapServer) {
        errors.push({
          field: 'contentSecurity.malwareScanning.icapServer',
          message: 'ICAP server URL is required when malware scanning is enabled',
          severity: 'error'
        });
      } else if (!this.isValidUrl(contentSecurity.malwareScanning.icapServer)) {
        errors.push({
          field: 'contentSecurity.malwareScanning.icapServer',
          message: 'Invalid ICAP server URL format',
          severity: 'error'
        });
      }
    }

    // Validate DLP patterns
    if (contentSecurity.dataLossPrevention?.enabled && contentSecurity.dataLossPrevention.sensitiveDataPatterns) {
      contentSecurity.dataLossPrevention.sensitiveDataPatterns.forEach((pattern: any, index: number) => {
        if (!pattern.name || pattern.name.trim().length === 0) {
          errors.push({
            field: `contentSecurity.dataLossPrevention.sensitiveDataPatterns[${index}].name`,
            message: 'Pattern name is required',
            severity: 'error'
          });
        }

        if (!pattern.pattern || pattern.pattern.trim().length === 0) {
          errors.push({
            field: `contentSecurity.dataLossPrevention.sensitiveDataPatterns[${index}].pattern`,
            message: 'Pattern is required',
            severity: 'error'
          });
        } else if (!this.isValidRegex(pattern.pattern)) {
          errors.push({
            field: `contentSecurity.dataLossPrevention.sensitiveDataPatterns[${index}].pattern`,
            message: 'Invalid regex pattern',
            severity: 'error'
          });
        }
      });
    }
  }

  private static validateTrafficControl(trafficControl: any, errors: ValidationError[], warnings: ValidationError[], info: ValidationError[]) {
    // Validate bandwidth limits
    if (trafficControl.bandwidthLimits) {
      if (trafficControl.bandwidthLimits.perUser && !this.isValidBandwidth(trafficControl.bandwidthLimits.perUser)) {
        errors.push({
          field: 'trafficControl.bandwidthLimits.perUser',
          message: 'Invalid bandwidth format. Expected format: 10Mbps, 1Gbps, etc.',
          severity: 'error'
        });
      }

      if (trafficControl.bandwidthLimits.total && !this.isValidBandwidth(trafficControl.bandwidthLimits.total)) {
        errors.push({
          field: 'trafficControl.bandwidthLimits.total',
          message: 'Invalid bandwidth format. Expected format: 10Mbps, 1Gbps, etc.',
          severity: 'error'
        });
      }
    }

    // Validate quotas
    if (trafficControl.quotas) {
      if (trafficControl.quotas.dailyDataPerUser && !this.isValidDataSize(trafficControl.quotas.dailyDataPerUser)) {
        errors.push({
          field: 'trafficControl.quotas.dailyDataPerUser',
          message: 'Invalid data size format. Expected format: 1GB, 500MB, etc.',
          severity: 'error'
        });
      }

      if (trafficControl.quotas.monthlyDataPerUser && !this.isValidDataSize(trafficControl.quotas.monthlyDataPerUser)) {
        errors.push({
          field: 'trafficControl.quotas.monthlyDataPerUser',
          message: 'Invalid data size format. Expected format: 1GB, 500MB, etc.',
          severity: 'error'
        });
      }
    }
  }

  private static validateHttpsInspection(httpsInspection: any, errors: ValidationError[], warnings: ValidationError[], info: ValidationError[]) {
    if (httpsInspection.enabled) {
      if (httpsInspection.bypassDomains && httpsInspection.bypassDomains.length > 0) {
        httpsInspection.bypassDomains.forEach((domain: string, index: number) => {
          if (!this.isValidDomain(domain)) {
            errors.push({
              field: `httpsInspection.bypassDomains[${index}]`,
              message: `Invalid domain format: ${domain}`,
              severity: 'error'
            });
          }
        });
      }

      if (httpsInspection.inspectDomains && httpsInspection.inspectDomains.length > 0) {
        httpsInspection.inspectDomains.forEach((domain: string, index: number) => {
          if (!this.isValidDomain(domain)) {
            errors.push({
              field: `httpsInspection.inspectDomains[${index}]`,
              message: `Invalid domain format: ${domain}`,
              severity: 'error'
            });
          }
        });
      }
    }
  }

  // Helper methods
  private static isValidNetworkFormat(network: string): boolean {
    const networkRegex = /^(\d{1,3}\.){3}\d{1,3}\/\d{1,2}$/;
    return networkRegex.test(network);
  }

  private static isValidPattern(pattern: string, ruleType: string): boolean {
    switch (ruleType) {
      case 'regex':
        return this.isValidRegex(pattern);
      case 'wildcard':
        return pattern.includes('*') || pattern.includes('?');
      case 'exact':
        return pattern.length > 0;
      case 'domain':
        return this.isValidDomain(pattern);
      case 'suffix':
        return pattern.startsWith('.') || pattern.includes('.');
      default:
        return true;
    }
  }

  private static isValidRegex(pattern: string): boolean {
    try {
      new RegExp(pattern);
      return true;
    } catch {
      return false;
    }
  }

  private static isValidUrl(url: string): boolean {
    try {
      new URL(url);
      return true;
    } catch {
      return false;
    }
  }

  private static isValidBandwidth(bandwidth: string): boolean {
    const bandwidthRegex = /^\d+(\.\d+)?(K|M|G|T)?bps$/i;
    return bandwidthRegex.test(bandwidth);
  }

  private static isValidDataSize(size: string): boolean {
    const sizeRegex = /^\d+(\.\d+)?(B|KB|MB|GB|TB)$/i;
    return sizeRegex.test(size);
  }

  private static isValidDomain(domain: string): boolean {
    const domainRegex = /^(\*\.)?[a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(\.[a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$/;
    return domainRegex.test(domain);
  }
}
