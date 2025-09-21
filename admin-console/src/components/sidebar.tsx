'use client';

import { useState, useEffect } from 'react';
import { 
  LayoutDashboard, 
  BarChart3, 
  Settings, 
  Activity, 
  TrendingUp,
  Menu,
  X,
  Shield,
  Users,
  FileText,
  ChevronDown,
  ChevronRight,
  Server,
  Network,
  Database,
  Monitor,
  AlertTriangle,
  Key,
  Globe,
  Lock,
  Eye,
  Filter,
  Search,
  Download,
  Upload,
  RefreshCw,
  Clock,
  Zap
} from 'lucide-react';
import { cn } from '@/lib/utils';

interface SidebarProps {
  currentPage: string;
  onPageChange: (page: string) => void;
  onCollapseChange?: (collapsed: boolean) => void;
}

interface MenuItem {
  id: string;
  label: string;
  icon: any;
  description: string;
  submenu?: MenuItem[];
}

const menuItems: MenuItem[] = [
  {
    id: 'dashboard',
    label: 'Dashboard',
    icon: LayoutDashboard,
    description: 'Overview and metrics'
  },
  {
    id: 'monitoring',
    label: 'Monitoring',
    icon: Monitor,
    description: 'System monitoring',
    submenu: [
      {
        id: 'analytics',
        label: 'Analytics',
        icon: BarChart3,
        description: 'Detailed analytics'
      },
      {
        id: 'metrics',
        label: 'Metrics',
        icon: Activity,
        description: 'System metrics'
      },
      {
        id: 'performance',
        label: 'Performance',
        icon: TrendingUp,
        description: 'Performance monitoring'
      },
      {
        id: 'alerts',
        label: 'Alerts',
        icon: AlertTriangle,
        description: 'System alerts'
      }
    ]
  },
  {
    id: 'security',
    label: 'Security',
    icon: Shield,
    description: 'Security management',
    submenu: [
      {
        id: 'policies',
        label: 'Policies',
        icon: FileText,
        description: 'Security policies'
      },
      {
        id: 'proxy-integration',
        label: 'Proxy Integration',
        icon: Server,
        description: 'Policy-to-proxy integration'
      },
      {
        id: 'firewall',
        label: 'Firewall',
        icon: Lock,
        description: 'Firewall rules'
      },
      {
        id: 'access-control',
        label: 'Access Control',
        icon: Key,
        description: 'Access management'
      },
      {
        id: 'threats',
        label: 'Threat Detection',
        icon: Eye,
        description: 'Threat monitoring'
      }
    ]
  },
  {
    id: 'network',
    label: 'Network',
    icon: Network,
    description: 'Network management',
    submenu: [
      {
        id: 'proxies',
        label: 'Proxies',
        icon: Server,
        description: 'Proxy servers'
      },
      {
        id: 'routing',
        label: 'Routing',
        icon: Globe,
        description: 'Network routing'
      },
      {
        id: 'bandwidth',
        label: 'Bandwidth',
        icon: Zap,
        description: 'Bandwidth monitoring'
      }
    ]
  },
  {
    id: 'data',
    label: 'Data',
    icon: Database,
    description: 'Data management',
    submenu: [
      {
        id: 'logs',
        label: 'Logs',
        icon: FileText,
        description: 'System logs'
      },
      {
        id: 'backups',
        label: 'Backups',
        icon: Download,
        description: 'Data backups'
      },
      {
        id: 'exports',
        label: 'Exports',
        icon: Upload,
        description: 'Data exports'
      }
    ]
  },
  {
    id: 'users',
    label: 'Users',
    icon: Users,
    description: 'User management'
  },
  {
    id: 'settings',
    label: 'Settings',
    icon: Settings,
    description: 'Configuration',
    submenu: [
      {
        id: 'general',
        label: 'General',
        icon: Settings,
        description: 'General settings'
      },
      {
        id: 'notifications',
        label: 'Notifications',
        icon: AlertTriangle,
        description: 'Notification settings'
      },
      {
        id: 'integrations',
        label: 'Integrations',
        icon: Network,
        description: 'Third-party integrations'
      },
      {
        id: 'categories',
        label: 'URL Categories',
        icon: Filter,
        description: 'URL categorization database'
      },
      {
        id: 'providers',
        label: 'Data Providers',
        icon: Database,
        description: 'External API providers'
      }
    ]
  }
];

