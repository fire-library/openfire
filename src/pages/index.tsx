import { FireIcon, WifiIcon } from "@heroicons/react/24/outline";
import { useTabs } from "src/tabs/tabProvider";
import { TabState, commands, Result, MethodType } from "src/bindings";
import { WrenchScrewdriverIcon } from "@heroicons/react/20/solid";

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
    name: "Parametric Temperature-Time Curves",
    reference: "EN 1991 Part 1-2 Annex A",
    tags: ["Fire Scenario", "Temp Vs Time"],
    svg: FireIcon,
    colors: "text-red-700 bg-red-50",
    stateFunction: commands.parametricFireDefaultState,
  },
  {
    name: "Heat Content of Plume",
    reference: "PD7974-2:2019 Section 7.1",
    tags: ["Fire Scenario", "Temp Vs Time"],
    svg: FireIcon,
    colors: "text-red-700 bg-red-50",
    method: "PD7974Part2Section7Equation1",
  },
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
    name: "BR187 Rdiation View Factors",
    reference: "BR 187 Appendix A",
    tags: ["Radiation", "Heat Flux"],
    svg: WifiIcon,
    colors: "text-yellow-700 bg-yellow-50",
    stateFunction: commands.br187BasicDefaultState,
  },
  {
    name: "Parametric Temperature-Time Curves",
    reference: "EN 1991 Part 1-2 Annex A",
    tags: ["Fire Scenario", "Temp Vs Time"],
    svg: FireIcon,
    colors: "text-red-700 bg-red-50",
    stateFunction: commands.parametricFireDefaultState,
  },
  {
    name: "BR187 Rdiation View Factors",
    reference: "BR 187 Appendix A",
    tags: ["Radiation", "Heat Flux"],
    svg: WifiIcon,
    colors: "text-yellow-700 bg-yellow-50",
    stateFunction: commands.br187BasicDefaultState,
  },
  {
    name: "Parametric Temperature-Time Curves",
    reference: "EN 1991 Part 1-2 Annex A",
    tags: ["Fire Scenario", "Temp Vs Time"],
    svg: FireIcon,
    colors: "text-red-700 bg-red-50",
    stateFunction: commands.parametricFireDefaultState,
  },
  {
    name: "Parametric Temperature-Time Curves",
    reference: "EN 1991 Part 1-2 Annex A",
    tags: ["Fire Scenario", "Temp Vs Time"],
    svg: FireIcon,
    colors: "text-red-700 bg-red-50",
    stateFunction: commands.parametricFireDefaultState,
  },
  {
    name: "Parametric Temperature-Time Curves",
    reference: "EN 1991 Part 1-2 Annex A",
    tags: ["Fire Scenario", "Temp Vs Time"],
    svg: FireIcon,
    colors: "text-red-700 bg-red-50",
    stateFunction: commands.parametricFireDefaultState,
  },
  {
    name: "Parametric Temperature-Time Curves",
    reference: "EN 1991 Part 1-2 Annex A",
    tags: ["Fire Scenario", "Temp Vs Time"],
    svg: FireIcon,
    colors: "text-red-700 bg-red-50",
    stateFunction: commands.parametricFireDefaultState,
  },
  {
    name: "Parametric Temperature-Time Curves",
    reference: "EN 1991 Part 1-2 Annex A",
    tags: ["Fire Scenario", "Temp Vs Time"],
    svg: FireIcon,
    colors: "text-red-700 bg-red-50",
    stateFunction: commands.parametricFireDefaultState,
  },
];

function IndexPage() {
  const { updateTab, currentTab } = useTabs();
  return (
    <div className="max-w-7xl">
      <form className="max-w-lg mx-auto mt-10">
        <div className="flex">
          <button
            id="dropdown-button"
            data-dropdown-toggle="dropdown"
            className="flex-shrink-0 z-10 inline-flex items-center py-2.5 px-4 text-sm font-medium text-center text-gray-900 bg-gray-100 border border-gray-300 rounded-s-lg hover:bg-gray-200 focus:ring-4 focus:outline-none focus:ring-gray-100"
            type="button"
          >
            Everything{" "}
            <svg
              className="w-2.5 h-2.5 ms-2.5"
              aria-hidden="true"
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 10 6"
            >
              <path
                stroke="currentColor"
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth="2"
                d="m1 1 4 4 4-4"
              />
            </svg>
          </button>
          <div
            id="dropdown"
            className="z-10 hidden bg-white divide-y divide-gray-100 rounded-lg shadow w-44"
          >
            <ul
              className="py-2 text-sm text-gray-700 dark:text-gray-200"
              aria-labelledby="dropdown-button"
            >
              <li>
                <button
                  type="button"
                  className="inline-flex w-full px-4 py-2 hover:bg-gray-100"
                >
                  Mockups
                </button>
              </li>
              <li>
                <button
                  type="button"
                  className="inline-flex w-full px-4 py-2 hover:bg-gray-100"
                >
                  Templates
                </button>
              </li>
              <li>
                <button
                  type="button"
                  className="inline-flex w-full px-4 py-2 hover:bg-gray-100"
                >
                  Design
                </button>
              </li>
              <li>
                <button
                  type="button"
                  className="inline-flex w-full px-4 py-2 hover:bg-gray-100"
                >
                  Logos
                </button>
              </li>
            </ul>
          </div>
          <div className="relative w-full">
            <input
              type="search"
              id="search-dropdown"
              className="block p-2.5 w-full z-20 text-sm text-gray-900 bg-gray-50 rounded-e-lg border-s-gray-50 border-s-2 border border-gray-300"
              placeholder="Search Flows, Methods, and more..."
              required
            />
          </div>
        </div>
      </form>

      <div className="flex flex-col flex-1 w-full">
        <div className="flex pt-4 sm:pt-6 h-full">
          <h1 className="text-2xl font-semibold leading-7 text-gray-900 flex flex-row">
            Builder
          </h1>
        </div>
        <div className="flex items-center justify-between py-0">
          <h1 className="text-xl font-semibold leading-7 text-gray-500">
            Create custom calculations and methods
          </h1>
        </div>
      </div>

      <button
        className="bg-white hover:bg-gray-100 w-full"
        onClick={() => commands.newMethodBuilder()}
      >
        <div className="flex w-full space-x-6 p-6 items-center">
          <div className="flex-1">
            <div className="flex flex-col items-start">
              <h3 className="text-lg font-medium text-gray-900">
                Method Builder
              </h3>
            </div>
          </div>
          <WrenchScrewdriverIcon
            className={`h-10 w-10 flex-shrink-0 rounded-full p-1 red-700 bg-red-50`}
          />
        </div>
      </button>

      <div className="flex flex-col flex-1 w-full">
        <div className="flex pt-4 sm:pt-6 h-full">
          <h1 className="text-2xl font-semibold leading-7 text-gray-900 flex flex-row">
            Flows
          </h1>
        </div>
        <div className="flex items-center justify-between py-0">
          <h1 className="text-xl font-semibold leading-7 text-gray-500">
            Calculation flows that combine multiple methods
          </h1>
        </div>
      </div>
      <ul className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3 mt-10">
        {implementations.map((implementation: ImplementationType, index) => {
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
        {implementations.map((implementation: ImplementationType, index) => {
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
