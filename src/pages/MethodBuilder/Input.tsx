import React, { useState } from "react";

export default function Input({
  className,
  defaultValue,
  label,
  updateFunction,
  id,
}: {
  className?: string;
  defaultValue?: string;
  label?: string;
  id: string;
  updateFunction?: (_id: string, _value: string) => void;
}) {
  const [error, setError] = useState<string | null>(null);

  const baseClasses =
    "block w-full rounded-md border-0 py-1.5 px-3 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6";
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
          defaultValue={defaultValue}
          type="text"
          className={`${baseClasses} ${errorClasses}`}
          onChange={(e) => {
            if (updateFunction) {
              updateFunction(id, e.target.value);
            }
          }}
        />
      </div>
      {error && (
        <p className="mt-1 ml-1 text-sm text-red-600" id="email-error">
          <>{error}</>
        </p>
      )}
    </div>
  );
}
