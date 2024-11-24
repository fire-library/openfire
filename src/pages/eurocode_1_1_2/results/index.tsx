import { useState, useCallback, ReactNode, useMemo } from "react";
import { ParametricFireResult, Tab, commands } from "src/bindings";
import { Card, CardBody, CardHeader, SuccessButton } from "src/components";
import LineChart from "src/components/LineChart";
import MaterialsCard from "./MaterialsCard";
import SurfacesCard from "./SurfacesCard";
import FireLoadCard from "./FireloadCard";
import HeatingTempCalcCard from "./HeatingTempCalcCard";
import CoolingTempCalcCard from "./CoolingTempCalcCard";
import TMaxCaclCard from "./TMaxCalcCard";
import BFactorCard from "./BFactorCard";
import CalculationCard from "./FireloadCalcCard";
import Tabs from "../../components/Tabs";
import { FixedSizeList as List } from "react-window";
import { ExclamationTriangleIcon } from "@heroicons/react/20/solid";
import { save } from "@tauri-apps/plugin-dialog";

const CalculationTab = ({ warning }: { warning: boolean }) => {
  return (
    <div key="calc" className="flex flex-row gap-1">
      {warning && (
        <ExclamationTriangleIcon className="w-5 h-5 text-yellow-400" />
      )}
      Calculation
    </div>
  );
};

const round = (num: number, digits: number) => {
  const divisor = Math.pow(10, digits);
  return Math.round(num * divisor) / divisor;
};

export default function Results({ tab }: { tab: Tab }) {
  const tabs: ReactNode[] = useMemo(() => {
    const calculation = (tab.state as ParametricFireResult).output.calculation;
    const warning =
      calculation.fireload.qTd.warning ||
      calculation.geometry.b.warning ||
      calculation.geometry.o.warning ||
      calculation.geometry.surfaces.some(
        (s) => s.b.warning || s.b1.warning || s.b2?.warning
      )
        ? true
        : false;

    return [
      "Results",
      "Input",
      <CalculationTab key="calculation" warning={warning} />,
    ];
  }, [tab.state]);

  const data = tab.state as ParametricFireResult;
  const [currentTab, setCurrentTab] = useState(tabs[0]);

  const saveToCsvDialog = () => {
    save({
      filters: [
        {
          name: "csv",
          extensions: ["csv"],
        },
      ],
    })
      .then((s) => {
        s && commands.saveToCsv(s, data);
      })
      .catch(() => {});
  };

  return (
    <div className="flex flex-row max-w-6xl w-full px-0 sm:px-5">
      <div className="flex flex-col w-full">
        <div className="flex items-center justify-between pt-4 sm:pt-6">
          <h1 className="text-2xl font-semibold leading-7 text-gray-900">
            Parametric Temperature-Time Curves - Results
          </h1>
        </div>
        <div className="flex items-center justify-between py-0 sm:pb-6">
          <h1 className="text-xl font-semibold leading-7 text-gray-500">
            Eurocode 1 Part 1-2 Annex A
          </h1>
        </div>
        <Tabs tabs={tabs} currentTab={currentTab} setTab={setCurrentTab} />
        {currentTab === tabs[0] && (
          <>
            <Card>
              <CardHeader>
                <div className="ml-4 mt-4">
                  <h3 className="text-base font-semibold leading-6 text-gray-900">
                    Temperature - Time Curve
                  </h3>
                  <p className="mt-1 text-sm text-gray-500">
                    The parametric fire calculated from the input data provided.
                  </p>
                </div>
              </CardHeader>
              {data ? (
                <LineChart
                  time={data.output.tempTimeCurve.time as number[]}
                  temp={data.output.tempTimeCurve.temperature as number[]}
                />
              ) : (
                <CardBody>No results to show yet yet</CardBody>
              )}
            </Card>
            <Card>
              <CardHeader>
                <div className="ml-4 mt-4">
                  <h3 className="text-base font-semibold leading-6 text-gray-900">
                    Temperature - Time Table
                  </h3>
                  <p className="mt-1 text-sm text-gray-500">
                    A full list of the temperature calculated at each timestep
                  </p>
                </div>
              </CardHeader>
              <CardBody>
                <div className="flex flex-row max-w-lg">
                  {data && <TimeTempTable data={data} />}
                </div>
                <div className="mt-10">
                  <SuccessButton onClick={saveToCsvDialog}>
                    Export to CSV
                  </SuccessButton>
                </div>
              </CardBody>
            </Card>
          </>
        )}
        {currentTab === tabs[1] && (
          <div className="flex flex-col">
            <MaterialsCard materials={data?.input.materials || []} />
            <SurfacesCard
              surfaces={data?.input.walls || []}
              materials={data?.input.materials || []}
            />
            {data?.input.fireLoad && (
              <FireLoadCard
                fireloadInput={data?.input.fireLoad}
                fireloadFactors={data.output.calculation.fireload.deltaQ2}
              />
            )}
          </div>
        )}
        {currentTab === tabs[2] && (
          <div className="flex flex-col w-full">
            {data && (
              <CalculationCard
                input={data.input}
                calculation={data.output.calculation}
              ></CalculationCard>
            )}
            {data && (
              <TMaxCaclCard
                input={data.input}
                calculation={data.output.calculation}
              ></TMaxCaclCard>
            )}
            {data && (
              <BFactorCard
                input={data.input}
                calculation={data.output.calculation}
              ></BFactorCard>
            )}
            {data && <HeatingTempCalcCard data={data}></HeatingTempCalcCard>}
            {data && <CoolingTempCalcCard data={data}></CoolingTempCalcCard>}
          </div>
        )}
      </div>
    </div>
  );
}

const Row = ({
  index,
  data,
  style,
}: {
  index: number;
  data: ParametricFireResult;
  style: React.CSSProperties;
}) => {
  const temp = data.output.tempTimeCurve.temperature[index];
  const time = data.output.tempTimeCurve.time[index];
  return (
    <div key={index} style={style} className="grid grid-cols-2">
      <div className="px-3 py-4 text-sm text-gray-500">{round(time, 5)}</div>
      <div className="px-3 py-4 text-sm text-gray-500">{round(temp, 2)}</div>
    </div>
  );
};

function TimeTempTable({ data }: { data: ParametricFireResult }) {
  const [width, setWidth] = useState(100);
  const tableRef = useCallback((node: HTMLTableSectionElement) => {
    if (node !== null) {
      setWidth(() => node.clientWidth);
    }
  }, []);

  return (
    <>
      <div className="flex flex-col w-full divide-y divide-gray-400">
        <div ref={tableRef} className="grid grid-cols-2 mt-4">
          <div className="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-0">
            Time (h)
          </div>
          <div className="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-0">
            Temperature (C)
          </div>
        </div>
        <List
          height={400}
          itemCount={data.output.tempTimeCurve.time.length}
          itemSize={35}
          width={width}
          itemData={data}
        >
          {Row}
        </List>
      </div>
    </>
  );
}
