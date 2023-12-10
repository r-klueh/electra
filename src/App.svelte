<script lang="ts">
    import {open} from "@tauri-apps/api/dialog";
    import {invoke} from "@tauri-apps/api/tauri";

    let extensions: string[] = []
    let selectedPath: string | string[] | null = ""

    let fileCount = 0

    const countFiles = async () => {
        try {
            selectedPath = await open({
                directory: true,
                multiple: false,
                title: "Ordner auswählen"
            })

            if (selectedPath == null) {
                return
            }

            extensions = await invoke("get_file_count", {directory: selectedPath})
        } catch (e) {
            console.error(e)
        }
    }

    const chooseDirectory = async () => {
        try {
            selectedPath = await open({
                directory: true,
                multiple: false,
                title: "Ordner auswählen"
            })

            if (selectedPath == null) {
                return
            }

            extensions = await invoke("get_all_file_types", {directory: selectedPath})
        } catch (e) {
            console.error(e)
        }
    }

</script>

<main class="container">
    <span>
    <button on:click={chooseDirectory}>Ordner auswählen</button>
    <button on:click={countFiles}>Files zählen</button>
    <span>{selectedPath}</span>
    <span>{fileCount}</span>
    </span>
    <ul>
        {#each extensions as extension}
            <li><input type="checkbox"/>{extension}</li>
        {/each}
    </ul>
</main>
<style>

</style>