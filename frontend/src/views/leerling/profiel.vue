<template>
    <div v-if="loading">Laden...</div>

    <div class="profiel">
      <h1>Profiel van {{ naam }}</h1>
      <button v-on:click="this.uitloggen()">Uitloggen</button> 
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
  methods: {
    uitloggen () {
      this.$cookie.removeCookie('student_auth')
      this.$router.push('/')
    }
  },
  mounted () {
    if (!this.$cookie.isCookieAvailable('student_auth')) {
      this.$router.push({ name: 'leerling.login', query: { redirect: this.$route.fullPath}})
    }
    else {
    this.axios
      .get('/api/student/info', {"headers": {"Authorization": this.$cookie.getCookie('student_auth')}})
      .then(response => {
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
