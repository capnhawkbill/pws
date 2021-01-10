<template>
  <div id="login">
    <h1>Inloggen</h1>
    <label for="username">Gebruikersnaam</label><br>
    <input type="text" v-model="username" id="username"/><br>
    <br>
    <label for="password">Wachtwoord</label><br>
    <input type="password" v-model="password" id="password"/><br>
    <button class="login" type="login" :disabled="disable_button" v-on:click="login">Aanmelden</button>
    <router-link tag="a" to="/leerling/aanmelden">Nog geen account?</router-link>
  </div>
</template>

<script>
import axios from 'axios'

export default {
  name: "login",
  data() {
    return {
      username: null,
      password: null
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
      const data = {"username": this.username, "password": this.password}
      axios
        .post('http://localhost:8000/api/student/login', data)
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
#login {
  display: inline-block;
  padding: 30px 30px 30px 30px;
  text-align: left;
  transition: 0.3s;
}
</style>
