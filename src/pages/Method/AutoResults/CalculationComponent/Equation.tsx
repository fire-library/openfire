import { InlineMath } from "react-katex";

export default function ViewFactors({ equation }: { equation: string }) {
  return (
    <div className="flex">
      <InlineMath>{equation}</InlineMath>
    </div>
  );
}
