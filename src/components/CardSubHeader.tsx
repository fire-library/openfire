export default function CardSubHeader({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div className="bg-white px-4 pt-4 sm:px-6">
      <div className="-ml-4 -mt-4 flex flex-wrap items-center justify-between sm:flex-nowrap">
        {children}
      </div>
    </div>
  );
}
