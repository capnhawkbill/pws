<template>
  <div v-if="loading">Loading...</div>

  <div v-else class="container">
    <h1>{{ klasnaam }} - {{ huiswerk.name }}</h1>

    <h4>Datum: {{ huiswerk.date }}</h4>

    <h4 v-if="huiswerk.description">Beschrijving</h4>
    <div class="left">
      {{huiswerk.description}}
    </div><br><br>

    <table class="students" v-if="studentinfo.length > 0">
      <thead>
        <tr>
          <th v-for="(column, index) in this.studentvcolumns" :key="index">{{column}}</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="student in this.sortedStudents" :key="student.name">
          <td v-for="(column, indexColumn) in this.studentcolumns" :key="indexColumn">
            {{student[column]}}
          </td>
          <td v-if="student.homework.includes(huiswerk.id)">Ja</td>
          <td v-else>Nee</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script>
export default {
  data () {
    return {
      auth: null,
      klasnaam: '',
      huiswerknaam: '',
      huiswerk: null,
      studentinfo: [],
      studentvcolumns: ["Naam", "Punten", "Gemaakt?"],
      studentcolumns: ["name", "points"],
    }
  },
  methods: {
    getStudent (studentid) {
      this.axios
      .get('/api/student/id?id=' + studentid, this.auth)
      .then(response => (this.studentinfo.push(response.data)))
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
    getStudents () {
      this.axios
      .get('/api/class/students?id=' + this.$route.params.id, this.auth)
      .then(response => {
        for (let i = 0; i < response.data.length; i++) {
          this.getStudent(response.data[i])
        }
      })
      .catch(error => {
        console.log(error)
      })
      .finally(this.loading = false)
    },
    getHomework() {
      this.axios
      .get('/api/homework/get?id=' + this.$route.params.hw, this.auth)
      .then(response => {
        this.huiswerk = response.data
        this.huiswerknaam = this.huiswerk.name
      })
      .catch(error => {
        console.log(error)
      })
    }
  },
  mounted () {
    this.user = this.$route.params.user
    if (!this.$cookie.isCookieAvailable(this.user)) {
      this.$router.push({ name: 'login', query: { redirect: this.$route.fullPath}})
    }
    else {
      this.auth = {'headers': {'Authorization': this.$cookie.getCookie(this.user)}}

      this.getHomework()    
      this.getClassName()
      this.getStudents()
    }
  },
  computed: {
    sortedStudents: function() {
      return this.studentinfo.slice().sort((a, b) => (a.name > b.name) ? 1 : -1)
    }
  }
}
</script>
