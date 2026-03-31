<script lang="ts">
  import { navButtonStyle } from '$lib/styles/button';
  import { cn } from '$lib/utils';
  import { Info, Tags } from '@lucide/svelte';
  import { Tabs } from 'bits-ui';
  import GeneralTab from './general-tab.svelte';
  import TagsTab from './tags-tab.svelte';
</script>

<Tabs.Root value="general" orientation="vertical" class="flex gap-6">
  <Tabs.List class="flex w-72 flex-col gap-2 rounded-3xl bg-card p-3">
    <Tabs.Trigger value="general" class={cn(navButtonStyle(), 'flex h-10 gap-2 px-4 text-lg font-medium')}>
      <Info size="20" /> General
    </Tabs.Trigger>
    <Tabs.Trigger value="tags" class={cn(navButtonStyle(), 'flex h-10 gap-2 px-4 text-lg font-medium')}>
      <Tags size="20" />Tags
    </Tabs.Trigger>
  </Tabs.List>
  <Tabs.Content value="general">
    <GeneralTab />
  </Tabs.Content>
  <Tabs.Content value="tags">
    <TagsTab />
  </Tabs.Content>
</Tabs.Root>
