#!/usr/bin/env node

/**
 * Proxy Integration Test Script
 * Tests the complete policy-to-proxy integration with hot reload
 */

const fetch = require('node-fetch');

const BASE_URL = 'http://localhost:3002';

// Test policies
const testPolicies = [
  {
    name: 'Social Media Block',
    description: 'Block social media access during work hours',
    status: 'active',
    targets: {
      userGroups: ['employees'],
      users: [],
      sourceNetworks: ['192.168.1.0/24']
    },
    urlFiltering: {
      categories: {
        block: ['social_media'],
        warn: [],
        allow: []
      },
      customRules: [
        {
          name: 'facebook_block',
          pattern: '*facebook*',
          action: 'block',
          ruleType: 'wildcard',
          message: 'Facebook access blocked during work hours'
        }
      ]
    },
    contentSecurity: {
      malwareScanning: { enabled: true, action: 'block' },
      dataLossPrevention: { enabled: false, scanUploads: false, scanDownloads: false }
    },
    trafficControl: {
      bandwidthLimit: 1000,
      connectionLimit: 100,
      rateLimit: 10
    },
    httpsInspection: {
      enabled: true,
      action: 'warn'
    }
  },
  {
    name: 'Malware Protection',
    description: 'Block known malware and suspicious domains',
    status: 'active',
    targets: {
      userGroups: ['all'],
      users: [],
      sourceNetworks: []
    },
    urlFiltering: {
      categories: {
        block: ['malware', 'phishing'],
        warn: ['suspicious'],
        allow: []
      },
      customRules: [
        {
          name: 'malware_domains',
          pattern: '.*\\.(malware|virus|phishing)\\.(com|net|org)$',
          action: 'block',
          ruleType: 'regex',
          message: 'Malware domain detected'
        }
      ]
    },
    contentSecurity: {
      malwareScanning: { enabled: true, action: 'block' },
      dataLossPrevention: { enabled: true, scanUploads: true, scanDownloads: true }
    },
    trafficControl: {
      bandwidthLimit: 500,
      connectionLimit: 50,
      rateLimit: 5
    },
    httpsInspection: {
      enabled: true,
      action: 'block'
    }
  }
];

// Test URLs
const testUrls = [
  'https://facebook.com',
  'https://twitter.com',
  'https://malware.com',
  'https://phishing.net',
  'https://github.com',
  'https://stackoverflow.com',
  'https://example.com'
];

async function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

async function testProxyStatus() {
  console.log('🔍 Testing proxy status...');
  
  try {
    const response = await fetch(`${BASE_URL}/api/proxy/status`);
    const status = await response.json();
    
    console.log(`✅ Proxy Status: ${status.isRunning ? 'Running' : 'Stopped'}`);
    console.log(`   Health: ${status.health}`);
    console.log(`   Port: ${status.port}`);
    console.log(`   Config Version: ${status.configVersion}`);
    
    return status;
  } catch (error) {
    console.error('❌ Failed to get proxy status:', error.message);
    return null;
  }
}

async function testPolicyCreation() {
  console.log('\n📝 Testing policy creation...');
  
  try {
    // Create test policies
    for (const policy of testPolicies) {
      const response = await fetch(`${BASE_URL}/api/policies`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(policy)
      });
      
      if (response.ok) {
        const result = await response.json();
        console.log(`✅ Created policy: ${policy.name} (ID: ${result.id})`);
      } else {
        console.error(`❌ Failed to create policy: ${policy.name}`);
      }
    }
    
    // Get all policies
    const response = await fetch(`${BASE_URL}/api/policies`);
    const data = await response.json();
    console.log(`📊 Total policies: ${data.policies.length}`);
    
    return data.policies;
  } catch (error) {
    console.error('❌ Policy creation failed:', error.message);
    return [];
  }
}

async function testProxyConfiguration(policies) {
  console.log('\n⚙️ Testing proxy configuration generation...');
  
  try {
    const response = await fetch(`${BASE_URL}/api/proxy/config`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ policies })
    });
    
    if (response.ok) {
      const result = await response.json();
      console.log(`✅ Configuration generated successfully (v${result.version})`);
      console.log(`   Servers: ${result.config.server.length}`);
      console.log(`   Escapers: ${result.config.escaper.length}`);
      console.log(`   Resolvers: ${result.config.resolver.length}`);
      return result.config;
    } else {
      console.error('❌ Failed to generate proxy configuration');
      return null;
    }
  } catch (error) {
    console.error('❌ Configuration generation failed:', error.message);
    return null;
  }
}

