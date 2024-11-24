import {
  UseFormRegister,
  FieldValues,
  FieldErrors,
  FieldPath,
} from "react-hook-form";
import lodash from "lodash";

export default function SelectMenu<T extends FieldValues, P>({
  register,
  name,
  label,
  type,
  disabled,
  errors,
  required = false,
  max,
  min,
  validation,
  className,
}: {
  name: FieldPath<T>;
  required?: boolean;
  disabled?: boolean;
  register: UseFormRegister<T>;
  errors: FieldErrors<T>;
  label?: React.ReactNode;
  type: string;
  max?: number;
  min?: number;
  className?: string;
  validation?: (_value: P) => boolean | string;
}) {
  const error = lodash.get(errors, name);
  const baseClasses = disabled
    ? "block w-full rounded-md border-0 py-1.5 px-3 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6 text-white bg-gray-800"
    : "block w-full rounded-md border-0 py-1.5 px-3 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6";

  const errorClasses = error ? "ring-red-300" : "ring-gray-300";
  return (
    <div className={className || "w-full"}>
      {label && (
        <label className="block text-sm font-medium leading-6 text-gray-900">
          {label}
        </label>
      )}
      <div className="mt-2">
        <input
          type={type}
          className={`${baseClasses} ${errorClasses}`}
          step={type === "number" ? "0.01" : undefined}
          {...register(name, {
            ...(type === "number" && { valueAsNumber: true }),
            ...((min || min === 0) && {
              min: { value: min, message: `Must be ${min} or more` },
            }),
            ...((max || max === 0) && {
              max: { value: max, message: `Must be ${max} or less` },
            }),
            ...(required && { required: "This field is required" }),
            ...(validation && { validate: validation }),
          })}
        />
      </div>
      {error && (
        <p className="mt-1 ml-1 text-sm text-red-600" id="email-error">
          <>{error?.message}</>
        </p>
      )}
    </div>
  );
}
