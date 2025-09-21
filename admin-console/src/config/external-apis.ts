// External API Configuration
// Add your API keys to environment variables or update this file directly

export const EXTERNAL_API_CONFIG = {
  // VirusTotal API (Free tier: 4 requests/minute)
  // Get your key at: https://www.virustotal.com/gui/my-apikey
  VIRUSTOTAL_API_KEY: process.env.VIRUSTOTAL_API_KEY || '',

  // URLVoid API (Free tier: 10 requests/minute)
  // Get your key at: https://www.urlvoid.com/api/
  URLVOID_API_KEY: process.env.URLVOID_API_KEY || '',

  // AbuseIPDB API (Free tier: 10 requests/minute)
  // Get your key at: https://www.abuseipdb.com/account/api
  ABUSEIPDB_API_KEY: process.env.ABUSEIPDB_API_KEY || '',

  // WebsiteCategorizationAPI (Free tier: 100 requests/minute)
  // Get your key at: https://www.websitecategorizationapi.com/
  WEBSITE_CATEGORIZATION_API_KEY: process.env.WEBSITE_CATEGORIZATION_API_KEY || '',

  // ContentModerationAPI (Free tier: 50 requests/minute)
  // Get your key at: https://www.contentmoderationapi.net/
  CONTENT_MODERATION_API_KEY: process.env.CONTENT_MODERATION_API_KEY || '',

  // Cache settings
  CACHE_TTL: parseInt(process.env.CATEGORY_CACHE_TTL || '86400000'), // 24 hours
  CACHE_MAX_SIZE: parseInt(process.env.CATEGORY_CACHE_MAX_SIZE || '10000'),

  // Rate limiting
  DEFAULT_RATE_LIMIT: parseInt(process.env.DEFAULT_RATE_LIMIT || '10'),
  MAX_CONCURRENT_REQUESTS: parseInt(process.env.MAX_CONCURRENT_REQUESTS || '5'),
};

export const API_ENDPOINTS = {
  VIRUSTOTAL: 'https://www.virustotal.com/api/v3',
  URLVOID: 'https://api.urlvoid.com/v1',
  ABUSEIPDB: 'https://api.abuseipdb.com/api/v2',
  WEBSITE_CATEGORIZATION: 'https://api.websitecategorizationapi.com/v1',
  CONTENT_MODERATION: 'https://api.contentmoderationapi.net/v1',
};

export const RATE_LIMITS = {
  VIRUSTOTAL: 4, // requests per minute
  URLVOID: 10,
  ABUSEIPDB: 10,
  WEBSITE_CATEGORIZATION: 100,
  CONTENT_MODERATION: 50,
};

export const CATEGORY_MAPPINGS = {
  // Map external categories to our internal categories
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

export const RISK_LEVEL_MAPPINGS = {
  'critical': 1.0,
  'high': 0.8,
  'medium': 0.5,
  'low': 0.2,
  'unknown': 0.5
};
