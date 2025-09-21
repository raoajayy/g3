'use client';

import { Bell, Search, User, Settings, LogOut } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { useLayout } from './layout';

export function Header() {
  const { sidebarCollapsed } = useLayout();
  
  return (
    <header className={`fixed top-0 left-0 right-0 z-30 bg-white border-b border-gray-200 shadow-sm transition-all duration-300 ${
      sidebarCollapsed ? 'md:left-16' : 'md:left-64'
    }`}>
      <div className="px-6 py-4 h-16">
        <div className="flex items-center justify-between">
          {/* Left side - Logo and Search */}
          <div className="flex items-center space-x-6">
            <div className="flex items-center space-x-3">
            </div>
            
            {/* Search Bar */}
            <div className="hidden md:block">
              <div className="relative">
                <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 w-4 h-4" />
                <Input
                  type="text"
                  placeholder="Search metrics, policies, users..."
                  className="pl-10 w-80 bg-gray-50 border-gray-200 focus:bg-white focus:border-blue-500"
                />
              </div>
            </div>
          </div>

          {/* Right side - Actions and User Menu */}
          <div className="flex items-center space-x-4">
            {/* Notifications */}
            <Button
              variant="ghost"
              size="sm"
              className="relative p-2 text-gray-500 hover:text-gray-700 hover:bg-gray-100"
            >
              <Bell className="w-5 h-5" />
              <span className="absolute -top-1 -right-1 w-3 h-3 bg-red-500 rounded-full text-xs text-white flex items-center justify-center">
                3
              </span>
            </Button>

            {/* Settings */}
            <Button
              variant="ghost"
              size="sm"
              className="p-2 text-gray-500 hover:text-gray-700 hover:bg-gray-100"
            >
              <Settings className="w-5 h-5" />
            </Button>

            {/* User Menu */}
            <div className="flex items-center space-x-3 pl-4 border-l border-gray-200">
              <div className="text-right">
                <p className="text-sm font-medium text-gray-900">Admin User</p>
                <p className="text-xs text-gray-500">admin@arcus.com</p>
              </div>
              <Button
                variant="ghost"
                size="sm"
                className="p-2 text-gray-500 hover:text-gray-700 hover:bg-gray-100"
              >
                <User className="w-5 h-5" />
              </Button>
              <Button
                variant="ghost"
                size="sm"
                className="p-2 text-gray-500 hover:text-gray-700 hover:bg-gray-100"
              >
                <LogOut className="w-5 h-5" />
              </Button>
            </div>
          </div>
        </div>
      </div>
    </header>
  );
}
