<template>
  <router-view />
</template>

<style>
#app {
  --lightgrey: #f6f6f6;
  --darkgrey: #32382E;
  --darkgreen: #129490; 
  --lightgreen: #8DD9D9;
  --faintgreen: #D1F0F0;
  --white: #FFFFFF;
  --brown: #B9440E;
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: var(--darkgrey);
}

::selection {
    color: var(--white);
    background-color: var(--darkgreen);
}

button {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  color: var(--white);
  font-weight: bold;
  background: var(--darkgreen);
  text-align: center;
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

button.copy {
  padding: 3px 5px 3px 5px;
  border-radius: 0px;
  font-weight: normal;
}

#menu-item {
  display: inline-block;
  padding: 1em 1em;
  font-size: 1em;
  font-weight: 700;
  letter-spacing: 0.02em;
  line-height: 1;
  text-decoration: none;
  cursor: pointer;
  transition: 0.3s;
}

.center {
  padding-top: 20%;
}

nav {
  position: fixed;
  width: 100%;
  top: 0%;
  left: 0%;
  background: var(--lightgrey);
}

nav a.router-link-exact-active {
  color: var(--brown);
  cursor: default;
}

main {
  margin-top: 100px;
}

a {
  font-weight: bold;
  color: var(--darkgreen);
  text-decoration: none;
  transition: 0.3s;
}
a:hover {
  color: var(--brown);
}

.left {
  margin-left: 20px;
  position: fixed;
  float: left;
}

.right {
  margin-right: 20px;
  position: fixed;
  float: right;
}

form {
  display: inline-block;
  padding: 30px 30px 30px 30px;
  text-align: left;
  transition: 0.3s;
}

form .form-group {
  margin-bottom: 20px;
}

span.error {
  font-size: 0.7em;
  font-weight: bold;
  color: var(--darkgreen);
}

button.confirm {
  width: 100%;
}

.container {
  width: 80%;
  margin: 0 auto 0 auto;
}

table {
  text-align: left;
  border: 1px solid #cccccc;
  border-collapse: collapse;
  margin: 0 auto 20px auto;
  width: 100%;
  background: var(--lightgrey);
}

th {
  border: 1px solid #cccccc;
  background: var(--faintgreen);
  padding-left: 5px; 
}

td {
  border: 1px solid #cccccc;
  padding-left: 5px; 
}

tr {
  border: 1px solid #cccccc;
}

tr:nth-child(odd) {
  background: var(--lightgrey)
}

tr:nth-child(even) {
  background: var(--faintgreen)
}

h4 {
  text-align: left;
  margin-block-end: 0.5em;
}
</style>
