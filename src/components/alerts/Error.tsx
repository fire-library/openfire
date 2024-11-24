import { XCircleIcon, XMarkIcon } from "@heroicons/react/20/solid";

export default function ErrorAlert({
  setError,
  text,
}: {
  setError: (_show: string | null) => void;
  text: string | null;
}) {
  if (!text) {
    return null;
  }
  return (
    <div className="rounded-md bg-red-50 p-4 my-4">
      <div className="flex flex-row">
        <div className="flex-shrink-0">
          <XCircleIcon className="h-5 w-5 text-red-400" aria-hidden="true" />
        </div>
        <div className="ml-3 flex-1">
          <h3 className="text-sm font-medium text-red-800">{text}</h3>
        </div>
        <button
          type="button"
          className="inline-flex rounded-md text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
          onClick={() => {
            setError(null);
          }}
        >
          <span className="sr-only">Close</span>
          <XMarkIcon className="h-5 w-5" aria-hidden="true" />
        </button>
      </div>
    </div>
  );
}
