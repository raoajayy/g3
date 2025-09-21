import { NextRequest, NextResponse } from 'next/server';
import { EnhancedCategoryDatabase } from '@/lib/enhanced-category-database';

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const { url, useExternal = true } = body;

    if (!url) {
      return NextResponse.json(
        { success: false, error: 'URL is required' },
        { status: 400 }
      );
    }

    // Get enhanced categorization with external data
    const result = await EnhancedCategoryDatabase.lookupURL(url, useExternal);

    return NextResponse.json({
      success: true,
      data: result
    });
  } catch (error) {
    console.error('Error in enhanced lookup:', error);
    return NextResponse.json(
      { success: false, error: 'Failed to perform enhanced lookup' },
      { status: 500 }
    );
  }
}