export function Sidebar({ currentPage, onPageChange, onCollapseChange }: SidebarProps) {
  const [isCollapsed, setIsCollapsed] = useState(false);
  const [isMobile, setIsMobile] = useState(false);
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);
  const [expandedMenus, setExpandedMenus] = useState<Set<string>>(new Set());

  useEffect(() => {
    const checkMobile = () => {
      setIsMobile(window.innerWidth < 768);
      if (window.innerWidth >= 768) {
        setIsMobileMenuOpen(false);
      }
    };

    checkMobile();
    window.addEventListener('resize', checkMobile);
    return () => window.removeEventListener('resize', checkMobile);
  }, []);

  const handlePageChange = (page: string) => {
    onPageChange(page);
    if (isMobile) {
      setIsMobileMenuOpen(false);
    }
  };

  const toggleSubmenu = (menuId: string) => {
    setExpandedMenus(prev => {
      const newSet = new Set(prev);
      if (newSet.has(menuId)) {
        newSet.delete(menuId);
      } else {
        newSet.add(menuId);
      }
      return newSet;
    });
  };

  const isSubmenuExpanded = (menuId: string) => {
    return expandedMenus.has(menuId);
  };

  const isPageActive = (pageId: string) => {
    return currentPage === pageId;
  };

  const isParentActive = (menuItem: MenuItem) => {
    if (!menuItem.submenu) return false;
    return menuItem.submenu.some(subItem => isPageActive(subItem.id));
  };

  const sidebarContent = (
    <div className={cn(
      "bg-white border-r border-gray-200 transition-all duration-300 flex flex-col h-screen fixed left-0 top-0 z-40",
      isCollapsed && !isMobile ? "w-16" : "w-64"
    )}>
      {/* Header */}
      <div className="px-6 py-4 h-16 border-b border-gray-200 flex items-center">
        <div className={cn(
          "flex items-center",
          isCollapsed && !isMobile ? "justify-center" : "justify-between"
        )}>
          {(!isCollapsed || isMobile) && (
            <div className="flex items-center space-x-3">
              <img 
                src="https://www.accops.com/_nuxt/img/Accops_Logo.653bdac.svg" 
                alt="Accops Logo" 
                style={{ height: '35px', width: '150px' }}
              />
            </div>
          )}
          <button
            onClick={() => {
              if (isMobile) {
                setIsMobileMenuOpen(!isMobileMenuOpen);
              } else {
                const newCollapsed = !isCollapsed;
                setIsCollapsed(newCollapsed);
                onCollapseChange?.(newCollapsed);
              }
            }}
            className={cn(
              "p-1.5 rounded-lg hover:bg-gray-100 transition-colors",
              isCollapsed && !isMobile ? "mx-auto" : ""
            )}
            aria-label={isMobile ? "Toggle mobile menu" : "Toggle sidebar"}
          >
            {isMobile ? (
              isMobileMenuOpen ? (
                <X className="w-4 h-4 text-gray-600" />
              ) : (
                <Menu className="w-4 h-4 text-gray-600" />
              )
            ) : (
              <Menu className="w-4 h-4 text-gray-600" />
            )}
          </button>
        </div>
      </div>

      {/* Navigation */}
      <nav className="flex-1 p-4 space-y-1 overflow-y-auto">
        {menuItems.map((item) => {
          const Icon = item.icon;
          const hasSubmenu = item.submenu && item.submenu.length > 0;
          const isExpanded = isSubmenuExpanded(item.id);
          const isActive = isPageActive(item.id);
          const isParentActiveItem = isParentActive(item);
          
          return (
            <div key={item.id} className="space-y-1">
              {/* Main menu item */}
              <button
                onClick={() => {
                  if (hasSubmenu) {
                    if (isCollapsed && !isMobile) {
                      // In collapsed state, clicking a parent item should navigate to first submenu item
                      if (item.submenu && item.submenu.length > 0) {
                        handlePageChange(item.submenu[0].id);
                      }
                    } else {
                      // In expanded state, toggle submenu
                      toggleSubmenu(item.id);
                    }
                  } else {
                    // For items without submenus, always navigate
                    handlePageChange(item.id);
                  }
                }}
                className={cn(
                  "w-full flex items-center rounded-lg text-left transition-colors group",
                  isCollapsed && !isMobile 
                    ? "px-3 py-3 justify-center" 
                    : "px-3 py-2.5",
                  (isActive || isParentActiveItem)
                    ? "bg-blue-50 text-blue-900 border border-blue-200"
                    : "text-gray-700 hover:bg-gray-50 hover:text-gray-900"
                )}
                aria-label={hasSubmenu && isCollapsed && !isMobile 
                  ? `Navigate to ${item.submenu?.[0]?.label || item.label}` 
                  : `Navigate to ${item.label}`}
                title={hasSubmenu && isCollapsed && !isMobile 
                  ? `Click to go to ${item.submenu?.[0]?.label || 'first option'}` 
                  : item.label}
              >
                <Icon className={cn(
                  "w-5 h-5 flex-shrink-0",
                  isCollapsed && !isMobile ? "mx-auto" : "",
                  (isActive || isParentActiveItem) ? "text-blue-600" : "text-gray-500 group-hover:text-gray-700"
                )} />
                {(!isCollapsed || isMobile) && (
                  <div className="ml-3 flex-1">
                    <div className={cn(
                      "font-medium",
                      (isActive || isParentActiveItem) ? "text-blue-900" : "text-gray-700"
                    )}>{item.label}</div>
                    <div className={cn(
                      "text-xs",
                      (isActive || isParentActiveItem) ? "text-blue-600" : "text-gray-500"
                    )}>{item.description}</div>
                  </div>
                )}
                {hasSubmenu && (!isCollapsed || isMobile) && (
                  <div className="ml-2">
                    {isExpanded ? (
                      <ChevronDown className="w-4 h-4 text-gray-500" />
                    ) : (
                      <ChevronRight className="w-4 h-4 text-gray-500" />
                    )}
                  </div>
                )}
              </button>

              {/* Submenu */}
              {hasSubmenu && isExpanded && (!isCollapsed || isMobile) && (
                <div className="ml-4 space-y-1 border-l border-gray-200 pl-4">
                  {item.submenu!.map((subItem) => {
                    const SubIcon = subItem.icon;
                    const isSubActive = isPageActive(subItem.id);
                    
                    return (
                      <button
                        key={subItem.id}
                        onClick={() => handlePageChange(subItem.id)}
                        className={cn(
                          "w-full flex items-center rounded-lg text-left transition-colors group text-sm",
                          isCollapsed && !isMobile 
                            ? "px-3 py-3 justify-center" 
                            : "px-3 py-2",
                          isSubActive
                            ? "bg-blue-50 text-blue-900 border border-blue-200"
                            : "text-gray-600 hover:bg-gray-50 hover:text-gray-900"
                        )}
                        aria-label={`Navigate to ${subItem.label}`}
                      >
                        <SubIcon className={cn(
                          "w-4 h-4 flex-shrink-0",
                          isCollapsed && !isMobile ? "mx-auto" : "",
                          isSubActive ? "text-blue-600" : "text-gray-400 group-hover:text-gray-600"
                        )} />
                        {(!isCollapsed || isMobile) && (
                          <div className="ml-3 flex-1">
                            <div className={cn(
                              "font-medium",
                              isSubActive ? "text-blue-900" : "text-gray-600"
                            )}>{subItem.label}</div>
                            <div className={cn(
                              "text-xs",
                              isSubActive ? "text-blue-600" : "text-gray-500"
                            )}>{subItem.description}</div>
                          </div>
                        )}
                      </button>
                    );
                  })}
                </div>
              )}
            </div>
          );
        })}
      </nav>

      {/* Footer */}
      <div className="p-4 border-t border-gray-200">
        {(!isCollapsed || isMobile) && (
          <div className="text-xs text-gray-500 text-center">
          </div>
        )}
        {isCollapsed && !isMobile && (
          <div className="text-xs text-gray-500 text-center">
            v1.0
          </div>
        )}
      </div>
    </div>
  );

  if (isMobile) {
    return (
      <>
        {/* Mobile overlay */}
        {isMobileMenuOpen && (
          <div 
            className="fixed inset-0 bg-black bg-opacity-50 z-40 md:hidden"
            onClick={() => setIsMobileMenuOpen(false)}
          />
        )}
        
        {/* Mobile sidebar */}
        <div className={cn(
          "fixed left-0 top-16 h-[calc(100vh-4rem)] z-50 transform transition-transform duration-300 md:hidden",
          isMobileMenuOpen ? "translate-x-0" : "-translate-x-full"
        )}>
          {sidebarContent}
        </div>
      </>
    );
  }

  return sidebarContent;
}