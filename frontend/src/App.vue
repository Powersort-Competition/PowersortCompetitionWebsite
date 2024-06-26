<template>
  <div style="margin-bottom: 2ex;">
    <BNavbar toggleable="sm" variant="primary">
      <BNavbarBrand to="/">Home</BNavbarBrand>
      <BNavbarToggle target="nav-collapse" />
      <BCollapse id="nav-collapse" is-nav>
        <BNavbarNav>
          <BNavItem to="/about">Details</BNavItem>
          <BNavItem to="/useful">FAQ</BNavItem>
          <BNavItem to="/stats">Leaderboard</BNavItem>
          <BNavItem to="/submission">Submit</BNavItem>
        </BNavbarNav>
        <BNavbarNav class="ms-auto">
          <BNavItem to="/login">Log In</BNavItem>
        </BNavbarNav>
      </BCollapse>
    </BNavbar>
  </div>
  <router-view/>
</template>

<style>

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  //text-align: left;
  color: #2c3e50;
  padding: 2rem;
  overflow: auto;
}

nav a {
  color: #2c3e50;
}

nav a.router-link-exact-active {
  font-weight: bold;
}



</style>
<script setup>
import {BNavbar, BNavbarBrand, BNavbarToggle, BNavbarNav, BNavItem, BNavItemDropdown, BDropdownItem, BCollapse} from "bootstrap-vue-next";
</script>
