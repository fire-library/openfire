import React, { useRef, useEffect, useMemo } from "react";
import { select, line, scaleLinear, axisBottom, axisLeft } from "d3";

interface LineChartProps {
  time: number[];
  temp: number[];
}

function getMax(arr: number[]) {
  let len = arr.length;
  let max = -Infinity;

  while (len--) {
    max = arr[len] > max ? arr[len] : max;
  }
  return max;
}

const LineChart: React.FC<LineChartProps> = ({ time, temp }) => {
  const data = useMemo(() => {
    return time.map((t, i) => ({ x: t * 3600, y: temp[i] }));
  }, [time, temp]);
  const maxTemp = useMemo(() => {
    return Math.ceil(getMax(temp) / 100) * 100;
  }, [temp]);
  const maxTime = useMemo(() => {
    return Math.ceil(getMax(time) * 3600) / 60;
  }, [time]);
  const svgRef = useRef(null);

  //draws chart
  useEffect(() => {
    const svg = select(svgRef.current).attr("width", 1000).attr("height", 500);

    //scales
    const xScale = scaleLinear().domain([0, maxTime]).range([100, 1000]);

    const yScale = scaleLinear().domain([0, maxTemp]).range([450, 10]);

    const xAxis = axisBottom(xScale);
    svg.append("g").attr("transform", `translate(0,${450})`).call(xAxis);

    const yAxis = axisLeft(yScale);
    svg.append("g").attr("transform", `translate(${100},0)`).call(yAxis);

    //line generator
    const myLine = line<{ x: number; y: number }>()
      .x((d) => xScale(d.x / 60))
      .y((d) => yScale(d.y));

    svg
      .append("path")
      .data([data])
      .attr("class", "line")
      .attr("stroke", "black")
      .attr("stroke-width", 1.5)
      .attr("d", myLine)
      .attr("fill", "none");
  }, [data, maxTemp, maxTime]);

  return <svg ref={svgRef}></svg>;
};

export default LineChart;
