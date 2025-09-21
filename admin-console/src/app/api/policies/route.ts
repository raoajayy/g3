import { NextRequest, NextResponse } from 'next/server';

const API_BASE_URL = process.env.API_BASE_URL || 'http://localhost:3003';

// Mock policies data since backend API is not available
const mockPolicies = [
  {
    id: '1',
    name: 'Block Malware Sites',
    description: 'Blocks access to known malware and phishing sites',
    status: 'active',
    priority: 'critical',
    lastModified: '2024-01-15T10:30:00Z',
    createdBy: 'admin@company.com',
    type: 'url-filtering',
    enabled: true
  },
  {
    id: '2',
    name: 'Social Media Warning',
    description: 'Shows warning page for social media sites during work hours',
    status: 'active',
    priority: 'medium',
    lastModified: '2024-01-14T14:20:00Z',
    createdBy: 'security@company.com',
    type: 'url-filtering',
    enabled: true
  },
  {
    id: '3',
    name: 'HTTPS Inspection',
    description: 'Inspects HTTPS traffic for security threats',
    status: 'active',
    priority: 'high',
    lastModified: '2024-01-13T09:15:00Z',
    createdBy: 'admin@company.com',
    type: 'https-inspection',
    enabled: true
  },
  {
    id: '4',
    name: 'Bandwidth Limits',
    description: 'Enforces bandwidth limits per user and group',
    status: 'draft',
    priority: 'low',
    lastModified: '2024-01-12T16:45:00Z',
    createdBy: 'admin@company.com',
    type: 'traffic-control',
    enabled: false
  },
  {
    id: '5',
    name: 'Data Loss Prevention',
    description: 'Scans content for sensitive data patterns',
    status: 'inactive',
    priority: 'high',
    lastModified: '2024-01-11T11:30:00Z',
    createdBy: 'security@company.com',
    type: 'content-security',
    enabled: false
  }
];

export async function GET() {
  try {
    // Try to fetch from backend API first
    try {
      const response = await fetch(`${API_BASE_URL}/policies`);
      if (response.ok) {
        const data = await response.json();
        return NextResponse.json(data);
      }
    } catch (backendError) {
      console.log('Backend API not available, using mock data');
    }
    
    // Fallback to mock data
    return NextResponse.json({
      policies: mockPolicies,
      total_count: mockPolicies.length
    });
  } catch (error) {
    console.error('Failed to fetch policies:', error);
    return NextResponse.json(
      { error: 'Failed to fetch policies' },
      { status: 500 }
    );
  }
}

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    
    // Try to create via backend API first
    try {
      const response = await fetch(`${API_BASE_URL}/policies`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(body),
      });
      if (response.ok) {
        const data = await response.json();
        return NextResponse.json(data, { status: response.status });
      }
    } catch (backendError) {
      console.log('Backend API not available, using mock response');
    }
    
    // Mock response for policy creation
    const newPolicy = {
      id: Date.now().toString(),
      ...body,
      lastModified: new Date().toISOString(),
      createdBy: 'admin@company.com'
    };
    
    return NextResponse.json({
      id: newPolicy.id,
      status: 'created',
      policy: newPolicy
    }, { status: 201 });
  } catch (error) {
    console.error('Failed to create policy:', error);
    return NextResponse.json(
      { error: 'Failed to create policy' },
      { status: 500 }
    );
  }
}
