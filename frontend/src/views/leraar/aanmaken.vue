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
      if (!this.$cookie.isCookieAvailable('teacher_auth')) {
        this.$router.push({ name: 'leerling.login', query: { redirect: this.$route.fullPath}})
      }
      else {
      this.axios
        .get('/api/class/create?name=' + this.name, {'headers': {'Authorization': this.$cookie.getCookie('teacher_auth')}})
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
}
</script>
