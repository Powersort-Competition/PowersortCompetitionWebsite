<template>
  <div style="margin-bottom: 2ex;">
    <BNavbar toggleable="sm" variant="primary-subtle">
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


<script setup>
import {BNavbar, BNavbarBrand, BNavbarToggle, BNavbarNav, BNavItem, BNavItemDropdown, BDropdownItem, BCollapse} from "bootstrap-vue-next";
</script>
