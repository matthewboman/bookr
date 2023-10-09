<script lang="ts">
    import anime from 'animejs'

    export let closedText  = 'hide filters'
    export let duration    = 300
    export let easing      = 'easeOutQuad'
    export let openText    = 'show filters'
    export let toggleClass = 'filter-container'
    export let toggleOpen  = false
    export let toggleText  = 'show filters'

    function toggle(){
        if (toggleOpen){
            anime({
                targets: `.${toggleClass}`,
                height:   54,
                paddingBottom: 0,
                easing,
                duration
            })

            anime({
                targets: '.toggle-btn span',
                opacity:  0,
                easing:  'linear',
                duration: 100,
                complete: () => {
                    toggleText = openText
                    anime({
                        targets: '.toggle-btn span',
                        opacity: 1,
                        easing: 'linear',
                        duration: 100,
                    })
                }
            })
        } else {
            anime({
                targets: `.${toggleClass}`,
                height:  322,
                width: '100vw',
                padding: 16,
                easing,
                duration
            })

            anime({
                targets:  '.toggle-btn span',
                opacity:  0,
                easing:   'linear',
                duration: 100,
                complete: () => {
                    toggleText = closedText
                    anime({
                        targets:  '.toggle-btn span',
                        opacity:  1,
                        easing:   'linear',
                        duration: 100,
                    })
                }
            })
        }
        toggleOpen = !toggleOpen
    }
</script>

<div class={toggleClass}>
    <button on:click={toggle} class="toggle-btn xl:hidden md:block text-center w-full pb-2">
        <span>{toggleText}</span>
    </button>
    <slot/>
</div>

<style>
    /* classes need to be defined here w/ the way svelte scopes */
    .filter-container {
        backdrop-filter: blur(2px);
        border-style: solid;
        border-top-width: 4px;
        z-index: 10;
        background-color: rgba(31, 41, 55,0.9);
        /* width: 100vw; */
        height: 54px; 
        padding: 16px 16px 0px;
    }
</style>