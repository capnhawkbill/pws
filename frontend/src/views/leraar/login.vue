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
      <router-link tag="a" to="/leraar/aanmelden">Nog geen account?</router-link>
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
        .get('/api/teacher/info', {'headers': {'Authorization': auth}})
        .then(response => {
          this.$cookie.setCookie('teacher_auth', auth)
          this.$cookie.setCookie('teacher', response.data)
          if (this.$route.query.redirect === undefined) {
          this.$router.push('/leerling/profiel')
          }
          else {
          this.$router.push(this.$route.query.redirect)
          }
        })
        .catch(error => {
          console.log(error)
          this.wrong_info()
      })
    },  
    wrong_info() {
      this.password = ''
      this.errormsg = 'Inloggegevens verkeerd'
    }
  }
}
</script>
