<template>
  <h1 class='left' v-if="this.user==='teacher'">Leraar</h1>
  <h1 class='left' v-else>Leerling</h1>
  <form>
    <h1>Aanmelden</h1>
    <div class=form-group>
      <label for="username">Gebruikersnaam</label><br>
      <input type="text" v-model="username" id="username"/><br>
    </div>
    <div class=form-group>
      <label for="password">Wachtwoord</label><br>
      <input type="password" v-model="password" id="password"/><br>
    </div>
    <div class=form-group>
      <label for="passwordcheck">Wachtwoord herhalen</label><br>
      <input type="password" v-model="passwordcheck" id="passwordcheck"/><br>
    <span class="error">{{ message }}</span>   
    </div>
    <button class="confirm" type="submit" :disabled="disable_button" v-on:click="signup">Aanmelden</button>
  </form>
</template>

<script>
export default {
  name: "aanmelden",
  data() {
    return {
      user: null,
      username: '',
      password: '',
      passwordcheck: '',
      message: '',
      disable_button: true
    }
  },
  watch: {
    password: function() {
      this.passwords_match()
    },
    passwordcheck: function() {
      this.passwords_match()
    }
  },
  methods: {
    passwords_match() {
      if(this.password.length != 0 && this.passwordcheck.length != 0) {
        if(this.password === this.passwordcheck) {
          this.message=""
          this.disable_button=false
        }
        else {
          this.message="Wachtwoorden komen niet overeen!"
          this.disable_button=true
        }
      }
      else {
        this.message=""
        this.disable_button=true
      }
    },
    signup() {
      const data = {"username": this.username, "password": this.password}
      this.axios
        .post('/api/' + this.user + '/signup', data)
        .then(() => {
          const auth = btoa(this.username + ":" + this.password)
          this.$cookie.setCookie(this.user, auth)
          this.redirect()
        })
        .catch(error => {
          this.used_username()
          console.log(error)
      })
    },
    used_username() {
      this.password = ''
      this.passwordcheck = ''
      this.username = ''
      this.message = 'Gebruikersnaam is al in gebruik!'
    },
    redirect() {
      if (this.$route.query.redirect === undefined) {
        this.$router.push(this.user + '/profiel')
      }
      else {
        this.$router.push(this.$route.query.redirect)
      }
    }
  },
  mounted() {
    this.user = this.$route.params.user
    if (this.$cookie.isCookieAvailable(this.user)) {
      this.redirect()
    }
  }
}
</script>
