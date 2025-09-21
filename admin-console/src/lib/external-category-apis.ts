export interface ExternalCategoryResult {
  source: string;
  category: string;
  confidence: number;
  riskLevel: 'low' | 'medium' | 'high' | 'critical';
  subcategories: string[];
  additionalData: Record<string, any>;
  timestamp: string;
}

export interface CategoryProvider {
  name: string;
  baseUrl: string;
  apiKey?: string;
  rateLimit: number; // requests per minute
  freeTier: boolean;
  categories: string[];
  enabled: boolean;
}

export class ExternalCategoryAPIs {
  private static providers: CategoryProvider[] = [
    {
      name: 'VirusTotal',
      baseUrl: 'https://www.virustotal.com/api/v3',
      apiKey: process.env.VIRUSTOTAL_API_KEY,
      rateLimit: 4, // 4 requests per minute for free tier
      freeTier: true,
      categories: ['malware', 'phishing', 'suspicious', 'clean'],
      enabled: !!process.env.VIRUSTOTAL_API_KEY
    },
    {
      name: 'URLVoid',
      baseUrl: 'https://api.urlvoid.com/v1',
      apiKey: process.env.URLVOID_API_KEY,
      rateLimit: 10, // 10 requests per minute for free tier
      freeTier: true,
      categories: ['malware', 'phishing', 'suspicious', 'clean'],
      enabled: !!process.env.URLVOID_API_KEY
    },
    {
      name: 'AbuseIPDB',
      baseUrl: 'https://api.abuseipdb.com/api/v2',
      apiKey: process.env.ABUSEIPDB_API_KEY,
      rateLimit: 10, // 10 requests per minute for free tier
      freeTier: true,
      categories: ['malware', 'phishing', 'suspicious', 'clean'],
      enabled: !!process.env.ABUSEIPDB_API_KEY
    },
    {
      name: 'WebsiteCategorizationAPI',
      baseUrl: 'https://api.websitecategorizationapi.com/v1',
      apiKey: process.env.WEBSITE_CATEGORIZATION_API_KEY,
      rateLimit: 100, // 100 requests per minute for free tier
      freeTier: true,
      categories: ['adult', 'business', 'education', 'entertainment', 'games', 'health', 'news', 'shopping', 'social', 'technology'],
      enabled: !!process.env.WEBSITE_CATEGORIZATION_API_KEY
    },
    {
      name: 'ContentModerationAPI',
      baseUrl: 'https://api.contentmoderationapi.net/v1',
      apiKey: process.env.CONTENT_MODERATION_API_KEY,
      rateLimit: 50, // 50 requests per minute for free tier
      freeTier: true,
      categories: ['adult', 'business', 'education', 'entertainment', 'games', 'health', 'news', 'shopping', 'social', 'technology'],
      enabled: !!process.env.CONTENT_MODERATION_API_KEY
    }
  ];

  private static rateLimitTracker: Map<string, { count: number; resetTime: number }> = new Map();

  static async lookupURL(url: string, preferredProviders?: string[]): Promise<ExternalCategoryResult[]> {
    const results: ExternalCategoryResult[] = [];
    const providers = preferredProviders 
      ? this.providers.filter(p => preferredProviders.includes(p.name) && p.enabled)
      : this.providers.filter(p => p.enabled);

    // Check rate limits and execute requests
    const promises = providers.map(async (provider) => {
      if (!this.canMakeRequest(provider)) {
        console.warn(`Rate limit exceeded for ${provider.name}`);
        return null;
      }

      try {
        const result = await this.queryProvider(provider, url);
        if (result) {
          this.updateRateLimit(provider);
          return result;
        }
      } catch (error) {
        console.error(`Error querying ${provider.name}:`, error);
      }
      return null;
    });

    const responses = await Promise.allSettled(promises);
    
    responses.forEach((response) => {
      if (response.status === 'fulfilled' && response.value) {
        results.push(response.value);
      }
    });

    return results;
  }

  private static async queryProvider(provider: CategoryProvider, url: string): Promise<ExternalCategoryResult | null> {
    const domain = this.extractDomain(url);
    
    switch (provider.name) {
      case 'VirusTotal':
        return this.queryVirusTotal(provider, domain);
      case 'URLVoid':
        return this.queryURLVoid(provider, domain);
      case 'AbuseIPDB':
        return this.queryAbuseIPDB(provider, domain);
      case 'WebsiteCategorizationAPI':
        return this.queryWebsiteCategorizationAPI(provider, url);
      case 'ContentModerationAPI':
        return this.queryContentModerationAPI(provider, url);
      default:
        return null;
    }
  }

