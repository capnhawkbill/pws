<template>
  <form>
    <h1>Inloggen</h1>
    <div class=form-group>
      <label for="username">Gebruikersnaam</label><br>
      <input type="text" v-model="username" id="username"/><br>
    </div>
    <div class=from-group>
      <label for="password">Wachtwoord</label><br>
      <input type="password" v-model="password" id="password"/><br>
      <span class=error>{{ errormsg }}</span><br>
    </div>
    <div class=form-group> 
      <button class="confirm" type="login" :disabled="disable_button" v-on:click="login">Login</button><br><br>
      <a v-on:click='toAanmelden'>Nog geen account?</a>
    </div>
  </form>
</template>

<script>
export default {
  data() {
    return {
      username: '',
      password: '',
      disable_button: true,
      errormsg: ''
    }
  },
  watch: {
    username: function() {
      if(this.password != '' && this.username != '') {
        this.disable_button = false
      }
      else {
        this.disable_button = true
      }
    },
    password: function() {
      if(this.password != '' && this.username != '') {
        this.disable_button = false
      }
      else {
        this.disable_button = true
      }
    }
  },
  methods: {
    login() {
      const auth = btoa(this.username + ":" + this.password)
      this.axios
        .get('/api/student/info', {'headers': {'Authorization': auth}})
        .then(() => {
          document.cookie = auth
          console.log(this.$route.query.redirect)
          if (this.$route.query.redirect === undefined) {
          this.$router.push('/leerling/profiel')
          }
          else {
          this.$router.push(this.$route.query.redirect)
          }
        })
        .catch(error => {
          console.log(error)
          this.wrongInfo()
      })
    },  
    wrongInfo() {
      this.password = ''
      this.errormsg = 'Inloggegevens verkeerd'
    },
    toAanmelden() {
        this.$router.push({ name: 'leerling.aanmelden', query: { redirect: this.$route.query.redirect}})
    }
  }
}
</script>
