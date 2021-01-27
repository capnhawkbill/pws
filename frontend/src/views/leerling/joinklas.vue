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
    if (!this.$cookie.isCookieAvailable('student_auth')) {
      this.$router.push({ name: 'leerling.login', query: { redirect: this.$route.fullPath}})
    }
    else {
    this.axios
      .get('/api/class/join?id=' + this.$route.params.id, {'headers': {'Authorization': this.$cookie.getCookie('student_auth')}})
      .catch(error => {
        console.log(error)
      })
      .finally(() => this.loading = false)
    }
  }
}
</script>
