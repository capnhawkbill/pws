<template>
    <div v-if="loading">Loading...</div>
  
    <h1>Jouw klassen</h1>
    <div class="container">
      <klastabel />
      <button v-on:click="this.$router.push('/leraar/klassen/aanmaken')">+ Maak klas aan</button>
    </div>
</template>

<script>
import klastabel from '@/components/klastabel.vue'

export default {
  components: {
    klastabel
  }
}

</script>
