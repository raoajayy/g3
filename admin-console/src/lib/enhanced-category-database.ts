import { URLCategoryDatabase, URLCategory, CategoryLookupResult } from './url-category-database';
import { ExternalCategoryAPIs, ExternalCategoryResult } from './external-category-apis';

export interface EnhancedCategoryResult extends CategoryLookupResult {
  externalResults: ExternalCategoryResult[];
  consensusCategory: string;
  consensusConfidence: number;
  riskScore: number;
  lastUpdated: string;
}

export class EnhancedCategoryDatabase extends URLCategoryDatabase {
  private static cache: Map<string, EnhancedCategoryResult> = new Map();
  private static cacheExpiry = 24 * 60 * 60 * 1000; // 24 hours

  static async lookupURL(url: string, useExternal: boolean = true): Promise<EnhancedCategoryResult> {
    const cacheKey = url.toLowerCase();
    const cached = this.cache.get(cacheKey);
    
    // Check cache validity
    if (cached && Date.now() - new Date(cached.lastUpdated).getTime() < this.cacheExpiry) {
      return cached;
    }

    // Get internal categorization
    const internalResults = super.lookupURL(url);
    const bestInternalMatch = internalResults.length > 0 ? internalResults[0] : null;

    let externalResults: ExternalCategoryResult[] = [];
    let consensusCategory = 'unknown';
    let consensusConfidence = 0;
    let riskScore = 0;

    if (useExternal) {
      try {
        externalResults = await ExternalCategoryAPIs.lookupURL(url);
        
        // Calculate consensus
        const consensus = this.calculateConsensus(internalResults, externalResults);
        consensusCategory = consensus.category;
        consensusConfidence = consensus.confidence;
        riskScore = consensus.riskScore;
      } catch (error) {
        console.error('Error fetching external categories:', error);
        // Fallback to internal results
        if (bestInternalMatch) {
          consensusCategory = bestInternalMatch.category.name;
          consensusConfidence = bestInternalMatch.confidence;
          riskScore = this.calculateRiskScore(bestInternalMatch.category.riskLevel);
        }
      }
    } else if (bestInternalMatch) {
      consensusCategory = bestInternalMatch.category.name;
      consensusConfidence = bestInternalMatch.confidence;
      riskScore = this.calculateRiskScore(bestInternalMatch.category.riskLevel);
    }

    const result: EnhancedCategoryResult = {
      category: bestInternalMatch?.category || this.getDefaultCategory(),
      confidence: consensusConfidence,
      matchedPatterns: bestInternalMatch?.matchedPatterns || [],
      matchedKeywords: bestInternalMatch?.matchedKeywords || [],
      matchedDomains: bestInternalMatch?.matchedDomains || [],
      externalResults,
      consensusCategory,
      consensusConfidence,
      riskScore,
      lastUpdated: new Date().toISOString()
    };

    // Cache the result
    this.cache.set(cacheKey, result);
    return result;
  }

  private static calculateConsensus(
    internalResults: CategoryLookupResult[],
    externalResults: ExternalCategoryResult[]
  ): { category: string; confidence: number; riskScore: number } {
    const allResults = [
      ...internalResults.map(r => ({ 
        category: r.category.name, 
        confidence: r.confidence, 
        riskLevel: r.category.riskLevel,
        source: 'internal'
      })),
      ...externalResults.map(r => ({ 
        category: r.category, 
        confidence: r.confidence, 
        riskLevel: r.riskLevel,
        source: r.source
      }))
    ];

    if (allResults.length === 0) {
      return { category: 'unknown', confidence: 0, riskScore: 0 };
    }

    // Group by category
    const categoryGroups = new Map<string, typeof allResults>();
    allResults.forEach(result => {
      const normalizedCategory = this.normalizeCategoryName(result.category);
      if (!categoryGroups.has(normalizedCategory)) {
        categoryGroups.set(normalizedCategory, []);
      }
      categoryGroups.get(normalizedCategory)!.push(result);
    });

    // Find the category with highest weighted confidence
    let bestCategory = 'unknown';
    let bestConfidence = 0;
    let bestRiskScore = 0;

    for (const [category, results] of categoryGroups) {
      const weightedConfidence = results.reduce((sum, result) => {
        const weight = result.source === 'internal' ? 1.2 : 1.0; // Slight preference for internal
        return sum + (result.confidence * weight);
      }, 0) / results.length;

      const avgRiskScore = results.reduce((sum, result) => {
        return sum + this.calculateRiskScore(result.riskLevel);
      }, 0) / results.length;

      if (weightedConfidence > bestConfidence) {
        bestCategory = category;
        bestConfidence = weightedConfidence;
        bestRiskScore = avgRiskScore;
      }
    }

    return {
      category: bestCategory,
      confidence: Math.min(bestConfidence, 1.0),
      riskScore: bestRiskScore
    };
  }

