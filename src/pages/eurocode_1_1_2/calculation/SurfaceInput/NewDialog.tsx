import {
  Control,
  FieldErrors,
  UseFormTrigger,
  UseFormWatch,
  UseFormSetValue,
  UseFieldArrayRemove,
} from "react-hook-form";
import { SuccessButton, CancelButton } from "src/components";
import Dialog from "src/components/Dialog";
import { ParametricFireInput } from "src/bindings";
import DialogInput from "./DialogInput";

export default function MaterialInput({
  control,
  errors,
  trigger,
  open,
  setOpen,
  wallIndex,
  watch,
  setValue,
  setWallIndex,
  remove,
}: {
  trigger: UseFormTrigger<ParametricFireInput>;
  control: Control<ParametricFireInput>;
  watch: UseFormWatch<ParametricFireInput>;
  errors: FieldErrors<ParametricFireInput>;
  open: boolean;
  setOpen: (_open: boolean) => void;
  remove: UseFieldArrayRemove;
  wallIndex: number;
  setValue: UseFormSetValue<ParametricFireInput>;
  setWallIndex: (_index: number) => void;
}) {
  return (
    <Dialog open={open} onClose={() => setOpen(true)} title={"Edit Material"}>
      <DialogInput
        control={control}
        errors={errors}
        watch={watch}
        setValue={setValue}
        wallIndex={wallIndex}
      />
      <div className="flex flex-row gap-x-2 justify-end">
        <SuccessButton
          onClick={() => {
            trigger(`walls.${wallIndex}`).then((success: boolean) => {
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
            remove(wallIndex);
            setWallIndex(0);
            setOpen(false);
          }}
        >
          Cancel
        </CancelButton>
      </div>
    </Dialog>
  );
}
