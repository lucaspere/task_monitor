/* eslint-disable @typescript-eslint/no-unused-vars */
import * as echarts from "echarts";
import ReactEChartsCore from "echarts-for-react/lib/core";
import { collection, doc, onSnapshot } from "firebase/firestore";
import { useEffect, useState } from "react";
import { sysinfoCollection } from "./firebase";

type EChartsOption = echarts.EChartsOption;

// const bytesToGigabyte = (bytes: number) => bytes / 1024 / 1024 / 1024;

const option: EChartsOption = {
  tooltip: {
    trigger: "axis",
    position: function (pt) {
      return [pt[0], "10%"];
    },
  },
  title: {
    left: "center",
    text: "CPU",
  },
  toolbox: {
    feature: {
      dataZoom: {
        yAxisIndex: "none",
      },
      restore: {},
      saveAsImage: {},
    },
  },
  xAxis: {
    type: "time",
    boundaryGap: ["0%", "100%"],
  },
  yAxis: {
    type: "value",
    name: "%",
    axisLabel: {
      formatter: "{value} %",
    },
  },
  dataZoom: [
    {
      type: "inside",
      start: 0,
      end: 20,
    },
    {
      start: 0,
      end: 20,
    },
  ],
  series: [],
};

/** useMemo*/
export const Charts = () => {
  const [data, setData] = useState<Array<[number, number]>>([]);
  const currentDate = new Date();
  const year = currentDate.getFullYear();
  const month = String(currentDate.getMonth() + 1).padStart(2, "0");
  const day = String(currentDate.getDate()).padStart(2, "0");
  const currentDoc = doc(sysinfoCollection, `${year}-${month}-${day}`);
  const metricCollection = collection(currentDoc, "metrics");

  useEffect(() => {
    onSnapshot(metricCollection, (data) => {
      data.docChanges().forEach((change) => {
        if (change.type === "added") {
          setData((prev) => {
            const metric = change.doc.data();
            const last = prev[prev.length - 1] || [metric.timestamp, 0];
            const newMetric = [last[0] + 1, metric.cpu.usage];
            return [...prev, ...[newMetric]] as Array<[number, number]>;
          });
        }
      });
    });
  }, []);

  return (
    <div id="main" style={{ width: "800px", height: "600px" }}>
      <h1>Charts</h1>

      <ReactEChartsCore
        echarts={echarts}
        option={{
          ...option,
          series: [
            {
              name: "CPU usaged",
              type: "line",
              smooth: true,
              symbol: "none",
              areaStyle: {},
              data: data,
            },
          ],
        }}
      />
    </div>
  );
};
