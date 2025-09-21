import { NextRequest, NextResponse } from 'next/server';
import { ProxyConfigGenerator } from '@/lib/proxy-config-generator';

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const { config, testUrl, policy } = body;

    if (!testUrl || !policy) {
      return NextResponse.json(
        { error: 'Missing required fields: testUrl and policy' },
        { status: 400 }
      );
    }

    // Simulate policy evaluation
    const result = await evaluatePolicyAgainstUrl(policy, testUrl);

    return NextResponse.json({
      success: true,
      testUrl,
      policyName: policy.name,
      result,
      timestamp: new Date().toISOString()
    });

  } catch (error) {
    console.error('Policy test failed:', error);
    return NextResponse.json(
      { 
        error: 'Policy test failed',
        details: error instanceof Error ? error.message : 'Unknown error'
      },
      { status: 500 }
    );
  }
}

async function evaluatePolicyAgainstUrl(policy: any, testUrl: string): Promise<{
  action: 'allow' | 'block' | 'warn' | 'inspect';
  reason: string;
  matchedRules: string[];
  testConfig: any;
  details: {
    urlCategory?: string;
    riskScore?: number;
    confidence?: number;
    processingTime: number;
  };
}> {
  const startTime = Date.now();
  
  try {
    // Parse URL
    const url = new URL(testUrl);
    const hostname = url.hostname;
    const pathname = url.pathname;
    const fullUrl = testUrl;

    // Simulate URL categorization
    const urlCategory = await categorizeUrl(testUrl);
    const riskScore = calculateRiskScore(testUrl, urlCategory);
    const confidence = Math.random() * 0.3 + 0.7; // 70-100% confidence

    // Check blocked categories
    const blockedCategories = policy.urlFiltering?.categories?.block || [];
    if (blockedCategories.includes(urlCategory)) {
      return {
        action: 'block',
        reason: `URL blocked by category: ${urlCategory}`,
        matchedRules: [`category_block_${urlCategory}`],
        testConfig: generateTestConfig(policy),
        details: {
          urlCategory,
          riskScore,
          confidence,
          processingTime: Date.now() - startTime
        }
      };
    }

    // Check warned categories
    const warnedCategories = policy.urlFiltering?.categories?.warn || [];
    if (warnedCategories.includes(urlCategory)) {
      return {
        action: 'warn',
        reason: `URL warning by category: ${urlCategory}`,
        matchedRules: [`category_warn_${urlCategory}`],
        testConfig: generateTestConfig(policy),
        details: {
          urlCategory,
          riskScore,
          confidence,
          processingTime: Date.now() - startTime
        }
      };
    }

    // Check custom rules
    const customRules = policy.urlFiltering?.customRules || [];
    const matchedRules: string[] = [];

    for (const rule of customRules) {
      if (matchesRule(testUrl, rule)) {
        matchedRules.push(rule.name);
        
        switch (rule.action) {
          case 'block':
            return {
              action: 'block',
              reason: `URL blocked by rule: ${rule.name}`,
              matchedRules,
              testConfig: generateTestConfig(policy),
              details: {
                urlCategory,
                riskScore,
                confidence,
                processingTime: Date.now() - startTime
              }
            };
          
          case 'warn':
            return {
              action: 'warn',
              reason: `URL warning by rule: ${rule.name}`,
              matchedRules,
              testConfig: generateTestConfig(policy),
              details: {
                urlCategory,
                riskScore,
                confidence,
                processingTime: Date.now() - startTime
              }
            };
          
          case 'inspect':
            return {
              action: 'inspect',
              reason: `URL requires inspection by rule: ${rule.name}`,
              matchedRules,
              testConfig: generateTestConfig(policy),
              details: {
                urlCategory,
                riskScore,
                confidence,
                processingTime: Date.now() - startTime
              }
            };
        }
      }
    }

    // Check HTTPS inspection
    if (policy.httpsInspection?.enabled && url.protocol === 'https:') {
      if (policy.httpsInspection.action === 'block') {
        return {
          action: 'block',
          reason: 'HTTPS inspection blocked',
          matchedRules: ['https_inspection_block'],
          testConfig: generateTestConfig(policy),
          details: {
            urlCategory,
            riskScore,
            confidence,
            processingTime: Date.now() - startTime
          }
        };
      } else if (policy.httpsInspection.action === 'warn') {
        return {
          action: 'warn',
          reason: 'HTTPS inspection warning',
          matchedRules: ['https_inspection_warn'],
          testConfig: generateTestConfig(policy),
          details: {
            urlCategory,
            riskScore,
            confidence,
            processingTime: Date.now() - startTime
          }
        };
      }
    }

    // Default: allow
    return {
      action: 'allow',
      reason: 'No blocking rules matched',
      matchedRules: [],
      testConfig: generateTestConfig(policy),
      details: {
        urlCategory,
        riskScore,
        confidence,
        processingTime: Date.now() - startTime
      }
    };

  } catch (error) {
    return {
      action: 'block',
      reason: `Evaluation error: ${error instanceof Error ? error.message : 'Unknown error'}`,
      matchedRules: ['evaluation_error'],
      testConfig: generateTestConfig(policy),
      details: {
        processingTime: Date.now() - startTime
      }
    };
  }
}

