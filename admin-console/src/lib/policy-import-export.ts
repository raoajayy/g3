export interface PolicyExportData {
  version: string;
  exportDate: string;
  policies: any[];
  metadata: {
    totalPolicies: number;
    categories: string[];
    tags: string[];
  };
}

export interface ImportResult {
  success: boolean;
  imported: number;
  failed: number;
  errors: string[];
  warnings: string[];
}

export class PolicyImporterExporter {
  static exportPolicies(policies: any[]): PolicyExportData {
    const categories = [...new Set(policies.map(p => p.category || 'uncategorized'))];
    const tags = [...new Set(policies.flatMap(p => p.tags || []))];

    return {
      version: '1.0',
      exportDate: new Date().toISOString(),
      policies: policies.map(policy => ({
        ...policy,
        exportedAt: new Date().toISOString()
      })),
      metadata: {
        totalPolicies: policies.length,
        categories,
        tags
      }
    };
  }

  static exportToJSON(policies: any[]): string {
    const exportData = this.exportPolicies(policies);
    return JSON.stringify(exportData, null, 2);
  }

  static exportToCSV(policies: any[]): string {
    const headers = [
      'Name',
      'Description',
      'Priority',
      'Enabled',
      'User Groups',
      'Users',
      'Source Networks',
      'Block Categories',
      'Warn Categories',
      'Allow Categories',
      'Custom Rules Count',
      'Malware Scanning',
      'Data Loss Prevention',
      'Bandwidth Limits',
      'HTTPS Inspection',
      'Created At',
      'Updated At'
    ];

    const rows = policies.map(policy => [
      policy.name || '',
      policy.description || '',
      policy.priority || '',
      policy.enabled ? 'Yes' : 'No',
      (policy.targets?.userGroups || []).join(';'),
      (policy.targets?.users || []).join(';'),
      (policy.targets?.sourceNetworks || []).join(';'),
      (policy.urlFiltering?.categories?.block || []).join(';'),
      (policy.urlFiltering?.categories?.warn || []).join(';'),
      (policy.urlFiltering?.categories?.allow || []).join(';'),
      (policy.urlFiltering?.customRules || []).length.toString(),
      policy.contentSecurity?.malwareScanning?.enabled ? 'Yes' : 'No',
      policy.contentSecurity?.dataLossPrevention?.enabled ? 'Yes' : 'No',
      policy.trafficControl?.bandwidthLimits?.perUser || '',
      policy.httpsInspection?.enabled ? 'Yes' : 'No',
      policy.createdAt || '',
      policy.updatedAt || ''
    ]);

    return [headers, ...rows]
      .map(row => row.map(field => `"${field.replace(/"/g, '""')}"`).join(','))
      .join('\n');
  }

  static async importFromJSON(jsonData: string): Promise<ImportResult> {
    try {
      const data = JSON.parse(jsonData);
      
      if (!data.policies || !Array.isArray(data.policies)) {
        return {
          success: false,
          imported: 0,
          failed: 0,
          errors: ['Invalid JSON format: policies array not found'],
          warnings: []
        };
      }

      const result: ImportResult = {
        success: true,
        imported: 0,
        failed: 0,
        errors: [],
        warnings: []
      };

      for (const policy of data.policies) {
        try {
          // Validate policy structure
          if (!policy.name) {
            result.errors.push(`Policy missing name: ${JSON.stringify(policy)}`);
            result.failed++;
            continue;
          }

          // Clean up policy data
          const cleanedPolicy = this.cleanPolicyData(policy);
          
          // Add import metadata
          cleanedPolicy.importedAt = new Date().toISOString();
          cleanedPolicy.importSource = 'json_import';

          result.imported++;
        } catch (error) {
          result.errors.push(`Error processing policy ${policy.name}: ${error}`);
          result.failed++;
        }
      }

      if (data.metadata) {
        result.warnings.push(`Import metadata: ${data.metadata.totalPolicies} policies from ${data.exportDate}`);
      }

      return result;
    } catch (error) {
      return {
        success: false,
        imported: 0,
        failed: 0,
        errors: [`JSON parsing error: ${error}`],
        warnings: []
      };
    }
  }

