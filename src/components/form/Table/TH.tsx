"use client";

export default function TH({
  children,
  colSpan,
  rowSpan,
  className,
}: {
  children?: React.ReactNode;
  colSpan?: number;
  rowSpan?: number;
  className?: string;
}) {
  const classes =
    "py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-3" +
    (className ? " " + className : "");
  return (
    <th scope="col" className={classes} colSpan={colSpan} rowSpan={rowSpan}>
      {children}
    </th>
  );
}
