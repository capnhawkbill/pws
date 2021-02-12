<template>
    <div v-if="loading">Laden...</div>

    <div v-else id="profiel">
      <h1>Profiel van {{ naam }}</h1>
      <div class="container">
        <div>
          <h4>Aantal klassen: {{ klassen.length }}</h4>
          <h4 v-if="user==='student'">Totaal aantal punten: {{ punten }}</h4>
        </div>

        <h4>Klassen</h4>
        <klastabel />
      </div>

      
      <button v-on:click="this.uitloggen()">Uitloggen</button> 
    </div>
</template>

<script>
import klastabel from '@/components/klastabel.vue'

export default {
  name: 'profiel',
  components: {
    klastabel
  },
  data () {
    return {
      user: null,
      naam: null,
      klassen: null,
      huiswerk: null,
      badges: null,
      punten: null,
      loading: true,
    }
  },
  methods: {
    uitloggen() {
      this.$cookie.removeCookie(this.user)
      this.$router.push({ name: 'start'})
    }
  },
  mounted () {
    this.user = this.$route.params.user
    if (!this.$cookie.isCookieAvailable(this.user)) {
      this.$router.push({ name: 'login', query: { redirect: this.$route.fullPath}})
    }
    else {
    this.axios
      .get('/api/' + this.user + '/info', {"headers": {"Authorization": this.$cookie.getCookie(this.user)}})
      .then(response => {
        this.naam = response.data.name
        this.klassen = response.data.classes
        this.badges = response.data.badges
        if (this.user === 'student') {
          this.punten = response.data.points
        }
      })
      .catch(error => {
        console.log(error)
      })
      .finally(() => this.loading = false)
    }
  }
}
</script>
