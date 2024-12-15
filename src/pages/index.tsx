import { useState, useCallback } from "react";
import { FireIcon, WifiIcon } from "@heroicons/react/24/outline";
import { useTabs } from "src/tabs/tabProvider";
import { TabState, commands, Result, MethodType } from "src/bindings";
import fuse from "fuse.js";
import Fuse from "fuse.js";

type ImplementationType = {
  name: string;
  reference: string;
  method?: MethodType;
  tags: string[];
  svg: React.FC<React.SVGProps<SVGSVGElement>>;
  colors: string;
  stateFunction?: () => Promise<Result<TabState, string>>;
};
const implementations: ImplementationType[] = [
  {
    name: "BR187 Ventilation Factor",
    reference: "BR187, Chapter 1, Equation 1",
    tags: ["Fire Scenario"],
    svg: FireIcon,
    colors: "text-red-700 bg-red-50",
    method: "BR187Chapter1Equation1",
  },
  {
    name: "Alpert",
    reference: "SFPE Handbook",
    tags: ["Fire Scenario"],
    svg: FireIcon,
    colors: "text-red-700 bg-red-50",
    method: "SFPEAlpertHeatReleaseFromTemperatureAndPosition",
  },
  {
    name: "Max Compartment Temp",
    reference: "PD7974-1:2019 Section 8.6",
    tags: ["Fire Scenario"],
    svg: FireIcon,
    colors: "text-red-700 bg-red-50",
    method: "PD7974Part1Section8MaximumEnclosureTemperature",
  },
  {
    name: "HRR at flashover",
    reference: "PD7974-1:2019 Section 8.6",
    tags: ["Fire Scenario"],
    svg: FireIcon,
    colors: "text-red-700 bg-red-50",
    method: "PD7974Part1Section8HRRAtFlashover",
  },
  {
    name: "Burning Regime",
    reference: "PD7974-1:2019 Section 8.6",
    tags: ["Fire Scenario"],
    svg: FireIcon,
    colors: "text-red-700 bg-red-50",
    method: "IntroductionToFireDynamcicsChapter10BurningRegime",
  },
];

function IndexPage() {
  const { updateTab, currentTab } = useTabs();
  const [search, setSearch] = useState("");
  const [fuse] = useState(
    new Fuse(implementations, { keys: ["name", "reference", "tags"] })
  );
  const get_methods: () => { item: ImplementationType }[] = useCallback(() => {
    if (search.trim() === "") {
      return implementations.map((item) => ({ item }));
    }
    return fuse.search(search);
  }, [fuse, search]);

  return (
    <div className="max-w-5xl w-full">
      <form className="max-w-lg mx-auto mt-10">
        <div className="flex">
          <div className="relative w-full">
            <input
              type="search"
              id="search-dropdown"
              className="block p-2.5 w-full z-20 text-sm text-gray-900 bg-gray-50 rounded-lg border-1 border border-gray-300"
              placeholder="Search"
              onChange={(e) => setSearch(e.target.value)}
              required
            />
          </div>
        </div>
      </form>

      <div className="flex flex-col flex-1 w-full">
        <div className="flex pt-4 sm:pt-6 h-full">
          <h1 className="text-2xl font-semibold leading-7 text-gray-900 flex flex-row">
            Methods
          </h1>
        </div>
        <div className="flex items-center justify-between py-0">
          <h1 className="text-xl font-semibold leading-7 text-gray-500">
            Individual calculations for isolated use
          </h1>
        </div>
      </div>
      <ul className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3 mt-10">
        {get_methods().map(({ item: implementation }, index) => {
          const Icon = implementation.svg;
          return (
            <li
              key={index}
              className="col-span-1 divide-y divide-gray-200 rounded-lg shadow"
            >
              <button
                className="bg-white hover:bg-gray-100 w-full"
                onClick={() => {
                  if (implementation.method) {
                    commands.setCurrentTabMethod(implementation.method);
                  } else if (implementation.stateFunction) {
                    implementation.stateFunction().then((state) => {
                      state.status == "ok" &&
                        updateTab(currentTab?.id, state.data);
                    });
                  }
                }}
              >
                <div className="flex w-full space-x-6 p-6">
                  <div className="flex-1">
                    <div className="flex flex-col items-start mb-2">
                      <h3 className="text-sm font-medium text-gray-900">
                        {implementation.name}
                      </h3>
                      <p className="mt-1 text-sm text-gray-500">
                        {implementation.reference}
                      </p>
                    </div>
                    <div className="flex gap-1">
                      {implementation.tags.map((tag) => (
                        <span
                          key={tag}
                          className="inline-flex flex-shrink-0 items-center rounded-full bg-green-50 px-1.5 py-0.5 text-xs font-medium text-green-700 ring-1 ring-inset ring-green-600/20"
                        >
                          {tag}
                        </span>
                      ))}
                    </div>
                  </div>
                  <Icon
                    className={`h-10 w-10 flex-shrink-0 rounded-full p-1 ${implementation.colors}`}
                  />
                </div>
              </button>
            </li>
          );
        })}
      </ul>
    </div>
  );
}

export default IndexPage;
