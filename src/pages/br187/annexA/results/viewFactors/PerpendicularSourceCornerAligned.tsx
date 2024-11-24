import React from "react";
import { ExposedSurfaceInput, SurfaceOutput } from "src/bindings";
import { InlineMath } from "react-katex";

export default function ParallelSourceCenterAligned({
  surfaceInput,
  surfaceCalc,
}: {
  surfaceInput: ExposedSurfaceInput;
  surfaceCalc: SurfaceOutput;
}) {
  return (
    <div className="flex flex-col gap-3">
      <h1 className="text-lg font-semibold">
        {surfaceCalc.id} - Perpendicular Source Corner Aligned
      </h1>
      <div className="flex flex-row gap-3">
        <InlineMath>{`X = W/s = ${surfaceInput.width} / ${surfaceInput.distance} = ${surfaceCalc.x}`}</InlineMath>{" "}
      </div>
      <div className="flex flex-row gap-3">
        <InlineMath>{`Y = H/s = ${surfaceInput.height} / ${surfaceInput.distance} = ${surfaceCalc.y}`}</InlineMath>{" "}
      </div>
      <div className="flex flex-row gap-3 text-lg">
        <InlineMath>{`\\phi = \\frac{1}{2\\pi}(\\tan^{-1}(X) - \\frac{1}{\\sqrt{Y^{2} + 1}}\\tan^{-1}\\frac{X}{\\sqrt{Y^{2} + 1}})`}</InlineMath>
      </div>
      <div className="flex flex-row gap-3 text-lg">
        <InlineMath>{`\\phi = \\frac{1}{2\\pi}(\\tan^{-1}(${surfaceCalc.x}) - \\frac{1}{\\sqrt{${surfaceCalc.y}^{2} + 1}}\\tan^{-1}\\frac{${surfaceCalc.x}}{\\sqrt{${surfaceCalc.y}^{2} + 1}})`}</InlineMath>
      </div>
      <div className="flex flex-row gap-3 text-lg">
        <InlineMath>{`\\phi = ${surfaceCalc.phi}`}</InlineMath> (
        {surfaceCalc.phi > 0 ? "Positive View Factor" : "Negative View Factor"})
      </div>
    </div>
  );
}
