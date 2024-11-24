import {
  UseFormRegister,
  useFieldArray,
  Control,
  UseFormWatch,
  FieldErrors,
  UseFormTrigger,
  UseFormGetValues,
  UseFormSetValue,
} from "react-hook-form";
import Table, { TH, TD } from "src/components/form/Table";
import { PlusIcon } from "@heroicons/react/20/solid";
import {
  Card,
  CardHeader,
  CardBody,
  DeleteButton,
  Input,
  Checkbox,
} from "src/components";
import SelectMenu from "src/components/form/SelectMenu";
import { Br187BasicInput } from "src/bindings";

export default function MaterialInput({
  control,
  watch,
  errors,
  // trigger,
  getValues,
  setValue,
}: {
  register: UseFormRegister<Br187BasicInput>;
  trigger: UseFormTrigger<Br187BasicInput>;
  control: Control<Br187BasicInput>;
  watch: UseFormWatch<Br187BasicInput>;
  errors: FieldErrors<Br187BasicInput>;
  getValues: UseFormGetValues<Br187BasicInput>;
  setValue: UseFormSetValue<Br187BasicInput>;
}) {
  const { fields, append, remove } = useFieldArray({
    control,
    name: `exposedSurfaces`,
  });
  const surfaces = watch(`exposedSurfaces`);

  return (
    <>
      <Card>
        <CardHeader>
          <div className="ml-4 mt-4 flex flex-col">
            <div className="flex flex-row items-center">
              <h3 className="text-base font-semibold leading-6 text-gray-900 mr-2">
                View Factors
              </h3>
              <button
                type="button"
                className="relative inline-flex items-center rounded-xl bg-indigo-600 px-1 py-1 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                onClick={() => {
                  append({
                    id: "Surface " + (surfaces?.length + 1 || 1),
                    width: 1,
                    height: 1,
                    distance: 1,
                    additive: true,
                    temperature: 800,
                    emissivity: 1,
                    viewFactorType: "parallelSourceCentreAligned",
                  });
                }}
              >
                <PlusIcon className="h-5 w-5" />
              </button>
            </div>
            <p className="mt-1 text-sm text-gray-500">
              A list of view factors that the receiver is exposed to.
            </p>
          </div>
        </CardHeader>
        <CardBody>
          <Table>
            <thead>
              <tr>
                <TH>ID</TH>
                <TH className="p-10">
                  View Factor <br /> Type
                </TH>
                <TH>
                  Height <br />
                  (m)
                </TH>
                <TH>
                  Width <br />
                  (m)
                </TH>
                <TH>
                  Distance to <br /> receiver (m)
                </TH>
                <TH>
                  Temperature <br />
                  (C)
                </TH>
                <TH>Emissivity</TH>
                <TH>Additive?</TH>
                <TH></TH>
              </tr>
            </thead>
            <tbody className="divide-y divide-gray-300">
              {surfaces &&
                surfaces.map((surface, index) => (
                  <tr key={index}>
                    <TD>
                      <Input
                        name={`exposedSurfaces.${index}.id`}
                        register={control.register}
                        errors={errors}
                        type="text"
                        required
                        className="w-40"
                        validation={(value: string) => {
                          if (
                            getValues().exposedSurfaces.filter(
                              (surface) =>
                                surface.id.toLowerCase() === value.toLowerCase()
                            ).length > 1
                          ) {
                            return "Surface ID must be unique";
                          }
                          return true;
                        }}
                      />
                    </TD>
                    <TD>
                      <SelectMenu
                        name={`exposedSurfaces.${index}.viewFactorType`}
                        control={control}
                        fields={{
                          parallelSourceCentreAligned:
                            "Parallel Source Centre Aligned",
                          parallelSourceCornerAligned:
                            "Parallel Source Corner Aligned",
                          perpendicularSourceCornerAligned:
                            "Perpendicular Source Corner Aligned",
                        }}
                      />
                    </TD>
                    <TD>
                      <Input
                        name={`exposedSurfaces.${index}.height`}
                        register={control.register}
                        errors={errors}
                        type="number"
                        required
                        className="w-20"
                        // validation={greaterThanZero}
                      />
                    </TD>
                    <TD>
                      <Input
                        name={`exposedSurfaces.${index}.width`}
                        register={control.register}
                        errors={errors}
                        type="number"
                        required
                        className="w-20"
                        // validation={greaterThanZero}
                      />
                    </TD>
                    <TD>
                      <Input
                        name={`exposedSurfaces.${index}.distance`}
                        register={control.register}
                        errors={errors}
                        type="number"
                        required
                        className="w-20"
                        // validation={greaterThanZero}
                      />
                    </TD>
                    <TD>
                      <Input
                        name={`exposedSurfaces.${index}.temperature`}
                        register={control.register}
                        errors={errors}
                        type="number"
                        required
                        className="w-20"
                        // validation={greaterThanZero}
                      />
                    </TD>
                    <TD>
                      <Input
                        name={`exposedSurfaces.${index}.emissivity`}
                        register={control.register}
                        errors={errors}
                        type="number"
                        required
                        className="w-20"
                        // validation={greaterThanZero}
                      />
                    </TD>
                    <TD>
                      <Checkbox
                        errors={errors}
                        control={control}
                        name={`exposedSurfaces.${index}.additive`}
                        setValue={setValue}
                        watch={watch}
                        box={false}
                      />
                    </TD>
                    <TD>
                      <div className="flex flex-row flex-wrap">
                        {fields.length > 1 && (
                          <DeleteButton
                            active={true}
                            onClick={() => remove(index)}
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
