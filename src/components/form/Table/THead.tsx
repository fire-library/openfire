"use client";
import React from "react";

export default function THead({ children }: { children: React.ReactNode }) {
  return <thead>{children}</thead>;
}
