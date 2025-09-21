# Arcus Admin Console

A comprehensive Next.js dashboard for managing the Arcus Secure Web Gateway, including real-time metrics monitoring, policy management, and user administration.

## Features

### 📊 **Real-time Metrics Dashboard**
- Live monitoring of G3StatsD metrics
- Interactive charts using Recharts
- Auto-refresh every 5 seconds
- System performance indicators

### 🛡️ **Security Policy Management**
- Create, edit, and manage security policies
- URL filtering with category-based rules
- Content security policies (malware scanning, DLP)
- Traffic control and bandwidth management
- HTTPS inspection configuration
- Policy validation and conflict detection

### 👥 **User Management**
- User and group administration
- Role-based access control
- Bandwidth and quota management
- User activity monitoring
- Integration with Active Directory

### 📈 **Analytics & Monitoring**
- Detailed analytics and reporting
- Performance monitoring
- Policy violation tracking
- Real-time alerts and notifications

## Architecture

The admin console consists of three main components:

1. **Next.js Frontend** - Modern React dashboard with real-time updates
2. **Rust Admin API** - HTTP API for policy and user management
3. **G3StatsD Integration** - Real-time metrics collection

## Quick Start

### Prerequisites

- Node.js 18+
- Rust (for the admin API)
- G3StatsD running with memory exporter

### 1. Install Dependencies

```bash
# Install Next.js dependencies
npm install

# Install Rust dependencies for admin API
cd metrics-api
cargo build --release
cd ..
```

### 2. Start the Admin API

```bash
# Start the Rust admin API server
cd metrics-api
cargo run --release
```

The API will be available at `http://localhost:3001`

### 3. Start the Admin Console

```bash
# Start the Next.js development server
npm run dev
```

The dashboard will be available at `http://localhost:3000`

## Configuration

### G3StatsD Configuration

The admin console works with G3StatsD configured to use a memory exporter:

```yaml
---
runtime:
  thread_number: 2

worker:
  thread_number: 2

importer:
  - name: statsd
    type: statsd
    collector: aggregate_1s
    listen: 127.0.0.1:8125

collector:
  - name: aggregate_1s
    type: aggregate
    emit_interval: 1s
    join_tags:
      - stat_id
    exporter: memory

exporter:
  - name: memory
    type: memory
    store_count: 1000
```

## API Endpoints

### Metrics
- `GET /health` - Health check
- `GET /metrics` - Get all metrics
- `GET /metrics/{name}` - Get specific metric by name

### Policies
- `GET /policies` - Get all policies
- `GET /policies/{id}` - Get specific policy
- `POST /policies` - Create new policy
- `PUT /policies/{id}` - Update policy
- `DELETE /policies/{id}` - Delete policy

### Users
- `GET /users` - Get all users
- `GET /users/{id}` - Get specific user
- `POST /users` - Create new user
- `PUT /users/{id}` - Update user
- `DELETE /users/{id}` - Delete user

## Dashboard Components

### Overview Cards
- **Total Requests** - Sum of all counter metrics
- **Active Connections** - Current connection count
- **Average Response Time** - Mean response time across all endpoints
- **Total Metrics** - Number of available metrics

### Policy Management
- **Policy List** - View all security policies with status and priority
- **Policy Editor** - Create and edit policies with YAML validation
- **Policy Validation** - Check for conflicts and syntax errors
- **Policy Deployment** - Deploy policies to G3proxy

### User Management
- **User List** - View all users with roles and groups
- **User Editor** - Create and edit user accounts
- **Group Management** - Manage user groups and permissions
- **Access Control** - Configure bandwidth limits and quotas

### Charts
- **Request Rate Over Time** - Line chart showing counter metrics
- **System Metrics** - Bar chart displaying gauge metrics
- **Policy Violations** - Track policy enforcement metrics
- **User Activity** - Monitor user access patterns

## Policy Framework

The admin console includes a comprehensive policy management system based on the Arcus Policy Creation Framework:

### Policy Types

1. **URL Filtering Policies**
   - Category-based filtering (malware, phishing, social media)
   - Custom pattern matching (wildcard, regex, exact)
   - Priority-based rule evaluation

2. **Content Security Policies**
   - Malware scanning integration
   - Data loss prevention (DLP)
   - File upload/download scanning

3. **Traffic Control Policies**
   - Bandwidth limits per user/group
   - Daily/monthly quotas
   - Time-based restrictions

4. **HTTPS Inspection Policies**
   - MITM certificate management
   - Bypass domain configuration
   - Selective inspection rules

### Policy Schema

Policies are defined using YAML with the following structure:

```yaml
apiVersion: arcus.v1
kind: SecurityPolicy
metadata:
  name: organization-web-policy
  version: "1.0"
  description: "Organization-wide web security policy"
  
spec:
  priority: 100
  enabled: true
  
  targets:
    userGroups:
      - "employees"
      - "contractors"
    users:
      - "admin@company.com"
    sourceNetworks:
      - "10.0.0.0/8"
  
  urlFiltering:
    categories:
      block:
        - "gambling"
        - "adult-content"
        - "malware"
      warn:
        - "social-media"
      allow:
        - "business-tools"
    
    customRules:
      - name: "block-cryptocurrency"
        action: "block"
        pattern: "*.crypto*"
        type: "wildcard"
        message: "Cryptocurrency sites are blocked by company policy"
```

## Development

### Project Structure

```
admin-console/
├── src/
│   ├── app/
│   │   └── page.tsx              # Main dashboard page
│   ├── components/
│   │   ├── ui/                   # Reusable UI components
│   │   ├── pages/                # Page components
│   │   │   ├── dashboard-page.tsx
│   │   │   ├── policies-page.tsx
│   │   │   ├── users-page.tsx
│   │   │   └── ...
│   │   ├── layout.tsx            # Main layout component
│   │   └── sidebar.tsx           # Navigation sidebar
│   └── lib/
│       └── utils.ts              # Utility functions
├── metrics-api/                  # Rust Admin API
│   ├── src/
│   │   └── main.rs              # API server implementation
│   └── Cargo.toml
├── g3statsd-config.yaml         # Sample G3StatsD configuration
└── README.md
```

### Adding New Features

1. **New Pages**: Add components to `src/components/pages/`
2. **API Endpoints**: Extend the Rust API in `metrics-api/src/main.rs`
3. **UI Components**: Create reusable components in `src/components/ui/`

### Customizing the UI

The dashboard uses Tailwind CSS for styling. You can customize:
- Colors in `tailwind.config.js`
- Components in `src/components/`
- Layout in `src/components/layout.tsx`

## Security Considerations

- All API endpoints require proper authentication
- Policy changes are validated before deployment
- User actions are logged for audit purposes
- Sensitive data is encrypted in transit and at rest

## Monitoring and Alerting

The admin console includes comprehensive monitoring capabilities:

- Real-time metrics collection
- Policy violation alerts
- System performance monitoring
- User activity tracking
- Automated reporting

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the Apache 2.0 License - see the LICENSE file for details.