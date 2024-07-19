<template>
  <div class="dnut">
    <Doughnut :data="data" :options="options"/>
  </div>
</template>

<script setup>
import axios from "axios";
import {ArcElement, Tooltip, Legend, Chart as chartJS, Title} from 'chart.js';
import {Doughnut} from 'vue-chartjs';
import colors from 'vuetify/util/colors';

chartJS.register(ArcElement, Tooltip, Legend, Title);

//const data = ref(null);

const options = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    title: {
      display: true,
      text: 'Number of track A submissions by weight class',
    }
  }
}
</script>

<script>
import {ref} from "vue";

const data = ref(null);
// Get data from backend server.
await axios.get("/composition_track_a")
  .then(response => {
    let _data = {
      labels: ['Flyweight', 'Mediumweight', 'Heavyweight'],
      datasets: [
        {
          backgroundColor: [colors.green.darken1, colors.blue.darken1, colors.red.darken1],
          data: response.data
        }
      ],
    hoverOffset: 4
  }

  data = _data;
})

data = _data;

export default {
  async setup() {
    return {data};
  }
}
</script>

<style scoped>
.dnut {
  position: relative;
  height: 40vh;
  width: 80vw;
}
</style>
