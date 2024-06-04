<template>
  <div>
    <Scatter :data="data" :options="options" />
  </div>
</template>

<script>
import {
  Chart as ChartJS,
  Legend,
  LinearScale,
  LineElement,
  PointElement,
  Title,
  Tooltip,
} from "chart.js";
import { Scatter } from "vue-chartjs";

ChartJS.register(
  LinearScale,
  PointElement,
  LineElement,
  Tooltip,
  Legend,
  Title,
);

export default {
  name: "App",
  components: {
    Scatter,
  },
  data() {
    return {
      data: {
        datasets: [{ data: [[0, 7]] }],
      },
      options: {
        title: {
          display: true,
          text: "hello",
        },
        axisY: {
          title: "hello",
        },
      },
    };
  },
};
</script>
