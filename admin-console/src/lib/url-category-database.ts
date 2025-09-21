export interface URLCategory {
  id: string;
  name: string;
  description: string;
  riskLevel: 'low' | 'medium' | 'high' | 'critical';
  parentCategory?: string;
  subcategories: string[];
  keywords: string[];
  domains: string[];
  patterns: string[];
  blockRecommended: boolean;
  warnRecommended: boolean;
  allowRecommended: boolean;
  lastUpdated: string;
  source: 'manual' | 'ai' | 'community' | 'vendor';
}

export interface CategoryLookupResult {
  category: URLCategory;
  confidence: number;
  matchedPatterns: string[];
  matchedKeywords: string[];
  matchedDomains: string[];
}

export class URLCategoryDatabase {
  private static categories: Map<string, URLCategory> = new Map();
  private static initialized = false;

  static initialize() {
    if (this.initialized) return;

    // Security Categories
    this.addCategory({
      id: 'malware',
      name: 'Malware',
      description: 'Sites hosting or distributing malicious software',
      riskLevel: 'critical',
      subcategories: ['virus', 'trojan', 'spyware', 'ransomware'],
      keywords: ['malware', 'virus', 'trojan', 'spyware', 'ransomware', 'keylogger', 'backdoor'],
      domains: ['malware-site.com', 'virus-distribution.net', 'trojan-horse.org'],
      patterns: ['*malware*', '*virus*', '*trojan*', '*spyware*', '*ransomware*'],
      blockRecommended: true,
      warnRecommended: false,
      allowRecommended: false,
      lastUpdated: '2024-01-15T10:00:00Z',
      source: 'vendor'
    });

    this.addCategory({
      id: 'phishing',
      name: 'Phishing',
      description: 'Sites attempting to steal personal information',
      riskLevel: 'critical',
      subcategories: ['fake-banking', 'fake-social', 'fake-email'],
      keywords: ['phishing', 'fake', 'scam', 'steal', 'password', 'login', 'banking'],
      domains: ['fake-bank.com', 'phishing-site.net', 'scam-email.org'],
      patterns: ['*phishing*', '*fake*', '*scam*', '*steal*'],
      blockRecommended: true,
      warnRecommended: false,
      allowRecommended: false,
      lastUpdated: '2024-01-15T10:00:00Z',
      source: 'vendor'
    });

    this.addCategory({
      id: 'adult-content',
      name: 'Adult Content',
      description: 'Sites containing adult or explicit content',
      riskLevel: 'medium',
      subcategories: ['pornography', 'adult-dating', 'adult-games'],
      keywords: ['adult', 'porn', 'sex', 'nude', 'explicit', 'xxx'],
      domains: ['adult-site.com', 'porn-hub.net', 'adult-dating.org'],
      patterns: ['*adult*', '*porn*', '*sex*', '*xxx*'],
      blockRecommended: true,
      warnRecommended: true,
      allowRecommended: false,
      lastUpdated: '2024-01-15T10:00:00Z',
      source: 'vendor'
    });

    // Productivity Categories
    this.addCategory({
      id: 'social-media',
      name: 'Social Media',
      description: 'Social networking and media sharing platforms',
      riskLevel: 'low',
      subcategories: ['social-networking', 'media-sharing', 'messaging'],
      keywords: ['social', 'facebook', 'twitter', 'instagram', 'linkedin', 'tiktok', 'snapchat'],
      domains: ['facebook.com', 'twitter.com', 'instagram.com', 'linkedin.com', 'tiktok.com', 'snapchat.com'],
      patterns: ['*facebook*', '*twitter*', '*instagram*', '*linkedin*', '*tiktok*', '*snapchat*'],
      blockRecommended: false,
      warnRecommended: true,
      allowRecommended: true,
      lastUpdated: '2024-01-15T10:00:00Z',
      source: 'vendor'
    });

    this.addCategory({
      id: 'streaming',
      name: 'Streaming Media',
      description: 'Video and audio streaming services',
      riskLevel: 'low',
      subcategories: ['video-streaming', 'music-streaming', 'live-streaming'],
      keywords: ['streaming', 'youtube', 'netflix', 'spotify', 'twitch', 'hulu', 'disney'],
      domains: ['youtube.com', 'netflix.com', 'spotify.com', 'twitch.tv', 'hulu.com', 'disney.com'],
      patterns: ['*youtube*', '*netflix*', '*spotify*', '*twitch*', '*hulu*', '*disney*'],
      blockRecommended: false,
      warnRecommended: true,
      allowRecommended: true,
      lastUpdated: '2024-01-15T10:00:00Z',
      source: 'vendor'
    });

    this.addCategory({
      id: 'gaming',
      name: 'Gaming',
      description: 'Online gaming platforms and game-related content',
      riskLevel: 'low',
      subcategories: ['online-gaming', 'gaming-news', 'gaming-streaming'],
      keywords: ['gaming', 'game', 'steam', 'epic', 'xbox', 'playstation', 'nintendo'],
      domains: ['steam.com', 'epicgames.com', 'xbox.com', 'playstation.com', 'nintendo.com'],
      patterns: ['*steam*', '*epic*', '*xbox*', '*playstation*', '*nintendo*'],
      blockRecommended: false,
      warnRecommended: true,
      allowRecommended: true,
      lastUpdated: '2024-01-15T10:00:00Z',
      source: 'vendor'
    });

    // Business Categories
    this.addCategory({
      id: 'business-tools',
      name: 'Business Tools',
      description: 'Professional productivity and business applications',
      riskLevel: 'low',
      subcategories: ['office-suites', 'project-management', 'communication'],
      keywords: ['office', 'microsoft', 'google', 'slack', 'teams', 'zoom', 'asana', 'trello'],
      domains: ['office.com', 'google.com', 'slack.com', 'teams.microsoft.com', 'zoom.us', 'asana.com', 'trello.com'],
      patterns: ['*office*', '*google*', '*slack*', '*teams*', '*zoom*', '*asana*', '*trello*'],
      blockRecommended: false,
      warnRecommended: false,
      allowRecommended: true,
      lastUpdated: '2024-01-15T10:00:00Z',
      source: 'vendor'
    });

    this.addCategory({
      id: 'banking',
      name: 'Banking & Finance',
      description: 'Financial institutions and banking services',
      riskLevel: 'low',
      subcategories: ['banks', 'credit-unions', 'investment', 'cryptocurrency'],
      keywords: ['bank', 'banking', 'finance', 'financial', 'investment', 'crypto', 'bitcoin'],
      domains: ['chase.com', 'bankofamerica.com', 'wellsfargo.com', 'paypal.com', 'coinbase.com'],
      patterns: ['*bank*', '*finance*', '*financial*', '*investment*', '*crypto*'],
      blockRecommended: false,
      warnRecommended: false,
      allowRecommended: true,
      lastUpdated: '2024-01-15T10:00:00Z',
      source: 'vendor'
    });

    // Educational Categories
    this.addCategory({
      id: 'education',
      name: 'Education',
      description: 'Educational institutions and learning platforms',
      riskLevel: 'low',
      subcategories: ['universities', 'online-learning', 'research'],
      keywords: ['education', 'university', 'college', 'school', 'learning', 'course', 'academic'],
      domains: ['harvard.edu', 'mit.edu', 'coursera.org', 'edx.org', 'khanacademy.org'],
      patterns: ['*.edu', '*university*', '*college*', '*school*', '*learning*'],
      blockRecommended: false,
      warnRecommended: false,
      allowRecommended: true,
      lastUpdated: '2024-01-15T10:00:00Z',
      source: 'vendor'
    });

    // News and Information
    this.addCategory({
      id: 'news',
      name: 'News & Information',
      description: 'News websites and information sources',
      riskLevel: 'low',
      subcategories: ['general-news', 'tech-news', 'business-news'],
      keywords: ['news', 'newspaper', 'article', 'journalism', 'reporter'],
      domains: ['cnn.com', 'bbc.com', 'reuters.com', 'techcrunch.com', 'bloomberg.com'],
      patterns: ['*news*', '*newspaper*', '*article*'],
      blockRecommended: false,
      warnRecommended: false,
      allowRecommended: true,
      lastUpdated: '2024-01-15T10:00:00Z',
      source: 'vendor'
    });

    // Shopping Categories
    this.addCategory({
      id: 'shopping',
      name: 'Shopping & E-commerce',
      description: 'Online shopping and e-commerce platforms',
      riskLevel: 'low',
      subcategories: ['retail', 'marketplaces', 'auctions'],
      keywords: ['shop', 'shopping', 'store', 'buy', 'sell', 'amazon', 'ebay'],
      domains: ['amazon.com', 'ebay.com', 'walmart.com', 'target.com', 'etsy.com'],
      patterns: ['*shop*', '*store*', '*buy*', '*sell*'],
      blockRecommended: false,
      warnRecommended: false,
      allowRecommended: true,
      lastUpdated: '2024-01-15T10:00:00Z',
      source: 'vendor'
    });

    // Technology Categories
    this.addCategory({
      id: 'technology',
      name: 'Technology',
      description: 'Technology news, resources, and tools',
      riskLevel: 'low',
      subcategories: ['tech-news', 'software', 'hardware', 'programming'],
      keywords: ['tech', 'technology', 'software', 'hardware', 'programming', 'coding'],
      domains: ['techcrunch.com', 'arstechnica.com', 'github.com', 'stackoverflow.com'],
      patterns: ['*tech*', '*software*', '*hardware*', '*programming*'],
      blockRecommended: false,
      warnRecommended: false,
      allowRecommended: true,
      lastUpdated: '2024-01-15T10:00:00Z',
      source: 'vendor'
    });

    this.initialized = true;
  }

