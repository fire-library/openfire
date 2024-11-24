import {
  Control,
  FieldErrors,
  UseFormTrigger,
  UseFieldArrayRemove,
  UseFormGetValues,
  UseFormSetValue,
  UseFormWatch,
} from "react-hook-form";
import { SuccessButton, CancelButton } from "src/components";
import Dialog from "src/components/Dialog";
import { Br187BasicInput } from "src/bindings";
import DialogInput from "./DialogInput";

export default function MaterialInput({
  // register,
  control,
  errors,
  trigger,
  remove,
  open,
  setOpen,
  index,
  setIndex,
  getValues,
  setValue,
  watch,
}: {
  trigger: UseFormTrigger<Br187BasicInput>;
  getValues: UseFormGetValues<Br187BasicInput>;
  setValue: UseFormSetValue<Br187BasicInput>;
  watch: UseFormWatch<Br187BasicInput>;
  control: Control<Br187BasicInput>;
  errors: FieldErrors<Br187BasicInput>;
  remove: UseFieldArrayRemove;
  open: boolean;
  setOpen: (_open: boolean) => void;
  index: number;
  setIndex: (_index: number) => void;
}) {
  return (
    <Dialog
      open={open}
      width="72"
      onClose={() => setOpen(true)}
      title={"New View Factor"}
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
        <CancelButton
          onClick={() => {
            remove(index);
            setIndex(0);
            setOpen(false);
          }}
        >
          Cancel
        </CancelButton>
      </div>
    </Dialog>
  );
}