  private static async queryVirusTotal(provider: CategoryProvider, domain: string): Promise<ExternalCategoryResult | null> {
    try {
      const response = await fetch(`${provider.baseUrl}/domains/${domain}`, {
        headers: {
          'X-Apikey': provider.apiKey || process.env.VIRUSTOTAL_API_KEY || '',
        },
      });

      if (!response.ok) {
        throw new Error(`VirusTotal API error: ${response.status}`);
      }

      const data = await response.json();
      const attributes = data.data?.attributes;
      
      if (!attributes) return null;

      // Analyze reputation scores
      const reputation = attributes.last_analysis_stats;
      const malicious = reputation?.malicious || 0;
      const suspicious = reputation?.suspicious || 0;
      const undetected = reputation?.undetected || 0;
      const harmless = reputation?.harmless || 0;

      let category = 'clean';
      let riskLevel: 'low' | 'medium' | 'high' | 'critical' = 'low';
      let confidence = 0.5;

      if (malicious > 0) {
        category = 'malware';
        riskLevel = malicious > 5 ? 'critical' : 'high';
        confidence = Math.min(malicious / 10, 1.0);
      } else if (suspicious > 0) {
        category = 'suspicious';
        riskLevel = 'medium';
        confidence = Math.min(suspicious / 5, 1.0);
      } else if (harmless > 0) {
        category = 'clean';
        riskLevel = 'low';
        confidence = Math.min(harmless / 10, 1.0);
      }

      return {
        source: provider.name,
        category,
        confidence,
        riskLevel,
        subcategories: [],
        additionalData: {
          malicious,
          suspicious,
          undetected,
          harmless,
          lastAnalysisDate: attributes.last_analysis_date
        },
        timestamp: new Date().toISOString()
      };
    } catch (error) {
      console.error('VirusTotal query error:', error);
      return null;
    }
  }

  private static async queryURLVoid(provider: CategoryProvider, domain: string): Promise<ExternalCategoryResult | null> {
    try {
      const response = await fetch(`${provider.baseUrl}/host/${domain}`, {
        headers: {
          'API-Key': provider.apiKey || process.env.URLVOID_API_KEY || '',
        },
      });

      if (!response.ok) {
        throw new Error(`URLVoid API error: ${response.status}`);
      }

      const data = await response.json();
      const result = data.result;
      
      if (!result) return null;

      const detections = result.detections || 0;
      const engines = result.engines || 0;
      
      let category = 'clean';
      let riskLevel: 'low' | 'medium' | 'high' | 'critical' = 'low';
      let confidence = 0.5;

      if (detections > 0) {
        category = 'malware';
        riskLevel = detections > 5 ? 'critical' : detections > 2 ? 'high' : 'medium';
        confidence = Math.min(detections / engines, 1.0);
      }

      return {
        source: provider.name,
        category,
        confidence,
        riskLevel,
        subcategories: [],
        additionalData: {
          detections,
          engines,
          country: result.country,
          ip: result.ip
        },
        timestamp: new Date().toISOString()
      };
    } catch (error) {
      console.error('URLVoid query error:', error);
      return null;
    }
  }

  private static async queryAbuseIPDB(provider: CategoryProvider, domain: string): Promise<ExternalCategoryResult | null> {
    try {
      const response = await fetch(`${provider.baseUrl}/check?domain=${domain}&maxAgeInDays=90&verbose`, {
        headers: {
          'Key': provider.apiKey || process.env.ABUSEIPDB_API_KEY || '',
          'Accept': 'application/json',
        },
      });

      if (!response.ok) {
        throw new Error(`AbuseIPDB API error: ${response.status}`);
      }

      const data = await response.json();
      const result = data.data;
      
      if (!result) return null;

      const abuseConfidence = result.abuseConfidencePercentage || 0;
      const usageType = result.usageType || 'unknown';
      
      let category = 'clean';
      let riskLevel: 'low' | 'medium' | 'high' | 'critical' = 'low';
      let confidence = 0.5;

      if (abuseConfidence > 75) {
        category = 'malware';
        riskLevel = 'critical';
        confidence = abuseConfidence / 100;
      } else if (abuseConfidence > 50) {
        category = 'suspicious';
        riskLevel = 'high';
        confidence = abuseConfidence / 100;
      } else if (abuseConfidence > 25) {
        category = 'suspicious';
        riskLevel = 'medium';
        confidence = abuseConfidence / 100;
      }

      return {
        source: provider.name,
        category,
        confidence,
        riskLevel,
        subcategories: [],
        additionalData: {
          abuseConfidence,
          usageType,
          country: result.countryCode,
          isp: result.isp
        },
        timestamp: new Date().toISOString()
      };
    } catch (error) {
      console.error('AbuseIPDB query error:', error);
      return null;
    }
  }

