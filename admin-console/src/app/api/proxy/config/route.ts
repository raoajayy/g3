import { NextRequest, NextResponse } from 'next/server';
import { ProxyConfigGenerator } from '@/lib/proxy-config-generator';

// Mock proxy configuration storage
let currentConfig: any = null;
let configVersion = 1;
let lastUpdate = new Date();

export async function GET() {
  try {
    return NextResponse.json({
      config: currentConfig,
      version: configVersion,
      lastUpdate: lastUpdate.toISOString(),
      status: 'loaded'
    });
  } catch (error) {
    return NextResponse.json(
      { error: 'Failed to get proxy configuration' },
      { status: 500 }
    );
  }
}

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const { config, version, timestamp } = body;

    // Validate configuration
    const generator = new ProxyConfigGenerator();
    const validation = generator.validateConfig(config);
    
    if (!validation.isValid) {
      return NextResponse.json(
        { 
          error: 'Configuration validation failed',
          details: validation.errors
        },
        { status: 400 }
      );
    }

    // Update configuration
    currentConfig = config;
    configVersion = version || configVersion + 1;
    lastUpdate = new Date(timestamp || new Date().toISOString());

    // Simulate proxy reload delay
    await new Promise(resolve => setTimeout(resolve, 1000));

    return NextResponse.json({
      success: true,
      version: configVersion,
      lastUpdate: lastUpdate.toISOString(),
      message: 'Configuration applied successfully'
    });

  } catch (error) {
    console.error('Failed to update proxy configuration:', error);
    return NextResponse.json(
      { error: 'Failed to update proxy configuration' },
      { status: 500 }
    );
  }
}

export async function PUT(request: NextRequest) {
  try {
    const body = await request.json();
    const { policies } = body;

    // Generate new configuration from policies
    const generator = new ProxyConfigGenerator();
    const config = generator.generateConfig(policies);

    // Validate configuration
    const validation = generator.validateConfig(config);
    
    if (!validation.isValid) {
      return NextResponse.json(
        { 
          error: 'Configuration validation failed',
          details: validation.errors
        },
        { status: 400 }
      );
    }

    // Update configuration
    currentConfig = config;
    configVersion++;
    lastUpdate = new Date();

    // Simulate proxy reload delay
    await new Promise(resolve => setTimeout(resolve, 1500));

    return NextResponse.json({
      success: true,
      config,
      version: configVersion,
      lastUpdate: lastUpdate.toISOString(),
      message: 'Configuration generated and applied successfully'
    });

  } catch (error) {
    console.error('Failed to generate proxy configuration:', error);
    return NextResponse.json(
      { error: 'Failed to generate proxy configuration' },
      { status: 500 }
    );
  }
}
