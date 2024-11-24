import {
  Control,
  FieldErrors,
  UseFormGetValues,
  UseFormSetValue,
  UseFormTrigger,
  UseFormWatch,
} from "react-hook-form";
import { SuccessButton } from "src/components";
import Dialog from "src/components/Dialog";
import { Br187BasicInput } from "src/bindings";
import DialogInput from "./DialogInput";

export default function MaterialInput({
  // register,
  control,
  errors,
  trigger,
  open,
  setOpen,
  index,
  getValues,
  setValue,
  watch,
}: {
  trigger: UseFormTrigger<Br187BasicInput>;
  getValues: UseFormGetValues<Br187BasicInput>;
  setValue: UseFormSetValue<Br187BasicInput>;
  control: Control<Br187BasicInput>;
  errors: FieldErrors<Br187BasicInput>;
  watch: UseFormWatch<Br187BasicInput>;
  open: boolean;
  setOpen: (_open: boolean) => void;
  index: number;
}) {
  return (
    <Dialog
      open={open}
      width="72"
      onClose={() => setOpen(true)}
      title={"Edit View Factor"}
    >
      <DialogInput
        watch={watch}
        setValue={setValue}
        control={control}
        errors={errors}
        index={index}
        getValues={getValues}
      />
      <div className="flex flex-row gap-x-2 justify-end">
        <SuccessButton
          onClick={() => {
            trigger(`exposedSurfaces.${index}`).then((success: boolean) => {
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
