<template>
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
    <button class="confirm" type="signup" :disabled="disable_button" v-on:click="signup">Aanmelden</button>
  </form>
</template>

<script>
export default {
  name: "aanmelden",
  data() {
    return {
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
        .post('/api/teacher/signup', data)
        .then(() => {
          const auth = btoa(this.username + ":" + this.password)
          this.$cookie.setCookie('teacher_auth', auth)
          this.$router.push('/leraar/profiel')
        })
        .catch(error => {
          this.used_username()
          console.log(error)
          this.errored = true
      })
    },
    used_username() {
      this.password = ''
      this.passwordcheck = ''
      this.username = ''
      this.message = 'Gebruikersnaam is al in gebruik!'
    }
  }
}
</script>
