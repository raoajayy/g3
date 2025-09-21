import { NextResponse } from 'next/server';

// Mock proxy status
let proxyStatus = {
  isRunning: true,
  port: 3128,
  pid: 12345,
  lastReload: new Date(),
  configVersion: 1,
  health: 'healthy' as 'healthy' | 'degraded' | 'unhealthy',
  uptime: 0,
  requestsProcessed: 0,
  blockedRequests: 0,
  allowedRequests: 0,
  errorRate: 0
};

// Initialize uptime
const startTime = Date.now();

export async function GET() {
  try {
    // Update uptime
    proxyStatus.uptime = Math.floor((Date.now() - startTime) / 1000);
    
    // Simulate some random activity
    proxyStatus.requestsProcessed += Math.floor(Math.random() * 10);
    proxyStatus.blockedRequests += Math.floor(Math.random() * 2);
    proxyStatus.allowedRequests = proxyStatus.requestsProcessed - proxyStatus.blockedRequests;
    
    // Calculate error rate (simulate some errors)
    proxyStatus.errorRate = Math.random() * 0.05; // 0-5% error rate

    // Determine health based on error rate and uptime
    if (proxyStatus.errorRate > 0.1) {
      proxyStatus.health = 'unhealthy';
    } else if (proxyStatus.errorRate > 0.05 || proxyStatus.uptime < 60) {
      proxyStatus.health = 'degraded';
    } else {
      proxyStatus.health = 'healthy';
    }

    return NextResponse.json({
      ...proxyStatus,
      lastReload: proxyStatus.lastReload.toISOString(),
      status: 'running'
    });

  } catch (error) {
    console.error('Failed to get proxy status:', error);
    return NextResponse.json(
      { 
        error: 'Failed to get proxy status',
        isRunning: false,
        health: 'unhealthy'
      },
      { status: 500 }
    );
  }
}

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const { action } = body;

    switch (action) {
      case 'start':
        proxyStatus.isRunning = true;
        proxyStatus.health = 'healthy';
        proxyStatus.lastReload = new Date();
        break;
      
      case 'stop':
        proxyStatus.isRunning = false;
        proxyStatus.health = 'unhealthy';
        break;
      
      case 'restart':
        proxyStatus.isRunning = true;
        proxyStatus.health = 'healthy';
        proxyStatus.lastReload = new Date();
        proxyStatus.configVersion++;
        break;
      
      case 'reload':
        proxyStatus.lastReload = new Date();
        proxyStatus.configVersion++;
        break;
      
      default:
        return NextResponse.json(
          { error: 'Invalid action' },
          { status: 400 }
        );
    }

    return NextResponse.json({
      success: true,
      status: proxyStatus,
      message: `Proxy ${action} completed successfully`
    });

  } catch (error) {
    console.error('Failed to execute proxy action:', error);
    return NextResponse.json(
      { error: 'Failed to execute proxy action' },
      { status: 500 }
    );
  }
}