  private static async queryWebsiteCategorizationAPI(provider: CategoryProvider, url: string): Promise<ExternalCategoryResult | null> {
    try {
      const response = await fetch(`${provider.baseUrl}/categorize?url=${encodeURIComponent(url)}`, {
        headers: {
          'Authorization': `Bearer ${provider.apiKey || process.env.WEBSITE_CATEGORIZATION_API_KEY || ''}`,
        },
      });

      if (!response.ok) {
        throw new Error(`WebsiteCategorizationAPI error: ${response.status}`);
      }

      const data = await response.json();
      
      if (!data.category) return null;

      const category = data.category.toLowerCase();
      const confidence = data.confidence || 0.5;
      
      // Map to our risk levels
      let riskLevel: 'low' | 'medium' | 'high' | 'critical' = 'low';
      if (['adult', 'malware', 'phishing'].includes(category)) {
        riskLevel = 'critical';
      } else if (['gambling', 'weapons', 'drugs'].includes(category)) {
        riskLevel = 'high';
      } else if (['entertainment', 'games', 'social'].includes(category)) {
        riskLevel = 'medium';
      }

      return {
        source: provider.name,
        category,
        confidence,
        riskLevel,
        subcategories: data.subcategories || [],
        additionalData: {
          iabCategory: data.iabCategory,
          country: data.country,
          language: data.language
        },
        timestamp: new Date().toISOString()
      };
    } catch (error) {
      console.error('WebsiteCategorizationAPI query error:', error);
      return null;
    }
  }

  private static async queryContentModerationAPI(provider: CategoryProvider, url: string): Promise<ExternalCategoryResult | null> {
    try {
      const response = await fetch(`${provider.baseUrl}/categorize`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${provider.apiKey || process.env.CONTENT_MODERATION_API_KEY || ''}`,
        },
        body: JSON.stringify({ url }),
      });

      if (!response.ok) {
        throw new Error(`ContentModerationAPI error: ${response.status}`);
      }

      const data = await response.json();
      
      if (!data.category) return null;

      const category = data.category.toLowerCase();
      const confidence = data.confidence || 0.5;
      
      // Map to our risk levels
      let riskLevel: 'low' | 'medium' | 'high' | 'critical' = 'low';
      if (['adult', 'malware', 'phishing'].includes(category)) {
        riskLevel = 'critical';
      } else if (['gambling', 'weapons', 'drugs'].includes(category)) {
        riskLevel = 'high';
      } else if (['entertainment', 'games', 'social'].includes(category)) {
        riskLevel = 'medium';
      }

      return {
        source: provider.name,
        category,
        confidence,
        riskLevel,
        subcategories: data.subcategories || [],
        additionalData: {
          iabCategory: data.iabCategory,
          webFilteringCategory: data.webFilteringCategory,
          country: data.country,
          domainAge: data.domainAge
        },
        timestamp: new Date().toISOString()
      };
    } catch (error) {
      console.error('ContentModerationAPI query error:', error);
      return null;
    }
  }

  private static canMakeRequest(provider: CategoryProvider): boolean {
    const now = Date.now();
    const tracker = this.rateLimitTracker.get(provider.name);
    
    if (!tracker) {
      this.rateLimitTracker.set(provider.name, { count: 1, resetTime: now + 60000 });
      return true;
    }

    if (now > tracker.resetTime) {
      this.rateLimitTracker.set(provider.name, { count: 1, resetTime: now + 60000 });
      return true;
    }

    return tracker.count < provider.rateLimit;
  }

  private static updateRateLimit(provider: CategoryProvider): void {
    const tracker = this.rateLimitTracker.get(provider.name);
    if (tracker) {
      tracker.count++;
    }
  }

  private static extractDomain(url: string): string {
    try {
      return new URL(url).hostname;
    } catch {
      return url;
    }
  }

  static getProviders(): CategoryProvider[] {
    return this.providers;
  }

  static updateProvider(name: string, updates: Partial<CategoryProvider>): boolean {
    const provider = this.providers.find(p => p.name === name);
    if (!provider) return false;

    Object.assign(provider, updates);
    return true;
  }

  static addProvider(provider: CategoryProvider): void {
    this.providers.push(provider);
  }

  static removeProvider(name: string): boolean {
    const index = this.providers.findIndex(p => p.name === name);
    if (index === -1) return false;

    this.providers.splice(index, 1);
    return true;
  }
}
