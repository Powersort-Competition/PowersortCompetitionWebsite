<template>
  <router-view/>
  <nav class="bg-green-lighten-5">
    <router-link to="/">Home</router-link>
    |
    <router-link to="/about">About</router-link>
    |
    <router-link to="/stats">Leaderboard</router-link>
<!--    <router-link to="/trackA">Track A Submission</router-link>-->
    |
    <router-link to="/useful">FAQ</router-link>
    &nbsp; | &nbsp;
    <router-link to="/submission">Submit!</router-link>
<!--    <router-link to="/trackB">Track B Submission</router-link>-->
<!--    |-->
<!--    <router-link to="/trackC">Track C Submission</router-link>-->
    |
    <router-link to="/login">Log In</router-link>
  </nav>
</template>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: left;
  color: #2c3e50;
  margin-top: 5%;
  padding: 2rem;
  overflow: auto;
}

nav {
  padding: 30px;
  position: fixed;
  top: 0;
  left: 0;
  z-index: 999;
  width: 100%;
  height: 23px;
}

nav a {
  font-weight: bold;
  color: #2c3e50;
}

nav a.router-link-exact-active {
  color: #42b983;
}
</style>
