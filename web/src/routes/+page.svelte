<script lang="ts">
  import Monaco from "./Monaco.svelte";
  import Menubar from "$lib/components/menubar/menubar.svelte";
  import init, { compile_code, get_code_schema } from "./dsa";
  import { onMount } from "svelte";
  import type { IDockviewPanel } from "dockview-core";
  import "dockview-core/dist/styles/dockview.css";
  import "tabulator-tables/dist/css/tabulator.min.css";
  import * as Panels from "./panels";

  let id = 0;
  let code: string;
  let codeEditor: Monaco;
  let viewPanel: IDockviewPanel;
  let schema = "";
  let dockView: HTMLElement;
  let log_event_listener: HTMLDivElement;
  let data: Array<Panels.LogMessageType> = [];
  $: {
    console.log("code changed");
    code = code;
    schema = schema;
    if (codeEditor) {
      codeEditor.$set({ schema });
      codeEditor.setCode(code);
      codeEditor.setFocus();
    }
    code = '';
  }

  function onLogEvent(e: Event) {
    var customEvent = e as CustomEvent;
    let more_data = {} as Panels.LogMessageType;
    more_data.time = new Date().toLocaleTimeString();
    more_data.message = customEvent.detail;
    more_data.severity = "info";
    more_data.node = "dsa";
    data.unshift(more_data);
  }

  onMount(async () => {
    let returnVal = {} as Panels.DockviewReturn;
    Panels.createDockviewInternal(dockView, schema, data, returnVal, loadExample);
    codeEditor = returnVal.codeEditor;
    viewPanel = returnVal.viewPanel;
    log_event_listener.addEventListener("dsa-log-event", onLogEvent);
    init()
      .catch((error) => {
        if (
          !error.message.startsWith(
            "Using exceptions for control flow, don't mind me. This isn't actually an error!",
          )
        ) {
          console.log("actual error location is here");
          throw error;
        }
      })
      .then(() => {
        schema = get_code_schema();
      });
  });

  function toggleFullscreen() {
    if (!viewPanel) return;
    if (viewPanel.api.isMaximized()) {
      viewPanel.api.exitMaximized();
    } else {
      viewPanel.api.maximize();
    }
  }

  function compileCode() {
    let b = compile_code(codeEditor.getCode());
    if (b.error_log && b.error_log.length > 0) {
      console.log(b.error_log);
    }
  }

  // Loads a tutorial chapter's YAML snippet straight into the editor - the
  // "Load this example" button in the Learn panel (`Help.svelte`) calls this
  // via the `loadExample` prop threaded through `Panels.createDockviewInternal`.
  function loadExample(filename: string) {
    fetch(`tutorial/${filename}`)
      .then((response) => response.text())
      .then((data) => {
        code = data;
      })
      .catch((error) => {
        console.error("Error:", error);
      });
  }
</script>

<div class="flex">
  <Menubar
    on:runClicked={() => compileCode()}
    on:fullscreenClicked={() => toggleFullscreen()}
  />
  <div class="flex" bind:this={dockView}></div>
</div>
<div id="dsa-log-event-listener" bind:this={log_event_listener} />
