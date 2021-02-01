<template>
  <div v-if="loading">Loading...</div>
  
  <h1>{{ klasnaam }}</h1>
  <div class="container">
    <table class="homework">
      <thead>
        <tr>
          <th v-for="(column, index) in this.homeworkvcolumns" :key="index">{{column}}</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="homework in this.homework" :key="homework.date">
          <td v-for="(column, indexColumn) in this.homeworkcolumns" :key="indexColumn">{{homework[column]}}</td>
          <td><a @click="removeHomework(homework['id'])">X</a></td>
        </tr>
      </tbody>
    </table>
    <table class="class">
      <thead>
          <tr>
              <th v-for="(column, index) in this.klasvcolumns" :key="index">{{column}}</th>
          </tr>
      </thead>
      <tbody>
          <tr v-for="(item, index) in this.klasleaderboard" :key="index">
              <td v-for="(column, indexColumn) in this.klascolumns" :key="indexColumn">{{item[column]}}</td>
          </tr>
      </tbody>
    </table>
    Nodig leerlingen uit met deze link:
    <button v-on:click="copyShareLink">Kopieer link</button>
    <button v-on:click="addHomework">+ Voeg huiswerk toe</button>
  </div>
</template>

<script>
export default {
  data () {
    return {
      klasnaam : '',
      klasvcolumns: ['Naam', 'Punten'],
      klascolumns: ['name', 'points'],
      klasleaderboard: null,
      homeworkvcolumns: ['Titel', 'Datum', 'Beschrijving', 'Punten', 'Acties'],
      homeworkcolumns: ['name', 'date', 'description', 'points'],
      homework: [],
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
    }
  },
  mounted () {
    if (!this.$cookie.isCookieAvailable('teacher_auth')) {
      this.$router.push({ name: 'leraar.login', query: { redirect: this.$route.fullPath}})
    }
    else {
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
}

</script>
