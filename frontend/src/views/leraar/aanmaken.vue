<template>
  <form>
    <h1>Klas aanmaken</h1>
    <div class=form-group>
      <label for="name">Klasnaam</label><br>
      <input type="text" v-model="name" id="name"/><br>
    </div>
    <button class="confirm" :disabled="!name.length > 0" v-on:click="create">Aanmaken</button>
  </form>
</template>

<script>
export default {
  name: "aanmelden",
  data() {
    return {
      name: '',
      message: '',
      disable_button: true
    }
  },
  methods: {
    create () {
      const auth = this.$cookie.getCookie('teacher_auth')
      this.axios
        .get('/api/class/create?name=' + this.name, {'headers': {'Authorization': auth}})
        .then(response => {
          const id = response.data
          this.$router.push('/leraar/klassen/' + id)
        })
        .catch(error => {
          console.log(error)
          this.errored = true
      })
    }
  }
}
</script>
