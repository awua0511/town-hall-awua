<script lang="ts">
  import './layout.css';
  import favicon from '$lib/assets/favicon.svg';
  import { Navbar } from '$lib/components/navbar';

  let { data, children } = $props();

  const me = $derived(data.me);
</script>

<svelte:head>
  <title>Town Hall</title>
  <link rel="icon" href={favicon} />
</svelte:head>

<div class="mx-auto max-w-7xl px-6">
  <Navbar {me} />
  {@render children()}
</div>
