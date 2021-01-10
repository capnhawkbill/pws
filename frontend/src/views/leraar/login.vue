<template>
  <div id="login">
    <h1>Inloggen</h1>
    <label for="username">Gebruikersnaam</label><br>
    <input type="text" v-model="username" id="username"/><br>
    <br>
    <label for="password">Wachtwoord</label><br>
    <input type="password" v-model="password" id="password"/><br>
    <br>
    <router-link tag="a" to="/leraar/aanmelden">Nog geen account?</router-link>
  </div>
</template>

<script>
export default {
  name: "login",
  data() {
    return {
      username: null,
      password: null
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
