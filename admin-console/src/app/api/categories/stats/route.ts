import { NextRequest, NextResponse } from 'next/server';
import { URLCategoryDatabase } from '@/lib/url-category-database';

export async function GET(request: NextRequest) {
  try {
    const stats = URLCategoryDatabase.getCategoryStats();

    return NextResponse.json({
      success: true,
      data: stats
    });
  } catch (error) {
    console.error('Error fetching category stats:', error);
    return NextResponse.json(
      { success: false, error: 'Failed to fetch category statistics' },
      { status: 500 }
    );
  }
}
