import { NextRequest, NextResponse } from 'next/server';
import { EnhancedCategoryDatabase } from '@/lib/enhanced-category-database';

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const { urls, useExternal = true } = body;

    if (!urls || !Array.isArray(urls)) {
      return NextResponse.json(
        { success: false, error: 'URLs array is required' },
        { status: 400 }
      );
    }

    if (urls.length > 100) {
      return NextResponse.json(
        { success: false, error: 'Maximum 100 URLs allowed per request' },
        { status: 400 }
      );
    }

    // Perform bulk lookup
    const results = await EnhancedCategoryDatabase.bulkLookupURLs(urls, useExternal);

    // Convert Map to Object for JSON response
    const resultsObject = Object.fromEntries(results);

    return NextResponse.json({
      success: true,
      data: resultsObject,
      total: results.size
    });
  } catch (error) {
    console.error('Error in bulk lookup:', error);
    return NextResponse.json(
      { success: false, error: 'Failed to perform bulk lookup' },
      { status: 500 }
    );
  }
}
