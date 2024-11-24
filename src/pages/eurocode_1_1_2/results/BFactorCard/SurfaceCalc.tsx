import {
  WallInput,
  ParametricFireCalcSheetGeometrySurface,
  MaterialInput,
  FloatWithWarning,
  OutOfRangeError,
} from "src/bindings";
import { InlineMath } from "react-katex";
import WarningAlertOptional from "src/components/alerts/Warning";

export default function FireloadCalcCard({
  materials,
  output_surface,
  surface,
  tMax,
}: {
  materials: MaterialInput[];
  output_surface: ParametricFireCalcSheetGeometrySurface;
  surface: WallInput;
  tMax: number;
}) {
  return (
    <div className="flex flex-col">
      <div className="font-semibold">{surface.name}</div>
      {surface.layers.length > 1 && output_surface?.b2?.value ? (
        <MultiLayer
          materials={materials}
          output_surface={output_surface}
          b2={output_surface.b2}
          surface={surface}
          tMax={tMax}
        />
      ) : (
        <SingleLayer
          materials={materials}
          output_surface={output_surface}
          surface={surface}
        />
      )}
    </div>
  );
}

function SingleLayer({
  materials,
  output_surface,
  surface,
}: {
  materials: MaterialInput[];
  output_surface: ParametricFireCalcSheetGeometrySurface;
  surface: WallInput;
}) {
  const layer_materials = surface.layers.map((layer) => {
    return materials[layer.material];
  });
  const b1_warning = output_surface.b1.warning;

  return (
    <WarningAlertOptional
      warning={!!b1_warning}
      popupHeading="Out Of Range"
      popupContent={
        b1_warning && (
          <BWarningPopup warning={b1_warning} subscript={`${surface.name}`} />
        )
      }
    >
      <InlineMath>{`b_{${surface.name}} =  \\sqrt{\\rho_{1} c_{1} \\lambda_{1}} =  \\sqrt{${layer_materials[0].density} \\cdot ${layer_materials[0].specificHeat} \\cdot ${layer_materials[0].thermalConductivity}} = ${output_surface.b1.value}`}</InlineMath>
    </WarningAlertOptional>
  );
}

export function BWarningPopup({
  warning,
  subscript,
}: {
  warning: OutOfRangeError;
  subscript?: string;
}) {
  return (
    <div className="flex flex-col gap-2">
      <p>
        <InlineMath>{`b_{${subscript || ""}}`}</InlineMath> is required to be
        within the range{" "}
        <InlineMath>{`${warning.min} \\le b_{${subscript || ""}} \\le ${warning.max} \\space [m^{1/2}]`}</InlineMath>{" "}
        to be within the limits of applicability of this method.
      </p>
    </div>
  );
}
function MultiLayer({
  materials,
  output_surface,
  surface,
  b2,
  tMax,
}: {
  materials: MaterialInput[];
  output_surface: ParametricFireCalcSheetGeometrySurface;
  b2: FloatWithWarning;
  surface: WallInput;
  tMax: number;
}) {
  const b1_warning = output_surface.b1.warning;
  const b2_warning = b2.warning;

  return (
    <div className="flex flex-col gap-3">
      <WarningAlertOptional
        warning={!!b1_warning}
        popupHeading="Out Of Range"
        popupContent={
          b1_warning && <BWarningPopup warning={b1_warning} subscript={"1"} />
        }
      >
        <InlineMath>{`b_{1} = \\sqrt{\\rho_{1} c_{1} \\lambda_{1}} =  \\sqrt{${materials[0].density} \\cdot ${materials[0].specificHeat} \\cdot ${materials[0].thermalConductivity}} = ${output_surface.b1.value}`}</InlineMath>
      </WarningAlertOptional>
      <WarningAlertOptional
        warning={!!b2_warning}
        popupHeading="Out Of Range"
        popupContent={
          b2_warning && <BWarningPopup warning={b2_warning} subscript={"2"} />
        }
      >
        <InlineMath>{`b_{2} = \\sqrt{\\rho_{2} c_{2} \\lambda_{2}} =  \\sqrt{${materials[1].density} \\cdot ${materials[1].specificHeat} \\cdot ${materials[1].thermalConductivity}} = ${b2.value}`}</InlineMath>
      </WarningAlertOptional>
      {output_surface.b1.value <= b2.value ? (
        <div className="flex flex-col gap-3">
          <InlineMath>{`b_{1} \\le b_{2}`}</InlineMath>
          <WarningAlertOptional
            warning={!!b1_warning}
            popupHeading="Out Of Range"
            popupContent={
              b1_warning && (
                <BWarningPopup
                  warning={b1_warning}
                  subscript={`${surface.name}`}
                />
              )
            }
          >
            <InlineMath>{`b_{${surface.name}} = ${output_surface.b1.value}`}</InlineMath>
          </WarningAlertOptional>
        </div>
      ) : (
        <div className="flex flex-col gap-3">
          <InlineMath>{`b_{1} > b_{2}`}</InlineMath>
          <HighB1
            materials={materials}
            output_surface={output_surface}
            surface={surface}
            b2={b2}
            tMax={tMax}
          />
        </div>
      )}
    </div>
  );
}

function HighB1({
  materials,
  output_surface,
  surface,
  b2,
  tMax,
}: {
  materials: MaterialInput[];
  output_surface: ParametricFireCalcSheetGeometrySurface;
  b2: FloatWithWarning;
  surface: WallInput;
  tMax: number;
}) {
  const b1_warning = output_surface.b1.warning;
  const b_warning = output_surface.b.warning;
  return (
    <div className="flex flex-col gap-3">
      <InlineMath>{`s_{lim} = \\sqrt{\\frac{3600t_{max}\\lambda_{1}}{c_{1}\\rho_{1}}} = \\sqrt{\\frac{3600 \\cdot ${tMax} \\cdot ${materials[0].thermalConductivity}}{${materials[0].specificHeat} \\cdot ${materials[0].density}}} = ${output_surface.sLim}`}</InlineMath>
      {surface.layers[0].thickness >= output_surface.sLim && (
        <div className="flex flex-col gap-3">
          <InlineMath>{`s_{1} \\ge s_{lim}`}</InlineMath>
          <WarningAlertOptional
            warning={!!b1_warning}
            popupHeading="Out Of Range"
            popupContent={
              b1_warning && (
                <BWarningPopup
                  warning={b1_warning}
                  subscript={`${surface.name}`}
                />
              )
            }
          >
            <InlineMath>{`b_{${surface.name}} = ${output_surface.b1.value}`}</InlineMath>
          </WarningAlertOptional>
        </div>
      )}
      {surface.layers[0].thickness < output_surface.sLim && (
        <div className="flex flex-col gap-3">
          <InlineMath>{`s_{1} < s_{lim}`}</InlineMath>
          <InlineMath>{`b_{${surface.name}} = \\frac{s_{1}}{s_{lim}}b_{1} + (1-\\frac{s_{1}}{s_{lim}})b_{2} = \\frac{${surface.layers[0].thickness}}{${output_surface.sLim}}${output_surface.b1.value} + (1-\\frac{${surface.layers[0].thickness}}{${output_surface.sLim}})${b2.value}`}</InlineMath>
          <WarningAlertOptional
            warning={!!b_warning}
            popupHeading="Out Of Range"
            popupContent={
              b_warning && (
                <BWarningPopup
                  warning={b_warning}
                  subscript={`${surface.name}`}
                />
              )
            }
          >
            <InlineMath>{`b_{${surface.name}} = ${output_surface.b.value}`}</InlineMath>
          </WarningAlertOptional>
        </div>
      )}
    </div>
  );
}
