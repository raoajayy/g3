import { NextRequest, NextResponse } from 'next/server';

const API_BASE_URL = process.env.API_BASE_URL || 'http://localhost:3003';

export async function GET(
  request: NextRequest,
  { params }: { params: Promise<{ id: string }> }
) {
  try {
    const resolvedParams = await params;
    const { id } = resolvedParams;
    
    // Try to fetch from backend API first
    try {
      const response = await fetch(`${API_BASE_URL}/policies/${id}`);
      if (response.ok) {
        const data = await response.json();
        return NextResponse.json(data, { status: response.status });
      }
    } catch (backendError) {
      console.log('Backend API not available, using mock response');
    }
    
    // Mock response - return a sample policy
    const mockPolicy = {
      id: id,
      name: 'Sample Policy',
      description: 'This is a sample policy for demonstration',
      status: 'active',
      priority: 'medium',
      lastModified: new Date().toISOString(),
      createdBy: 'admin@company.com',
      type: 'url-filtering',
      enabled: true
    };
    
    return NextResponse.json(mockPolicy, { status: 200 });
  } catch (error) {
    console.error('Failed to fetch policy:', error);
    return NextResponse.json(
      { error: 'Failed to fetch policy' },
      { status: 500 }
    );
  }
}

export async function PUT(
  request: NextRequest,
  { params }: { params: Promise<{ id: string }> }
) {
  try {
    const resolvedParams = await params;
    const { id } = resolvedParams;
    const body = await request.json();
    
    // Try to update via backend API first
    try {
      const response = await fetch(`${API_BASE_URL}/policies/${id}`, {
        method: 'PUT',
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
    
    // Mock response for policy update
    return NextResponse.json({
      id: id,
      status: 'updated',
      policy: { ...body, id: id, lastModified: new Date().toISOString() }
    }, { status: 200 });
  } catch (error) {
    console.error('Failed to update policy:', error);
    return NextResponse.json(
      { error: 'Failed to update policy' },
      { status: 500 }
    );
  }
}

export async function DELETE(
  request: NextRequest,
  { params }: { params: Promise<{ id: string }> }
) {
  try {
    const resolvedParams = await params;
    const { id } = resolvedParams;
    
    // Try to delete via backend API first
    try {
      const response = await fetch(`${API_BASE_URL}/policies/${id}`, {
        method: 'DELETE',
      });
      if (response.ok) {
        const data = await response.json();
        return NextResponse.json(data, { status: response.status });
      }
    } catch (backendError) {
      console.log('Backend API not available, using mock response');
    }
    
    // Mock response for policy deletion
    return NextResponse.json({
      id: id,
      status: 'deleted'
    }, { status: 200 });
  } catch (error) {
    console.error('Failed to delete policy:', error);
    return NextResponse.json(
      { error: 'Failed to delete policy' },
      { status: 500 }
    );
  }
}
