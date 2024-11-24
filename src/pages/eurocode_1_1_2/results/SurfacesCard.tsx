import { WallInput, MaterialInput } from "src/bindings";
import { Card, CardBody, CardHeader } from "src/components";
import Table, { TH, TD } from "src/components/form/Table";

export default function Results({
  surfaces,
  materials,
}: {
  surfaces: WallInput[];
  materials: MaterialInput[];
}) {
  return (
    <Card>
      <CardHeader>
        <div className="ml-4 mt-4">
          <h3 className="text-base font-semibold leading-6 text-gray-900">
            Exposed Surfaces
          </h3>
          <p className="mt-1 text-sm text-gray-500">
            The exposed surfaces provided for the calculation.
          </p>
        </div>
      </CardHeader>
      <CardBody>
        <Table>
          <thead>
            <tr>
              <TH>Name</TH>
              <TH>
                Surface Area (m<sup>2</sup>)
              </TH>
              <TH>Floor</TH>
              <TH>Vertical Surface</TH>
              <TH>Openings</TH>
              <TH>Layers</TH>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-300">
            {surfaces.map((wall, index) => {
              return (
                <tr key={index}>
                  <TD>{wall.name}</TD>
                  <TD>{wall.surfaceArea}</TD>
                  <TD>{wall.isFloor ? "Yes" : "No"}</TD>
                  <TD>{wall.isVertical ? "Yes" : "No"}</TD>
                  <TD>
                    <div className="flex flex-col">
                      {wall.openings.length > 0
                        ? wall.openings.map((opening, index) => (
                            <div key={index}>
                              area: {opening.area} m<sup>2</sup>, height:{" "}
                              {opening.height} m
                            </div>
                          ))
                        : "-"}
                    </div>
                  </TD>
                  <TD className="flex flex-row">
                    <div className="flex flex-col">
                      {wall.layers.map((layer, index) => (
                        <div key={index}>
                          {layer.thickness} m - {materials[layer.material].name}
                        </div>
                      ))}
                    </div>
                  </TD>
                </tr>
              );
            })}
          </tbody>
        </Table>
      </CardBody>
    </Card>
  );
}