  private static addCategory(category: URLCategory) {
    this.categories.set(category.id, category);
  }

  static getCategory(id: string): URLCategory | undefined {
    this.initialize();
    return this.categories.get(id);
  }

  static getAllCategories(): URLCategory[] {
    this.initialize();
    return Array.from(this.categories.values());
  }

  static getCategoriesByRiskLevel(riskLevel: string): URLCategory[] {
    this.initialize();
    return Array.from(this.categories.values()).filter(cat => cat.riskLevel === riskLevel);
  }

  static getCategoriesBySource(source: string): URLCategory[] {
    this.initialize();
    return Array.from(this.categories.values()).filter(cat => cat.source === source);
  }

  static lookupURL(url: string): CategoryLookupResult[] {
    this.initialize();
    const results: CategoryLookupResult[] = [];
    const domain = this.extractDomain(url);
    const path = this.extractPath(url);

    for (const category of this.categories.values()) {
      let confidence = 0;
      const matchedPatterns: string[] = [];
      const matchedKeywords: string[] = [];
      const matchedDomains: string[] = [];

      // Check domain matches
      for (const categoryDomain of category.domains) {
        if (this.matchesDomain(domain, categoryDomain)) {
          confidence += 0.8;
          matchedDomains.push(categoryDomain);
        }
      }

      // Check pattern matches
      for (const pattern of category.patterns) {
        if (this.matchesPattern(url, pattern)) {
          confidence += 0.6;
          matchedPatterns.push(pattern);
        }
      }

      // Check keyword matches in URL
      for (const keyword of category.keywords) {
        if (url.toLowerCase().includes(keyword.toLowerCase())) {
          confidence += 0.4;
          matchedKeywords.push(keyword);
        }
      }

      // Check keyword matches in path
      for (const keyword of category.keywords) {
        if (path.toLowerCase().includes(keyword.toLowerCase())) {
          confidence += 0.3;
          matchedKeywords.push(keyword);
        }
      }

      if (confidence > 0.1) {
        results.push({
          category,
          confidence: Math.min(confidence, 1.0),
          matchedPatterns,
          matchedKeywords,
          matchedDomains
        });
      }
    }

    return results.sort((a, b) => b.confidence - a.confidence);
  }

