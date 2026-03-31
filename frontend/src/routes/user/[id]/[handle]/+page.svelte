<script lang="ts">
  const { data } = $props();
  const user = $derived(data.targetUser);
</script>

<h1>{user.handle}'s profile page</h1>
