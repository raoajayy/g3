'use client';

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Select } from '@/components/ui/select';
import { Checkbox } from '@/components/ui/checkbox';
import { Database, Bell, Shield, Palette, Save } from 'lucide-react';

export function SettingsPage() {
  return (
    <div className="p-6">
      <div className="max-w-4xl mx-auto">
        <div className="mb-8">
          <h1 className="text-3xl font-bold text-gray-900">Settings</h1>
          <p className="text-gray-600 mt-2">
            Configure your G3StatsD admin console preferences
          </p>
        </div>

        <div className="space-y-6">
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center space-x-2">
                <Database className="w-5 h-5 text-blue-600" />
                <span>Data Sources</span>
              </CardTitle>
              <CardDescription>
                Configure metrics data sources and connections
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-4">
                <Input
                  label="G3StatsD API URL"
                  defaultValue="http://localhost:3001"
                  helperText="URL for the G3StatsD metrics API"
                />
                <Input
                  label="InfluxDB URL"
                  placeholder="http://localhost:8086"
                  helperText="Optional: InfluxDB connection URL"
                />
                <Input
                  label="InfluxDB Token"
                  type="password"
                  placeholder="Enter InfluxDB token"
                  helperText="Optional: Authentication token for InfluxDB"
                />
              </div>
              <div className="flex justify-end">
                <Button variant="outline" size="sm">
                  Test Connection
                </Button>
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle className="flex items-center space-x-2">
                <Bell className="w-5 h-5 text-orange-600" />
                <span>Notifications</span>
              </CardTitle>
              <CardDescription>
                Set up alerts and notification preferences
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <Checkbox
                label="Email Alerts"
                helperText="Receive alerts via email"
                defaultChecked
              />
              <Checkbox
                label="Browser Notifications"
                helperText="Show browser notifications"
              />
              <Input
                label="Email Address"
                type="email"
                placeholder="admin@example.com"
                helperText="Email address for receiving alerts"
              />
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle className="flex items-center space-x-2">
                <Shield className="w-5 h-5 text-green-600" />
                <span>Security</span>
              </CardTitle>
              <CardDescription>
                Security and authentication settings
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <Checkbox
                label="Require Authentication"
                helperText="Enable login for admin access"
              />
              <Select
                label="Session Timeout"
                helperText="How long before session expires"
                options={[
                  { value: "15", label: "15 minutes" },
                  { value: "30", label: "30 minutes" },
                  { value: "60", label: "1 hour" },
                  { value: "120", label: "2 hours" }
                ]}
                defaultValue="30"
              />
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle className="flex items-center space-x-2">
                <Palette className="w-5 h-5 text-purple-600" />
                <span>Appearance</span>
              </CardTitle>
              <CardDescription>
                Customize the look and feel of the dashboard
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <Select
                label="Theme"
                helperText="Choose your preferred theme"
                options={[
                  { value: "light", label: "Light" },
                  { value: "dark", label: "Dark" },
                  { value: "auto", label: "Auto" }
                ]}
                defaultValue="light"
              />
              <Select
                label="Refresh Interval"
                helperText="How often to update data"
                options={[
                  { value: "5", label: "5 seconds" },
                  { value: "10", label: "10 seconds" },
                  { value: "30", label: "30 seconds" },
                  { value: "60", label: "1 minute" }
                ]}
                defaultValue="5"
              />
            </CardContent>
          </Card>
        </div>
        
        {/* Save Button */}
        <div className="mt-8 flex justify-end">
          <Button size="lg" className="flex items-center space-x-2">
            <Save className="w-4 h-4" />
            <span>Save Settings</span>
          </Button>
        </div>
      </div>
    </div>
  );
}
