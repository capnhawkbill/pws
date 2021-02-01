<template>
  <div v-if="loading">Loading...</div>
  
  <h1>{{ klasnaam }}</h1>
  <div class="container">
    <h4 v-if="this.homework.length > 0">Huiswerk</h4>
    <table class="homework" v-if="this.homework.length > 0">
      <thead>
        <tr>
          <th v-for="(column, index) in this.homeworkvcolumns" :key="index">{{column}}</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="homework in this.sortedhomework" :key="homework.date">
          <td v-for="(column, indexColumn) in this.homeworkcolumns" :key="indexColumn">{{homework[column]}}</td>
          <td v-if="this.isDone(homework['id'])"><a @click="toggleHomework(homework['id'])">Niet klaar?</a></td>
          <td v-else><a @click="toggleHomework(homework['id'])">Klaar?</a></td>
        </tr>
      </tbody>
    </table>
    <h4>Leaderboard</h4>
    <table class="class">
      <thead>
          <tr>
              <th v-for="(column, index) in this.klasvcolumns" :key="index"> {{column}}</th>
          </tr>
      </thead>
      <tbody>
          <tr v-for="(item, index) in this.klasleaderboard" :key="index">
              <td>{{index+1}}</td>
              <td v-for="(column, indexColumn) in this.klascolumns" :key="indexColumn">{{item[column]}}</td>
          </tr>
      </tbody>
    </table>
  </div>
</template>

<script>
export default {
  data () {
    return {
      klasnaam : '',
      klasvcolumns: ['Rang', 'Naam', 'Punten'],
      klascolumns: ['name', 'points'],
      klasleaderboard: null,
      homeworkvcolumns: ['Titel', 'Datum', 'Beschrijving', 'Punten', 'Acties'],
      homeworkcolumns: ['name', 'date', 'description', 'points'],
      homework: [],
      leerling: null,
      loading: true,
    }
  },
  methods: {
    isDone (homeworkid) {
      if (this.leerling['homework'].indexOf(homeworkid) >= 0) {
        console.log('true')
        return true
      }
      else {
        console.log('false')
        return false
      }
    },
    toggleHomework (homeworkid) {
      if (this.isDone(homeworkid)) {
        this.axios
        .get('/api/homework/undone?class=' + this.$route.params.id + '&homework=' + homeworkid, {'headers': {'Authorization': this.$cookie.getCookie('student_auth')}})
        .then(() => {
          console.log('Ok')
          this.reload()
        })
        .catch(error => (console.log(error)))
      }
      else {
        this.axios
        .get('/api/homework/done?class=' + this.$route.params.id + '&homework=' + homeworkid, {'headers': {'Authorization': this.$cookie.getCookie('student_auth')}})
        .then(() => {
          console.log('Ok')
          this.reload()
        })
        .catch(error => (console.log(error)))
      }
    },
    reload () {
      if (!this.$cookie.isCookieAvailable('student_auth')) {
        this.$router.push({ name: 'leerling.login', query: { redirect: this.$route.fullPath}})
      }
      else {
        this.homework = []
        const getHomework = (homeworkid) => {
          this.axios
          .get('/api/homework/get?id=' + homeworkid, {'headers': {'Authorization': this.$cookie.getCookie('student_auth')}})
          .then(response => {
          const homework = response.data
          this.homework.push(homework)
          })
          .catch(error => {
            console.log(error)
          })
        }

        this.axios
        .get('/api/class/name?id=' + this.$route.params.id, {'headers': {'Authorization': this.$cookie.getCookie('student_auth')}})
        .then(response => (this.klasnaam = response.data))
        .catch(error => {
          console.log(error)
        })
        this.axios
        .get('/api/class/leaderboard?class=' + this.$route.params.id, {'headers': {'Authorization': this.$cookie.getCookie('student_auth')}})
        .then(response => (this.klasleaderboard = response.data))
        .catch(error => {
          console.log(error)
        })
        this.axios
        .get('/api/student/info', {'headers': {'Authorization': this.$cookie.getCookie('student_auth')}})
        .then(response => (this.leerling = response.data))
        .catch(error => {
          console.log(error)
        })
        this.axios
        .get('/api/homework/get/class?id=' + this.$route.params.id, {'headers': {'Authorization': this.$cookie.getCookie('student_auth')}})
        .then(response => {
          for (let i = 0; i < response.data.length; i++) {
            getHomework(response.data[i])
          }
        })
        .catch(error => {
          console.log(error)
        })
        .finally(() => {
        this.loading = false
        })
      }
    }
  },
  mounted () { 
    this.reload()
  },
  computed: {
    sortedhomework: function() {
      return this.homework.slice().sort((a, b) => (a.date > b.date) ? 1 : -1)
    }
  }
}

</script>
