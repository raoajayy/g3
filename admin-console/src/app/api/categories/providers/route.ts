import { NextRequest, NextResponse } from 'next/server';
import { ExternalCategoryAPIs } from '@/lib/external-category-apis';

export async function GET(request: NextRequest) {
  try {
    const providers = ExternalCategoryAPIs.getProviders();

    return NextResponse.json({
      success: true,
      data: providers
    });
  } catch (error) {
    console.error('Error fetching providers:', error);
    return NextResponse.json(
      { success: false, error: 'Failed to fetch providers' },
      { status: 500 }
    );
  }
}

export async function PUT(request: NextRequest) {
  try {
    const body = await request.json();
    const { name, updates } = body;

    if (!name || !updates) {
      return NextResponse.json(
        { success: false, error: 'Provider name and updates are required' },
        { status: 400 }
      );
    }

    const success = ExternalCategoryAPIs.updateProvider(name, updates);

    if (!success) {
      return NextResponse.json(
        { success: false, error: 'Provider not found' },
        { status: 404 }
      );
    }

    return NextResponse.json({
      success: true,
      message: 'Provider updated successfully'
    });
  } catch (error) {
    console.error('Error updating provider:', error);
    return NextResponse.json(
      { success: false, error: 'Failed to update provider' },
      { status: 500 }
    );
  }
}

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const { name, baseUrl, apiKey, rateLimit, freeTier, categories, enabled } = body;

    if (!name || !baseUrl) {
      return NextResponse.json(
        { success: false, error: 'Provider name and base URL are required' },
        { status: 400 }
      );
    }

    ExternalCategoryAPIs.addProvider({
      name,
      baseUrl,
      apiKey,
      rateLimit: rateLimit || 10,
      freeTier: freeTier || false,
      categories: categories || [],
      enabled: enabled !== false
    });

    return NextResponse.json({
      success: true,
      message: 'Provider added successfully'
    });
  } catch (error) {
    console.error('Error adding provider:', error);
    return NextResponse.json(
      { success: false, error: 'Failed to add provider' },
      { status: 500 }
    );
  }
}

export async function DELETE(request: NextRequest) {
  try {
    const { searchParams } = new URL(request.url);
    const name = searchParams.get('name');

    if (!name) {
      return NextResponse.json(
        { success: false, error: 'Provider name is required' },
        { status: 400 }
      );
    }

    const success = ExternalCategoryAPIs.removeProvider(name);

    if (!success) {
      return NextResponse.json(
        { success: false, error: 'Provider not found' },
        { status: 404 }
      );
    }

    return NextResponse.json({
      success: true,
      message: 'Provider removed successfully'
    });
  } catch (error) {
    console.error('Error removing provider:', error);
    return NextResponse.json(
      { success: false, error: 'Failed to remove provider' },
      { status: 500 }
    );
  }
}
