import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { PolicyEditor } from '@/components/policy-editor';

const mockPolicy = {
  id: '1',
  name: 'Test Policy',
  description: 'Test Description',
  priority: 'medium',
  enabled: true,
  targets: {
    userGroups: ['employees'],
    users: [],
    sourceNetworks: ['10.0.0.0/8'],
  },
  urlFiltering: {
    categories: {
      block: ['malware'],
      warn: ['social-media'],
      allow: ['business-tools'],
    },
    customRules: [],
  },
  contentSecurity: {
    malwareScanning: {
      enabled: false,
      icapServer: '',
      action: 'block',
      timeout: '30s',
    },
    dataLossPrevention: {
      enabled: false,
      scanUploads: false,
      scanDownloads: false,
      sensitiveDataPatterns: [],
    },
  },
  trafficControl: {
    bandwidthLimits: {
      perUser: '',
      total: '',
    },
    quotas: {
      dailyDataPerUser: '',
      monthlyDataPerUser: '',
    },
  },
  httpsInspection: {
    enabled: false,
    mode: 'selective',
    certificateGeneration: 'automatic',
    bypassDomains: [],
    inspectDomains: [],
  },
};

describe('PolicyEditor', () => {
  const mockOnSave = jest.fn();
  const mockOnCancel = jest.fn();

  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('renders create mode correctly', () => {
    render(
      <PolicyEditor
        onSave={mockOnSave}
        onCancel={mockOnCancel}
        mode="create"
      />
    );
    
    expect(screen.getByText('Create Policy')).toBeInTheDocument();
    expect(screen.getByText('Create a new security policy')).toBeInTheDocument();
  });

  it('renders edit mode correctly', () => {
    render(
      <PolicyEditor
        policy={mockPolicy}
        onSave={mockOnSave}
        onCancel={mockOnCancel}
        mode="edit"
      />
    );
    
    expect(screen.getByText('Edit Policy')).toBeInTheDocument();
    expect(screen.getByDisplayValue('Test Policy')).toBeInTheDocument();
  });

  it('renders view mode correctly', () => {
    render(
      <PolicyEditor
        policy={mockPolicy}
        onSave={mockOnSave}
        onCancel={mockOnCancel}
        mode="view"
      />
    );
    
    expect(screen.getByText('View Policy')).toBeInTheDocument();
    expect(screen.getByDisplayValue('Test Policy')).toBeInTheDocument();
  });

  it('shows all tabs', () => {
    render(
      <PolicyEditor
        onSave={mockOnSave}
        onCancel={mockOnCancel}
        mode="create"
      />
    );
    
    expect(screen.getByText('Basic Info')).toBeInTheDocument();
    expect(screen.getByText('Targets')).toBeInTheDocument();
    expect(screen.getByText('URL Filtering')).toBeInTheDocument();
    expect(screen.getByText('Content Security')).toBeInTheDocument();
    expect(screen.getByText('Traffic Control')).toBeInTheDocument();
    expect(screen.getByText('HTTPS Inspection')).toBeInTheDocument();
  });

  it('switches between tabs', () => {
    render(
      <PolicyEditor
        onSave={mockOnSave}
        onCancel={mockOnCancel}
        mode="create"
      />
    );
    
    fireEvent.click(screen.getByText('Targets'));
    expect(screen.getByText('Policy Targets')).toBeInTheDocument();
    
    fireEvent.click(screen.getByText('URL Filtering'));
    expect(screen.getByText('URL Filtering')).toBeInTheDocument();
  });

  it('validates required fields', async () => {
    render(
      <PolicyEditor
        onSave={mockOnSave}
        onCancel={mockOnCancel}
        mode="create"
      />
    );
    
    fireEvent.click(screen.getByText('Save Policy'));
    
    await waitFor(() => {
      expect(screen.getByText('Policy name is required')).toBeInTheDocument();
    });
  });

  it('calls onSave when save is clicked with valid data', async () => {
    render(
      <PolicyEditor
        policy={mockPolicy}
        onSave={mockOnSave}
        onCancel={mockOnCancel}
        mode="edit"
      />
    );
    
    fireEvent.click(screen.getByText('Save Policy'));
    
    await waitFor(() => {
      expect(mockOnSave).toHaveBeenCalledWith(mockPolicy);
    });
  });

  it('calls onCancel when cancel is clicked', () => {
    render(
      <PolicyEditor
        onSave={mockOnSave}
        onCancel={mockOnCancel}
        mode="create"
      />
    );
    
    fireEvent.click(screen.getByText('Cancel'));
    expect(mockOnCancel).toHaveBeenCalled();
  });

  it('disables form fields in view mode', () => {
    render(
      <PolicyEditor
        policy={mockPolicy}
        onSave={mockOnSave}
        onCancel={mockOnCancel}
        mode="view"
      />
    );
    
    const nameInput = screen.getByDisplayValue('Test Policy');
    expect(nameInput).toBeDisabled();
  });
});
