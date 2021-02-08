<template>
  <div v-if="loading">Laden...</div>
  <div v-else>Je bent toegevoegd aan de klas!</div>
</template>

<script>
export default {
  data () {
    return {
      loading: true
    }
  },
  mounted () {
    this.user = this.$route.params.user
    if (!this.$cookie.isCookieAvailable(this.user)) {
      this.$router.push({ name: 'login', query: { redirect: this.$route.fullPath}})
    }
    else {
    this.axios
      .get('/api/class/join?id=' + this.$route.params.id, {'headers': {'Authorization': this.$cookie.getCookie(this.user)}})
      .catch(error => {
        console.log(error)
      })
      .finally(() => this.loading = false)
    }
  }
}
</script>
