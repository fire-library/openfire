import React from "react";
import { ParametricFireInput, ParametricFireCalcSheet } from "src/bindings";
import { Card, CardBody, CardHeader } from "src/components";
import { InlineMath } from "react-katex";
import SurfaceCalc, { BWarningPopup } from "./SurfaceCalc";
import WarningAlertOptional from "src/components/alerts/Warning";

export default function FireloadCalcCard({
  input,
  calculation,
}: {
  input: ParametricFireInput;
  calculation: ParametricFireCalcSheet;
}) {
  const b_warning = calculation.geometry.b.warning;

  return (
    <Card>
      <div className="flex justify-center">
        <h2 className="text-2xl font-bold leading-7 sm:tracking-tight">
          B Factor
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
                <InlineMath>{`b`}</InlineMath>
              </td>
              <td className="pl-4">
                <div>
                  b factor of the enclosure (J/m<sup>2</sup>s<sup>1/2</sup>
                  K)
                </div>
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`b_{surface}`}</InlineMath>
              </td>
              <td className="pl-4">
                <div>
                  b factor of a specific surface (J/m<sup>2</sup>s<sup>1/2</sup>
                  K)
                </div>
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`b_{n}`}</InlineMath>
              </td>
              <td className="pl-4">
                <div>
                  b factor of the n<sup>th</sup> layer of an exposed surface
                  (J/m
                  <sup>2</sup>s<sup>1/2</sup>K)
                </div>
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`\\rho`}</InlineMath>
              </td>
              <td className="pl-4">
                <div>
                  density of boundary of enclosure (Kg/m<sup>3</sup>)
                </div>
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`\\rho_{n}`}</InlineMath>
              </td>
              <td className="pl-4">
                <div>
                  density of boundary of the n<sup>th</sup> layer of an exposed
                  surface (Kg/m<sup>3</sup>)
                </div>
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`c`}</InlineMath>
              </td>
              <td className="pl-4">
                specific heat of boundary of enclosure (J/KgK)
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`c_{n}`}</InlineMath>
              </td>
              <td className="pl-4">
                specific heat of the n<sup>th</sup> layer of an exposed surface
                (J/KgK)
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`\\lambda`}</InlineMath>
              </td>
              <td className="pl-4">
                thermal conductivity of boundary of enclosure (W/mK)
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`\\lambda_{n}`}</InlineMath>
              </td>
              <td className="pl-4">
                thermal conductivity of the n<sup>th</sup> layer of an exposed
                surface (W/mK)
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`s_{1}`}</InlineMath>
              </td>
              <td className="pl-4">
                thickness of first layer of the exposed surface (mm)
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`s_{lim}`}</InlineMath>
              </td>
              <td className="pl-4">
                limit thickness of first layer of the exposed surface (mm)
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`t_{max}`}</InlineMath>
              </td>
              <td className="pl-4">time at the end of the heating phase (h)</td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`A_{t}`}</InlineMath>
              </td>
              <td className="pl-4">
                the total surface area of the enclosure, including openings (m
                <sup>2</sup>)
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`A_{v}`}</InlineMath>
              </td>
              <td className="pl-4">
                the total area of the vertical openings on all walls (m
                <sup>2</sup>)
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`A_{surface}`}</InlineMath>
              </td>
              <td className="pl-4">
                the total area of the surface, not including openings (m
                <sup>2</sup>)
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
            <InlineMath>{`b = \\sqrt{\\rho c \\lambda}`}</InlineMath> where
            <InlineMath>{`100 \\le b \\le 2200`}</InlineMath>
            (A.1)
          </div>
          <h3 className="h-3 mt-2 text-sm font-semibold">
            When a surface has multiple layers
          </h3>
          <div className="flex flex-col pl-4 pt-3 gap-2">
            <div className="flex flex-row gap-3 text-sm">
              if:
              <InlineMath>{`b_{1} \\le b_{2},`}</InlineMath>
              <InlineMath>{`b = b_{1}`}</InlineMath>
            </div>
            <div className="flex flex-row gap-3 text-sm">
              else if:
              <InlineMath>{`b_{1} > b_{2},`}</InlineMath>
            </div>
            <div className="flex flex-col pl-4 gap-2">
              <InlineMath>{`s_{lim} = \\sqrt{\\frac{3600t_{max}\\lambda_{1}}{c_{1}\\rho_{1}}}`}</InlineMath>
              <div className="flex flex-row gap-3 text-sm">
                if:
                <InlineMath>{`s_{1} \\ge s_{lim},`}</InlineMath>
                <InlineMath>{`b=b_{1}`}</InlineMath>
              </div>
              <div className="flex flex-row gap-3 text-sm">
                else if:
                <InlineMath>{`s_{1} < s_{lim},`}</InlineMath>
                <InlineMath>{`b=\\frac{s_{1}}{s_{lim}}b_{1} + (1-\\frac{s_{1}}{s_{lim}})b_{2}`}</InlineMath>
              </div>
            </div>
          </div>
          <h3 className="h-3 mt-2 text-sm font-semibold">
            When an enclosure has multiple surfaces
          </h3>
          <div className="flex flex-col pl-4 pt-3 gap-2">
            <InlineMath>{`b=(\\sum(b_{surface}A_{surface})) / (A_{t} - A_{v})`}</InlineMath>
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
        <div className="flex flex-col">
          <div className="grid grid-cols-3">
            {calculation.geometry.surfaces.map((surface, index) => {
              const inputSurface = input.walls[index];
              return (
                <div key={`surface_${index}`} className="flex shrink">
                  <div className="flex flex-col">
                    <h3 className="text-sm font-bold">{surface.name}</h3>
                    <table>
                      <thead>
                        <tr className="mt-4">
                          <th></th>
                          <th></th>
                        </tr>
                      </thead>
                      <tbody>
                        <tr>
                          <td className="flex">
                            <InlineMath>{`A_{${surface.name}}`}</InlineMath>
                          </td>
                          <td className="pl-4">
                            {surface.areaWithOpenings} m<sup>2</sup>
                          </td>
                        </tr>
                        <tr>
                          <td>
                            <InlineMath>{`s_{1}`}</InlineMath>
                          </td>
                          <td className="pl-4">
                            {inputSurface.layers[0].thickness} m
                          </td>
                        </tr>
                        {inputSurface.layers.map(
                          (layer, layerIndex: number) => {
                            const material = input.materials[layer.material];

                            return (
                              <React.Fragment key={layerIndex}>
                                <tr>
                                  <td>
                                    <InlineMath>{`\\rho_{${layerIndex + 1}}`}</InlineMath>
                                  </td>
                                  <td className="pl-4">
                                    {material.density} Kg/m<sup>3</sup>
                                  </td>
                                </tr>
                                <tr>
                                  <td>
                                    <InlineMath>{`c_{${layerIndex + 1}}`}</InlineMath>
                                  </td>
                                  <td className="pl-4">
                                    {material.specificHeat} J/KgK
                                  </td>
                                </tr>
                                <tr>
                                  <td>
                                    <InlineMath>{`\\lambda_{${layerIndex + 1}}`}</InlineMath>
                                  </td>
                                  <td className="pl-4">
                                    {material.thermalConductivity} W/mK
                                  </td>
                                </tr>
                              </React.Fragment>
                            );
                          }
                        )}
                      </tbody>
                    </table>
                  </div>
                </div>
              );
            })}
          </div>
          <div className="flex shrink mt-5">
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
                  <td className="pl-4">{calculation.tMax}</td>
                </tr>
              </tbody>
            </table>
          </div>
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
        <div className="flex flex-col gap-6">
          {input.walls &&
            input.walls.map((surface, index) => {
              const b_factor_surface = calculation.geometry.surfaces[index];
              return (
                <SurfaceCalc
                  key={index}
                  materials={input.materials}
                  output_surface={b_factor_surface}
                  surface={surface}
                  tMax={calculation.tMax}
                />
              );
            })}
        </div>

        <div className="text-sm font-bold mt-5">B Factor of Enclosure</div>
        <div className="flex flex-col gap-3 pt-3">
          <InlineMath>{`b=(\\sum(b_{surface}A_{surface})) / (A_{t} - A_{v})`}</InlineMath>
          <InlineMath>{`b=(${input.walls
            .map((wall, index) => {
              return `(${calculation.geometry.surfaces[index].b.value} \\cdot ${calculation.geometry.surfaces[index].areaWithoutOpenings})`;
            })
            .join(
              " + "
            )}) / (${calculation.geometry.totalSurfaceArea} - ${calculation.geometry.areaOfVerticalOpenings})`}</InlineMath>
          <WarningAlertOptional
            warning={!!b_warning}
            popupHeading="Out Of Range"
            popupContent={b_warning && <BWarningPopup warning={b_warning} />}
          >
            <InlineMath>{`b = ${calculation.geometry.b.value}`}</InlineMath>
          </WarningAlertOptional>
        </div>
      </CardBody>
    </Card>
  );
}
