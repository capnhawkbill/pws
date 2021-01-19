<template>
  <section v-if="errored">
    <p>Error met API request. Weet je zeker dat deze klas bestaat?</p>
  </section>

  <section v-else>
    <div v-if="loading">Loading...</div>
  
    <div id="class">
      {{ klasinfo }}
    </div>
  </section>
</template>

<script>
export default {
  data () {
    return {
      klasinfo: null,
      loading: true,
      errored: false
    }
  },
  mounted () {
    this.axios
      .get('/api/class/leaderboard?class=' + this.$route.params.id, {'headers': {'Authorization': document.cookie}})
      .then(response => (this.klasinfo = response.data))
      .catch(error => {
        console.log(error)
        this.errored = true
      })
      .finally(() => this.loading = false)
  }
}

</script>
