import { Parameter } from "../../bindings";

export function parameterValue(param: Parameter): String {
  if (param.value == null) {
    return "";
  }

  if (param.parameter_type == "String") {
    return param.value as string;
  }

  const decimalPlaces =
    param.display_options.find((option) => "DecimalPlaces" in option)
      ?.DecimalPlaces || null;

  if (decimalPlaces == null) {
    return param.value.toString();
  } else {
    return (param.value as number).toFixed(decimalPlaces);
  }
}
