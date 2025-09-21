# Policy CRUD Operations Test Plan

## Test Environment Setup
- Frontend: Next.js running on http://localhost:3002
- Backend: Mock API responses (since Rust backend unavailable)

## 1. CREATE Operations

### Test Case 1.1: Create New Policy
```bash
# Test creating a URL filtering policy
curl -X POST http://localhost:3002/api/policies \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Block Social Media",
    "description": "Blocks access to social media sites during work hours",
    "priority": "medium",
    "enabled": true,
    "type": "url-filtering",
    "targets": {
      "userGroups": ["employees"],
      "users": [],
      "sourceNetworks": ["10.0.0.0/8"]
    },
    "urlFiltering": {
      "categories": {
        "block": ["social-media"],
        "warn": [],
        "allow": []
      },
      "customRules": []
    }
  }'
```

**Expected Result:**
- Policy created successfully
- Returns 201 status with policy ID
- Policy appears in the policies list
- Statistics cards update accordingly

### Test Case 1.2: Create Policy with Validation Errors
```bash
# Test creating policy with missing required fields
curl -X POST http://localhost:3002/api/policies \
  -H "Content-Type: application/json" \
  -d '{
    "name": "",
    "description": "Test policy",
    "priority": "medium"
  }'
```

**Expected Result:**
- Returns 400 status with validation errors
- Error messages displayed in UI
- Form validation prevents submission

## 2. READ Operations

### Test Case 2.1: Get All Policies
```bash
curl -X GET http://localhost:3002/api/policies
```

**Expected Result:**
- Returns array of all policies
- Each policy includes complete metadata
- Policies displayed in table with proper formatting

### Test Case 2.2: Get Specific Policy
```bash
curl -X GET http://localhost:3002/api/policies/{policy_id}
```

**Expected Result:**
- Returns specific policy details
- Policy data populates edit form correctly
- All policy sections properly loaded

### Test Case 2.3: Search and Filter Policies
```bash
# Test search functionality
curl -X GET "http://localhost:3002/api/policies?search=malware"

# Test status filtering
curl -X GET "http://localhost:3002/api/policies?status=active"

# Test type filtering
curl -X GET "http://localhost:3002/api/policies?type=url-filtering"

# Test priority filtering
curl -X GET "http://localhost:3002/api/policies?priority=critical"
```

**Expected Result:**
- Search returns matching policies
- Filters work correctly
- UI updates to show filtered results
- Clear indication of active filters

## 3. UPDATE Operations

### Test Case 3.1: Update Policy Status
```bash
# Toggle policy status from active to inactive
curl -X PUT http://localhost:3002/api/policies/{policy_id} \
  -H "Content-Type: application/json" \
  -d '{
    "status": "inactive",
    "enabled": false
  }'
```

**Expected Result:**
- Policy status updated
- UI reflects new status immediately
- Statistics cards update
- Action button changes (Play/Pause)

### Test Case 3.2: Update Policy Configuration
```bash
# Update policy details
curl -X PUT http://localhost:3002/api/policies/{policy_id} \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Updated Policy Name",
    "description": "Updated description",
    "priority": "high",
    "urlFiltering": {
      "categories": {
        "block": ["malware", "phishing", "adult-content"],
        "warn": ["social-media"],
        "allow": ["business-tools"]
      }
    }
  }'
```

**Expected Result:**
- Policy details updated
- Changes reflected in table
- Last Modified timestamp updated
- Form validation ensures data integrity

## 4. DELETE Operations

### Test Case 4.1: Delete Policy
```bash
curl -X DELETE http://localhost:3002/api/policies/{policy_id}
```

**Expected Result:**
- Policy removed from system
- Returns 200 status with confirmation
- Policy disappears from table
- Statistics cards update
- User confirmation dialog shown

### Test Case 4.2: Delete Non-existent Policy
```bash
curl -X DELETE http://localhost:3002/api/policies/invalid-id
```

**Expected Result:**
- Returns 404 status
- Error message displayed to user
- No changes to existing policies

## 5. UI/UX Testing

### Test Case 5.1: Loading States
- Navigate to policies page
- Verify loading indicators appear
- Check skeleton loaders for cards and table

### Test Case 5.2: Error Handling
- Simulate network errors
- Verify error messages are user-friendly
- Check retry mechanisms work

### Test Case 5.3: Responsive Design
- Test on different screen sizes
- Verify mobile responsiveness
- Check tablet layout

### Test Case 5.4: Accessibility
- Test keyboard navigation
- Verify screen reader compatibility
- Check color contrast ratios

## 6. Performance Testing

### Test Case 6.1: Large Dataset
- Test with 100+ policies
- Verify pagination works
- Check search performance

### Test Case 6.2: Concurrent Operations
- Multiple users editing policies
- Verify data consistency
- Check for race conditions

## Expected Issues and Solutions

### Issue 1: Backend API Not Available
**Problem:** Rust backend not running due to missing Cargo
**Solution:** 
1. Install Rust toolchain
2. Start backend API server
3. Update API_BASE_URL configuration

### Issue 2: 404 Errors on API Calls
**Problem:** API endpoints returning 404
**Solution:**
1. Verify backend is running
2. Check API route configuration
3. Test API endpoints directly

### Issue 3: Missing Error Handling
**Problem:** No user feedback on API failures
**Solution:**
1. Add error boundary components
2. Implement retry mechanisms
3. Add user-friendly error messages

## Test Results Summary

| Operation | Status | Notes |
|-----------|--------|-------|
| CREATE | ⚠️ Pending | Backend API required |
| READ | ⚠️ Pending | Backend API required |
| UPDATE | ⚠️ Pending | Backend API required |
| DELETE | ⚠️ Pending | Backend API required |
| UI/UX | ✅ Good | Well-designed interface |
| Responsive | ✅ Good | Mobile-friendly design |
| Accessibility | ⚠️ Needs Review | Requires testing |

## Recommendations

1. **Immediate Actions:**
   - Install Rust toolchain to enable backend API
   - Add comprehensive error handling
   - Implement loading states

2. **Short-term Improvements:**
   - Add bulk operations
   - Implement policy templates
   - Add audit trail functionality

3. **Long-term Enhancements:**
   - Real-time updates via WebSockets
   - Advanced search and filtering
   - Policy testing/simulation mode
