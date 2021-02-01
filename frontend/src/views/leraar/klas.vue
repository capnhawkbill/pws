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
        <tr v-for="homework in this.homework" :key="homework.date">
          <td v-for="(column, indexColumn) in this.homeworkcolumns" :key="indexColumn">{{homework[column]}}</td>
          <td><a @click="removeHomework(homework['id'])">Verwijder</a></td>
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
          <tr v-for="(item, index) in this.klasleaderboard" :key="index">
              <td>{{index+1}}</td>
              <td v-for="(column, indexColumn) in this.klascolumns" :key="indexColumn">{{item[column]}}</td>
          </tr>
      </tbody>
    </table>
    Nodig leerlingen uit met deze link:<br>
    <div style="margin-top: 5px;">
      <input type="text" v-model="this.link" readonly><button class="copy" v-on:click="copyShareLink">Kopieer link</button><br><br>
    </div>
    <button v-on:click="addHomework">+ Voeg huiswerk toe</button>
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
      homeworkcolumns: ['name', 'date', 'description', 'points'],
      homework: [],
      link: '',
      loading: true,
    }
  },
  methods: {
    removeHomework (huiswerkid) {
      this.axios
      .get('/api/homework/remove?class=' + this.$route.params.id + '&homework=' + huiswerkid, {'headers': {'Authorization': this.$cookie.getCookie('teacher_auth')}})
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
      var dummy = document.createElement("textarea");
      document.body.appendChild(dummy);
      dummy.value = window.location.origin + '/#/leerling/klassen/join/' + this.$route.params.id;
      dummy.select();
      document.execCommand("copy");
      document.body.removeChild(dummy);
    },
    addHomework() {
      this.$router.push({ name: 'leraar.huiswerk.aanmaken'})
    },
    reload () {
      if (!this.$cookie.isCookieAvailable('teacher_auth')) {
        this.$router.push({ name: 'leraar.login', query: { redirect: this.$route.fullPath}})
      }
      else {
        this.homework = []
        this.klasleaderboard = []
        this.link = window.location.origin + '/#/leerling/klassen/join/' + this.$route.params.id;
        const getHomework = (homeworkid) => {
          this.axios
          .get('/api/homework/get?id=' + homeworkid, {'headers': {'Authorization': this.$cookie.getCookie('teacher_auth')}})
          .then(response => {
          const homework = response.data
          this.homework.push(homework)
          })
          .catch(error => {
            console.log(error)
          })
        }

        this.axios
        .get('/api/class/name?id=' + this.$route.params.id, {'headers': {'Authorization': this.$cookie.getCookie('teacher_auth')}})
        .then(response => (this.klasnaam = response.data))
        .catch(error => {
          console.log(error)
        })
        this.axios
        .get('/api/class/leaderboard?class=' + this.$route.params.id, {'headers': {'Authorization': this.$cookie.getCookie('teacher_auth')}})
        .then(response => (this.klasleaderboard = response.data))
        .catch(error => {
          console.log(error)
        })
        this.axios
        .get('/api/homework/get/class?id=' + this.$route.params.id, {'headers': {'Authorization': this.$cookie.getCookie('teacher_auth')}})
        .then(response => {
          for (let i = 0; i < response.data.length; i++) {
            getHomework(response.data[i])
          }
        })
        .catch(error => {
          console.log(error)
        })
        .finally(() => this.loading = false)
      }
    }
  },
  mounted () {
    this.reload()
  }
}

</script>
