<template>
  <div class="dnut">
    <Doughnut :data="data" :options="options"/>
  </div>
</template>

<script setup>
import {ArcElement, Tooltip, Legend, Chart as chartJS, Title} from 'chart.js';
import {Doughnut} from 'vue-chartjs';
import colors from 'vuetify/util/colors';

chartJS.register(ArcElement, Tooltip, Legend, Title);

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

const data = {
  labels: ['Flyweight', 'Mediumweight', 'Heavyweight'],
  datasets: [
    {
      backgroundColor: [colors.green.darken1, colors.blue.darken1, colors.red.darken1],
      data: [30, 20, 100]
    }
  ],
  hoverOffset: 4
}
</script>

<style scoped>
.dnut {
  position: relative;
  height: 40vh;
  width: 80vw;
}
</style>