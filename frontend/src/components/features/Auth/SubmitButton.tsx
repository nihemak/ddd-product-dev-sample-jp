import React from 'react';
// import { Loader2 } from 'lucide-react'; // ローディングアイコンの例

interface SubmitButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  isLoading?: boolean;
  // children は React.ButtonHTMLAttributes に含まれる
}

export const SubmitButton: React.FC<SubmitButtonProps> = ({
  children,
  isLoading = false,
  disabled,
  className,
  ...props
}) => {
  const combinedClassName = `flex w-full justify-center rounded-md border border-transparent bg-indigo-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 ${
    isLoading || disabled ? 'cursor-not-allowed opacity-75' : ''
  } ${className || ''}`;

  return (
    <button
      type="submit"
      disabled={isLoading || disabled}
      className={combinedClassName.trim()}
      {...props}
    >
      {isLoading ? (
        <>
          {/* <Loader2 className="mr-2 h-4 w-4 animate-spin" /> */}
          <span>処理中...</span>
        </>
      ) : (
        children
      )}
    </button>
  );
};
