<template>
    <div v-if="loading">Laden...</div>
    <div class="profiel">
      <h1>Profiel van {{ naam }}</h1>
      {{huiswerk}}
    </div>
</template>

<script>
export default {
  name: 'profiel',
  data () {
    return {
      naam: null,
      klassen: null,
      huiswerk: null,
      badges: null,
      punten: null,
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
      .then(response => {
        this.$cookie.setCookie('teacher', response.data)
        this.naam = response.data.name
        this.klassen = response.data.classes
        this.huiswerk = response.data.homework
        this.badges = response.data.badges
        this.punten = response.data.points
      })
      .catch(error => {
        console.log(error)
      })
      .finally(() => this.loading = false)
    }
  }
}
</script>
