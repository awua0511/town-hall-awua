<script lang="ts">
  import { resolve } from '$app/paths';
  import { page } from '$app/state';
  import * as NavigationMenu from '$lib/components/ui/navigation-menu';
  import { Compass, Users, Scale } from '@lucide/svelte';

  let current_path = $derived(page.url.pathname);
</script>

<NavigationMenu.Root>
  <NavigationMenu.List>
    <NavigationMenu.Item>
      <NavigationMenu.Link active={current_path === '/q'}>
        {#snippet child({ props })}
          <a {...props} href={resolve('/q')}>
            <Compass class="size-5" />Quests
          </a>
        {/snippet}
      </NavigationMenu.Link>
    </NavigationMenu.Item>
    <NavigationMenu.Item>
      <NavigationMenu.Link active={current_path === '/u'}>
        {#snippet child({ props })}
          <a {...props} href={resolve('/u')}>Profile</a>
        {/snippet}
      </NavigationMenu.Link>
    </NavigationMenu.Item>
    <NavigationMenu.Item>
      <NavigationMenu.Link active={current_path === '/guilds'}>
        {#snippet child({ props })}
          <a {...props} href={resolve('/guilds')}>
            <Users class="size-5" />Guilds
          </a>
        {/snippet}
      </NavigationMenu.Link>
    </NavigationMenu.Item>
    <NavigationMenu.Item>
      <NavigationMenu.Link active={current_path === '/council/cases'}>
        {#snippet child({ props })}
          <a {...props} href={resolve('/council/cases')}>
            <Scale class="size-5" />Council
          </a>
        {/snippet}
      </NavigationMenu.Link>
    </NavigationMenu.Item>
  </NavigationMenu.List>
</NavigationMenu.Root>
