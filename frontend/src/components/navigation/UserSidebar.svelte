<script lang="ts">
    import { page } from '$app/stores'
    import { Sidebar, SidebarGroup, SidebarItem, SidebarWrapper } from 'flowbite-svelte'
    import { BookOutline, ProfileCardOutline, StarOutline } from 'flowbite-svelte-icons'

    $: activeUrl  = $page.url.pathname

    let activeClass = 'flex items-center p-2 text-base font-normal text-primary-900 bg-primary-200 dark:bg-primary-700 rounded-lg dark:text-white hover:bg-primary-100 dark:hover:bg-gray-700';
    let nonActiveClass = 'flex items-center p-2 text-base font-normal text-green-900 rounded-lg dark:text-white hover:bg-green-100 dark:hover:bg-green-700';
</script>
  
  <Sidebar {activeUrl} {activeClass} {nonActiveClass}>
    <SidebarWrapper>
      <SidebarGroup>
        <SidebarItem label="Profile" href="/user/profile" active={activeUrl === "/user/profile"}>
            <svelte:fragment slot="icon">
              <ProfileCardOutline class="w-5 h-5 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white" />
            </svelte:fragment>
          </SidebarItem>
        <SidebarItem label="Contacts" href="/user/contacts" active={activeUrl === "/user/contacts"}>
          <svelte:fragment slot="icon">
            <BookOutline class="w-5 h-5 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white" />
          </svelte:fragment>
        </SidebarItem>
        <SidebarItem label="Reviews" href="/user/reviews" active={activeUrl === "/user/reviews"}>
            <svelte:fragment slot="icon">
              <StarOutline class="w-5 h-5 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white" />
            </svelte:fragment>
          </SidebarItem>
      </SidebarGroup>
    </SidebarWrapper>
  </Sidebar>