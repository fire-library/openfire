export default function ProgressBar({ percent }: { percent: number }) {
  return (
    <div className="flex w-full mt-1 mx-2 text-xs">
      Downloading
      <div className="overflow-hidden rounded-full bg-gray-400 w-full h-2">
        <div
          style={{ width: `${percent}%` }}
          className="h-2 rounded-full bg-indigo-600"
        />
      </div>
    </div>
  );
}
