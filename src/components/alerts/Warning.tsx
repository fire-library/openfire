import { ReactNode, useState, Fragment } from "react";
import { ExclamationTriangleIcon } from "@heroicons/react/20/solid";
import { Popover, Transition } from "@headlessui/react";

export default function WarningAlertOptional({
  children,
  warning,
  popupHeading,
  popupContent,
}: {
  children: ReactNode;
  warning: boolean;
  popupHeading?: ReactNode;
  popupContent?: ReactNode;
}) {
  const [popoverShowing, setPopoeverShowing] = useState(false);

  if (!warning) {
    return children;
  }

  return (
    <Popover>
      <Popover.Button>
        <div
          data-popover-target="popover-default"
          className="rounded-md bg-yellow-50 p-2"
          onMouseEnter={() => setPopoeverShowing(true)}
          onMouseLeave={() => setPopoeverShowing(false)}
        >
          <div className="flex items-center">
            <div className="flex-shrink-0">
              <ExclamationTriangleIcon
                aria-hidden="true"
                className="h-5 w-5 text-yellow-400"
              />
            </div>
            <div className="ml-3">
              <div className="mt-2 text-sm text-yellow-700">{children}</div>
            </div>
          </div>
        </div>
      </Popover.Button>

      <Transition
        as={Fragment}
        show={popoverShowing}
        enter="transition ease-out duration-200"
        enterFrom="opacity-0 translate-y-1"
        enterTo="opacity-100 translate-y-0"
        leave="transition ease-in duration-150"
        leaveFrom="opacity-100 translate-y-0"
        leaveTo="opacity-0 translate-y-1"
      >
        <Popover.Panel className="absolute z-10 mt-3 max-w-xl">
          <div className="bg-white border border-gray-200 rounded-lg shadow-sm">
            <div className="px-3 py-2 bg-gray-200 border-b border-gray-300 rounded-t-lg">
              <h3 className="font-semibold text-gray-900">{popupHeading}</h3>
            </div>
            <div className="px-3 py-2">{popupContent}</div>
          </div>
        </Popover.Panel>
      </Transition>
    </Popover>
  );
}
