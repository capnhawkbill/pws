<script>
export default {
  mounted () {
    this.axios
      .get('/api/class/join?id=' + this.$route.params.id, {'headers': {'Authorization': document.cookie}})
      .catch(error => {
        this.$router.push({ name: 'leerling.login', query: { redirect: '/leerling/klassen/join/' + this.$route.params.id}})
        console.log(error)
      })
  }
}
</script>
