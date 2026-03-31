<script lang="ts">
  import Links from './links.svelte';
  import LoginButton from './login-button.svelte';
  import Logo from './logo.svelte';
  import PfpMenu from './pfp-menu.svelte';
  import QuestButton from './quest-button.svelte';

  let { me } = $props();
</script>

<nav class="flex items-center py-6">
  <div class="flex-1">
    <Logo />
  </div>
  <div class="flex-none">
    <Links />
  </div>
  <div class="flex flex-1 justify-end gap-4">
    {#if me}
      <QuestButton />
      <PfpMenu user={me} />
    {:else}
      <LoginButton />
    {/if}
  </div>
</nav>
