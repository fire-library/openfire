"use client";
import TH from "./TH";
import TD from "./TD";
import THead from "./THead";

function classNames(...classes: string[]) {
  return classes.filter(Boolean).join(" ");
}

export default function Table({
  children,
  className,
}: {
  children: React.ReactNode;
  className?: string;
}) {
  return (
    <table
      className={classNames(
        "min-w-full divide-y divide-gray-400",
        className || ""
      )}
    >
      {children}
    </table>
  );
}

export { TH, TD, THead };
