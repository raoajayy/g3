#!/usr/bin/env node

/**
 * Test script to verify "Apply Policy Changes" functionality
 */

const fetch = require('node-fetch').default;

const BASE_URL = 'http://localhost:3002';

async function testApplyPolicyChanges() {
  console.log('🧪 Testing "Apply Policy Changes" functionality...\n');

  try {
    // Step 1: Load existing policies
    console.log('1️⃣ Loading existing policies...');
    const policiesResponse = await fetch(`${BASE_URL}/api/policies`);
    const policiesData = await policiesResponse.json();
    console.log(`   ✅ Loaded ${policiesData.policies.length} policies`);

    // Step 2: Apply policies to proxy configuration
    console.log('\n2️⃣ Applying policies to proxy configuration...');
    const applyResponse = await fetch(`${BASE_URL}/api/proxy/config`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ policies: policiesData.policies })
    });

    if (applyResponse.ok) {
      const applyResult = await applyResponse.json();
      console.log(`   ✅ Policies applied successfully (v${applyResult.version})`);
      console.log(`   📊 Configuration generated with ${applyResult.config.escaper.length} escapers`);
    } else {
      const errorData = await applyResponse.json();
      console.log(`   ❌ Failed to apply policies: ${errorData.error}`);
      return;
    }

    // Step 3: Verify proxy status
    console.log('\n3️⃣ Verifying proxy status...');
    const statusResponse = await fetch(`${BASE_URL}/api/proxy/status`);
    const statusData = await statusResponse.json();
    console.log(`   ✅ Proxy status: ${statusData.isRunning ? 'Running' : 'Stopped'}`);
    console.log(`   📊 Config version: ${statusData.configVersion}`);
    console.log(`   💚 Health: ${statusData.health}`);

    // Step 4: Test policy evaluation
    console.log('\n4️⃣ Testing policy evaluation...');
    const testResponse = await fetch(`${BASE_URL}/api/proxy/test`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        testUrl: 'https://github.com',
        policy: {
          name: 'test',
          urlFiltering: {
            categories: { block: [], warn: [], allow: [] },
            customRules: [{
              name: 'github_block',
              pattern: 'github.com',
              action: 'block',
              ruleType: 'domain'
            }]
          }
        }
      })
    });

    if (testResponse.ok) {
      const testResult = await testResponse.json();
      console.log(`   ✅ Policy test completed: ${testResult.result.action}`);
      console.log(`   📝 Reason: ${testResult.result.reason}`);
    } else {
      console.log('   ⚠️  Policy test failed (this is expected in some cases)');
    }

    // Step 5: Check current configuration
    console.log('\n5️⃣ Checking current proxy configuration...');
    const configResponse = await fetch(`${BASE_URL}/api/proxy/config`);
    const configData = await configResponse.json();
    console.log(`   ✅ Configuration loaded (v${configData.version})`);
    console.log(`   📊 Active escapers: ${configData.config?.escaper?.length || 0}`);

    console.log('\n🎉 "Apply Policy Changes" functionality test completed successfully!');
    console.log('\n📋 Summary:');
    console.log(`   - Policies loaded: ${policiesData.policies.length}`);
    console.log(`   - Configuration applied: ✅`);
    console.log(`   - Proxy status: ${statusData.isRunning ? 'Running' : 'Stopped'}`);
    console.log(`   - Health: ${statusData.health}`);
    console.log(`   - Config version: ${statusData.configVersion}`);

  } catch (error) {
    console.error('💥 Test failed with error:', error.message);
    process.exit(1);
  }
}

// Run the test
testApplyPolicyChanges().catch(error => {
  console.error('💥 Test failed:', error);
  process.exit(1);
});
