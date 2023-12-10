<script lang="ts">
    import {open} from "@tauri-apps/api/dialog";
    import {invoke} from "@tauri-apps/api/tauri";

    let extensions: string[] = []
    let selectedPath: string | string[] | null = ""

    const chooseDirectory = async () => {
        try {
            selectedPath = await open({
                directory: true,
                multiple: false,
                title: "Ordner auswählen"
            })

            extensions = await invoke("get_all_file_types", {directory: selectedPath})

            console.log(extensions)
        } catch (e) {
            console.error(e)
        }
    }

</script>

<main class="container">
    <span>
    <button on:click={chooseDirectory}>Ordner auswählen</button>
    <span>{selectedPath}</span>
    </span>
    <ul>
        {#each extensions as extension}
            <li><input type="checkbox"/>{extension}</li>
        {/each}
    </ul>
</main>
<style>

</style>