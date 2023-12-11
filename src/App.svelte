<script lang="ts">
  import {message, open} from "@tauri-apps/api/dialog";
  import {invoke} from "@tauri-apps/api/tauri";

  type actionType = "idle" | "indexing" | "deleting"

  type stateType = {
    extensions: string[]
    selectedExtensions: string[],
    selectedPath: string | string[] | null
    action: actionType
  }

  const initialState: stateType = {
    extensions: [],
    selectedExtensions: [],
    selectedPath: "some looooooooooonlooooooooooonlooooooooooonlooooooooooon test",
    action: "idle"
  }

  let state: stateType = {...initialState}

  const chooseDirectory = async () => {
    try {
      state.selectedPath = await open({
        directory: true,
        multiple: false,
        title: "Ordner auswählen"
      })

      if (state.selectedPath == null) {
        state.selectedPath = ""
        return
      }

      state.extensions = []

      state.action = "indexing"

      invoke("get_all_file_types", {directory: state.selectedPath})
        .then(result => {
          state.extensions = result as string[];
          state.extensions.sort((a, b) => a.localeCompare(b));
        })
        .finally(() => state.action = "idle")

    } catch (e) {
      console.error(e)
    }
  }

  const updateExtensionSelection = (event: Event) => {
    const extension = (event.target as HTMLInputElement).value;
    if (state.selectedExtensions.find(ext => ext === extension)) {
      state.selectedExtensions = state.selectedExtensions.filter(ext => ext !== extension)
    } else {
      state.selectedExtensions = [...state.selectedExtensions, extension]
    }
  }

  const deleteFiles = async () => {
    try {
      state.action = "deleting"
      let deletedFileCount = await invoke("delete_files", {
        directory: state.selectedPath,
        extensions: state.selectedExtensions.join("|")
      });
      await message(`Anzahl gelöschter Dateien: ${deletedFileCount}`,
        {title: "Electra: Löschen erfolgreich"}
      )
    } catch (e) {
      console.error(e)
    } finally {
      state = {...initialState}
    }
  }


</script>

<main class="container">
  <section class="button-section">
      <button on:click={chooseDirectory}
              disabled={state.action === "indexing"}>
        Ordner auswählen
        {#if state.action === "indexing" }
          <div class="loader"/>
        {/if}
      </button>
      <span>{state.selectedPath}</span>
  </section>
  {#if (state.selectedPath?.length || 0) > 0}
    <hr/>
  {/if}
  <section id="list-section">
    <div>
      <ul>
        {#each state.extensions as extension}
          <li>
            <input type="checkbox"
                   value={extension}
                   checked={!!state.selectedExtensions.find(ext=>ext === extension)}
                   on:change={updateExtensionSelection}
            />
            <span class="extension-list-item">{extension}</span>
          </li>
        {/each}
      </ul>
    </div>
  </section>
  {#if state.extensions.length > 0}
    <hr/>
    <section class="button-section">
      <button on:click={deleteFiles}
              disabled={state.action === "deleting" || state.extensions.length === 0}>
        Löschen
        {#if state.action === "deleting" }
          <div class="loader"/>
        {/if}
      </button>
    </section>
  {/if}
</main>
