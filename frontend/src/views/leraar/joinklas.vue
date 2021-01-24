<script>
export default {
  mounted () {
    const auth = this.$cookie.getCookie('teacher_auth')
    this.axios
      .get('/api/class/join?id=' + this.$route.params.id, {'headers': {'Authorization': auth}})
      .catch(error => {
        this.$router.push({ name: 'leraar.login', query: { redirect: window.location}})
        console.log(error)
      })
  }
}
</script>
