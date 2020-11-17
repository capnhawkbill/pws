<template>
    <component :is="type" :to="to" class="button">
      <slot/>
    </component>
  </template>
  
  <script>
    export default {
      props: {
        href: {
          type: String,
          default: null
        },
        to: {
          type: String,
          default: null
        }
      },
      computed: {
        type() {
          if (this.to) {
            return 'router-link'
          } else {
            return 'button'
          }
        }
      }
    }
  </script>

<style scoped>
    .button {
      display: inline-block;
      margin: 0.5em 0;
      padding: 1em 2em;
      background: rgb(51, 51, 51);
      border: 2px solid rgb(51, 51, 51);
      border-radius: 3px;
      color: rgb(165, 165, 165);
      font-family: "Quicksand", sans-serif;
      font-size: 1em;
      font-weight: 700;
      letter-spacing: 0.02em;
      line-height: 1;
      text-decoration: none;
      text-transform: uppercase;
      cursor: pointer;
      transition: 0.3s;
    }
  
    .button:hover {
      background: rgb(51, 51, 51);
      color: #fff;
    }
  </style>