async function categorizeUrl(url: string): Promise<string> {
  // Simulate URL categorization with some common patterns
  const urlObj = new URL(url);
  const hostname = urlObj.hostname.toLowerCase();
  
  // Social media
  if (hostname.includes('facebook') || hostname.includes('twitter') || 
      hostname.includes('instagram') || hostname.includes('linkedin')) {
    return 'social_media';
  }
  
  // News
  if (hostname.includes('news') || hostname.includes('cnn') || 
      hostname.includes('bbc') || hostname.includes('reuters')) {
    return 'news';
  }
  
  // Shopping
  if (hostname.includes('amazon') || hostname.includes('ebay') || 
      hostname.includes('shop') || hostname.includes('store')) {
    return 'shopping';
  }
  
  // Entertainment
  if (hostname.includes('youtube') || hostname.includes('netflix') || 
      hostname.includes('spotify') || hostname.includes('gaming')) {
    return 'entertainment';
  }
  
  // Technology
  if (hostname.includes('github') || hostname.includes('stackoverflow') || 
      hostname.includes('tech') || hostname.includes('developer')) {
    return 'technology';
  }
  
  // Malware (simulate some suspicious patterns)
  if (hostname.includes('malware') || hostname.includes('virus') || 
      hostname.includes('phishing') || hostname.includes('scam')) {
    return 'malware';
  }
  
  // Default
  return 'general';
}

function calculateRiskScore(url: string, category: string): number {
  const riskScores: Record<string, number> = {
    malware: 0.9,
    phishing: 0.8,
    social_media: 0.3,
    news: 0.2,
    shopping: 0.4,
    entertainment: 0.3,
    technology: 0.1,
    general: 0.2
  };
  
  return riskScores[category] || 0.5;
}

function matchesRule(url: string, rule: any): boolean {
  if (!rule.pattern) return false;
  
  const pattern = rule.pattern;
  
  switch (rule.ruleType) {
    case 'exact':
      return url === pattern;
    
    case 'domain':
      return url.includes(pattern);
    
    case 'suffix':
      return url.endsWith(pattern);
    
    case 'wildcard':
      const wildcardPattern = pattern.replace(/\*/g, '.*');
      const regex = new RegExp(`^${wildcardPattern}$`);
      return regex.test(url);
    
    case 'regex':
      try {
        const regex = new RegExp(pattern);
        return regex.test(url);
      } catch {
        return false;
      }
    
    default:
      return false;
  }
}

function generateTestConfig(policy: any): any {
  const generator = new ProxyConfigGenerator();
  return generator.generateTestConfig(policy, 'test-url');
}
