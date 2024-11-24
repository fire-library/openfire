import {
  Control,
  FieldErrors,
  UseFormGetValues,
  UseFormSetValue,
  UseFormWatch,
} from "react-hook-form";
import { Input, Checkbox } from "src/components";
import SelectMenu from "src/components/form/SelectMenu";
import { Br187BasicInput } from "src/bindings";

const greaterThanZero = (value: number) => {
  if (value <= 0) {
    return "Value must be greater than 0";
  }
  return true;
};

export default function DialogInput({
  control,
  errors,
  index,
  setValue,
  watch,
  getValues,
}: {
  control: Control<Br187BasicInput>;
  errors: FieldErrors<Br187BasicInput>;
  index: number;
  getValues: UseFormGetValues<Br187BasicInput>;
  setValue: UseFormSetValue<Br187BasicInput>;
  watch: UseFormWatch<Br187BasicInput>;
}) {
  return (
    <div className="flex flex-col gap-y-4 mb-3 sm:w-64 mb-5">
      <Input
        name={`exposedSurfaces.${index}.id`}
        register={control.register}
        errors={errors}
        label="ID"
        type="text"
        required={true}
        validation={(value: string) => {
          if (
            getValues().exposedSurfaces.filter(
              (surface) => surface.id.toLowerCase() === value.toLowerCase()
            ).length > 1
          ) {
            return "Surface ID must be unique";
          }
          return true;
        }}
      />
      <SelectMenu
        label="View Factor Type:"
        name={`exposedSurfaces.${index}.viewFactorType`}
        control={control}
        fields={{
          parallelSourceCentreAligned: "Parallel Source Centre Aligned",
          parallelSourceCornerAligned: "Parallel Source Corner Aligned",
          perpendicularSourceCornerAligned:
            "Perpendicular Source Corner Aligned",
        }}
      />
      <div className="flex gap-2">
        <Input
          name={`exposedSurfaces.${index}.width`}
          register={control.register}
          errors={errors}
          label="Width (m)"
          type="number"
          required={true}
          validation={greaterThanZero}
        />
        <Input
          name={`exposedSurfaces.${index}.height`}
          register={control.register}
          errors={errors}
          label="Height (m)"
          type="number"
          required
          validation={greaterThanZero}
        />
      </div>
      <Input
        name={`exposedSurfaces.${index}.distance`}
        register={control.register}
        errors={errors}
        label="Distance to receiver (m)"
        type="number"
        required
        validation={greaterThanZero}
      />
      <div className="flex gap-2">
        <Input
          name={`exposedSurfaces.${index}.temperature`}
          register={control.register}
          errors={errors}
          label="Temperture (C)"
          type="number"
          required
          validation={greaterThanZero}
        />
        <Input
          name={`exposedSurfaces.${index}.emissivity`}
          register={control.register}
          errors={errors}
          label="Emissivity"
          type="number"
          required
          validation={greaterThanZero}
        />
      </div>
      <Checkbox
        errors={errors}
        control={control}
        name={`exposedSurfaces.${index}.additive`}
        setValue={setValue}
        label="Positive View Factor?"
        watch={watch}
      />
    </div>
  );
}