  static async importFromCSV(csvData: string): Promise<ImportResult> {
    try {
      const lines = csvData.split('\n').filter(line => line.trim());
      if (lines.length < 2) {
        return {
          success: false,
          imported: 0,
          failed: 0,
          errors: ['CSV file must contain at least a header row and one data row'],
          warnings: []
        };
      }

      const headers = this.parseCSVLine(lines[0]);
      const result: ImportResult = {
        success: true,
        imported: 0,
        failed: 0,
        errors: [],
        warnings: []
      };

      for (let i = 1; i < lines.length; i++) {
        try {
          const values = this.parseCSVLine(lines[i]);
          const policy = this.csvRowToPolicy(headers, values);
          
          if (!policy.name) {
            result.errors.push(`Row ${i + 1}: Policy name is required`);
            result.failed++;
            continue;
          }

          result.imported++;
        } catch (error) {
          result.errors.push(`Row ${i + 1}: ${error}`);
          result.failed++;
        }
      }

      return result;
    } catch (error) {
      return {
        success: false,
        imported: 0,
        failed: 0,
        errors: [`CSV parsing error: ${error}`],
        warnings: []
      };
    }
  }

  private static cleanPolicyData(policy: any): any {
    return {
      name: policy.name?.trim(),
      description: policy.description?.trim() || '',
      priority: policy.priority || 'medium',
      enabled: Boolean(policy.enabled),
      targets: {
        userGroups: Array.isArray(policy.targets?.userGroups) ? policy.targets.userGroups : [],
        users: Array.isArray(policy.targets?.users) ? policy.targets.users : [],
        sourceNetworks: Array.isArray(policy.targets?.sourceNetworks) ? policy.targets.sourceNetworks : []
      },
      urlFiltering: policy.urlFiltering ? {
        categories: {
          block: Array.isArray(policy.urlFiltering.categories?.block) ? policy.urlFiltering.categories.block : [],
          warn: Array.isArray(policy.urlFiltering.categories?.warn) ? policy.urlFiltering.categories.warn : [],
          allow: Array.isArray(policy.urlFiltering.categories?.allow) ? policy.urlFiltering.categories.allow : []
        },
        customRules: Array.isArray(policy.urlFiltering.customRules) ? policy.urlFiltering.customRules : []
      } : undefined,
      contentSecurity: policy.contentSecurity ? {
        malwareScanning: {
          enabled: Boolean(policy.contentSecurity.malwareScanning?.enabled),
          icapServer: policy.contentSecurity.malwareScanning?.icapServer || '',
          action: policy.contentSecurity.malwareScanning?.action || 'block',
          timeout: policy.contentSecurity.malwareScanning?.timeout || '30s'
        },
        dataLossPrevention: {
          enabled: Boolean(policy.contentSecurity.dataLossPrevention?.enabled),
          scanUploads: Boolean(policy.contentSecurity.dataLossPrevention?.scanUploads),
          scanDownloads: Boolean(policy.contentSecurity.dataLossPrevention?.scanDownloads),
          sensitiveDataPatterns: Array.isArray(policy.contentSecurity.dataLossPrevention?.sensitiveDataPatterns) 
            ? policy.contentSecurity.dataLossPrevention.sensitiveDataPatterns 
            : []
        }
      } : undefined,
      trafficControl: policy.trafficControl ? {
        bandwidthLimits: {
          perUser: policy.trafficControl.bandwidthLimits?.perUser || '',
          total: policy.trafficControl.bandwidthLimits?.total || ''
        },
        quotas: {
          dailyDataPerUser: policy.trafficControl.quotas?.dailyDataPerUser || '',
          monthlyDataPerUser: policy.trafficControl.quotas?.monthlyDataPerUser || ''
        }
      } : undefined,
      httpsInspection: policy.httpsInspection ? {
        enabled: Boolean(policy.httpsInspection.enabled),
        mode: policy.httpsInspection.mode || 'selective',
        certificateGeneration: policy.httpsInspection.certificateGeneration || 'automatic',
        bypassDomains: Array.isArray(policy.httpsInspection.bypassDomains) ? policy.httpsInspection.bypassDomains : [],
        inspectDomains: Array.isArray(policy.httpsInspection.inspectDomains) ? policy.httpsInspection.inspectDomains : []
      } : undefined,
      createdAt: policy.createdAt || new Date().toISOString(),
      updatedAt: policy.updatedAt || new Date().toISOString()
    };
  }

