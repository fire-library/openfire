import React from "react";
import { ParametricFireInput, ParametricFireCalcSheet } from "src/bindings";
import { Card, CardBody, CardHeader } from "src/components";
import { InlineMath } from "react-katex";
import WarningAlert from "src/components/alerts/Warning";

export default function FireloadCalcCard({
  input,
  calculation,
}: {
  input: ParametricFireInput;
  calculation: ParametricFireCalcSheet;
}) {
  const q_td_warning = calculation.fireload.qTd.warning;
  const o_warning = calculation.geometry.o.warning;

  return (
    <Card>
      <div className="flex justify-center">
        <h2 className="text-2xl font-bold leading-7 sm:tracking-tight">
          Time at the End of the Heating Phase
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
                <InlineMath>{`t_{max}`}</InlineMath>
              </td>
              <td className="pl-4">time at the end of the heating phase (h)</td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`t_{lim}`}</InlineMath>
              </td>
              <td className="pl-4">
                time at the end of the heating phase, as dictated by the fire
                growth rate (h)
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`q_{t,d}`}</InlineMath>
              </td>
              <td className="pl-4">
                the design value of the fire load density related to the total
                surface area At of the enclosure (MJ/m<sup>2</sup>)
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`q_{f,d}`}</InlineMath>
              </td>
              <td className="pl-4">
                the design fire load density (MJ/m<sup>2</sup>)
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`A_{f}`}</InlineMath>
              </td>
              <td className="pl-4">
                compartment floor area (m<sup>2</sup>)
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`A_{t}`}</InlineMath>
              </td>
              <td className="pl-4">
                total area of enclosure (m<sup>2</sup>)
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`A_{v}`}</InlineMath>
              </td>
              <td className="pl-4">
                total area of the vertical openings (m<sup>2</sup>)
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`O`}</InlineMath>
              </td>
              <td className="pl-4">
                opening factor (m<sup>1/2</sup>)
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`h_{eq}`}</InlineMath>
              </td>
              <td className="pl-4">
                weighted average window heights on all walls (m<sup>2</sup>)
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
            <InlineMath>{`t_{max} = max[(0.2\\cdot10^{-3}\\cdot q_{t,d}/O ; t_{lim})]`}</InlineMath>{" "}
            (A.7)
          </div>
          <div className="flex flex-row gap-3 text-sm">
            <InlineMath>{`q_{t,d} = q_{f,d} \\cdot A_{f} / A_{t}`}</InlineMath>
          </div>
          <div className="flex flex-row gap-3 text-sm">
            <InlineMath>{`O = A_{v}\\sqrt{h_{eq}}/A_{t}`}</InlineMath>
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
        <div className="flex flex-col sm:flex-row sm:gap-16">
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
                  <InlineMath>{`q_{f,d}`}</InlineMath>
                </td>
                <td className="pl-4">
                  {calculation.fireload.qFd} MJ/m
                  <sup>2</sup>
                </td>
              </tr>
              <tr>
                <td>
                  <InlineMath>{`A_{f}`}</InlineMath>
                </td>
                <td className="pl-4">
                  {calculation.geometry.floorArea} m<sup>2</sup>
                </td>
              </tr>
              <tr>
                <td>
                  <InlineMath>{`A_{t}`}</InlineMath>
                </td>
                <td className="pl-4">
                  {calculation.geometry.totalSurfaceArea} m<sup>2</sup>
                </td>
              </tr>
              <tr>
                <td>
                  <InlineMath>{`A_{v}`}</InlineMath>
                </td>
                <td className="pl-4">
                  {calculation.geometry.areaOfVerticalOpenings} m<sup>2</sup>
                </td>
              </tr>
              <tr>
                <td>
                  <InlineMath>{`h_{eq}`}</InlineMath>
                </td>
                <td className="pl-4">{calculation.geometry.hEq} m</td>
              </tr>
              <tr>
                <td>
                  <InlineMath>{`t_{lim}`}</InlineMath>
                </td>
                <td className="pl-4">
                  {calculation.tLim} h (Based on the fire growth rate provided:{" "}
                  {input.fireLoad.growthRate.toUpperCase()})
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </CardBody>
      <CardHeader>
        <div className="ml-4 mt-4">
          <h3 className="text-base font-semibold leading-6 text-gray-900">
            Calculation
          </h3>
        </div>
      </CardHeader>
      <CardBody>
        <div className="flex flex-col gap-3">
          <WarningAlert
            warning={!!q_td_warning}
            popupHeading="Out Of Range"
            popupContent={
              q_td_warning && (
                <div className="flex flex-col gap-2">
                  <p>
                    <InlineMath>{`q_{t,d}`}</InlineMath> is required to be
                    within the range{" "}
                    <InlineMath>{`${q_td_warning.min} \\le q_{t,d} \\le ${q_td_warning.max} \\space [MJ/m^{2}]`}</InlineMath>{" "}
                    to be within the limits of applicability of this method.
                  </p>
                </div>
              )
            }
          >
            <div className={"flex flex-row gap-3 text-sm"}>
              <InlineMath>{`q_{t,d} = q_{f,d} \\cdot A_{f} / A_{t} = ${calculation.fireload.qFd} \\cdot ${calculation.geometry.floorArea} / ${calculation.geometry.totalSurfaceArea} = ${calculation.fireload.qTd.value} MJ/m^{2}`}</InlineMath>
            </div>
          </WarningAlert>

          <WarningAlert
            warning={!!o_warning}
            popupHeading="Out Of Range"
            popupContent={
              o_warning && (
                <div className="flex flex-col gap-2">
                  <p>
                    <InlineMath>{`O`}</InlineMath> is required to be within the
                    range{" "}
                    <InlineMath>{`${o_warning.min} \\le q_{t,d} \\le ${o_warning.max} \\space [m^{1/2}]`}</InlineMath>{" "}
                    to be within the limits of applicability of this method.
                  </p>
                </div>
              )
            }
          >
            <div className="flex flex-row gap-3 text-sm">
              <InlineMath>{`O = A_{v}\\sqrt{h_{eq}}/A_{t} = ${calculation.geometry.areaOfVerticalOpenings} \\sqrt{${calculation.geometry.hEq}}/${calculation.geometry.totalSurfaceArea} = ${calculation.geometry.o.value}m^{1/2}`}</InlineMath>
            </div>
          </WarningAlert>
          <div className="flex flex-row gap-3 text-sm">
            <InlineMath>{`t_{max} = max[(0.2\\cdot10^{-3}\\cdot ${calculation.fireload.qTd.value}/${calculation.geometry.o.value} ; ${calculation.tLim})] = ${calculation.tMax} h`}</InlineMath>
          </div>
        </div>
      </CardBody>
    </Card>
  );
}
