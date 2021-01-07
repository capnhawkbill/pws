<template>
  <div id="login">
    <h1>Inloggen</h1>
    <label for="username">Gebruikersnaam</label><br>
    <input type="text" v-model="username" id="username"/><br>
    <br>
    <label for="password">Wachtwoord</label><br>
    <input type="password" v-model="password" id="password"/><br>
    <br>
    <router-link tag="a" to="/aanmelden">Nog geen account?</router-link>
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
  background: var(--lightgrey);
  box-shadow: 1px 1px 2px grey;
  transition: 0.3s;
}
</style>