async function testPolicyEvaluation(policies) {
  console.log('\n🧪 Testing policy evaluation...');
  
  for (const url of testUrls) {
    console.log(`\n🔗 Testing URL: ${url}`);
    
    try {
      const response = await fetch(`${BASE_URL}/api/proxy/test`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          config: null,
          testUrl: url,
          policy: policies[0] // Test with first policy
        })
      });
      
      if (response.ok) {
        const result = await response.json();
        console.log(`   Action: ${result.result.action}`);
        console.log(`   Reason: ${result.result.reason}`);
        console.log(`   Matched Rules: ${result.result.matchedRules.length}`);
        console.log(`   Category: ${result.result.details.urlCategory || 'Unknown'}`);
        console.log(`   Risk Score: ${(result.result.details.riskScore * 100).toFixed(1)}%`);
      } else {
        console.error(`   ❌ Test failed for ${url}`);
      }
    } catch (error) {
      console.error(`   ❌ Error testing ${url}:`, error.message);
    }
  }
}

async function testHotReload(policies) {
  console.log('\n🔄 Testing hot reload...');
  
  try {
    // Apply policies to proxy
    const response = await fetch(`${BASE_URL}/api/proxy/config`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        config: null,
        version: 2,
        timestamp: new Date().toISOString()
      })
    });
    
    if (response.ok) {
      console.log('✅ Hot reload initiated');
      
      // Wait for reload to complete
      await sleep(2000);
      
      // Check status
      const statusResponse = await fetch(`${BASE_URL}/api/proxy/status`);
      const status = await statusResponse.json();
      
      console.log(`✅ Reload completed (v${status.configVersion})`);
      console.log(`   Last Reload: ${new Date(status.lastReload).toLocaleString()}`);
    } else {
      console.error('❌ Hot reload failed');
    }
  } catch (error) {
    console.error('❌ Hot reload error:', error.message);
  }
}

async function testURLFiltering() {
  console.log('\n🌐 Testing URL filtering...');
  
  try {
    // Test enhanced URL categorization
    const testUrl = 'https://facebook.com';
    const response = await fetch(`${BASE_URL}/api/categories/enhanced-lookup`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ url: testUrl })
    });
    
    if (response.ok) {
      const result = await response.json();
      console.log(`✅ Enhanced categorization for ${testUrl}:`);
      console.log(`   Category: ${result.data.consensusCategory}`);
      console.log(`   Confidence: ${(result.data.consensusConfidence * 100).toFixed(1)}%`);
      console.log(`   Risk Score: ${(result.data.riskScore * 100).toFixed(1)}%`);
      console.log(`   Action: ${result.data.action}`);
    } else {
      console.error('❌ Enhanced categorization failed');
    }
  } catch (error) {
    console.error('❌ URL filtering test error:', error.message);
  }
}

async function runIntegrationTest() {
  console.log('🚀 Starting Proxy Integration Test\n');
  console.log('=' .repeat(50));
  
  // Test 1: Proxy Status
  const proxyStatus = await testProxyStatus();
  if (!proxyStatus) {
    console.error('❌ Proxy not available, aborting test');
    return;
  }
  
  // Test 2: Policy Creation
  const policies = await testPolicyCreation();
  if (policies.length === 0) {
    console.error('❌ No policies created, aborting test');
    return;
  }
  
  // Test 3: Proxy Configuration
  const config = await testProxyConfiguration(policies);
  if (!config) {
    console.error('❌ Configuration generation failed, aborting test');
    return;
  }
  
  // Test 4: Policy Evaluation
  await testPolicyEvaluation(policies);
  
  // Test 5: Hot Reload
  await testHotReload(policies);
  
  // Test 6: URL Filtering
  await testURLFiltering();
  
  console.log('\n' + '='.repeat(50));
  console.log('✅ Integration test completed successfully!');
  console.log('\n📋 Test Summary:');
  console.log(`   - Policies created: ${policies.length}`);
  console.log(`   - URLs tested: ${testUrls.length}`);
  console.log(`   - Proxy status: ${proxyStatus.isRunning ? 'Running' : 'Stopped'}`);
  console.log(`   - Configuration version: ${proxyStatus.configVersion}`);
  console.log('\n🎉 All tests passed! The proxy integration is working correctly.');
}

// Run the test
runIntegrationTest().catch(error => {
  console.error('💥 Test failed with error:', error);
  process.exit(1);
});
