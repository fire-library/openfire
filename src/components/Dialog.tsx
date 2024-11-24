import { Fragment, ReactNode } from "react";
import { Dialog as HeroDialog, Transition } from "@headlessui/react";

function classNames(...classes: string[]) {
  return classes.filter(Boolean).join(" ");
}

export default function Dialog({
  children,
  open,
  onClose,
  title,
  width,
}: {
  title?: ReactNode;
  children: ReactNode;
  onClose: () => void;
  open: boolean;
  width?: string;
}) {
  const widthClass = width ? `sm:w-${width}` : "sm:w-fit";

  return (
    <Transition.Root show={open} as={Fragment}>
      <HeroDialog as="div" className="relative z-10" onClose={onClose}>
        <Transition.Child
          as={Fragment}
          enter="ease-out duration-300"
          enterFrom="opacity-0"
          enterTo="opacity-100"
          leave="ease-in duration-200"
          leaveFrom="opacity-100"
          leaveTo="opacity-0"
        >
          <div className="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity" />
        </Transition.Child>

        <div className="fixed inset-0 z-10 w-screen overflow-y-auto">
          <div className="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
            <Transition.Child
              as={Fragment}
              enter="ease-out duration-300"
              enterFrom="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
              enterTo="opacity-100 translate-y-0 sm:scale-100"
              leave="ease-in duration-200"
              leaveFrom="opacity-100 translate-y-0 sm:scale-100"
              leaveTo="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
            >
              <HeroDialog.Panel
                className={classNames(
                  "relative transform rounded-lg bg-white px-4 pb-4 pt-5 text-left shadow-xl transition-all sm:mb-8 sm:mt-20 sm:p-6",
                  widthClass
                )}
              >
                {title && (
                  <div className="w-full text-center p-2">
                    <HeroDialog.Title
                      as="h3"
                      className="text-base font-semibold leading-6 text-gray-900"
                    >
                      {title}
                    </HeroDialog.Title>
                  </div>
                )}
                {children}
              </HeroDialog.Panel>
            </Transition.Child>
          </div>
        </div>
      </HeroDialog>
    </Transition.Root>
  );
}
