import { ParameterType, Parameter } from "./bindings";

export function stringedParam(param: ParameterType): Parameter<string> | null {
  if ("String" in param) {
    return param.String;
  } else if ("Float" in param) {
    const decimalPlaces =
      param.Float.display_options.find((option) => "DecimalPlaces" in option)
        ?.DecimalPlaces || null;

    if (decimalPlaces == null) {
      const p = {
        ...param.Float,
        value: param.Float.value?.toString() || null,
      };
      return p;
    } else {
      const p = {
        ...param.Float,
        value: param.Float.value?.toFixed(decimalPlaces).toString() || null,
      };
      return p;
    }
  } else if ("Bool" in param) {
    const p = { ...param.Bool, value: param.Bool.value?.toString() || null };
    return p;
  } else {
    return null;
  }
}
