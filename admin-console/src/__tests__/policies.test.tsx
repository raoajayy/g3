import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { PoliciesPage } from '@/components/pages/policies-page';

// Mock the API client
jest.mock('@/lib/api', () => ({
  apiClient: {
    getPolicies: jest.fn(),
    createPolicy: jest.fn(),
    updatePolicy: jest.fn(),
    deletePolicy: jest.fn(),
  },
}));

describe('PoliciesPage', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('renders policies page with correct title', () => {
    render(<PoliciesPage />);
    expect(screen.getByText('Security Policies')).toBeInTheDocument();
    expect(screen.getByText('Manage web security policies and rules')).toBeInTheDocument();
  });

  it('displays policy statistics cards', () => {
    render(<PoliciesPage />);
    expect(screen.getByText('Total Policies')).toBeInTheDocument();
    expect(screen.getByText('Active')).toBeInTheDocument();
    expect(screen.getByText('Draft')).toBeInTheDocument();
    expect(screen.getByText('Critical')).toBeInTheDocument();
  });

  it('shows create policy button', () => {
    render(<PoliciesPage />);
    expect(screen.getByText('Create Policy')).toBeInTheDocument();
  });

  it('filters policies by search term', async () => {
    render(<PoliciesPage />);
    const searchInput = screen.getByPlaceholderText('Search policies...');
    fireEvent.change(searchInput, { target: { value: 'malware' } });
    
    await waitFor(() => {
      expect(screen.getByText('Block Malware Sites')).toBeInTheDocument();
    });
  });

  it('filters policies by status', async () => {
    render(<PoliciesPage />);
    const statusFilter = screen.getByDisplayValue('All Status');
    fireEvent.change(statusFilter, { target: { value: 'active' } });
    
    await waitFor(() => {
      const activePolicies = screen.getAllByText('active');
      expect(activePolicies.length).toBeGreaterThan(0);
    });
  });

  it('filters policies by type', async () => {
    render(<PoliciesPage />);
    const typeFilter = screen.getByDisplayValue('All Types');
    fireEvent.change(typeFilter, { target: { value: 'url-filtering' } });
    
    await waitFor(() => {
      expect(screen.getByText('url filtering')).toBeInTheDocument();
    });
  });

  it('filters policies by priority', async () => {
    render(<PoliciesPage />);
    const priorityFilter = screen.getByDisplayValue('All Priorities');
    fireEvent.change(priorityFilter, { target: { value: 'critical' } });
    
    await waitFor(() => {
      expect(screen.getByText('critical')).toBeInTheDocument();
    });
  });
});
