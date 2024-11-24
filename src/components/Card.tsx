import React from "react";

export default function Card({ children }: { children: React.ReactNode }) {
  return (
    <div className="mx-auto w-full rounded-3xl ring-1 ring-gray-200 mt-4 sm:mt-8 lg:mx-0 shadow-lg p-6 bg-white flex-1">
      {children}
    </div>
  );
}
