import "katex/dist/katex.min.css";
import { useTabs } from "../../../tabs/tabProvider";
import { XMarkIcon } from "@heroicons/react/20/solid";

function classNames(...classes: string[]) {
  return classes.filter(Boolean).join(" ");
}

export default function TabBar() {
  const { tabs, setCurrentTab, deleteTab } = useTabs();

  return (
    <div className="fixed w-full h-10 z-50">
      <div className="sm:hidden">
        <label htmlFor="tabs" className="sr-only">
          Select a tab
        </label>
        <select
          id="tabs"
          name="tabs"
          className="block w-full border-gray-300 focus:border-indigo-500 focus:ring-indigo-500"
          defaultValue={"Billing"}
        >
          {tabs.map((tab) => (
            <option key={tab.id}>{tab.title || "Untitled"}</option>
          ))}
        </select>
      </div>
      <div className="hidden sm:block">
        <nav
          className="isolate flex divide-gray-200 shadow bg-gray-50"
          aria-label="Tabs"
        >
          {tabs.map((tab, tabIdx) => (
            <span
              key={tab.id}
              onClick={() => setCurrentTab(tab.id)}
              onKeyDown={() => setCurrentTab(tab.id)}
              role="tab"
              tabIndex={tabIdx}
              className={classNames(
                tab.current
                  ? "text-gray-900 bg-indigo-50"
                  : "text-gray-500 hover:text-gray-700 hover:bg-gray-100 bg-white",
                "group flex flex-col min-w-32 overflow-hidden text-xs font-medium focus:z-10 h-9 cursor-pointer select-none"
              )}
              aria-current={tab.current ? "page" : undefined}
            >
              <div className="flex w-full justify-center items-center h-full px-1">
                <span className="truncate px-2 w-full flex justify-center">
                  {!tab.saved && "*"}
                  {tab.title || "Untitled"}
                </span>
                <button
                  className="h-5 w-5 hover:bg-gray-200"
                  onClick={(e) => {
                    e.stopPropagation();
                    deleteTab(tabIdx);
                  }}
                >
                  <XMarkIcon />
                </button>
              </div>
              <span
                aria-hidden="true"
                className={classNames(
                  tab.current ? "bg-indigo-500" : "bg-transparent",
                  "h-0.5 w-full"
                )}
              />
            </span>
          ))}
        </nav>
      </div>
    </div>
  );
}
