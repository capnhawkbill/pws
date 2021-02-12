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
  data() {
    return {
      name: '',
      message: '',
      disable_button: true
    }
  },
  methods: {
    create () {
      if (!this.$cookie.isCookieAvailable('teacher')) {
        this.$router.push({ name: 'login', query: { redirect: this.$route.fullPath}})
      }
      else {
      this.axios
        .get('/api/class/create?name=' + this.name, {'headers': {'Authorization': this.$cookie.getCookie('teacher')}})
        .then(response => {
          const id = response.data
          this.$router.push({ name: 'klas', params: { id: id }})
        })
        .catch(error => {
          console.log(error)
        })
      }
    }
  }
}
</script>
