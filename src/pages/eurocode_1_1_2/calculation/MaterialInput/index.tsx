import { useState } from "react";
import {
  UseFormRegister,
  useFieldArray,
  Control,
  UseFormWatch,
  FieldErrors,
  UseFormTrigger,
  UseFormGetValues,
} from "react-hook-form";
import Table, { TH, TD } from "src/components/form/Table";
import { PlusIcon } from "@heroicons/react/20/solid";
import { Card, CardHeader, CardBody, DeleteButton } from "src/components";
import { ParametricFireInput } from "src/bindings";
import EditDialog from "./EditDialog";
import NewDialog from "./NewDialog";

export default function MaterialInput({
  control,
  watch,
  errors,
  trigger,
  getValues,
}: {
  register: UseFormRegister<ParametricFireInput>;
  trigger: UseFormTrigger<ParametricFireInput>;
  control: Control<ParametricFireInput>;
  watch: UseFormWatch<ParametricFireInput>;
  errors: FieldErrors<ParametricFireInput>;
  getValues: UseFormGetValues<ParametricFireInput>;
}) {
  const {
    fields,
    append,
    remove: removeMaterial,
  } = useFieldArray({
    control,
    name: `materials`,
  });
  const materials = watch(`materials`);
  const walls = watch(`walls`);
  const [newDialogOpen, setNewDialogOpen] = useState(false);
  const [editDialogOpen, setEditDialogOpen] = useState(false);
  const [materialIndex, setMaterialIndex] = useState(0);

  const openNewDialog = (index: number) => {
    setMaterialIndex(index);
    setNewDialogOpen(true);
  };
  const openEditDialog = (index: number) => {
    setMaterialIndex(index);
    setEditDialogOpen(true);
  };

  const materialInUse = (index: number): boolean => {
    return walls.some((wall) =>
      wall.layers.some((layer) => layer.material === index)
    );
  };

  return (
    <>
      <Card>
        <CardHeader>
          <div className="ml-4 mt-4 flex flex-col">
            <div className="flex flex-row items-center">
              <h3 className="text-base font-semibold leading-6 text-gray-900 mr-2">
                Materials
              </h3>
              <button
                type="button"
                className="relative inline-flex items-center rounded-xl bg-indigo-600 px-1 py-1 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                onClick={() => {
                  append({
                    name: `Material ${(materials?.length || 0) + 1}`,
                    density: 0,
                    specificHeat: 0,
                    thermalConductivity: 0,
                  });
                  openNewDialog(materials?.length || 0);
                }}
              >
                <PlusIcon className="h-5 w-5" />
              </button>
            </div>
            <p className="mt-1 text-sm text-gray-500">
              A list of all of the materials used in the construction of the
              exposed surfaces.
            </p>
          </div>
        </CardHeader>
        <CardBody>
          <NewDialog
            control={control}
            trigger={trigger}
            errors={errors}
            remove={removeMaterial}
            open={newDialogOpen}
            setOpen={setNewDialogOpen}
            materialIndex={materialIndex}
            setMaterialIndex={setMaterialIndex}
            getValues={getValues}
          />
          <EditDialog
            control={control}
            trigger={trigger}
            errors={errors}
            open={editDialogOpen}
            setOpen={setEditDialogOpen}
            materialIndex={materialIndex}
            getValues={getValues}
          />
          <Table>
            <thead>
              <tr>
                <TH>Name</TH>
                <TH>
                  Density (kg/m<sup>3</sup>)
                </TH>
                <TH>Specific Heat (J/kgK)</TH>
                <TH>Thermal Conductivity (W/mK)</TH>
                <TH></TH>
              </tr>
            </thead>
            <tbody className="divide-y divide-gray-300">
              {materials &&
                materials.map((material, index) => (
                  <tr key={index}>
                    <TD className="max-w-lg">{material?.name}</TD>
                    <TD>{material?.density}</TD>
                    <TD>{material?.specificHeat}</TD>
                    <TD>{material?.thermalConductivity}</TD>
                    <TD>
                      <div className="flex flex-row flex-wrap">
                        <button
                          type="button"
                          className="text-indigo-600 hover:text-indigo-900 mr-4"
                          onClick={() => openEditDialog(index)}
                        >
                          Edit
                        </button>
                        {fields.length > 1 && (
                          <DeleteButton
                            active={!materialInUse(index)}
                            onClick={() =>
                              !materialInUse(index)
                                ? removeMaterial(index)
                                : null
                            }
                          />
                        )}
                      </div>
                    </TD>
                  </tr>
                ))}
            </tbody>
          </Table>
        </CardBody>
      </Card>
    </>
  );
}