  private static parseCSVLine(line: string): string[] {
    const result: string[] = [];
    let current = '';
    let inQuotes = false;
    
    for (let i = 0; i < line.length; i++) {
      const char = line[i];
      
      if (char === '"') {
        if (inQuotes && line[i + 1] === '"') {
          current += '"';
          i++; // Skip next quote
        } else {
          inQuotes = !inQuotes;
        }
      } else if (char === ',' && !inQuotes) {
        result.push(current);
        current = '';
      } else {
        current += char;
      }
    }
    
    result.push(current);
    return result;
  }

  private static csvRowToPolicy(headers: string[], values: string[]): any {
    const policy: any = {};
    
    for (let i = 0; i < headers.length && i < values.length; i++) {
      const header = headers[i].toLowerCase().replace(/\s+/g, '');
      const value = values[i];
      
      switch (header) {
        case 'name':
          policy.name = value;
          break;
        case 'description':
          policy.description = value;
          break;
        case 'priority':
          policy.priority = value;
          break;
        case 'enabled':
          policy.enabled = value.toLowerCase() === 'yes';
          break;
        case 'usergroups':
          policy.targets = policy.targets || {};
          policy.targets.userGroups = value ? value.split(';').map((g: string) => g.trim()) : [];
          break;
        case 'users':
          policy.targets = policy.targets || {};
          policy.targets.users = value ? value.split(';').map((u: string) => u.trim()) : [];
          break;
        case 'sourcenetworks':
          policy.targets = policy.targets || {};
          policy.targets.sourceNetworks = value ? value.split(';').map((n: string) => n.trim()) : [];
          break;
        case 'blockcategories':
          policy.urlFiltering = policy.urlFiltering || { categories: {} };
          policy.urlFiltering.categories.block = value ? value.split(';').map((c: string) => c.trim()) : [];
          break;
        case 'warncategories':
          policy.urlFiltering = policy.urlFiltering || { categories: {} };
          policy.urlFiltering.categories.warn = value ? value.split(';').map((c: string) => c.trim()) : [];
          break;
        case 'allowcategories':
          policy.urlFiltering = policy.urlFiltering || { categories: {} };
          policy.urlFiltering.categories.allow = value ? value.split(';').map((c: string) => c.trim()) : [];
          break;
        case 'malwarescanning':
          policy.contentSecurity = policy.contentSecurity || {};
          policy.contentSecurity.malwareScanning = { enabled: value.toLowerCase() === 'yes' };
          break;
        case 'datalossprevention':
          policy.contentSecurity = policy.contentSecurity || {};
          policy.contentSecurity.dataLossPrevention = { enabled: value.toLowerCase() === 'yes' };
          break;
        case 'bandwidthlimits':
          policy.trafficControl = policy.trafficControl || {};
          policy.trafficControl.bandwidthLimits = { perUser: value };
          break;
        case 'httpsinspection':
          policy.httpsInspection = { enabled: value.toLowerCase() === 'yes' };
          break;
        case 'createdat':
          policy.createdAt = value;
          break;
        case 'updatedat':
          policy.updatedAt = value;
          break;
      }
    }
    
    return policy;
  }

  static downloadFile(content: string, filename: string, mimeType: string) {
    const blob = new Blob([content], { type: mimeType });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = filename;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);
  }
}
