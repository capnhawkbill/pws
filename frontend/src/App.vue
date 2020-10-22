<template>
  <header>
    <nav>
      <ul>
        <router-link tag="li" id="button" to="/">Home</router-link>
        <router-link tag="li" id="button" to="/rang">Rang</router-link> 
        <router-link tag="li" id="button" to="/klas">Klas</router-link> 
        <router-link tag="li" id="button" to="/winkel">Winkel</router-link> 
        <router-link tag="li" id="button" to="/profiel">Profiel</router-link> 
        <router-link tag="li" id="button" to="/opties">Opties</router-link>
      </ul>
    </nav>
  </header>
  <main>
    <router-view/>
  </main>
</template>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}

#button {
  display: inline-block;
  margin: 0.5em 0;
  padding: 1em 2em;
  font-size: 1em;
  font-weight: 700;
  letter-spacing: 0.02em;
  line-height: 1;
  text-decoration: none;
  cursor: pointer;
  transition: 0.3s;
}

#button:hover {
  color: #ffffff;
}

nav {
  position: fixed;
  height: 100px;
  width: 100%;
  top: 0%;
  left: 0%;
  background: rgb(53, 53, 53);
}

nav a {
  font-weight: bold;
  color: #888888;
}

nav a.router-link-exact-active {
  color: #c43939;
  cursor: default;
}

main {
  margin-top: 100px;
}
</style>
