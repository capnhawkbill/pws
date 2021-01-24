<template>
  <section v-if="errored">
    <p>Error met API request</p>
  </section>

  <section v-else>
    <div v-if="loading">Laden...</div>
    <div class="profiel">
      <h1>Profiel van {{ naam }}</h1>
      {{huiswerk}}
    </div>
  </section>
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
      errored: false
    }
  },
  mounted () {
    const auth = this.$cookie.getCookie('student_auth')
    console.log(auth)
    this.axios
      .get('/api/student/info', {"headers": {"Authorization": auth}})
      .then(response => {
        this.naam = response.data.name
        this.klassen = response.data.classes
        this.huiswerk = response.data.homework
        this.badges = response.data.badges
        this.punten = response.data.points
      })
      .catch(error => {
        console.log(error)
        this.errored = true
        this.$router.push('/leerling/login')
      })
      .finally(() => this.loading = false)
    }
  }
</script>