  static getBestMatch(url: string): CategoryLookupResult | null {
    const results = this.lookupURL(url);
    return results.length > 0 ? results[0] : null;
  }

  static getRecommendedAction(url: string, policyType: 'block' | 'warn' | 'allow'): string {
    const bestMatch = this.getBestMatch(url);
    if (!bestMatch) return 'allow';

    const category = bestMatch.category;
    
    switch (policyType) {
      case 'block':
        return category.blockRecommended ? 'block' : 'allow';
      case 'warn':
        return category.warnRecommended ? 'warn' : 'allow';
      case 'allow':
        return category.allowRecommended ? 'allow' : 'warn';
      default:
        return 'allow';
    }
  }

  static searchCategories(query: string): URLCategory[] {
    this.initialize();
    const lowercaseQuery = query.toLowerCase();
    
    return Array.from(this.categories.values()).filter(category => 
      category.name.toLowerCase().includes(lowercaseQuery) ||
      category.description.toLowerCase().includes(lowercaseQuery) ||
      category.keywords.some(keyword => keyword.toLowerCase().includes(lowercaseQuery)) ||
      category.subcategories.some(sub => sub.toLowerCase().includes(lowercaseQuery))
    );
  }

  static getCategoryStats(): {
    total: number;
    byRiskLevel: Record<string, number>;
    bySource: Record<string, number>;
    blockRecommended: number;
    warnRecommended: number;
    allowRecommended: number;
  } {
    this.initialize();
    const categories = Array.from(this.categories.values());
    
    const byRiskLevel: Record<string, number> = {};
    const bySource: Record<string, number> = {};
    
    let blockRecommended = 0;
    let warnRecommended = 0;
    let allowRecommended = 0;

    for (const category of categories) {
      byRiskLevel[category.riskLevel] = (byRiskLevel[category.riskLevel] || 0) + 1;
      bySource[category.source] = (bySource[category.source] || 0) + 1;
      
      if (category.blockRecommended) blockRecommended++;
      if (category.warnRecommended) warnRecommended++;
      if (category.allowRecommended) allowRecommended++;
    }

    return {
      total: categories.length,
      byRiskLevel,
      bySource,
      blockRecommended,
      warnRecommended,
      allowRecommended
    };
  }

