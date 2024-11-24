import {
  UseFormSetValue,
  FieldValues,
  FieldErrors,
  FieldPath,
  Control,
  PathValue,
  UseFormWatch,
} from "react-hook-form";
import { useRef } from "react";

export default function Checkbox<T extends FieldValues>({
  name,
  label,
  setValue,
  watch,
  box = true,
}: {
  name: FieldPath<T>;
  setValue: UseFormSetValue<T>;
  errors: FieldErrors<T>;
  label?: React.ReactNode;
  control: Control<T>;
  watch: UseFormWatch<T>;
  box?: boolean;
}) {
  const value = watch(name);
  const checkbox = useRef<HTMLInputElement>(null);
  return (
    <button
      className={`w-full flex flex-row items-center p-4 ${box ? "border rounded-lg shadow-md hover:bg-gray-100 cursor-pointer active:bg-gray-200" : ""}`}
      type="button"
      onClick={(e) => {
        e.stopPropagation();
        setValue(
          name,
          !checkbox.current?.checked as PathValue<T, FieldPath<T>>
        );
      }}
    >
      <input
        type="checkbox"
        className="min-h-4 min-w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-600 mr-3 cursor-pointer"
        onChange={(e) => {
          e.stopPropagation();
          setValue(name, e.target.checked as PathValue<T, FieldPath<T>>);
        }}
        checked={value}
        ref={checkbox}
      />
      <label className="block text-left text-sm leading-6 text-gray-900 cursor-pointer">
        {label}
      </label>
    </button>
  );
}
