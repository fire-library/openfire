import {
  Control,
  FieldErrors,
  UseFormGetValues,
  UseFormTrigger,
} from "react-hook-form";
import { SuccessButton } from "src/components";
import Dialog from "src/components/Dialog";
import { ParametricFireInput } from "src/bindings";
import DialogInput from "./DialogInput";

export default function MaterialInput({
  // register,
  control,
  errors,
  trigger,
  open,
  setOpen,
  materialIndex,
  getValues,
}: {
  trigger: UseFormTrigger<ParametricFireInput>;
  getValues: UseFormGetValues<ParametricFireInput>;
  control: Control<ParametricFireInput>;
  errors: FieldErrors<ParametricFireInput>;
  open: boolean;
  setOpen: (_open: boolean) => void;
  materialIndex: number;
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
      </div>
    </Dialog>
  );
}
