export default function Fab({
  onClick,
  children,
}: {
  onClick: () => void;
  children: React.ReactNode;
}) {
  return (
    <div className="group fixed bottom-10 right-10">
      <button
        type="button"
        className="w-10 h-10 rounded-xl bg-indigo-600 px-2 py-1 mb-2 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
        onClick={onClick}
      >
        {children}
      </button>
      <div className="invisible group-hover:visible absolute -left-28 -top-12 flex flex-col items-center justify-center w-40 h-10 bg-white rounded-lg shadow-md p-2 text-sm text-gray-600">
        Give us your feedback
      </div>
    </div>
  );
}
