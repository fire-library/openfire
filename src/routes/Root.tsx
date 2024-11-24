import "katex/dist/katex.min.css";
import Navbar from "../components/Navbar";
import TabBar from "./explore/Tabs";
import {
  PlayIcon,
  PlusIcon,
  FolderOpenIcon,
} from "@heroicons/react/24/outline";
import { useTabs } from "../tabs/tabProvider";
import {
  save as saveDialog,
  open as openDialog,
} from "@tauri-apps/plugin-dialog";
import { commands } from "src/bindings";
import Explore from "src/routes/explore";

export default function Root() {
  const { newTab, currentTab } = useTabs();

  const saveCurrentTab = () => {
    if (currentTab?.filename) {
      commands.save(currentTab.filename);
    } else {
      saveDialog({
        filters: [
          {
            name: "yaml",
            extensions: ["yaml"],
          },
        ],
      })
        .then((s) => {
          s && commands.save(s);
        })
        .catch(() => {});
    }
  };

  return (
    <main className="flex flex-col h-screen">
      <Navbar />
      <div className="flex flex-col pt-10 pl-12 bg-slate-50 h-screen">
        <TabBar />
        <Explore />
      </div>
      <div className="fixed flex flex-col h-full left-0 pt-10 justify-center">
        <div className="flex flex-col w-12 bg-gray-800">
          <button
            onClick={() => newTab(null, null)}
            className="flex items-center justify-center h-12 w-12 text-gray-400 hover:text-white"
          >
            <PlusIcon className="h-6 w-6" />
          </button>
        </div>
        <div className="flex flex-col w-12 bg-gray-800">
          <button
            onClick={() =>
              openDialog({
                filters: [
                  {
                    name: "yaml",
                    extensions: ["yaml"],
                  },
                ],
              }).then((s) => {
                if (s && s.constructor === Array) {
                  const b = s[0];
                  open(b);
                } else {
                  open(s as string);
                }
              })
            }
            className="flex items-center justify-center h-12 w-12 text-gray-400 hover:text-white"
          >
            <FolderOpenIcon className="h-6 w-6" />
          </button>
        </div>
        <div className="flex flex-col w-12 h-12 bg-gray-800">
          <button
            onClick={saveCurrentTab}
            className="flex items-center justify-center h-12 w-12 text-gray-400 hover:text-white"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              strokeWidth="1.5"
              stroke="currentColor"
              className="h-6 w-6"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                d="M3 3v18h18V6l-3-3H3ZM7.5 3v6h9V3M6 21v-9h12v9M14.25 5.25v1.5"
              />
            </svg>
          </button>
        </div>
        <div className="flex grow w-12 bg-gray-800" />
        <div className="flex flex-col w-12 h-12 bg-gray-800">
          <button
            onClick={commands.runCalculation}
            className="flex items-center justify-center h-12 w-12 text-gray-400 hover:text-white"
          >
            <PlayIcon className="h-6 w-6" />
          </button>
        </div>
      </div>
    </main>
  );
}
