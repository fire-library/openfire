export default function Checkbox({ className }: { className?: string }) {
  return (
    <button className={`w-full flex flex-row items-center py-2`} type="button">
      <input
        id="required-checkbox"
        type="checkbox"
        className={`min-h-4 min-w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-600 mr-3 cursor-pointer ${className}`}
      />
    </button>
  );
}
