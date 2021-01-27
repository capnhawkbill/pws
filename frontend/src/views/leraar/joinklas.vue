<script>
export default {
  mounted () {
    if (!this.$cookie.isCookieAvailable('teacher_auth')) {
      this.$router.push({ name: 'leraar.login', query: { redirect: this.$route.fullPath}})
    }
    else {
    this.axios
      .get('/api/class/join?id=' + this.$route.params.id, {'headers': {'Authorization': this.$cookie.getCookie('teacher_auth')}})
      .catch(error => {
        console.log(error)
      })
    }
  }
}
</script>
