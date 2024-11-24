import { Fragment, useState } from "react";
import { PlusIcon } from "@heroicons/react/20/solid";
import {
  Control,
  UseFormWatch,
  useFieldArray,
  FieldErrors,
  UseFormSetValue,
  UseFormTrigger,
} from "react-hook-form";
import Table, { TH, TD } from "src/components/form/Table";
import { ParametricFireInput } from "src/bindings";
import EditDialog from "./EditDialog";
import NewDialog from "./NewDialog";
import { Card, CardHeader, DeleteButton, CardBody } from "src/components";

export default function SurfaceInput({
  control,
  watch,
  errors,
  setValue,
  trigger,
}: {
  control: Control<ParametricFireInput>;
  watch: UseFormWatch<ParametricFireInput>;
  errors: FieldErrors<ParametricFireInput>;
  setValue: UseFormSetValue<ParametricFireInput>;
  trigger: UseFormTrigger<ParametricFireInput>;
}) {
  const { append, remove } = useFieldArray({
    control,
    name: `walls`,
  });

  const walls = watch("walls");
  const materials = watch("materials");

  const [newDialogOpen, setNewDialogOpen] = useState(false);
  const [editDialogOpen, setEditDialogOpen] = useState(false);
  const [wallIndex, setWallIndex] = useState(0);

  const openNewWallDialog = (index: number) => {
    setWallIndex(index);
    setNewDialogOpen(true);
  };

  const openEditWallDialog = (index: number) => {
    setWallIndex(index);
    setEditDialogOpen(true);
  };

  return (
    <Fragment>
      <NewDialog
        setWallIndex={setWallIndex}
        remove={remove}
        trigger={trigger}
        control={control}
        watch={watch}
        errors={errors}
        setValue={setValue}
        open={newDialogOpen}
        setOpen={setNewDialogOpen}
        wallIndex={wallIndex}
      />
      <EditDialog
        trigger={trigger}
        control={control}
        watch={watch}
        errors={errors}
        setValue={setValue}
        open={editDialogOpen}
        setOpen={setEditDialogOpen}
        wallIndex={wallIndex}
      />
      <Card>
        <CardHeader>
          <div className="ml-4 mt-4 flex flex-col">
            <div className="flex flex-row items-center">
              <h3 className="text-base font-semibold leading-6 text-gray-900 mr-2">
                Exposed Surfaces
              </h3>
              <button
                type="button"
                className="relative inline-flex items-center rounded-xl bg-indigo-600 px-1 py-1 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                onClick={() => {
                  append({
                    name: "Surface " + ((walls?.length || 0) + 1),
                    isFloor: false,
                    isVertical: true,
                    surfaceArea: 10,
                    openings: [],
                    layers: [
                      {
                        material: 0,
                        thickness: 0.015,
                      },
                    ],
                  });
                  openNewWallDialog(walls?.length || 0);
                }}
              >
                <PlusIcon className="h-5 w-5" />
              </button>
            </div>
            <p className="mt-1 text-sm text-gray-500">
              A list of all of the exposed surfaces in the compartment being
              modelled.
            </p>
          </div>
        </CardHeader>
        <CardBody>
          <div className="py-6 divide-y border-white">
            <div className="flex flex-row flex-wrap divide-x">
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
                    <TH></TH>
                  </tr>
                </thead>
                <tbody className="divide-y divide-gray-300">
                  {walls?.map((wall, index) => {
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
                                    area: {opening.area} m<sup>2</sup> - height:{" "}
                                    {opening.height} m
                                  </div>
                                ))
                              : "-"}
                          </div>
                        </TD>
                        <TD>
                          {wall.layers.length > 0
                            ? wall.layers.map((layer, index) => (
                                <div key={index}>
                                  {layer.thickness} m -{" "}
                                  {materials[layer.material].name}
                                </div>
                              ))
                            : "-"}
                        </TD>
                        <TD>
                          <div className="flex flex-row flex-wrap">
                            <button
                              type="button"
                              className="text-indigo-600 hover:text-indigo-900 mr-4"
                              onClick={() => openEditWallDialog(index)}
                            >
                              Edit
                            </button>
                            {(walls?.length || 0) > 1 && (
                              <DeleteButton onClick={() => remove(index)} />
                            )}
                          </div>
                        </TD>
                      </tr>
                    );
                  })}
                </tbody>
              </Table>
            </div>
          </div>
        </CardBody>
      </Card>
    </Fragment>
  );
}