  private static normalizeCategoryName(category: string): string {
    const mappings: Record<string, string> = {
      'malware': 'malware',
      'virus': 'malware',
      'trojan': 'malware',
      'spyware': 'malware',
      'ransomware': 'malware',
      'phishing': 'phishing',
      'fake': 'phishing',
      'scam': 'phishing',
      'adult': 'adult-content',
      'porn': 'adult-content',
      'adult-content': 'adult-content',
      'social': 'social-media',
      'social-media': 'social-media',
      'facebook': 'social-media',
      'twitter': 'social-media',
      'instagram': 'social-media',
      'linkedin': 'social-media',
      'streaming': 'streaming',
      'youtube': 'streaming',
      'netflix': 'streaming',
      'spotify': 'streaming',
      'gaming': 'gaming',
      'games': 'gaming',
      'steam': 'gaming',
      'epic': 'gaming',
      'business': 'business-tools',
      'business-tools': 'business-tools',
      'office': 'business-tools',
      'google': 'business-tools',
      'banking': 'banking',
      'bank': 'banking',
      'finance': 'banking',
      'education': 'education',
      'university': 'education',
      'school': 'education',
      'news': 'news',
      'newspaper': 'news',
      'shopping': 'shopping',
      'shop': 'shopping',
      'ecommerce': 'shopping',
      'technology': 'technology',
      'tech': 'technology',
      'programming': 'technology',
      'clean': 'business-tools',
      'suspicious': 'malware'
    };

    return mappings[category.toLowerCase()] || category.toLowerCase();
  }

  private static calculateRiskScore(riskLevel: string): number {
    switch (riskLevel) {
      case 'critical': return 1.0;
      case 'high': return 0.8;
      case 'medium': return 0.5;
      case 'low': return 0.2;
      default: return 0.5;
    }
  }

  private static getDefaultCategory(): URLCategory {
    return {
      id: 'unknown',
      name: 'Unknown',
      description: 'Category not determined',
      riskLevel: 'low',
      subcategories: [],
      keywords: [],
      domains: [],
      patterns: [],
      blockRecommended: false,
      warnRecommended: false,
      allowRecommended: true,
      lastUpdated: new Date().toISOString(),
      source: 'manual'
    };
  }

  static async bulkLookupURLs(urls: string[], useExternal: boolean = true): Promise<Map<string, EnhancedCategoryResult>> {
    const results = new Map<string, EnhancedCategoryResult>();
    
    // Process in batches to respect rate limits
    const batchSize = 5;
    for (let i = 0; i < urls.length; i += batchSize) {
      const batch = urls.slice(i, i + batchSize);
      const batchPromises = batch.map(async (url) => {
        const result = await this.lookupURL(url, useExternal);
        return { url, result };
      });
      
      const batchResults = await Promise.allSettled(batchPromises);
      batchResults.forEach((promise) => {
        if (promise.status === 'fulfilled') {
          results.set(promise.value.url, promise.value.result);
        }
      });
      
      // Add delay between batches to respect rate limits
      if (i + batchSize < urls.length) {
        await new Promise(resolve => setTimeout(resolve, 1000));
      }
    }
    
    return results;
  }

  static getCacheStats(): { size: number; hitRate: number } {
    return {
      size: this.cache.size,
      hitRate: 0.85 // This would be calculated from actual usage
    };
  }

  static clearCache(): void {
    this.cache.clear();
  }

  static async updateFromExternalSources(): Promise<void> {
    // This would periodically update the internal database with external data
    // For now, we'll just clear the cache to force fresh lookups
    this.clearCache();
  }

  static getExternalProviders(): any[] {
    return ExternalCategoryAPIs.getProviders();
  }

  static async testExternalProvider(providerName: string, testUrl: string): Promise<ExternalCategoryResult | null> {
    try {
      const results = await ExternalCategoryAPIs.lookupURL(testUrl, [providerName]);
      return results.find(r => r.source === providerName) || null;
    } catch (error) {
      console.error(`Error testing provider ${providerName}:`, error);
      return null;
    }
  }
}
