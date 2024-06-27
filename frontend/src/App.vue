<template>
  <div style="margin-bottom: 2ex;">
    <BNavbar toggleable="sm" variant="primary-subtle">
      <BNavbarBrand to="/"><font-awesome-icon icon="home" /> Home</BNavbarBrand>
      <BNavbarToggle target="nav-collapse"/>
      <BCollapse id="nav-collapse" is-nav>
        <BNavbarNav>
          <BNavItemDropdown text='Instructions'>
            <BDropdownItem to="/aboutA">
              <font-awesome-icon icon="puzzle-piece" fixed-width/>
              Track A
            </BDropdownItem>
            <BDropdownItem to="/aboutB">
              <font-awesome-icon icon="database" fixed-width/>
              Track B
            </BDropdownItem>
            <BDropdownItem to="/aboutC">
              <font-awesome-icon icon="code-merge" fixed-width/>
              Track C
            </BDropdownItem>
            <BDropdownDivider />
            <BDropdownItem to="/useful">
              <font-awesome-icon icon="clipboard-question" fixed-width/>
              FAQ
            </BDropdownItem>
          </BNavItemDropdown>
          <BNavItem to="/stats">
            <font-awesome-icon icon="trophy"/>
            Leaderboard
          </BNavItem>
          <BNavItem to="/submission">
            <font-awesome-icon icon="upload"/>
            Submit
          </BNavItem>
        </BNavbarNav>
        <BNavbarNav class="ms-auto">
          <BNavItem to="/login"><font-awesome-icon icon="right-to-bracket"/> Log In</BNavItem>
        </BNavbarNav>
      </BCollapse>
    </BNavbar>
  </div>
  <router-view/>
</template>


<script setup>
import {
  BNavbar,
  BNavbarBrand,
  BNavbarToggle,
  BNavbarNav,
  BNavItem,
  BNavItemDropdown,
  BDropdownItem,
  BCollapse,
  BDropdownDivider
} from "bootstrap-vue-next";
import {FontAwesomeIcon} from "@fortawesome/vue-fontawesome";
</script>
