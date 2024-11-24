export default function TH({
  children,
  colSpan,
  rowSpan,
  group,
  className,
}: {
  children?: React.ReactNode;
  colSpan?: number;
  rowSpan?: number;
  group?: boolean;
  className?: string;
}) {
  const classes =
    (group
      ? "bg-gray-200 py-2 pl-4 pr-3 text-left text-sm font-semibold text-gray-00 sm:pl-3"
      : "px-3 py-4 text-sm text-gray-500") + (className ? " " + className : "");
  return (
    <td className={classes} colSpan={colSpan} rowSpan={rowSpan}>
      {children}
    </td>
  );
}
