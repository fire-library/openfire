import { PlayIcon } from "@heroicons/react/20/solid";

export default function CommandBar() {
  return (
    <div className="flex grow bg-gray-300 fixed w-full h-10">
      <button className="flex items-center justify-center w-10 hover:bg-gray-400">
        <PlayIcon className="w-5 h-5 text-indigo-600" />
      </button>
    </div>
  );
}
