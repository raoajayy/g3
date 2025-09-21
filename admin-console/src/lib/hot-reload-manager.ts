/**
 * Hot Reload Manager
 * Manages real-time policy updates and proxy configuration reloading
 */

import { ProxyConfigGenerator, PolicyFormData } from './proxy-config-generator';

export interface ReloadStatus {
  status: 'idle' | 'reloading' | 'success' | 'error';
  message: string;
  timestamp: Date;
  configVersion: number;
}

export interface PolicyChangeEvent {
  type: 'create' | 'update' | 'delete' | 'activate' | 'deactivate';
  policyId: string;
  policyName: string;
  timestamp: Date;
  changes?: Partial<PolicyFormData>;
}

export interface ProxyStatus {
  isRunning: boolean;
  port: number;
  pid?: number;
  lastReload: Date;
  configVersion: number;
  health: 'healthy' | 'degraded' | 'unhealthy';
}

export class HotReloadManager {
  private configGenerator: ProxyConfigGenerator;
  private currentConfig: any = null;
  private configVersion: number = 1;
  private reloadStatus: ReloadStatus;
  private proxyStatus: ProxyStatus;
  private eventListeners: Map<string, Function[]> = new Map();
  private reloadTimeout: NodeJS.Timeout | null = null;
  private isReloading: boolean = false;

  constructor() {
    this.configGenerator = new ProxyConfigGenerator();
    this.reloadStatus = {
      status: 'idle',
      message: 'Ready',
      timestamp: new Date(),
      configVersion: this.configVersion
    };
    this.proxyStatus = {
      isRunning: false,
      port: 3128,
      lastReload: new Date(),
      configVersion: this.configVersion,
      health: 'unhealthy'
    };
  }

  /**
   * Initialize the hot reload manager
   */
  async initialize(): Promise<void> {
    console.log('Initializing Hot Reload Manager...');
    
    // Check proxy status
    await this.checkProxyStatus();
    
    // Load current configuration
    await this.loadCurrentConfig();
    
    this.reloadStatus = {
      status: 'success',
      message: 'Hot reload manager initialized',
      timestamp: new Date(),
      configVersion: this.configVersion
    };
    
    this.emit('initialized', { status: this.reloadStatus });
  }

  /**
   * Apply policy changes and trigger hot reload
   */
  async applyPolicyChanges(
    policies: PolicyFormData[], 
    changeEvent?: PolicyChangeEvent
  ): Promise<ReloadStatus> {
    if (this.isReloading) {
      return {
        status: 'error',
        message: 'Reload already in progress',
        timestamp: new Date(),
        configVersion: this.configVersion
      };
    }

    this.isReloading = true;
    this.reloadStatus = {
      status: 'reloading',
      message: 'Applying policy changes...',
      timestamp: new Date(),
      configVersion: this.configVersion
    };

    this.emit('reloadStarted', { status: this.reloadStatus, changeEvent });

    try {
      // Generate new configuration
      const newConfig = this.configGenerator.generateConfig(policies);
      
      // Validate configuration
      const validation = this.configGenerator.validateConfig(newConfig);
      if (!validation.isValid) {
        throw new Error(`Configuration validation failed: ${validation.errors.join(', ')}`);
      }

      // Update current configuration
      this.currentConfig = newConfig;
      this.configVersion++;

      // Apply configuration to proxy using the API
      const response = await fetch('/api/proxy/config', {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ policies })
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(`Failed to apply configuration: ${errorData.error || response.statusText}`);
      }

      const result = await response.json();
      
      // Update proxy status
      this.proxyStatus.configVersion = result.version;
      this.proxyStatus.lastReload = new Date();

      this.reloadStatus = {
        status: 'success',
        message: `Configuration reloaded successfully (v${result.version})`,
        timestamp: new Date(),
        configVersion: result.version
      };

      this.emit('reloadCompleted', { 
        status: this.reloadStatus, 
        config: result.config,
        changeEvent 
      });

    } catch (error) {
      console.error('Policy application failed:', error);
      this.reloadStatus = {
        status: 'error',
        message: `Reload failed: ${error instanceof Error ? error.message : 'Unknown error'}`,
        timestamp: new Date(),
        configVersion: this.configVersion
      };

      this.emit('reloadFailed', { 
        status: this.reloadStatus, 
        error: error instanceof Error ? error.message : 'Unknown error',
        changeEvent 
      });
    } finally {
      this.isReloading = false;
    }

