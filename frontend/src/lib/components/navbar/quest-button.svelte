<script lang="ts">
  import { Plus } from '@lucide/svelte';

  import * as Dialog from '$lib/components/ui/dialog';
  import { Button, buttonVariants } from '$lib/components/ui/button/';
  import { Input } from '$lib/components/ui/input';
  import { Label } from '$lib/components/ui/label';
  import { Spinner } from '$lib/components/ui/spinner';

  import { goto } from '$app/navigation';
  import { createQuest } from '$lib/backend/quest';

  let dialogOpen = $state(false);
  let createQuestLoading = $state(false);
  let questTitle = $state('');
  let questError = $state('');

  async function handleCreateQuest(event: SubmitEvent) {
    event.preventDefault();
    questError = '';
    createQuestLoading = true;

    const result = await createQuest(questTitle);

    createQuestLoading = false;

    if (!result.ok) {
      questError = result.body;
      return;
    }

    const questId = result.data;
    dialogOpen = false;
    questTitle = '';
    await goto(`/q/${questId}-new-quest`);
  }
</script>

<form id="create-quest-form" onsubmit={handleCreateQuest}></form>

<Dialog.Root bind:open={dialogOpen}>
  <Dialog.Trigger type="button" class={buttonVariants({ variant: 'default' })}>
    <Plus />New Quest
  </Dialog.Trigger>

  <Dialog.Content class="sm:max-w-106.25">
    <Dialog.Header>
      <Dialog.Title>New Quest</Dialog.Title>
      <Dialog.Description>Issue a new quest for Adventurers to complete!</Dialog.Description>
    </Dialog.Header>

    <div class="grid gap-4">
      <div class="grid gap-3">
        <Label for="title-1">Title</Label>
        <Input
          form="create-quest-form"
          id="title-1"
          name="title"
          bind:value={questTitle}
          placeholder="Enter quest title..."
          required
          minlength={10}
          maxlength={100}
          class="invalid:border-destructive" />
        {#if questError}
          <p class="text-destructive">{questError}</p>
        {/if}
      </div>
    </div>

    <Dialog.Footer>
      <Dialog.Close type="button" class={buttonVariants({ variant: 'outline' })}>Cancel</Dialog.Close>

      <Button form="create-quest-form" type="submit" disabled={createQuestLoading || !questTitle.trim()}>
        {#if createQuestLoading}
          <Spinner />
        {:else}
          <Plus />
        {/if}
        Create draft
      </Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
