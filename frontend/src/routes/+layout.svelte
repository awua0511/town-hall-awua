<script lang="ts">
  import './layout.css';
  import favicon from '$lib/assets/favicon.svg';
  import * as Nav from '$lib/components/navigation';

  let { data, children } = $props();

  const me = $derived(data.me);
</script>

<svelte:head>
  <title>Town Hall</title>
  <link rel="icon" href={favicon} />
</svelte:head>

<div class="mx-auto max-w-7xl px-6">
  <nav class="flex items-center py-6">
    <div class="flex-1">
      <Nav.Logo />
    </div>
    <div class="flex-none">
      <Nav.Links />
    </div>
    <div class="flex flex-1 justify-end gap-4">
      {#if me}
        <Nav.CreateQuest />
        <Nav.PfpMenu user={me} />
      {:else}
        <Nav.GithubLogin />
      {/if}
    </div>
  </nav>
  {@render children()}
</div>
