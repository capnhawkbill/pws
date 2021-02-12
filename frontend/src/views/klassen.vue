<template>
    <div v-if="loading">Loading...</div>
  
    <h1>Jouw klassen</h1>
    <div class="container">
      <klastabel />
      <button v-if="this.user==='teacher'" v-on:click="this.$router.push({ name: 'klas.aanmaken' })">+ Maak klas aan</button>
    </div>
</template>

<script>
import klastabel from '@/components/klastabel.vue'

export default {
  components: {
    klastabel
  },
  data () {
    return {
      user: null
    }
  },
  mounted () {
    this.user = this.$route.params.user
  }
}

</script>
