import { forwardRef } from 'react';
import { cn } from '@/lib/utils';

interface CheckboxProps extends React.InputHTMLAttributes<HTMLInputElement> {
  label?: string;
  error?: string;
  helperText?: string;
}

export const Checkbox = forwardRef<HTMLInputElement, CheckboxProps>(
  ({ className, label, error, helperText, ...props }, ref) => {
    return (
      <div className="space-y-1">
        <label className="flex items-start space-x-3 cursor-pointer">
          <input
            type="checkbox"
            className={cn(
              "mt-1 h-4 w-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500 focus:ring-2",
              error && "border-red-300 focus:ring-red-500",
              className
            )}
            ref={ref}
            {...props}
          />
          <div className="flex-1">
            {label && (
              <span className="text-sm font-medium text-gray-700">{label}</span>
            )}
            {helperText && !error && (
              <p className="text-sm text-gray-500 mt-1">{helperText}</p>
            )}
            {error && (
              <p className="text-sm text-red-600 mt-1">{error}</p>
            )}
          </div>
        </label>
      </div>
    );
  }
);

Checkbox.displayName = 'Checkbox';
