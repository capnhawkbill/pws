<template>
    <div v-if="loading">Loading...</div>
  
    <h1>Jouw klassen</h1>
    <div class="container">
      <table class="class">
        <thead>
          <tr>
            <th v-for="(column, index) in this.columns" :key="index"> {{column}}</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(klas, index) in this.klasinfo" :key="index">
            <td><router-link :to="{ name: 'leerling.klas', params: { id: klas.code }}">{{klas.name}}</router-link></td>
          </tr>
        </tbody>
      </table>
    </div>
</template>

<script>
export default {
  data () {
    return {
      columns: ['Naam'],
      klasinfo: [],
      loading: false,
    }
  },
  mounted () {
    if (!this.$cookie.isCookieAvailable('student_auth')) {
      this.$router.push({ name: 'leerling.login', query: { redirect: this.$route.fullPath}})
    }
    else {
      const getName = (klascode) => {
        this.axios
        .get('/api/class/name?id=' + klascode, {'headers': {'Authorization': this.$cookie.getCookie('student_auth')}})
        .then(response => {
        const klas = {'name': response.data, 'code': klascode}
        this.klasinfo.push(klas)
        })
        .catch(error => {
          console.log(error)
        })
      }

      this.axios
      .get('/api/student/info', {'headers': {'Authorization': this.$cookie.getCookie('student_auth')}})
      .then(response => {
        for (var i = 0; i < response.data.classes.length; i++) {
          getName(response.data.classes[i])
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
