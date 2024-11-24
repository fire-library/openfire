import React, { useCallback, useState } from "react";
import { ParametricFireResult, TempTimeCurve } from "src/bindings";
import { Card, CardBody, CardHeader } from "src/components";
import { InlineMath } from "react-katex";
import { FixedSizeList as List } from "react-window";
import WarningAlertOptional from "src/components/alerts/Warning";

const round = (num: number, digits: number) => {
  const divisor = Math.pow(10, digits);
  return Math.round(num * divisor) / divisor;
};

export default function HeatingTempCalcCard({
  data,
}: {
  data: ParametricFireResult;
}) {
  const q_td_warning = data.output.calculation.fireload.qTd.warning;
  const o_warning = data.output.calculation.geometry.o.warning;

  return (
    <Card>
      <div className="flex justify-center">
        <h2 className="text-2xl font-bold leading-7 sm:tracking-tight">
          Temperature in the Cooling Phase
        </h2>
      </div>
      <CardHeader>
        <div className="ml-4 mt-4">
          <h3 className="text-base font-semibold leading-6 text-gray-900">
            Nomenclature
          </h3>
        </div>
      </CardHeader>
      <CardBody>
        <table>
          <thead>
            <tr>
              <th></th>
              <th></th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td>
                <InlineMath>{`\\Theta_{g}`}</InlineMath>
              </td>
              <td className="pl-4">
                <div>temperature of the gas (C)</div>
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`\\Theta_{max}`}</InlineMath>
              </td>
              <td className="pl-4">
                <div>the maximum gas temperature achieved (C)</div>
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`t_{max}`}</InlineMath>
              </td>
              <td className="pl-4">
                <div>time at the end of the heating phase (h)</div>
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`t_{lim}`}</InlineMath>
              </td>
              <td className="pl-4">
                <div>
                  time at the end of the heating phase, as dictated by the fire
                  growth rate (h)
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </CardBody>
      <CardHeader>
        <div className="ml-4 mt-4">
          <h3 className="text-base font-semibold leading-6 text-gray-900">
            Equations
          </h3>
        </div>
      </CardHeader>
      <CardBody>
        <div className="flex flex-col gap-3">
          <div className="flex flex-row gap-3">
            if: <InlineMath>{`t^{*}_{max} \\le 0.5`}</InlineMath>
          </div>
          <div className="flex flex-col ml-4 pl-4 gap-2">
            <div className="flex flex-row gap-3">
              <InlineMath>{`\\Theta_{g} = \\Theta_{max} - 625(t^{*} - t^{*}_{max} \\cdot x)`}</InlineMath>
            </div>
          </div>
          <div className="flex flex-row justify-left gap-3 mt-5">
            else if: <InlineMath>{`0.5 < t^{*}_{max} < 2.0`}</InlineMath>
          </div>
          <div className="flex flex-col justify-left gap-3 pl-4 ml-4">
            <InlineMath>{`\\Theta_{g} = \\Theta_{max} - 250(3 \\cdot t^{*}_{max})(t^{*} - t^{*}_{max} \\cdot x)`}</InlineMath>
          </div>
          <div className="flex flex-row justify-left gap-3 mt-5">
            else if: <InlineMath>{`t^{*}_{max} \\ge 2.0`}</InlineMath>
          </div>
          <div className="flex flex-col justify-left gap-3 pl-4 ml-4">
            <InlineMath>{`\\Theta_{g} = \\Theta_{max} - 250(t^{*} - t^{*}_{max} \\cdot x)`}</InlineMath>
          </div>
          <div className="flex flex-row gap-3">
            <InlineMath>{`x = 1.0`}</InlineMath>
            if
            <InlineMath>{`t_{max} > t_{lim}`}</InlineMath>
            or
            <InlineMath>{`x = t_{lim} \\cdot \\Gamma / t^{*}_{max}`}</InlineMath>
            if
            <InlineMath>{`t_{max} = t_{lim}`}</InlineMath>
          </div>
          <div className="flex flex-col gap-3">
            <InlineMath>{`t^{*} = t \\cdot \\Gamma`}</InlineMath>
          </div>
          <div className="flex flex-col gap-3">
            <InlineMath>{`t^{*}_{max} = (0.2 \\cdot 10^{-3} \\cdot q_{t,d} / O) \\cdot \\Gamma`}</InlineMath>
          </div>
        </div>
      </CardBody>
      <CardHeader>
        <div className="ml-4 mt-4">
          <h3 className="text-base font-semibold leading-6 text-gray-900">
            Input
          </h3>
        </div>
      </CardHeader>
      <CardBody>
        <table>
          <thead>
            <tr className="mt-4">
              <th></th>
              <th></th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td>
                <InlineMath>{`t_{max}`}</InlineMath>
              </td>
              <td className="pl-4">{data.output.calculation.tMax} h</td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`t_{lim}`}</InlineMath>
              </td>
              <td className="pl-4">{data.output.calculation.tLim} h</td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`\\Gamma`}</InlineMath>
              </td>
              <td className="pl-4">{data.output.calculation.gamma}</td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`\\Theta_{max}`}</InlineMath>
              </td>
              <td className="pl-4">{data.output.calculation.thetaMax} C</td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`q_{t,d}`}</InlineMath>
              </td>
              <td className="pl-4">
                <WarningAlertOptional
                  warning={!!q_td_warning}
                  popupHeading="Out Of Range"
                  popupContent={
                    q_td_warning && (
                      <div className="flex flex-col gap-2">
                        <p>
                          <InlineMath>{`q_{t,d}`}</InlineMath> is required to be
                          within the range{" "}
                          <InlineMath>{`${q_td_warning.min} \\le q_{t,d} \\le ${q_td_warning.max} \\space [MJ/m^{2}]`}</InlineMath>{" "}
                          to be within the limits of applicability of this
                          method.
                        </p>
                      </div>
                    )
                  }
                >
                  {data.output.calculation.fireload.qTd.value} MJ/m<sup>2</sup>
                </WarningAlertOptional>
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`O`}</InlineMath>
              </td>
              <td className="pl-4">
                <WarningAlertOptional
                  warning={!!o_warning}
                  popupHeading="Out Of Range"
                  popupContent={
                    o_warning && (
                      <div className="flex flex-col gap-2">
                        <p>
                          <InlineMath>{`O`}</InlineMath> is required to be
                          within the range{" "}
                          <InlineMath>{`${o_warning.min} \\le q_{t,d} \\le ${o_warning.max} \\space [m^{1/2}]`}</InlineMath>{" "}
                          to be within the limits of applicability of this
                          method.
                        </p>
                      </div>
                    )
                  }
                >
                  {data.output.calculation.geometry.o.value} m<sup>1/2</sup>
                </WarningAlertOptional>
              </td>
            </tr>
          </tbody>
        </table>
      </CardBody>
      <CardHeader>
        <div className="ml-4 mt-4">
          <h3 className="text-base font-semibold leading-6 text-gray-900">
            Caclulation
          </h3>
        </div>
      </CardHeader>
      <CardBody>
        <div className="flex flex-col gap-3">
          <div className="flex flex-row justify-left gap-3">
            <InlineMath>{`t^{*}_{max} = (0.2 \\cdot 10^{-3} \\cdot q_{t,d} / O) \\cdot \\Gamma = (0.2 \\cdot 10^{-3} \\cdot ${data.output.calculation.fireload.qTd.value} / ${data.output.calculation.geometry.o.value}) \\cdot ${data.output.calculation.gamma} = ${data.output.calculation.tStarMaxCooling}`}</InlineMath>
          </div>
          {data.output.calculation.tMax == data.output.calculation.tLim && (
            <>
              <div className="flex flex-row justify-left gap-3">
                <InlineMath>{`t_{max} = t_{lim}`}</InlineMath>
              </div>
              <div className="flex flex-row justify-left gap-3">
                <InlineMath>{`x = t_{lim} \\cdot \\Gamma / t^{*}_{max} = ${data.output.calculation.tLim} \\cdot ${data.output.calculation.gamma} / ${data.output.calculation.tStarMaxCooling} = ${data.output.calculation.x}`}</InlineMath>
              </div>
            </>
          )}
          {data.output.calculation.tMax > data.output.calculation.tLim && (
            <>
              <div className="flex flex-row justify-left gap-3">
                <InlineMath>{`t_{max} > t_{lim}`}</InlineMath>
              </div>
              <div className="flex flex-row justify-left gap-3">
                <InlineMath>{`x = 1.0`}</InlineMath>
              </div>
            </>
          )}
          {data.output.calculation.tMax <= 0.5 && (
            <>
              <div className="flex flex-row justify-left gap-3">
                <InlineMath>{`t_{max} \\le 0.5`}</InlineMath>
                Therefore:
              </div>
              <div className="flex flex-row justify-left gap-3">
                <InlineMath>{`\\Theta_{g} = \\Theta_{max} - 625(t^{*} - t^{*}_{max} \\cdot x)`}</InlineMath>
              </div>
            </>
          )}
          {data.output.calculation.tMax > 0.5 &&
            data.output.calculation.tMax < 2.0 && (
              <>
                <div className="flex flex-row justify-left gap-3">
                  <InlineMath>{`0.5 < t_{max} < 2.0`}</InlineMath>
                </div>
                <div className="flex flex-row justify-left gap-3">
                  <InlineMath>{`\\Theta_{g} = \\Theta_{max} - 250(3 \\cdot t^{*}_{max})(t^{*} - t^{*}_{max} \\cdot x)`}</InlineMath>
                </div>
              </>
            )}
          {data.output.calculation.tMax >= 2.0 && (
            <>
              <div className="flex flex-row justify-left gap-3">
                <InlineMath>{`t_{max} \\ge 2.0`}</InlineMath>
              </div>
              <div className="flex flex-row justify-left gap-3">
                <InlineMath>{`\\Theta_{g} = \\Theta_{max} - 250(t^{*} - t^{*}_{max} \\cdot x)`}</InlineMath>
              </div>
            </>
          )}
          <TimeTempTable data={data} />
        </div>
      </CardBody>
    </Card>
  );
}

