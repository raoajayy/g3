import { NextRequest, NextResponse } from 'next/server';
import { URLCategoryDatabase } from '@/lib/url-category-database';

export async function GET(request: NextRequest) {
  try {
    const { searchParams } = new URL(request.url);
    const riskLevel = searchParams.get('riskLevel');
    const source = searchParams.get('source');
    const search = searchParams.get('search');

    let categories = URLCategoryDatabase.getAllCategories();

    // Filter by risk level
    if (riskLevel) {
      categories = categories.filter(cat => cat.riskLevel === riskLevel);
    }

    // Filter by source
    if (source) {
      categories = categories.filter(cat => cat.source === source);
    }

    // Search categories
    if (search) {
      categories = URLCategoryDatabase.searchCategories(search);
    }

    return NextResponse.json({
      success: true,
      data: categories,
      total: categories.length
    });
  } catch (error) {
    console.error('Error fetching categories:', error);
    return NextResponse.json(
      { success: false, error: 'Failed to fetch categories' },
      { status: 500 }
    );
  }
}

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    
    // Validate required fields
    if (!body.name || !body.description) {
      return NextResponse.json(
        { success: false, error: 'Name and description are required' },
        { status: 400 }
      );
    }

    const categoryId = URLCategoryDatabase.addCustomCategory({
      name: body.name,
      description: body.description,
      riskLevel: body.riskLevel || 'low',
      subcategories: body.subcategories || [],
      keywords: body.keywords || [],
      domains: body.domains || [],
      patterns: body.patterns || [],
      blockRecommended: body.blockRecommended || false,
      warnRecommended: body.warnRecommended || false,
      allowRecommended: body.allowRecommended || true,
      parentCategory: body.parentCategory,
      source: 'manual'
    });

    return NextResponse.json({
      success: true,
      data: { id: categoryId },
      message: 'Category created successfully'
    });
  } catch (error) {
    console.error('Error creating category:', error);
    return NextResponse.json(
      { success: false, error: 'Failed to create category' },
      { status: 500 }
    );
  }
}
