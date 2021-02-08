<template>
  <table class="class">
    <thead>
      <tr>
        <th v-for="(column, index) in this.columns" :key="index"> {{column}}</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="(klas, index) in this.klasinfo" :key="index">
        <td><router-link :to="{ name: 'klas', params: { id: klas.code }}">{{klas.name}}</router-link></td>
      </tr>
    </tbody>
  </table>
</template>

<script>
export default {
  name: 'klastabel',
  data () {
    return {
      user: null,
      columns: ['Naam'],
      klasinfo: [],
      loading: false,
    }
  },
  mounted () {
    this.user = this.$route.params.user
    if (!this.$cookie.isCookieAvailable(this.user)) {
      this.$router.push({ name: 'login', query: { redirect: this.$route.fullPath}})
    }
    else {
      const headers = {'headers': {'Authorization': this.$cookie.getCookie(this.user)}}
      const getName = (klascode) => {
        this.axios
        .get('/api/class/name?id=' + klascode, headers)
        .then(response => {
        const klas = {'name': response.data, 'code': klascode}
        this.klasinfo.push(klas)
        })
        .catch(error => {
          console.log(error)
        })
      }

      this.axios
      .get('/api/' + this.user + '/info', headers)
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