const Row = ({
  index,
  data,
  style,
}: {
  index: number;
  data: TempTimeCurve & { multiplier: number };
  style: React.CSSProperties;
}) => {
  const temp = data.temperature[index];
  const time = data.time[index];
  return (
    <div key={index} style={style} className="grid grid-cols-3">
      <div className="px-3 py-4 text-sm text-gray-500">{round(time, 5)}</div>
      <div className="px-3 py-4 text-sm text-gray-500">
        {round(time * data.multiplier, 5)}
      </div>
      <div className="px-3 py-4 text-sm text-gray-500">{round(temp, 2)}</div>
    </div>
  );
};

function TimeTempTable({ data }: { data: ParametricFireResult }) {
  const [width, setWidth] = useState(100);
  const [filteredData] = useState(() => {
    const time = data.output.tempTimeCurve.time.filter(
      (time) => time > data.output.calculation.tMax
    );
    const temperature = data.output.tempTimeCurve.temperature.filter(
      (_, index) =>
        data.output.tempTimeCurve.time[index] > data.output.calculation.tMax
    );
    return { time, temperature };
  });

  const tableRef = useCallback((node: HTMLTableSectionElement) => {
    if (node !== null) {
      setWidth(() => node.clientWidth);
    }
  }, []);

  return (
    <>
      <div className="flex flex-col w-full divide-y divide-gray-400">
        <div ref={tableRef} className="grid grid-cols-3 mt-4">
          <div className="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-0">
            Time (h)
          </div>
          <div className="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-0">
            t<sup>*</sup> (h)
          </div>
          <div className="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-0">
            Temperature (C)
          </div>
        </div>
        <List
          height={400}
          itemCount={filteredData.time.length}
          itemSize={35}
          width={width}
          itemData={{
            ...filteredData,
            multiplier: data.output.calculation.gamma,
          }}
        >
          {Row}
        </List>
      </div>
    </>
  );
}
