'use client';

import { createContext, useContext, useState } from 'react';
import { Sidebar } from './sidebar';
import { Header } from './header';

interface LayoutContextType {
  currentPage: string;
  setCurrentPage: (page: string) => void;
  sidebarCollapsed: boolean;
  setSidebarCollapsed: (collapsed: boolean) => void;
}

const LayoutContext = createContext<LayoutContextType | undefined>(undefined);

export function useLayout() {
  const context = useContext(LayoutContext);
  if (!context) {
    throw new Error('useLayout must be used within a Layout');
  }
  return context;
}

interface LayoutProps {
  children: React.ReactNode;
}

export function Layout({ children }: LayoutProps) {
  const [currentPage, setCurrentPage] = useState('dashboard');
  const [sidebarCollapsed, setSidebarCollapsed] = useState(false);

  const handlePageChange = (page: string) => {
    setCurrentPage(page);
  };

  return (
    <LayoutContext.Provider value={{ 
      currentPage, 
      setCurrentPage, 
      sidebarCollapsed, 
      setSidebarCollapsed 
    }}>
      <div className="min-h-screen bg-gray-50">
        <Sidebar 
          currentPage={currentPage} 
          onPageChange={handlePageChange}
          onCollapseChange={setSidebarCollapsed}
        />
        <Header />
        <div className={`flex flex-col transition-all duration-300 ${
          sidebarCollapsed ? 'md:ml-16' : 'md:ml-64'
        }`}>
          <main className="flex-1 overflow-auto p-6 pt-20">
            <div className="max-w-7xl mx-auto">
              {children}
            </div>
          </main>
        </div>
      </div>
    </LayoutContext.Provider>
  );
}
