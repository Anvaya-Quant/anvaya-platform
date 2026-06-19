export default function LoadingSpinner({ message = 'Loading...' }: { message?: string }) {
  return (
    <div className="flex flex-col items-center justify-center p-8 gap-3">
      <div className="w-8 h-8 border-4 border-anvaya-cobalt border-t-transparent rounded-full animate-spin" />
      <span className="text-sm text-gray-400">{message}</span>
    </div>
  );
}
