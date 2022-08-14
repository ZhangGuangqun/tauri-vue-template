<template>
  <div id="app">
    <top-navbar />
    <router-view></router-view>
  </div>
</template>

<script>
import TopNavbar from '@/components/TopNavbar/index.vue';

export default {
  name: "App",
  components: {
    TopNavbar
  },
};
</script>

<style lang="scss">
* {
  -webkit-box-sizing: border-box;
  -moz-box-sizing: border-box;
  box-sizing: border-box;
  -webkit-tap-highlight-color: transparent;
  outline: none;
}

*:not(input, textarea) {
  -webkit-touch-callout: none;
  -webkit-user-select: none;
  user-select: none;
}
</style>
