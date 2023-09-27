<script lang="ts">
    import { page } from '$app/stores'
    import { Sidebar, SidebarGroup, SidebarItem, SidebarWrapper } from 'flowbite-svelte'
    import { BookOutline, StarOutline, StarSolid } from 'flowbite-svelte-icons'

    $: activeUrl  = $page.url.pathname

    let activeClass = 'flex items-center p-2 text-base font-normal text-primary-900 bg-primary-200 dark:bg-primary-700 rounded-lg dark:text-white hover:bg-primary-100 dark:hover:bg-gray-700';
    let nonActiveClass = 'flex items-center p-2 text-base font-normal text-green-900 rounded-lg dark:text-white hover:bg-green-100 dark:hover:bg-green-700';
</script>
  
  <Sidebar {activeUrl} {activeClass} {nonActiveClass}>
    <SidebarWrapper>
      <SidebarGroup>
        <SidebarItem label="Pending Contacts" href="/admin/pending-contacts" active={activeUrl === "/admin/pending-contacts"}>
          <svelte:fragment slot="icon">
            <BookOutline class="w-5 h-5 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white" />
          </svelte:fragment>
        </SidebarItem>
        <SidebarItem label="Recent Reviews" href="/admin/recent-reviews" active={activeUrl === "/admin/recent-reviews"}>
            <svelte:fragment slot="icon">
              <StarSolid class="w-5 h-5 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white" />
            </svelte:fragment>
          </SidebarItem>
        <SidebarItem label="Reviews by user" href="/admin/user-reviews" active={activeUrl.match("/admin/user-reviews") != null }>
            <svelte:fragment slot="icon">
              <StarOutline class="w-5 h-5 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white" />
            </svelte:fragment>
          </SidebarItem>
      </SidebarGroup>
    </SidebarWrapper>
  </Sidebar>