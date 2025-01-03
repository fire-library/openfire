import { useEffect, useState } from "react";
import Dialog from "src/components/Dialog";
import { useUpdate } from "../../tabs/useUpdate";
import Success from "src/components/Button/Success";
import Cancel from "src/components/Button/Cancel";
import { commands } from "src/bindings";

export default function UserAgreement() {
  const { update, doUpdate } = useUpdate();
  const [skipped, setSkipped] = useState<boolean | null>(null);
  const open = (update?.available || false) && !skipped && skipped != null;

  useEffect(() => {
    if (update?.available) {
      commands.getUpdateSkipped(update.version).then((event) => {
        if (event.status == "ok") {
          setSkipped(event.data);
        }
      });
    }
  }, [update]);

  if (!update || !update.available || skipped) return null;

  return (
    <Dialog title="Update Available" open={open}>
      <div className="max-w-md flex flex-col gap-4 h-100 overflow-y-scroll">
        <div className="max-w-xl">
          A new version of OpenFire (v{update.version}) is available. Would you
          like to update now?
        </div>
      </div>
      <div className="text-center mt-5">
        <Success
          onClick={() => {
            doUpdate();
          }}
        >
          Update now
        </Success>
        <Cancel
          onClick={() => {
            commands.setUpdateSkipped(update.version);
            setSkipped(true);
          }}
        >
          Skip version
        </Cancel>
      </div>
    </Dialog>
  );
}
