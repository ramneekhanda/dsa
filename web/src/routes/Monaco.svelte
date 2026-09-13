<script lang="ts">
  import "../app.css";
  import type monaco from "monaco-editor";
  import { configureMonacoYaml } from "monaco-yaml";
  import { onMount } from "svelte";
  import EditorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker";
  import YamlWorker from "./monaco_yaml.worker.js?worker";

  let Monaco: typeof monaco;
  export let schema = "";
  let editorElement: HTMLDivElement;
  let editor: monaco.editor.IStandaloneCodeEditor;

  function configureMonaco() {
    if (typeof Monaco !== "undefined" && schema !== "") {
      try {
        configureMonacoYaml(Monaco, {
          enableSchemaRequest: true,
          completion: true,
          validate: true,
          schemas: [
            {
              fileMatch: ["*"],
              schema: JSON.parse(schema),
              uri: "inmemory://schema.json",
            },
          ],
        });
      } catch (e) {
        console.error("Failed to configure Monaco YAML schema:", e);
      }
    }
  }

  $: if (schema !== "") {
    configureMonaco();
  }

  export let value: string = "";
  export let onChange: (code: string) => void = () => {};
  let debounceTimer: any = null;

  export function getCode() {
    return editor ? editor.getValue() : "";
  }

  export function setCode(s: string) {
    if (editor == null) {
      return;
    }
    const model = editor.getModel();
    if (model) {
      model.setValue(s);
      if (Monaco) {
        Monaco.editor.setModelLanguage(model, "yaml");
      }
    } else {
      editor.setValue(s);
    }
  }

  export function setFocus() {
    if (editor == null) {
      return;
    }
    editor.focus();
  }

  onMount(async () => {
    // @ts-ignore
    window.MonacoEnvironment = {
      getWorker: function (_moduleId: any, label: string) {
        if (label === "yaml") {
          return new YamlWorker();
        }
        return new EditorWorker();
      },
    };

    Monaco = await import("monaco-editor");

    editor = Monaco.editor.create(editorElement, {
      minimap: { enabled: false },
      automaticLayout: true,
      scrollBeyondLastLine: false,
      language: "yaml",
      theme: "vs",
      quickSuggestions: {
        other: true,
        comments: false,
        strings: true,
      },
    });

    configureMonaco();

    const modelUri = Monaco.Uri.parse("inmemory://mymodel.yaml");
    let yamlModel = Monaco.editor.getModel(modelUri);
    if (!yamlModel) {
      yamlModel = Monaco.editor.createModel(value, "yaml", modelUri);
    } else {
      yamlModel.setValue(value);
      Monaco.editor.setModelLanguage(yamlModel, "yaml");
    }
    editor.setModel(yamlModel);

    editor.onDidChangeModelContent(() => {
      if (debounceTimer) clearTimeout(debounceTimer);
      debounceTimer = setTimeout(() => {
        onChange(editor.getValue());
      }, 400);
    });

    editor.addCommand(Monaco.KeyMod.CtrlCmd | Monaco.KeyCode.Enter, () => {
      if (debounceTimer) clearTimeout(debounceTimer);
      onChange(editor.getValue());
    });

    return () => {
      if (debounceTimer) clearTimeout(debounceTimer);
      editor.dispose();
    };
  });
</script>
<div class="flex h-fit">
  <div class="monaco-container flex h-fit" bind:this={editorElement} />
</div>
