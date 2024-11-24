import React, { useCallback, useState } from "react";
import {
  ParametricFireCalcSheet,
  ParametricFireResult,
  TempTimeCurve,
} from "src/bindings";
import { Card, CardBody, CardHeader } from "src/components";
import { InlineMath } from "react-katex";
import { FixedSizeList as List } from "react-window";
import WarningAlertOptional from "src/components/alerts/Warning";
import { BWarningPopup } from "./BFactorCard/SurfaceCalc";

export default function HeatingTempCalcCard({
  data,
}: {
  data: ParametricFireResult;
}) {
  const o_warning = data.output.calculation.geometry.o.warning;
  const q_td_warning = data.output.calculation.fireload.qTd.warning;
  const b_warning = data.output.calculation.geometry.b.warning;

  return (
    <Card>
      <div className="flex justify-center">
        <h2 className="text-2xl font-bold leading-7 sm:tracking-tight">
          Temperature in the Heating Phase
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
            <tr>
              <td>
                <InlineMath>{`b`}</InlineMath>
              </td>
              <td className="pl-4">
                <div>b factor of the enclosure</div>
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`q_{t,d}`}</InlineMath>
              </td>
              <td className="pl-4">
                <div>
                  the design value of the fire load density related to the total
                  surface area At of the enclosure (MJ/m<sup>2</sup>)
                </div>
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`O`}</InlineMath>
              </td>
              <td className="pl-4">
                <div>
                  opening factor (m<sup>1/2</sup>)
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
          <div className="flex flex-row justify-left gap-3">
            <InlineMath>
              {`\\Theta_{g} = 20 + 1325  (1 - 0.324\\exp^{-0.2t^{*}} -0.204\\exp^{-1.7t^{*}} - 0.472\\exp^{-19t^{*}})`}
            </InlineMath>
            (A.1)
          </div>
          <div className="flex flex-col pl-4 pt-3 gap-2">
            <div className="flex flex-row gap-3 text-sm">
              if: <InlineMath>{`t_{max} = t_{lim}`}</InlineMath> :{" "}
              <InlineMath>{`t^{*} = t \\cdot \\Gamma_{lim}`}</InlineMath>
            </div>
            <div className="flex flex-col ml-4 pl-4 gap-2 text-sm">
              <div className="flex flex-row gap-3 text-sm">
                <InlineMath>{`\\Gamma_{lim} = k \\cdot [O_{lim}/b]^{2} / (0.04/1160)^{2}`}</InlineMath>
              </div>
              <div className="flex flex-row gap-3 text-sm">
                <InlineMath>{`O_{lim} = 0.1 \\cdot 10^{-3} \\cdot q_{t,d} / t_{lim}`}</InlineMath>
              </div>
              <div className="flex flex-row justify-left gap-3 text-sm">
                if: <InlineMath>{`O > 0.04`}</InlineMath> and:{" "}
                <InlineMath>{`q_{t,d} < 75`}</InlineMath>
                and: <InlineMath>{`b < 1160`}</InlineMath>
              </div>
              <div className="flex flex-col pl-4 gap-2">
                <InlineMath>{`k = 1 + (\\frac{O - 0.04}{0.04})(\\frac{q_{t, d} - 75}{75})(\\frac{1160 - b}{1160})`}</InlineMath>
              </div>
              <div className="flex flex-row justify-left gap-3 text-sm">
                else:
              </div>
              <div className="flex flex-col pl-4 gap-2">
                <InlineMath>{`k = 1`}</InlineMath>
              </div>
            </div>
            <div className="flex flex-row justify-left gap-3 mt-5 text-sm">
              else if: <InlineMath>{`t_{max} \\ne t_{lim}`}</InlineMath> :{" "}
            </div>
            <div className="flex flex-col justify-left gap-3 text-sm pl-4 ml-4">
              <InlineMath>{`t^{*} = t \\cdot \\Gamma`}</InlineMath>
              <InlineMath>{`\\Gamma = [O/b]^{2} / (0.04/1160)^{2}`}</InlineMath>
            </div>
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
                <InlineMath>{`t_{lim}`}</InlineMath>
              </td>
              <td className="pl-4">{data.output.calculation.tLim} h</td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`t_{max}`}</InlineMath>
              </td>
              <td className="pl-4">{data.output.calculation.tMax} h</td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`b`}</InlineMath>
              </td>
              <td className="pl-4">
                <WarningAlertOptional
                  warning={!!b_warning}
                  popupHeading="Out Of Range"
                  popupContent={
                    b_warning && (
                      <BWarningPopup warning={b_warning} subscript={""} />
                    )
                  }
                >
                  {data.output.calculation.geometry.b.value} J/m<sup>2</sup>s
                  <sup>1/2</sup>K
                </WarningAlertOptional>
              </td>
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
            Calcuation
          </h3>
        </div>
      </CardHeader>
      <CardBody>
        <div className="flex flex-col gap-3">
          {data.output.calculation.tMax == data.output.calculation.tLim ? (
            <EqualTMaxTLim calculation={data.output.calculation} />
          ) : (
            <NotEqualTMaxTLim calculation={data.output.calculation} />
          )}
          <TimeTempTable data={data} />
        </div>
      </CardBody>
    </Card>
  );
}

