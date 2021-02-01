<template>
  <form>
    <h1>Huiswerk toevoegen</h1>
    <div class=form-group>
      <label for="name">Titel</label><br>
      <input type="text" v-model="name" id="name"/><br>
    </div>
    <div class=form-group>
      <label for="date">Datum</label><br>
      <input type="date" v-model="date" id="date"/><br>
    </div>
    <div class=form-group>
      <label for="info">Beschrijving</label><br>
      <input type="text" v-model="info" id="info"/><br>
    </div>
    <div class=form-group>
      <label for="points">Aantal punten</label><br>
      <input type="number" v-model="points" id="points"/><br>
    </div>
    <button class="confirm" :disabled="!name.length > 0" v-on:click="add">Aanmaken</button>
  </form>
</template>

<script>
export default {
  data() {
    return {
      name: '',
      date: '',
      info: '',
      points: 0,
      message: '',
      disable_button: true
    }
  },
  methods: {
    add () {
      if (!this.$cookie.isCookieAvailable('teacher_auth')) {
        this.$router.push({ name: 'leerling.login', query: { redirect: this.$route.fullPath}})
      }
      else {
      let data = {
        name: this.name,
        date: this.date,
        description: this.info,
        points: Number(this.points)
      }
      this.axios
        .post('/api/homework/add?class=' + this.$route.params.id, data, {'headers': {'Authorization': this.$cookie.getCookie('teacher_auth')}})
        .then(() => {
          this.$router.push('/leraar/klassen/' + this.$route.params.id)
        })
        .catch(error => {
          console.log(error)
        })
      }
    }
  }
}
</script>
