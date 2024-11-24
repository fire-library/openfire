import { TrashIcon } from "@heroicons/react/20/solid";

export default function Delete({
  onClick,
  active = true,
}: {
  onClick: () => void;
  active?: boolean;
}) {
  const extraClasses = active
    ? ""
    : " opacity-50 cursor-not-allowed hover:bg-slate-600";
  return (
    <div className="group">
      <button
        type="button"
        className={
          "rounded-full bg-slate-600 p-2 mx-4 mb-2 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600" +
          extraClasses
        }
        onClick={onClick}
      >
        <TrashIcon className="h-3 w-3" aria-hidden="true" />
      </button>
      {!active && (
        <div className="invisible group-hover:visible absolute bg-white rounded-xl ring-1 ring-gray-200 mt-2 shadow-lg p-2 z-40">
          Currently in use
        </div>
      )}
    </div>
  );
}
