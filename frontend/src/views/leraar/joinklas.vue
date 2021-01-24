<script>
export default {
  mounted () {
    this.axios
      .get('/api/class/join?id=' + this.$route.params.id, {'headers': {'Authorization': document.cookie}})
      .catch(error => {
        this.$router.push({ name: 'leraar.login', query: { redirect: window.location}})
        console.log(error)
      })
  }
}
</script>
