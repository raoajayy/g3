import { NextRequest, NextResponse } from 'next/server';
import { EnhancedCategoryDatabase } from '@/lib/enhanced-category-database';

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const { providerName, testUrl } = body;

    if (!providerName || !testUrl) {
      return NextResponse.json(
        { success: false, error: 'Provider name and test URL are required' },
        { status: 400 }
      );
    }

    const result = await EnhancedCategoryDatabase.testExternalProvider(providerName, testUrl);

    if (!result) {
      return NextResponse.json(
        { success: false, error: 'Provider test failed or no result returned' },
        { status: 404 }
      );
    }

    return NextResponse.json({
      success: true,
      data: result
    });
  } catch (error) {
    console.error('Error testing provider:', error);
    return NextResponse.json(
      { success: false, error: 'Failed to test provider' },
      { status: 500 }
    );
  }
}
