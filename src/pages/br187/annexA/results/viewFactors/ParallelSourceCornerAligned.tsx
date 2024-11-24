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
        {surfaceCalc.id} - Parallel Source Corner Aligned
      </h1>
      <div className="flex flex-row gap-3">
        <InlineMath>{`X = W/s = ${surfaceInput.width} / ${surfaceInput.distance} = ${surfaceCalc.x}`}</InlineMath>{" "}
      </div>
      <div className="flex flex-row gap-3">
        <InlineMath>{`Y = H/s = ${surfaceInput.height} / ${surfaceInput.distance} = ${surfaceCalc.y}`}</InlineMath>{" "}
      </div>
      <div className="flex flex-row gap-3 text-lg">
        <InlineMath>{`\\phi = \\frac{1}{2\\pi}(\\frac{X}{\\sqrt{1+X^{2}}}\\tan^{-1}\\frac{Y}{\\sqrt{1+X^{2}}} + \\frac{Y}{\\sqrt{1+Y^{2}}}\\tan^{-1}\\frac{X}{\\sqrt{1+Y^{2}}})`}</InlineMath>
      </div>
      <div className="flex flex-row gap-3 text-lg">
        <InlineMath>{`\\phi = \\frac{1}{2\\pi}(\\frac{${surfaceCalc.x}}{\\sqrt{1+${surfaceCalc.x}^{2}}}\\tan^{-1}\\frac{${surfaceCalc.y}}{\\sqrt{1+${surfaceCalc.x}^{2}}} + \\frac{${surfaceCalc.y}}{\\sqrt{1+${surfaceCalc.y}^{2}}}\\tan^{-1}\\frac{${surfaceCalc.x}}{\\sqrt{1+${surfaceCalc.y}^{2}}})`}</InlineMath>
      </div>
      <div className="flex flex-row gap-3 text-lg">
        <InlineMath>{`\\phi = ${surfaceCalc.phi}`}</InlineMath> (
        {surfaceCalc.phi > 0 ? "Positive View Factor" : "Negative View Factor"})
      </div>
    </div>
  );
}