    return this.reloadStatus;
  }


  /**
   * Wait for proxy to acknowledge configuration changes
   */
  private async waitForProxyAcknowledgment(timeoutMs: number = 5000): Promise<void> {
    const startTime = Date.now();
    
    while (Date.now() - startTime < timeoutMs) {
      try {
        const response = await fetch('/api/proxy/status');
        if (response.ok) {
          const status = await response.json();
          if (status.configVersion >= this.configVersion) {
            return;
          }
        }
      } catch (error) {
        // Continue waiting
      }
      
      await new Promise(resolve => setTimeout(resolve, 100));
    }
    
    throw new Error('Proxy acknowledgment timeout');
  }

  /**
   * Check proxy status and health
   */
  async checkProxyStatus(): Promise<ProxyStatus> {
    try {
      const response = await fetch('/api/proxy/status');
      if (response.ok) {
        const status = await response.json();
        this.proxyStatus = {
          ...this.proxyStatus,
          ...status,
          health: this.determineHealthStatus(status)
        };
      } else {
        this.proxyStatus.health = 'unhealthy';
      }
    } catch (error) {
      console.error('Failed to check proxy status:', error);
      this.proxyStatus.health = 'unhealthy';
    }

    return this.proxyStatus;
  }

  /**
   * Determine proxy health status
   */
  private determineHealthStatus(status: any): 'healthy' | 'degraded' | 'unhealthy' {
    if (!status.isRunning) {
      return 'unhealthy';
    }

    const timeSinceLastReload = Date.now() - new Date(status.lastReload).getTime();
    if (timeSinceLastReload > 300000) { // 5 minutes
      return 'degraded';
    }

    return 'healthy';
  }

  /**
   * Load current configuration from proxy
   */
  private async loadCurrentConfig(): Promise<void> {
    try {
      const response = await fetch('/api/proxy/config');
      if (response.ok) {
        this.currentConfig = await response.json();
      }
    } catch (error) {
      console.error('Failed to load current configuration:', error);
    }
  }

  /**
   * Test policy against specific URL
   */
  async testPolicy(policy: PolicyFormData, testUrl: string): Promise<{
    action: 'allow' | 'block' | 'warn' | 'inspect';
    reason: string;
    matchedRules: string[];
    testConfig: any;
  }> {
    const testConfig = this.configGenerator.generateTestConfig(policy, testUrl);
    
    try {
      const response = await fetch('/api/proxy/test', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          config: testConfig,
          testUrl,
          policy: policy
        })
      });

      if (response.ok) {
        const result = await response.json();
        return {
          action: result.result?.action || 'block',
          reason: result.result?.reason || 'Unknown',
          matchedRules: result.result?.matchedRules || [],
          testConfig: result.result?.testConfig || testConfig
        };
      } else {
        throw new Error(`Test failed: ${response.statusText}`);
      }
    } catch (error) {
      console.error('Policy test failed:', error);
      return {
        action: 'block',
        reason: `Test failed: ${error instanceof Error ? error.message : 'Unknown error'}`,
        matchedRules: [],
        testConfig
      };
    }
  }

  /**
   * Get current reload status
   */
  getReloadStatus(): ReloadStatus {
    return { ...this.reloadStatus };
  }

  /**
   * Get current proxy status
   */
  getProxyStatus(): ProxyStatus {
    return { ...this.proxyStatus };
  }

  /**
   * Get current configuration
   */
  getCurrentConfig(): any {
    return this.currentConfig;
  }

  /**
   * Add event listener
   */
  on(event: string, listener: Function): void {
    if (!this.eventListeners.has(event)) {
      this.eventListeners.set(event, []);
    }
    this.eventListeners.get(event)!.push(listener);
  }

  /**
   * Remove event listener
   */
  off(event: string, listener: Function): void {
    const listeners = this.eventListeners.get(event);
    if (listeners) {
      const index = listeners.indexOf(listener);
      if (index > -1) {
        listeners.splice(index, 1);
      }
    }
  }

  /**
   * Emit event
   */
  private emit(event: string, data: any): void {
    const listeners = this.eventListeners.get(event);
    if (listeners) {
      listeners.forEach(listener => {
        try {
          listener(data);
        } catch (error) {
          console.error(`Error in event listener for ${event}:`, error);
        }
      });
    }
  }

  /**
   * Start monitoring proxy health
   */
  startHealthMonitoring(intervalMs: number = 30000): void {
    setInterval(async () => {
      await this.checkProxyStatus();
      this.emit('healthCheck', { status: this.proxyStatus });
    }, intervalMs);
  }

  /**
   * Stop monitoring
   */
  stopHealthMonitoring(): void {
    if (this.reloadTimeout) {
      clearTimeout(this.reloadTimeout);
      this.reloadTimeout = null;
    }
  }

  /**
   * Cleanup resources
   */
  destroy(): void {
    this.stopHealthMonitoring();
    this.eventListeners.clear();
  }
}
