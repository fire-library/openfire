import { Control, FieldErrors, UseFormGetValues } from "react-hook-form";
import { Input } from "src/components";
import { ParametricFireInput } from "src/bindings";

const greaterThanZero = (value: number) => {
  if (value <= 0) {
    return "Value must be greater than 0";
  }
  return true;
};

export default function DialogInput({
  control,
  errors,
  materialIndex,
  getValues,
}: {
  control: Control<ParametricFireInput>;
  errors: FieldErrors<ParametricFireInput>;
  materialIndex: number;
  getValues: UseFormGetValues<ParametricFireInput>;
}) {
  return (
    <div className="flex flex-col gap-y-4 mb-3 sm:w-64 mb-5">
      <Input
        name={`materials.${materialIndex}.name`}
        register={control.register}
        errors={errors}
        label="Material Name"
        type="text"
        required
        validation={(value: string) => {
          if (
            getValues().materials.filter(
              (material) => material.name.toLowerCase() === value.toLowerCase()
            ).length > 1
          ) {
            return "Material name must be unique";
          }
          return true;
        }}
      />
      <Input
        name={`materials.${materialIndex}.density`}
        register={control.register}
        errors={errors}
        label={
          <>
            Density (kg/m<sup>3</sup>)
          </>
        }
        type="number"
        required={true}
        validation={greaterThanZero}
      />
      <Input
        name={`materials.${materialIndex}.specificHeat`}
        register={control.register}
        errors={errors}
        label="Specific heat (J/kgK)"
        type="number"
        required
        validation={greaterThanZero}
      />
      <Input
        name={`materials.${materialIndex}.thermalConductivity`}
        register={control.register}
        errors={errors}
        label="Thermal Conductivity (W/mK)"
        type="number"
        required
        validation={greaterThanZero}
      />
    </div>
  );
}