  // Helper methods
  private static extractDomain(url: string): string {
    try {
      return new URL(url).hostname;
    } catch {
      return url;
    }
  }

  private static extractPath(url: string): string {
    try {
      return new URL(url).pathname;
    } catch {
      return url;
    }
  }

  private static matchesDomain(domain: string, pattern: string): boolean {
    if (pattern.startsWith('*.')) {
      const suffix = pattern.substring(2);
      return domain.endsWith(suffix);
    }
    return domain === pattern;
  }

  private static matchesPattern(url: string, pattern: string): boolean {
    const regex = new RegExp('^' + pattern.replace(/\*/g, '.*') + '$', 'i');
    return regex.test(url);
  }

  // Add custom category
  static addCustomCategory(category: Omit<URLCategory, 'id' | 'lastUpdated'>): string {
    this.initialize();
    const id = `custom-${Date.now()}`;
    const newCategory: URLCategory = {
      ...category,
      id,
      lastUpdated: new Date().toISOString(),
      source: 'manual'
    };
    this.categories.set(id, newCategory);
    return id;
  }

  // Update category
  static updateCategory(id: string, updates: Partial<URLCategory>): boolean {
    this.initialize();
    const category = this.categories.get(id);
    if (!category) return false;

    const updatedCategory = {
      ...category,
      ...updates,
      lastUpdated: new Date().toISOString()
    };
    this.categories.set(id, updatedCategory);
    return true;
  }

  // Delete category
  static deleteCategory(id: string): boolean {
    this.initialize();
    return this.categories.delete(id);
  }
}
