import React, { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import {
  Parameter,
  ValidationErrorEvent,
  commands,
  ParameterValue,
} from "src/bindings";
import { InlineMath } from "react-katex";

const updateField = async (
  id: string,
  value: string | null,
  setError: (_error: string | null) => void
) => {
  commands.updateField(id, value).then((result) => {
    if (result.status == "error") {
      setError(result.error.ValidationError.message);
    } else {
      setError(null);
    }
  });
};

export default function Input({
  className,
  field,
  doQuickCalc,
}: {
  className?: string;
  field: Parameter;
  doQuickCalc?: () => void;
}) {
  const [error, setError] = useState<string | null>(null);
  const [fieldValue, setFieldValue] = useState<string>(
    field.value?.toString() || ""
  );

  useEffect(() => {
    const unlisten = listen(
      "validation-error",
      (event: { event: string; payload: [ValidationErrorEvent] }) => {
        const error = event.payload.find((e) => e.field_id == field.id);
        if (error) {
          setError(error.error);
        } else {
          setError(null);
        }
      }
    );

    return () => {
      unlisten.then((f: () => void) => f());
    };
  }, [field.id]);

  useEffect(() => {
    const unlisten = listen("validation-ok", () => {
      setError(null);
    });

    return () => {
      unlisten.then((f: () => void) => f());
    };
  }, []);

  useEffect(() => {
    async function update() {
      await updateField(field.id, fieldValue, setError);
      doQuickCalc && doQuickCalc();
    }

    update();
  }, [fieldValue]);

  const baseClasses =
    "block w-full rounded-md border-0 py-1.5 px-3 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6";
  const errorClasses = error ? "ring-red-300" : "ring-gray-300";

  return (
    <div className={className || "w-full"}>
      <label className="block text-sm font-medium leading-6 text-gray-900">
        {field.name},{" "}
        {field.id && <InlineMath>{String.raw`${field.id}`}</InlineMath>}{" "}
        {field.units && <InlineMath>{String.raw`(${field.units})`}</InlineMath>}
      </label>
      <div className="mt-2">
        <input
          defaultValue={fieldValue}
          type="text"
          className={`${baseClasses} ${errorClasses}`}
          onChange={(e) => setFieldValue(e.target.value)}
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
