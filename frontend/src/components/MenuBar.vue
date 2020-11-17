<template>
    <div class="menubar">
        <router-link to="/">Home</router-link>
        
    </div>
</template>

<script>
    import Btn from "./Btn.vue"

    export default {
        name: "MenuBar",
        components: {
            Btn
        }
    }
</script>

<style scoped>
    .menubar {
        background: rgb(51, 51, 51);
        color: grey;
        position: fixed;
        width: 100%;
        top: 0%;
        left: 0%;
    }

    .button {
        background: rgb(51, 51, 51);
    }
</style>