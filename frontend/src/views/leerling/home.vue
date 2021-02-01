<template>
    <div v-if="loading">Loading...</div>
  
    <div id="class">
      {{ klasinfo }}
    </div>
</template>

<script>
export default {
  name: 'classes',
  data () {
    return {
      klasinfo: null,
      loading: true,
    }
  },
  mounted () {
    if (!this.$cookie.isCookieAvailable('student_auth')) {
      this.$router.push({ name: 'leerling.login', query: { redirect: this.$route.fullPath}})
    }
    else {
    this.axios
      .get('/api/student/info', {"headers": {"Authorization": this.$cookie.getCookie('student_auth')}})
      .then(response => (this.klassen = response.data.classes))
      .catch(error => {
        console.log(error)
      })
      .finally(() => this.loading = false)
    }
  }
}
</script>
