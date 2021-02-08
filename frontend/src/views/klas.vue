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
        <tr v-for="homework in this.sortedHomework" :key="homework.date">
          <td>
            <router-link :to="{ name: 'huiswerk', params: { hw: homework.id }}">{{homework.name}}</router-link>
          </td>
          <td v-for="(column, indexColumn) in this.homeworkcolumns" :key="indexColumn">
            {{homework[column]}}
          </td>
          <td>
            <a v-if="this.user==='teacher'" @click="removeHomework(homework['id'])">Verwijder</a>
            <a v-if="this.isDone(homework['id']) && this.user==='student'" @click="toggleHomework(homework['id'])">Niet klaar?</a>
            <a v-if="!this.isDone(homework['id']) && this.user==='student'" @click="toggleHomework(homework['id'])">Klaar?</a>
          </td>
        </tr>
      </tbody>
    </table>
    <h4 v-if="this.klasleaderboard.length > 0">Leaderboard</h4>
    <h3 v-else>Deze klas is nog leeg.</h3>
    <table class="class" v-if="this.klasleaderboard.length > 0">
      <thead>
          <tr>
              <th v-for="(column, index) in this.klasvcolumns" :key="index">{{column}}</th>
          </tr>
      </thead>
      <tbody>
          <tr v-for="(item, index) in this.sortedLeaderboard" :key="index">
              <td>{{index+1}}</td>
              <td v-for="(column, indexColumn) in this.klascolumns" :key="indexColumn">{{item[column]}}</td>
          </tr>
      </tbody>
    </table>
    <div v-if="this.user==='teacher'">
      Nodig leerlingen uit met deze link:<br>
      <div style="margin-top: 5px;">
        <input type="text" v-model="this.link" readonly><button class="copy" v-on:click="copyShareLink">Kopieer link</button><br><br>
      </div>
    </div>
    <button v-if="this.user==='teacher'" v-on:click="addHomework">+ Voeg huiswerk toe</button>
  </div>
</template>

<script>
export default {
  data () {
    return {
      klasnaam : '',
      klasvcolumns: ['Rang', 'Naam', 'Punten'],
      klascolumns: ['name', 'points'],
      klasleaderboard: [],
      homeworkvcolumns: ['Titel', 'Datum', 'Beschrijving', 'Punten', 'Acties'],
      homeworkcolumns: ['date', 'description', 'points'],
      homework: [],
      leerling: null,
      link: '',
      loading: true,
    }
  },
  methods: {
    isDone (homeworkid) {
      if (this.user === 'teacher') {
        return false
      }
      if (this.leerling['homework'].indexOf(homeworkid) >= 0) {
        return true
      }
      else {
        return false
      }
    },
    toggleHomework (homeworkid) {
      if (this.isDone(homeworkid)) {
        this.axios
        .get('/api/homework/undone?class=' + this.$route.params.id + '&homework=' + homeworkid, this.auth)
        .then(() => {
          this.reload()
        })
        .catch(error => (console.log(error)))
      }
      else {
        this.axios
        .get('/api/homework/done?class=' + this.$route.params.id + '&homework=' + homeworkid, this.auth)
        .then(() => {
          this.reload()
        })
        .catch(error => (console.log(error)))
      }
    }, 
    removeHomework (huiswerkid) {
      this.axios
      .get('/api/homework/remove?class=' + this.$route.params.id + '&homework=' + huiswerkid, this.auth)
      .then(() => {
        for (let i = 0; i < this.homework.length; i++) {
          if (this.homework['id'] === huiswerkid) {
            this.homework = this.homework.splice(i, 1) //remove homework
          }
        }
      })
      .catch(error => {
        console.log(error)
      })
      this.reload()
    },
    copyShareLink () {
      var dummy = document.createElement("textarea")
      document.body.appendChild(dummy)
      dummy.value = this.link 
      dummy.select()
      document.execCommand("copy")
      document.body.removeChild(dummy)
    },
    addHomework () {
      this.$router.push({ name: 'huiswerk.aanmaken'})
    },
    getHomework (homeworkid) {
      this.axios
      .get('/api/homework/get?id=' + homeworkid, this.auth)
      .then(response => (this.homework.push(response.data)))
      .catch(error => {
        console.log(error)
      })
    },
    getClassName () {
      this.axios
      .get('/api/class/name?id=' + this.$route.params.id, this.auth)
      .then(response => (this.klasnaam = response.data))
      .catch(error => {
        console.log(error)
      })
    },
    getStudent() {
      this.axios
      .get('/api/student/info', this.auth)
      .then(response => (this.leerling = response.data))
      .catch(error => {
        console.log(error)
      })
    },
    getLeaderboard () {
      this.axios
      .get('/api/class/leaderboard?class=' + this.$route.params.id, this.auth)
      .then(response => (this.klasleaderboard = response.data))
      .catch(error => {
        console.log(error)
      })
    },
    getAllHomework () {
      this.axios
      .get('/api/homework/get/class?id=' + this.$route.params.id, this.auth)
      .then(response => {
        for (let i = 0; i < response.data.length; i++) {
          this.getHomework(response.data[i])
        }
      })
      .catch(error => {
        console.log(error)
      })
      .finally(() => this.loading = false)
    },
    reload () {
      if (!this.$cookie.isCookieAvailable(this.user)) {
        this.$router.push({ name: 'login', query: { redirect: this.$route.fullPath}})
      }
      else {
        this.homework = []
        this.klasleaderboard = []
        this.link = window.location.origin + '/#/student/klassen/join/' + this.$route.params.id;

        this.getClassName()
        if (this.user === 'student') {
          this.getStudent()
        }
        this.getLeaderboard()
        this.getAllHomework()
      }
    }
  },
  mounted () {
    this.user = this.$route.params.user
    this.auth = {'headers': {'Authorization': this.$cookie.getCookie(this.user)}}
    this.reload()
  },
  computed: {
    sortedHomework: function() {
      return this.homework.slice().sort((a, b) => (a.date > b.date) ? 1 : -1)
    },
    sortedLeaderboard: function() {
      return this.klasleaderboard.slice().sort((a, b) => (a.points < b.points) ? 1 : -1)
    }
  }
}

</script>
