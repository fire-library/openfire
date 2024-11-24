export default function CardHeader({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div className="border-b border-gray-200 bg-white px-4 py-4 sm:px-6">
      <div className="-ml-4 -mt-4 flex flex-wrap items-center justify-between sm:flex-nowrap">
        {children}
      </div>
    </div>
  );
}
