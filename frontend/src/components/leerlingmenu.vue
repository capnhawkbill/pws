<template>
  <header>
    <nav>
      <ol>
        <router-link tag="li" id="menu-item" to="/leerling">Home</router-link>
        <router-link tag="li" id="menu-item" to="/leerling/klassen">Klassen</router-link> 
        <router-link tag="li" id="menu-item" to="/leerling/profiel">Profiel</router-link> 
        <router-link tag="li" id="menu-item" to="/leerling/login">Login</router-link> 
      </ol>
    </nav>
  </header>
  <main>
    <router-view/>
  </main>
</template>

<script>
  export default {
    name: 'leerlingmenu'
  }
</script>
