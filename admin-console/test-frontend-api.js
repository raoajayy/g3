#!/usr/bin/env node

const fetch = require('node-fetch').default;

async function testFrontendAPI() {
  console.log('🧪 Testing Frontend API Access...\n');

  try {
    // Test the metrics API
    console.log('1️⃣ Testing /api/metrics endpoint...');
    const response = await fetch('http://localhost:3002/api/metrics');
    const data = await response.json();
    
    console.log(`   ✅ Status: ${response.status}`);
    console.log(`   📊 Metrics count: ${data.total_count || 0}`);
    console.log(`   🔄 Source: ${data.source || 'unknown'}`);
    
    if (data.metrics && data.metrics.length > 0) {
      console.log('   📈 Sample metrics:');
      data.metrics.slice(0, 3).forEach(metric => {
        console.log(`      - ${metric.name}: ${metric.values?.[0]?.value || 'N/A'} (${metric.type})`);
      });
    }
    
    // Test the dashboard page
    console.log('\n2️⃣ Testing dashboard page...');
    const dashboardResponse = await fetch('http://localhost:3002/');
    const dashboardHtml = await dashboardResponse.text();
    
    console.log(`   ✅ Dashboard status: ${dashboardResponse.status}`);
    
    // Check for specific content
    const hasDebugSection = dashboardHtml.includes('Debug Information');
    const hasLoadingState = dashboardHtml.includes('Loading metrics...');
    const hasMetricsData = dashboardHtml.includes('requests.total');
    
    console.log(`   🔍 Debug section found: ${hasDebugSection ? '✅' : '❌'}`);
    console.log(`   ⏳ Loading state: ${hasLoadingState ? '⚠️  Still loading' : '✅ Not loading'}`);
    console.log(`   📊 Metrics data in HTML: ${hasMetricsData ? '✅' : '❌'}`);
    
    console.log('\n🎉 Frontend API test completed!');
    
  } catch (error) {
    console.error('❌ Test failed:', error.message);
  }
}

testFrontendAPI();
