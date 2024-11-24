import {
  Control,
  FieldErrors,
  UseFormTrigger,
  UseFormWatch,
  UseFormSetValue,
} from "react-hook-form";
import { SuccessButton } from "src/components";
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
}: {
  trigger: UseFormTrigger<ParametricFireInput>;
  control: Control<ParametricFireInput>;
  watch: UseFormWatch<ParametricFireInput>;
  errors: FieldErrors<ParametricFireInput>;
  open: boolean;
  setOpen: (_open: boolean) => void;
  wallIndex: number;
  setValue: UseFormSetValue<ParametricFireInput>;
}) {
  return (
    <Dialog open={open} onClose={() => setOpen(true)} title={"Edit Surface"}>
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
      </div>
    </Dialog>
  );
}
