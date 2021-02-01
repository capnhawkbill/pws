<template>
  <button class='left' v-on:click="create_class">+ Maak klas aan</button>
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
    if (!this.$cookie.isCookieAvailable('teacher_auth')) {
      this.$router.push({ name: 'leraar.login', query: { redirect: this.$route.fullPath}})
    }
    else {
    this.axios
      .get('/api/teacher/info', {"headers": {"Authorization": this.$cookie.getCookie('teacher_auth')}})
      .then(response => (this.klassen = response.data.classes))
      .catch(error => {
        console.log(error)
      })
      .finally(() => this.loading = false)
    }
  }
}
</script>
