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
      const headers = {
        "headers": {
        "Authorization": auth
        }
      }
      this.axios
        .get('/api/teacher/info', headers)
        .then(() => {
          document.cookie = auth
          this.$router.push('/leraar/profiel')
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
