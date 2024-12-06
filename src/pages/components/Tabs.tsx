import { SetStateAction, Dispatch } from "react";

function classNames(...classes: string[]) {
  return classes.filter(Boolean).join(" ");
}

export type Tab = {
  name: string;
  current: boolean;
  available: boolean;
};

export default function Tabs({
  tabs,
  setTabs,
}: {
  tabs: Tab[];
  setTabs: Dispatch<SetStateAction<Tab[]>>;
}) {
  return (
    <div>
      <div className="sm:hidden">
        <label htmlFor="tabs" className="sr-only">
          Select a tab
        </label>
        <select
          id="tabs"
          name="tabs"
          className="block w-full rounded-md border-gray-300 py-2 pl-3 pr-10 text-base focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm"
        >
          {tabs.map((tab, index) => (
            <option key={index}>{tab.name}</option>
          ))}
        </select>
      </div>
      <div className="hidden sm:block">
        <div className="border-b border-gray-200">
          <nav className="-mb-px flex space-x-8" aria-label="Tabs">
            {tabs.map((tab, index) => (
              <button
                type="button"
                key={index}
                className={classNames(
                  tab.current
                    ? "border-indigo-500 text-indigo-600"
                    : "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700",
                  "whitespace-nowrap border-b-2 px-1 py-4 text-sm font-medium",
                  !tab.available ? "opacity-50 cursor-not-allowed" : ""
                )}
                aria-current={tab ? "page" : undefined}
                onClick={() =>
                  tab.available &&
                  setTabs((prevTabs: Tab[]) => {
                    return prevTabs.map((prevTab, i) => {
                      return {
                        ...prevTab,
                        current: i === index,
                      };
                    });
                  })
                }
              >
                {tab.name}
              </button>
            ))}
          </nav>
        </div>
      </div>
    </div>
  );
}
