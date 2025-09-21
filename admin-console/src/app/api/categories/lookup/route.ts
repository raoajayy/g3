import { NextRequest, NextResponse } from 'next/server';
import { URLCategoryDatabase } from '@/lib/url-category-database';

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const { url, policyType } = body;

    if (!url) {
      return NextResponse.json(
        { success: false, error: 'URL is required' },
        { status: 400 }
      );
    }

    // Get category lookup results
    const results = URLCategoryDatabase.lookupURL(url);
    const bestMatch = URLCategoryDatabase.getBestMatch(url);
    const recommendedAction = URLCategoryDatabase.getRecommendedAction(url, policyType || 'allow');

    return NextResponse.json({
      success: true,
      data: {
        url,
        results,
        bestMatch,
        recommendedAction,
        totalMatches: results.length
      }
    });
  } catch (error) {
    console.error('Error looking up URL categories:', error);
    return NextResponse.json(
      { success: false, error: 'Failed to lookup URL categories' },
      { status: 500 }
    );
  }
}
