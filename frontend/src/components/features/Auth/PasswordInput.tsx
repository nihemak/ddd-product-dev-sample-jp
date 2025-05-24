import React, { useState } from 'react';
// 将来的にアイコンを使うかもしれないので、仮でインポート (例: lucide-react)
// import { Eye, EyeOff } from 'lucide-react';

interface PasswordInputProps
  extends React.InputHTMLAttributes<HTMLInputElement> {
  label: string;
  name: string;
  error?: string;
  // register?: any;
}

export const PasswordInput: React.FC<PasswordInputProps> = ({
  label,
  name,
  error,
  // register,
  ...props
}) => {
  const [showPassword, setShowPassword] = useState(false);

  const toggleShowPassword = () => {
    setShowPassword((prev) => !prev);
  };

  return (
    <div>
      <label htmlFor={name} className="block text-sm font-medium text-gray-700">
        {label}
      </label>
      <div className="relative mt-1">
        <input
          type={showPassword ? 'text' : 'password'}
          id={name}
          name={name}
          // {...(register ? register(name) : {})}
          className={`block w-full rounded-md border-gray-300 px-3 py-2 pr-10 shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm ${
            error
              ? 'border-red-500 focus:border-red-500 focus:ring-red-500'
              : ''
          }`}
          {...props}
        />
        <button
          type="button"
          onClick={toggleShowPassword}
          className="absolute inset-y-0 right-0 flex items-center px-3 text-gray-500 hover:text-gray-700 focus:outline-none"
          aria-label={showPassword ? 'パスワードを隠す' : 'パスワードを表示'}
        >
          {/* ここにアイコンを配置 (例: Eye/EyeOff) */}
          {showPassword ? '隠す' : '表示'}
          {/* {showPassword ? <EyeOff size={18} /> : <Eye size={18} />} */}
        </button>
      </div>
      {error && <p className="mt-1 text-sm text-red-600">{error}</p>}
    </div>
  );
};
