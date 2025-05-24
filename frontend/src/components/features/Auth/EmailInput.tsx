import React from 'react';

interface EmailInputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  label: string;
  name: string;
  error?: string;
  // react-hook-form連携用のregister関数を想定 (今回は簡易的にstringで型定義)
  // register?: any; // UseFormRegister<FieldValues>; ← 具体的な型はフォームライブラリ導入後
}

export const EmailInput: React.FC<EmailInputProps> = ({
  label,
  name,
  error,
  // register,
  ...props
}) => {
  return (
    <div>
      <label htmlFor={name} className="block text-sm font-medium text-gray-700">
        {label}
      </label>
      <input
        type="email"
        id={name}
        name={name}
        // {...(register ? register(name) : {})} // react-hook-form等と連携する場合
        className={`mt-1 block w-full rounded-md border-gray-300 px-3 py-2 shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm ${
          error ? 'border-red-500 focus:border-red-500 focus:ring-red-500' : ''
        }`}
        {...props}
      />
      {error && <p className="mt-1 text-sm text-red-600">{error}</p>}
    </div>
  );
};
