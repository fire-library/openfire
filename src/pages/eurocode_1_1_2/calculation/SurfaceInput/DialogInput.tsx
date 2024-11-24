import { PlusIcon } from "@heroicons/react/20/solid";
import {
  Control,
  UseFormWatch,
  useFieldArray,
  FieldErrors,
  UseFormSetValue,
} from "react-hook-form";
import Table, { TH, TD } from "src/components/form/Table";
import { IndexSelectMenu } from "src/components/form/SelectMenu";
import { ParametricFireInput } from "src/bindings";
import { Input, Checkbox, DeleteButton } from "src/components";

export default function InputDialog({
  control,
  watch,
  errors,
  setValue,
  wallIndex,
}: {
  control: Control<ParametricFireInput>;
  watch: UseFormWatch<ParametricFireInput>;
  errors: FieldErrors<ParametricFireInput>;
  setValue: UseFormSetValue<ParametricFireInput>;
  wallIndex: number;
}) {
  const { append: appendLayer } = useFieldArray({
    control,
    name: `walls.${wallIndex}.layers`,
  });

  const { append: appendOpening } = useFieldArray({
    control,
    name: `walls.${wallIndex}.openings`,
  });

  return (
    <div className="flex flex-col gap-y-4">
      <Input
        name={`walls.${wallIndex}.name`}
        register={control.register}
        errors={errors}
        label="Name"
        type="text"
        required
      />
      <Input
        name={`walls.${wallIndex}.surfaceArea`}
        register={control.register}
        errors={errors}
        label={
          <>
            Surface Area (m<sup>2</sup>)
          </>
        }
        type="number"
        required
      />
      <Checkbox
        errors={errors}
        control={control}
        name={`walls.${wallIndex}.isFloor`}
        setValue={setValue}
        watch={watch}
        label="Floor"
      />
      <Checkbox
        errors={errors}
        control={control}
        name={`walls.${wallIndex}.isVertical`}
        setValue={setValue}
        watch={watch}
        label="Vertical surface"
      />
      <Table>
        <thead>
          <tr>
            <TH>Layer</TH>
            <TH>Material</TH>
            <TH>Thickness (m)</TH>
            <TH></TH>
          </tr>
        </thead>
        <tbody className="divide-y divide-gray-300">
          <SurfaceRow
            key={wallIndex}
            wallIndex={wallIndex}
            watch={watch}
            control={control}
            errors={errors}
          />
        </tbody>
      </Table>
      <div className="flex flex-row">
        <button
          type="button"
          className="relative inline-flex items-center rounded-xl bg-indigo-600 px-1 pr-3 py-1 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
          onClick={() => {
            appendLayer({
              material: 0,
              thickness: 0.015,
            });
          }}
        >
          <PlusIcon className="h-5 w-5" /> Layer
        </button>
      </div>

      <Table>
        <thead>
          <tr>
            <TH>Opening</TH>
            <TH>
              Area (m<sup>2</sup>)
            </TH>
            <TH>Height (m)</TH>
            <TH></TH>
          </tr>
        </thead>
        <tbody className="divide-y divide-gray-300">
          <OpeningRow
            key={wallIndex}
            wallIndex={wallIndex}
            watch={watch}
            control={control}
            errors={errors}
          />
        </tbody>
      </Table>
      <div className="flex flex-row">
        <button
          type="button"
          className="relative inline-flex items-center rounded-xl bg-indigo-600 px-1 pr-3 py-1 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
          onClick={() => {
            appendOpening({
              area: 0,
              height: 0,
            });
          }}
        >
          <PlusIcon className="h-5 w-5" /> Opening
        </button>
      </div>
    </div>
  );
}

function SurfaceRow({
  watch,
  wallIndex,
  control,
  errors,
}: {
  watch: UseFormWatch<ParametricFireInput>;
  wallIndex: number;
  control: Control<ParametricFireInput>;
  errors: FieldErrors<ParametricFireInput>;
}) {
  const materials = watch(`materials`);
  const layers = watch(`walls.${wallIndex}.layers`);
  const { remove } = useFieldArray({
    control,
    name: `walls.${wallIndex}.layers`,
  });

  return (
    <>
      {layers.map((layer, layerIndex) => {
        return (
          <tr key={layerIndex}>
            <TD className="max-w-lg">{`${layerIndex + 1}`}</TD>
            <TD className="max-w-lg">
              <IndexSelectMenu
                control={control}
                name={`walls.${wallIndex}.layers.${layerIndex}.material`}
                indexedOptions={materials || []}
              />
            </TD>
            <TD className="max-w-lg">
              <Input
                name={`walls.${wallIndex}.layers.${layerIndex}.thickness`}
                register={control.register}
                errors={errors}
                type="number"
                required
              />
            </TD>
            <TD className="max-w-lg">
              <div className="flex flex-row flex-wrap">
                {layers.length > 1 && (
                  <DeleteButton onClick={() => remove(layerIndex)} />
                )}
              </div>
            </TD>
          </tr>
        );
      })}
    </>
  );
}

function OpeningRow({
  watch,
  wallIndex,
  control,
  errors,
}: {
  watch: UseFormWatch<ParametricFireInput>;
  wallIndex: number;
  control: Control<ParametricFireInput>;
  errors: FieldErrors<ParametricFireInput>;
}) {
  const openings = watch(`walls.${wallIndex}.openings`);
  const { remove } = useFieldArray({
    control,
    name: `walls.${wallIndex}.openings`,
  });

  return (
    <>
      {openings.length > 0 ? (
        openings.map((opening, openingIndex) => {
          return (
            <tr key={openingIndex}>
              <TD className="max-w-lg">{`${openingIndex + 1}`}</TD>
              <TD className="max-w-lg">
                <Input
                  name={`walls.${wallIndex}.openings.${openingIndex}.area`}
                  register={control.register}
                  errors={errors}
                  type="number"
                  required
                />
              </TD>
              <TD className="max-w-lg">
                <Input
                  name={`walls.${wallIndex}.openings.${openingIndex}.height`}
                  register={control.register}
                  errors={errors}
                  type="number"
                  required
                />
              </TD>
              <TD className="max-w-lg">
                <div className="flex flex-row flex-wrap">
                  <DeleteButton onClick={() => remove(openingIndex)} />
                </div>
              </TD>
            </tr>
          );
        })
      ) : (
        <tr>
          <TD className="max-w-lg">-</TD>
          <TD className="max-w-lg">-</TD>
          <TD className="max-w-lg">-</TD>
          <TD className="max-w-lg">-</TD>
        </tr>
      )}
    </>
  );
}
