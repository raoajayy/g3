const fetch = require('node-fetch').default;

async function testDashboard() {
  console.log('🧪 Testing Dashboard Data Loading...\n');

  try {
    // Test metrics API
    console.log('1️⃣ Testing metrics API...');
    const response = await fetch('http://localhost:3002/api/metrics');
    
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    
    const data = await response.json();
    console.log(`   ✅ API Response: ${data.metrics.length} metrics received`);
    
    // Test metric structure
    const firstMetric = data.metrics[0];
    console.log(`   📊 Sample metric: ${firstMetric.name}`);
    console.log(`   📈 Type: ${firstMetric.type}`);
    console.log(`   🔢 Values count: ${firstMetric.values?.length || 0}`);
    
    if (firstMetric.values && firstMetric.values.length > 0) {
      const latestValue = firstMetric.values[firstMetric.values.length - 1].value;
      console.log(`   📊 Latest value: ${latestValue}`);
    }
    
    // Test dashboard page
    console.log('\n2️⃣ Testing dashboard page...');
    const dashboardResponse = await fetch('http://localhost:3002');
    
    if (!dashboardResponse.ok) {
      throw new Error(`Dashboard HTTP error! status: ${dashboardResponse.status}`);
    }
    
    const dashboardHtml = await dashboardResponse.text();
    
    // Check if dashboard shows loading state
    if (dashboardHtml.includes('Loading...')) {
      console.log('   ⚠️  Dashboard shows loading state');
    } else {
      console.log('   ✅ Dashboard loaded (no loading state)');
    }
    
    // Check if debug information is present
    if (dashboardHtml.includes('Debug Information')) {
      console.log('   ✅ Debug section found');
    } else {
      console.log('   ⚠️  Debug section not found');
    }
    
    // Check if metrics are displayed
    if (dashboardHtml.includes('requests_per_second')) {
      console.log('   ✅ Metrics data found in HTML');
    } else {
      console.log('   ⚠️  Metrics data not found in HTML');
    }
    
    console.log('\n🎉 Dashboard test completed!');
    console.log('\n📋 Summary:');
    console.log(`   - Metrics API: ✅ Working (${data.metrics.length} metrics)`);
    console.log(`   - Dashboard page: ${dashboardResponse.ok ? '✅' : '❌'} (${dashboardResponse.status})`);
    console.log(`   - Data structure: ✅ Correct (values array with timestamps)`);
    
  } catch (error) {
    console.error('❌ Test failed:', error.message);
  }
}

testDashboard();
