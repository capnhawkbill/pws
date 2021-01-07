<template>
  <div class=wrapper>
    <div id="aanmelden">
      <h1>Aanmelden</h1>
      <label for="username">Gebruikersnaam</label><br>
      <input type="text" v-model="username" id="username"/><br>
      <label for="password">Wachtwoord</label><br>
      <input type="password" v-model="password" id="password"/><br>
      <label for="passwordcheck">Wachtwoord herhalen</label><br>
      <input type="password" v-model="passwordcheck" id="passwordcheck"/><br>
      <span class="error">{{ message }}</span>   
      <button class="signup" type="signup" :disabled="disable_button" v-on:click="signup">Aanmelden</button>
    </div>
  </div>
</template>

<script>
import axios from 'axios'

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
      const headers = {headers: {}}
      axios
        .post('http://localhost:8000/api/student/signup', data, { headers })
        .then(response => (this.apikey = response.apikey))
        .catch(error => {
          console.log(error)
          this.errored = true
      })
    }
  }
}
</script>

<style>
#aanmelden {
  display: inline-block;
  text-align: left;
}

.wrapper {
  text-align: center;
  width: 90%;
  display: inline-block;
  padding: 30px 30px 30px 30px;
  transition: 0.3s;
}

button {
  color: var(--white);
  font-weight: bold;
  background: var(--darkgreen);
  text-align: center;
  width: 100%;
  padding: 10px;
  border-radius: 4px;
  border: none;
  cursor: pointer;  
  transition: 0.3s;
}

button:hover {
  background: var(--brown);
}

button:disabled {
  background: var(--lightgreen);
  cursor: default; 
}

input {
  margin-bottom: 20px;
}

.error {
  font-size: 0.7em;
  font-weight: bold;
  color: var(--darkgreen);
}
</style>
