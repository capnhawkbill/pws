<template>
    <div v-if="loading">Loading...</div>
  
    <div v-else id="home">
      <div class='container'>
        <h1>Huiswerk</h1>

        <div>
          <h4 v-if="recenthomework.length > 0">Huiswerk voor de komende week:</h4>
          <h3 v-else>Er is geen huiswerk voor de komende week!</h3>
          <table class="homework" v-if="recenthomework.length > 0">
            <thead>
              <tr>
                <th v-for="(column, index) in this.homeworkvcolumns" :key="index">{{column}}</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="homework in this.recenthomework" :key="homework.date">
                <td>
                  <router-link :to="{ name: 'huiswerk', params: { id: homework.class, hw: homework.id }}">{{homework.name}}</router-link>
                </td>

                <td v-for="(column, indexColumn) in this.homeworkcolumns" :key="indexColumn">
                  {{homework[column]}}
                </td>

                <td>
                  <a v-if="this.isDone(homework['id']) && user==='student'" @click="toggleHomework(homework['id'], homework['class'])">Niet klaar? - </a>
                  <a v-if="!this.isDone(homework['id']) && user==='student'" @click="toggleHomework(homework['id'], homework['class'])">Klaar? - </a>
                  <router-link :to="{ name: 'klas', params: { id: homework.class }}">Zie klas</router-link>
                </td>

              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
</template>

<script>
export default {
  data () {
    return {
      auth: null,
      user: null,
      homeworkcolumns: ['date', 'description', 'points'],
      homeworkvcolumns: ['Titel', 'Datum', 'Beschrijving', 'Punten', 'Acties'],
      leerling: null,
      homework: [],
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
    toggleHomework (homeworkid, classid) {
      if (this.isDone(homeworkid)) {
        this.axios
        .get('/api/homework/undone?class=' + classid + '&homework=' + homeworkid, this.auth)
        .then(() => {
          this.reload()
        })
        .catch(error => (console.log(error)))
      }
      else {
        this.axios
        .get('/api/homework/done?class=' + classid + '&homework=' + homeworkid, this.auth)
        .then(() => {
          this.reload()
        })
        .catch(error => (console.log(error)))
      }
    }, 
    getHomework (homeworkid, classid) {
      this.axios
      .get('/api/homework/get?id=' + homeworkid, this.auth)
      .then(response => {
        let hw = response.data
        this.homework.push({'id': hw.id, 'name': hw.name, 'date': hw.date, 'description': hw.description, 'points': hw.points, 'class': classid})
      })
      .catch(error => {
        console.log(error)
      })
    },
    getAllHomework (classid) {
      this.axios
      .get('/api/homework/get/class?id=' + classid, this.auth)
      .then(response => {
        for (let i = 0; i < response.data.length; i++) {
          this.getHomework(response.data[i], classid)
        }
      })
      .catch(error => {
        console.log(error)
      })
      .finally(() => this.loading = false)
    },
    reload () {
      this.user = this.$route.params.user
      this.auth = {'headers': {'Authorization': this.$cookie.getCookie(this.user)}}
      this.homework = []

      if (!this.$cookie.isCookieAvailable(this.user)) {
        this.$router.push({ name: 'login', query: { redirect: this.$route.fullPath}})
      }
      else {
        this.axios
        .get('/api/' + this.user + '/info', {"headers": {"Authorization": this.$cookie.getCookie(this.user)}})
        .then(response => {
          this.leerling = response.data
          for (let i = 0; i < this.leerling.classes.length; i++) {
            this.getAllHomework(this.leerling.classes[i])
          }
          if (this.leerling.classes.length === 0) {
            this.loading = false
          }
        })
        .catch(error => {
          console.log(error)
        })
      }
    }
  },
  mounted () {
    this.reload()
  },
  computed: {
    recenthomework: function () {
      let weekhomework = []
      let today = new Date();
      let nextWeek = new Date(today.getFullYear(), today.getMonth(), today.getDate() + 7);

      for (let i = 0; i < this.homework.length; i++) {
        let homework = this.homework[i]
        let hwdate = new Date(homework.date.replace(/-/g,"/"))
        if (hwdate <= nextWeek && hwdate >= today) {
          
          weekhomework.push(homework)
        }
      }
      return weekhomework.slice().sort((a, b) => (a.date > b.date) ? 1 : -1)
    }
  }
}
</script>
