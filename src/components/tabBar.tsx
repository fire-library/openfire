import { Link, Outlet } from "react-router-dom";

export type Tab = {
  name: string;
  href: string;
};

function classNames(...classes: string[]) {
  return classes.filter(Boolean).join(" ");
}

export default function TabBar({
  tabs,
  currentTab,
}: {
  tabs: Tab[];
  currentTab: Tab;
}) {
  return (
    <div>
      <div className="hidden sm:block">
        <div className="border-b border-gray-300">
          <nav className="-mb-px flex space-x-8" aria-label="Tabs">
            {tabs.map((tab) => (
              <Link to={tab.href} key={tab.name}>
                <button
                  className={classNames(
                    tab.name === currentTab.name
                      ? "border-indigo-500 text-indigo-600"
                      : "border-transparent text-gray-900 hover:border-gray-300 hover:text-gray-700",
                    "whitespace-nowrap border-b-2 py-4 px-1 text-sm font-medium"
                  )}
                >
                  {tab.name}
                </button>
              </Link>
            ))}
          </nav>
        </div>
      </div>
      <Outlet />
    </div>
  );
}
