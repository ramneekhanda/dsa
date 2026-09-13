<script lang="ts">
  import Monaco from "./Monaco.svelte";
  import Menubar from "$lib/components/menubar/menubar.svelte";
  import init, { compile_code, compile_code_with_sources, get_import_urls, get_code_schema } from "./dsa";
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
  let remoteSourcesCache: Record<string, string> = {};

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
    Panels.createDockviewInternal(
      dockView,
      schema,
      data,
      returnVal,
      loadExample,
      (newCode) => resolveAndCompile(newCode)
    );
    codeEditor = returnVal.codeEditor;
    viewPanel = returnVal.viewPanel;
    // @ts-ignore
    window.__procsimCodeEditor = codeEditor;
    // @ts-ignore
    window.__procsimCompile = resolveAndCompile;
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
        loadExample("ch1/02_first_graph.yml");
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

  async function resolveAndCompile(sourceCode: string) {
    let urls: string[] = [];
    try {
      if (typeof get_import_urls === "function") {
        const raw = get_import_urls(sourceCode);
        urls = typeof raw === "string" ? JSON.parse(raw) : (raw || []);
      }
    } catch (e) {
      console.warn("Failed to scan import urls:", e);
    }

    const missingUrls = urls.filter((u) => !remoteSourcesCache[u]);
    if (missingUrls.length > 0) {
      await Promise.all(
        missingUrls.map(async (url) => {
          try {
            const resp = await fetch(url);
            if (resp.ok) {
              remoteSourcesCache[url] = await resp.text();
            } else {
              console.error(`Failed to fetch remote import from ${url}: ${resp.statusText}`);
            }
          } catch (err) {
            console.error(`Error fetching remote import from ${url}:`, err);
          }
        })
      );
    }

    let b = typeof compile_code_with_sources === "function"
      ? compile_code_with_sources(sourceCode, JSON.stringify(remoteSourcesCache))
      : compile_code(sourceCode);
    if (b.error_log && b.error_log.length > 0) {
      console.log(b.error_log);
    }
  }

  async function compileCode() {
    if (codeEditor) {
      await resolveAndCompile(codeEditor.getCode());
    }
  }

  // Loads a tutorial chapter's YAML snippet straight into the editor - the
  // "Load this example" button in the Learn panel (`Help.svelte`) calls this
  // via the `loadExample` prop threaded through `Panels.createDockviewInternal`.
  function loadExample(filename: string) {
    fetch(`tutorial/${filename}`)
      .then((response) => response.text())
      .then(async (data) => {
        code = data;
        await resolveAndCompile(data);
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
