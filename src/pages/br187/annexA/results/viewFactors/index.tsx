import { Fragment } from "react";
import { Br187BasicOutput, SurfaceOutput, Tab } from "src/bindings";
import { Card, CardBody, CardHeader } from "src/components";
import { InlineMath } from "react-katex";
import ParallelSourceCenterAligned from "./ParallelSourceCenterAligned";
import ParallelSourceCornerAligned from "./ParallelSourceCornerAligned";
import PerperndicularSourceCornerAligned from "./PerpendicularSourceCornerAligned";

export default function ViewFactors({ tab }: { tab: Tab }) {
  const data = tab.state as Br187BasicOutput;

  return (
    <Card>
      <div className="flex justify-center">
        <h2 className="text-2xl font-bold leading-7 sm:tracking-tight">
          View Factors
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
                <InlineMath>{`W`}</InlineMath>
              </td>
              <td className="pl-4">width of radiating surface (m)</td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`H`}</InlineMath>
              </td>
              <td className="pl-4">height of radiating surface (m)</td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`s`}</InlineMath>
              </td>
              <td className="pl-4">distance to receiver (m)</td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`\\phi`}</InlineMath>
              </td>
              <td className="pl-4">view factor</td>
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
        <div className="my-4">
          <h3 className="text-base font-semibold leading-6 text-gray-900">
            Parallel Source Centre Aligned
          </h3>
        </div>
        <div className="flex flex-col gap-3">
          <div className="flex flex-row gap-3">
            <InlineMath>{`X = W/2s`}</InlineMath>{" "}
          </div>
          <div className="flex flex-row gap-3">
            <InlineMath>{`Y = H/2s`}</InlineMath>{" "}
          </div>
          <div className="flex flex-row gap-3 text-lg">
            <InlineMath>{`\\phi = \\frac{2}{\\pi}(\\frac{X}{\\sqrt{1+X^{2}}}\\tan^{-1}\\frac{Y}{\\sqrt{1+X^{2}}} + \\frac{Y}{\\sqrt{1+Y^{2}}}\\tan^{-1}\\frac{X}{\\sqrt{1+Y^{2}}})`}</InlineMath>
          </div>
        </div>
        <div className="my-4">
          <h3 className="text-base font-semibold leading-6 text-gray-900">
            Parallel Source Corner Aligned
          </h3>
        </div>
        <div className="flex flex-col gap-3">
          <div className="flex flex-row gap-3">
            <InlineMath>{`X = W/s`}</InlineMath>{" "}
          </div>
          <div className="flex flex-row gap-3">
            <InlineMath>{`Y = H/s`}</InlineMath>{" "}
          </div>
          <div className="flex flex-row gap-3 text-lg">
            <InlineMath>{`\\phi = \\frac{1}{2\\pi}(\\frac{X}{\\sqrt{1+X^{2}}}\\tan^{-1}\\frac{Y}{\\sqrt{1+X^{2}}} + \\frac{Y}{\\sqrt{1+Y^{2}}}\\tan^{-1}\\frac{X}{\\sqrt{1+Y^{2}}})`}</InlineMath>
          </div>
        </div>
        <div className="my-4">
          <h3 className="text-base font-semibold leading-6 text-gray-900">
            Perpendicular Source Corner Aligned
          </h3>
        </div>
        <div className="flex flex-col gap-3">
          <div className="flex flex-row gap-3">
            <InlineMath>{`X = W/s`}</InlineMath>{" "}
          </div>
          <div className="flex flex-row gap-3">
            <InlineMath>{`Y = H/s`}</InlineMath>{" "}
          </div>
          <div className="flex flex-row gap-3 text-lg">
            <InlineMath>{`\\phi = \\frac{1}{2\\pi}(\\tan^{-1}(X) - \\frac{1}{\\sqrt{Y^{2} + 1}}\\tan^{-1}\\frac{X}{\\sqrt{Y^{2} + 1}})`}</InlineMath>
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
        <div className="grid grid-cols-2 sm:grid-cols-3 gap-4">
          {data.input.exposedSurfaces.map((surface) => (
            <Fragment key={surface.id}>
              <table>
                <thead>
                  <tr className="mt-4">
                    <th></th>
                    <th></th>
                  </tr>
                </thead>
                <tbody>
                  <tr className="flex flex-shrink">
                    <td>
                      <InlineMath>{`W_{${surface.id}}`}</InlineMath>
                    </td>
                    <td className="pl-4">{surface.width} m</td>
                  </tr>
                  <tr className="flex flex-shrink">
                    <td>
                      <InlineMath>{`H_{${surface.id}}`}</InlineMath>
                    </td>
                    <td className="pl-4">{surface.height} m</td>
                  </tr>
                  <tr className="flex flex-shrink">
                    <td>
                      <InlineMath>{`s_{${surface.id}}`}</InlineMath>
                    </td>
                    <td className="pl-4">{surface.distance} m</td>
                  </tr>
                </tbody>
              </table>
            </Fragment>
          ))}
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
        <div className="flex flex-col gap-7">
          {data.input.exposedSurfaces.map((surface) => {
            const surfaceCalc = data.calculation.surfaces.find(
              (s) => s.id === surface.id
            ) as SurfaceOutput;
            return (
              <Fragment key={surface.id}>
                {surface.viewFactorType === "parallelSourceCentreAligned" && (
                  <ParallelSourceCenterAligned
                    surfaceCalc={surfaceCalc}
                    surfaceInput={surface}
                  />
                )}
                {surface.viewFactorType === "parallelSourceCornerAligned" && (
                  <ParallelSourceCornerAligned
                    surfaceCalc={surfaceCalc}
                    surfaceInput={surface}
                  />
                )}
                {surface.viewFactorType ===
                  "perpendicularSourceCornerAligned" && (
                  <PerperndicularSourceCornerAligned
                    surfaceCalc={surfaceCalc}
                    surfaceInput={surface}
                  />
                )}
              </Fragment>
            );
          })}
        </div>
      </CardBody>
    </Card>
  );
}
