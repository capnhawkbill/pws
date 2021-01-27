<template>
  <section v-if="errored">
    <p>Error with API request.</p>
  </section>

  <section v-else>
    <div v-if="loading">Loading...</div>
  
      <h1>Jouw klassen</h1>
      <table class="class">
        <thead>
          <tr>
            <th v-for="(column, index) in this.columns" :key="index"> {{column}}</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(item, index) in this.klasinfo" :key="index">
            <router-link tag='td' to='/leerling/klas/123'>{{item}}</router-link>
          </tr>
        </tbody>
      </table>
      <button v-on:click="this.$router.push('/leraar/klassen/aanmaken')">+ Maak klas aan</button>
  </section>
</template>

<script>
export default {
  data () {
    return {
      columns: ['Naam'],
      klasinfo: ['V5a', 'H1b', 'V4c', 'V3a'],
      loading: false,
      errored: false
    }
  },
  mounted () {
    if (!this.$cookie.isCookieAvailable('teacher_auth')) {
      this.$router.push({ name: 'leraar.login', query: { redirect: this.$route.fullPath}})
    }
    else {
      this.axios
      .get('/api/teacher/info', {'headers': {'Authorization': this.$cookie.getCookie('teacher_auth')}})
      .then(response => {
        const teacher = response.data 
        console.log(teacher)
        for (var i = 0; i < teacher.classes.length; i++) {
          console.log(teacher.classes[i])
          this.axios
          .get('/api/class/name?id=' + teacher.classes[i], {'headers': {'Authorization': this.$cookie.getCookie('teacher_auth')}})
          .then(response => {
            console.log(response.data)
          })
          .catch(error => {
            console.log(error)
            //this.errored = true
          })
        }
      })
      .catch(error => {
        console.log(error)
        this.errored = true
      })
      .finally(() => this.loading = false)
    }
  }
}

</script>

<style>
table {
  border: 1px solid #cccccc;
  border-collapse: collapse;
  margin: 0 auto 20px auto;
  width: 80%;
  background: var(--lightgrey);
}

tr th {
  border: 1px solid #cccccc;
  background: var(--faintgreen)
}

tr {
  border: 1px solid #cccccc;
}

tr:nth-child(odd) {
  background: var(--lightgrey)
}

tr:nth-child(even) {
  background: var(--faintgreen)
}
</style>