const round = (num: number, digits: number) => {
  const divisor = Math.pow(10, digits);
  return Math.round(num * divisor) / divisor;
};

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
      (time) => time < data.output.calculation.tMax
    );
    const temperature = data.output.tempTimeCurve.temperature.filter(
      (_, index) => index < time.length - 1
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
            <InlineMath>{`t^{*}`}</InlineMath>
          </div>
          <div className="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-0">
            Temperature (C)
          </div>
        </div>
        <List
          height={400}
          itemCount={filteredData.temperature.length}
          itemSize={35}
          width={width}
          itemData={{
            ...filteredData,
            multiplier:
              data.output.calculation.tMax == data.output.calculation.tLim
                ? data.output.calculation.gammaLim
                : data.output.calculation.gamma,
          }}
        >
          {Row}
        </List>
      </div>
    </>
  );
}

function EqualTMaxTLim({
  calculation,
}: {
  calculation: ParametricFireCalcSheet;
}) {
  return (
    <>
      <div className="flex flex-row justify-left gap-3">
        <InlineMath>{`t_{max} = t_{lim}`}</InlineMath>
      </div>
      {calculation.geometry.o.value > 0.04 &&
      calculation.fireload.qTd.value < 75 &&
      calculation.geometry.b.value < 1160 ? (
        <>
          <div className="flex flex-row justify-left gap-3 text-sm">
            <InlineMath>{`O > 0.04`}</InlineMath> and:{" "}
            <InlineMath>{`q_{t,d} < 75`}</InlineMath>
            and: <InlineMath>{`b < 1160`}</InlineMath>
          </div>
          <div className="flex flex-col pl-4 gap-2">
            <InlineMath>{`k = 1 + (\\frac{O - 0.04}{0.04})(\\frac{q_{t, d} - 75}{75})(\\frac{1160 - b}{1160})`}</InlineMath>
            <InlineMath>{`k = 1 + (\\frac{${calculation.geometry.o.value} - 0.04}{0.04})(\\frac{${calculation.fireload.qTd.value} - 75}{75})(\\frac{1160 - ${calculation.geometry.b.value}}{1160}) = ${calculation.k}`}</InlineMath>
          </div>
          <div className="flex flex-row gap-3 text-sm">
            <InlineMath>{`O_{lim} = 0.1 \\cdot 10^{-3} \\cdot q_{t,d} / t_{lim} = 0.1 \\cdot 10^{-3} \\cdot ${calculation.fireload.qTd.value} / ${calculation.tLim} = ${calculation.geometry.oLim} \\space m^{1/2}`}</InlineMath>
          </div>
          <div className="flex flex-row gap-3 text-sm">
            <InlineMath>{`\\Gamma_{lim} = k \\cdot [O_{lim}/b]^{2} / (0.04/1160)^{2} = ${calculation.k} \\cdot [${calculation.geometry.oLim}/${calculation.geometry.b.value}]^{2} / (0.04/1160)^{2} = ${calculation.gammaLim}`}</InlineMath>
          </div>
          <div className="flex flex-row gap-3 text-sm mt-5">
            <InlineMath>{`t^{*} = t \\cdot \\Gamma_{lim}`}</InlineMath>
            and
            <InlineMath>
              {`\\Theta_{g} = 20 + 1325  (1 - 0.324\\exp^{-0.2t^{*}} -0.204\\exp^{-1.7t^{*}} - 0.472\\exp^{-19t^{*}})`}
            </InlineMath>
          </div>
        </>
      ) : (
        <div className="flex flex-row justify-left gap-3">
          <InlineMath>{`k=1`}</InlineMath>
        </div>
      )}
    </>
  );
}

function NotEqualTMaxTLim({
  calculation,
}: {
  calculation: ParametricFireCalcSheet;
}) {
  return (
    <div className="mt-5 flex flex-col gap-3">
      <div className="flex flex-row justify-left gap-3">
        <InlineMath>{`t_{max} \\ne t_{lim}`}</InlineMath>
      </div>
      <div className="flex flex-row justify-left gap-3">
        <InlineMath>{`\\Gamma = [O/b]^{2} / (0.04/1160)^{2} = [${calculation.geometry.o.value} / ${calculation.geometry.b.value}]^{2} / (0.04/1160)^{2} = ${calculation.gamma}`}</InlineMath>
      </div>
      <div className="flex flex-row justify-left gap-3">
        <InlineMath>{`t^{*} = t \\cdot \\Gamma`}</InlineMath>
      </div>
    </div>
  );
}
