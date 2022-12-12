<script lang="ts">
  import { Router, Route } from "svelte-routing";
  import NewDeck from "./lib/NewDeck.svelte";
  import HomePage from "./lib/HomePage.svelte";
  import NewCard from "./lib/NewCard.svelte";
  import StudyCard from "./lib/StudyCard.svelte";
</script>

<Router>
  <Route path="/" component={HomePage} />
  <Route path="/new-deck" component={NewDeck} />
  <Route path="/new-card" component={NewCard} />
  <Route path="/study-card" component={StudyCard} />
</Router>

<style>
</style>
