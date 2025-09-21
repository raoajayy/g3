'use client';

import { Layout, useLayout } from '@/components/layout';
import { Dashboard } from '@/components/pages/dashboard';
import { AnalyticsPage } from '@/components/pages/analytics-page';
import { MetricsPage } from '@/components/pages/metrics-page';
import { PerformancePage } from '@/components/pages/performance-page';
import { PoliciesPage } from '@/components/pages/policies-page';
import { UsersPage } from '@/components/pages/users-page';
import { SettingsPage } from '@/components/pages/settings-page';
import { AlertsPage } from '@/components/pages/alerts-page';
import { FirewallPage } from '@/components/pages/firewall-page';
import { AccessControlPage } from '@/components/pages/access-control-page';
import { ThreatsPage } from '@/components/pages/threats-page';
import { CategoriesPage } from '@/components/pages/categories-page';
import { ProvidersPage } from '@/components/pages/providers-page';
import { ProxyIntegrationPage } from '@/components/pages/proxy-integration-page';

function PageContent() {
  const { currentPage } = useLayout();

  switch (currentPage) {
      case 'dashboard':
        return <Dashboard />;
    case 'analytics':
      return <AnalyticsPage />;
    case 'metrics':
      return <MetricsPage />;
    case 'performance':
      return <PerformancePage />;
    case 'alerts':
      return <AlertsPage />;
    case 'policies':
      return <PoliciesPage />;
    case 'proxy-integration':
      return <ProxyIntegrationPage />;
    case 'categories':
      return <CategoriesPage />;
    case 'providers':
      return <ProvidersPage />;
    case 'firewall':
      return <FirewallPage />;
    case 'access-control':
      return <AccessControlPage />;
    case 'threats':
      return <ThreatsPage />;
    case 'users':
      return <UsersPage />;
    case 'settings':
      return <SettingsPage />;
    case 'general':
      return <SettingsPage />;
    case 'notifications':
      return <SettingsPage />;
    case 'integrations':
      return <SettingsPage />;
    default:
      return <Dashboard />;
  }
}

export default function AdminConsole() {
  return (
    <Layout>
      <PageContent />
    </Layout>
  );
}