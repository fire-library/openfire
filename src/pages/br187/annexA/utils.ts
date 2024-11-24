export const viewFactorText = (type: string): string => {
  if (type === "parallelSourceCentreAligned")
    return "Parallel Source Centre Aligned";
  if (type === "parallelSourceCornerAligned")
    return "Parallel Source Corner Aligned";
  if (type === "perpendicularSourceCornerAligned")
    return "Perpendicular Source Corner Aligned";
  return "";
};
