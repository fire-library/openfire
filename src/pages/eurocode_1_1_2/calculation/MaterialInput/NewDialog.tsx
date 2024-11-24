import {
  Control,
  FieldErrors,
  UseFormTrigger,
  UseFieldArrayRemove,
  UseFormGetValues,
} from "react-hook-form";
import { SuccessButton, CancelButton } from "src/components";
import Dialog from "src/components/Dialog";
import { ParametricFireInput } from "src/bindings";
import DialogInput from "./DialogInput";

export default function MaterialInput({
  // register,
  control,
  errors,
  trigger,
  remove,
  open,
  setOpen,
  materialIndex,
  setMaterialIndex,
  getValues,
}: {
  trigger: UseFormTrigger<ParametricFireInput>;
  getValues: UseFormGetValues<ParametricFireInput>;
  control: Control<ParametricFireInput>;
  errors: FieldErrors<ParametricFireInput>;
  remove: UseFieldArrayRemove;
  open: boolean;
  setOpen: (_open: boolean) => void;
  materialIndex: number;
  setMaterialIndex: (_index: number) => void;
}) {
  return (
    <Dialog open={open} onClose={() => setOpen(true)} title={"Edit Material"}>
      <DialogInput
        control={control}
        errors={errors}
        materialIndex={materialIndex}
        getValues={getValues}
      />
      <div className="flex flex-row gap-x-2 justify-end">
        <SuccessButton
          onClick={() => {
            trigger(`materials.${materialIndex}`).then((success: boolean) => {
              if (success) {
                setOpen(false);
              }
            });
          }}
        >
          Save
        </SuccessButton>
        <CancelButton
          onClick={() => {
            remove(materialIndex);
            setMaterialIndex(0);
            setOpen(false);
          }}
        >
          Cancel
        </CancelButton>
      </div>
    </Dialog>
  );
}
