<script lang="ts">
  import {message, open} from "@tauri-apps/api/dialog";
  import {invoke} from "@tauri-apps/api/tauri";

  type actionType = "idle" | "indexing" | "deleting"

  type stateType = {
    extensions: Map<String, number>
    selectedExtensions: string[],
    selectedPath: string | string[] | null
    action: actionType
  }

  const initialState: stateType = {
    extensions: [],
    selectedExtensions: [],
    selectedPath: "",
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

      state.extensions = new Map()

      state.action = "indexing"

      invoke("get_all_file_types", {directory: state.selectedPath})
        .then(result => {
          console.log(new Map(Object.entries(result)))
          state.extensions = new Map(Object.entries(result))
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

  const selectAll = () => {
    if (state.selectedExtensions.length !== state.extensions.size) {
      state.selectedExtensions = [...state.extensions.keys()]
    } else {
      state.selectedExtensions = []
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
    <section id="table-section">
      <div>
        <table style="border-spacing: 0">
          <tr>
            <th>
              <input type="checkbox"
                     checked={state.selectedExtensions.length === state.extensions.size}
                     on:change={selectAll}
              />
            </th>
            <th colspan="2">
              <span class="value-cell">Alle auswählen</span>
            </th>
          </tr>
          {#each state.extensions.entries() as [extension, count]}
            <tr>
              <td>
                <input type="checkbox"
                       value={extension}
                       checked={!!state.selectedExtensions.find(ext=>ext === extension)}
                       on:change={updateExtensionSelection}
                />
              </td>
              <td>
                <span class="value-cell">{extension}</span>
              </td>
              <td style="text-align: right">
                <span class="value-cell">{count}</span>
              </td>
            </tr>
          {/each}
        </table>
      </div>
    </section>
  {/if}
  {#if state.extensions.size > 0}
    <hr/>
    <section class="button-section">
      <button on:click={deleteFiles}
              disabled={state.action === "deleting" || state.extensions.size === 0}>
        Löschen
        {#if state.action === "deleting" }
          <div class="loader"/>
        {/if}
      </button>
    </section>
  {/if}
</main>
