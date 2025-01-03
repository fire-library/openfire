import { useEffect, useState } from "react";
import Dialog from "src/components/Dialog";
import { useUpdate } from "../tabs/useUpdate";
import Success from "src/components/Button/Success";
import Cancel from "src/components/Button/Cancel";
import { relaunch } from "@tauri-apps/plugin-process";

export default function UserAgreement() {
  const { awaitingRestart } = useUpdate();
  const [skippedRestart, setSkippedRestart] = useState(false);
  const [open, setOpen] = useState(false);

  useEffect(() => {
    if (awaitingRestart && !skippedRestart) {
      setOpen(true);
    } else {
      setOpen(false);
    }
  }, [awaitingRestart, skippedRestart]);

  if (awaitingRestart && !skippedRestart)
    return (
      <Dialog title="Restart Required" open={open}>
        <div className="max-w-md flex flex-col gap-4 h-100 overflow-y-scroll">
          <div className="max-w-xl">
            OpenFire has been updated. To apply the changes, you need to restart
            the application.
          </div>
        </div>
        <div className="text-center mt-5">
          <Success onClick={relaunch}>Restart now</Success>
          <Cancel onClick={() => setSkippedRestart(true)}>Restart later</Cancel>
        </div>
      </Dialog>
    );

  return null;
}
