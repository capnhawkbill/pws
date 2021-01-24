<script>
export default {
  mounted () {
    const auth = this.$cookie.getCookie('student_auth')
    this.axios
      .get('/api/class/join?id=' + this.$route.params.id, {'headers': {'Authorization': auth}})
      .catch(error => {
        this.$router.push({ name: 'leerling.login', query: { redirect: '/leerling/klassen/join/' + this.$route.params.id}})
        console.log(error)
      })
  }
}
</script>
