<script lang="ts">
  import { Carta, Markdown } from "carta-md";
  import { code } from '@cartamd/plugin-code';
  import { tutorialChapters, type TutorialChapter, type TutorialLesson } from "./tutorial";

  const carta = new Carta({
    extensions: [code()],
    // Content only ever comes from this repo's own static/tutorial/*.md, never
    // from user/network input, so skipping sanitization is safe here.
    sanitizer: false,
  });

  // Called with a tutorial/<chapter dir>/<file> path when the reader clicks
  // "Load this example" - wired by panels.ts to the same editor-loading logic
  // the old Examples menu used (see `+page.svelte`'s `loadExample`).
  export let loadExample: (filename: string) => void = () => {};

  let selectedChapter: TutorialChapter = tutorialChapters[0];
  let selectedLesson: TutorialLesson = selectedChapter.lessons[0];
  let value = "";

  function selectLesson(chapter: TutorialChapter, lesson: TutorialLesson) {
    selectedChapter = chapter;
    selectedLesson = lesson;
    fetch(`tutorial/${chapter.dir}/${lesson.markdown}`)
      .then((response) => response.text())
      .then((data) => {
        value = data;
      })
      .catch((error) => {
        console.error("Error:", error);
      });
  }

  selectLesson(selectedChapter, selectedLesson);
</script>

<div class="tutorial">
  <nav class="chapters">
    {#each tutorialChapters as chapter (chapter.id)}
      <div class="chapter-title">{chapter.title}</div>
      <ul>
        {#each chapter.lessons as lesson (lesson.id)}
          <!-- svelte-ignore a11y-click-events-have-key-events -->
          <!-- svelte-ignore a11y-no-static-element-interactions -->
          <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
          <li
            class="lesson"
            class:active={lesson.id === selectedLesson.id}
            on:click={() => selectLesson(chapter, lesson)}
          >
            {lesson.title}
          </li>
        {/each}
      </ul>
    {/each}
  </nav>
  <div class="content">
    {#if selectedLesson.yaml}
      <button class="load-btn" on:click={() => loadExample(`${selectedChapter.dir}/${selectedLesson.yaml}`)}>
        ▶ Load this example
      </button>
    {/if}
    <div class="mkdown">
      {#key value}
        <Markdown {carta} {value} />
      {/key}
    </div>
  </div>
</div>

<style>
  .tutorial {
    display: flex;
    width: 100%;
    height: 100%;
    overflow: hidden;
  }
  .chapters {
    flex: 0 0 240px;
    overflow-y: auto;
    border-right: 1px solid #ddd;
    padding: 0.5rem;
  }
  .chapter-title {
    font-weight: 700;
    font-size: 0.85rem;
    text-transform: uppercase;
    letter-spacing: 0.02em;
    color: #555;
    margin: 0.9rem 0 0.3rem 0.2rem;
  }
  .chapter-title:first-child {
    margin-top: 0.2rem;
  }
  ul {
    list-style: none;
    margin: 0;
    padding: 0;
  }
  .lesson {
    padding: 0.4rem 0.6rem;
    border-radius: 0.4rem;
    cursor: pointer;
    font-size: 0.9rem;
  }
  .lesson:hover {
    background: #f0f0f0;
  }
  .lesson.active {
    background: #e0e7ff;
    font-weight: 600;
  }
  .content {
    flex: 1;
    overflow-y: auto;
    padding: 0 1rem;
  }
  .load-btn {
    position: sticky;
    top: 0.5rem;
    margin: 0.5rem 0;
    padding: 0.4rem 0.8rem;
    border-radius: 0.4rem;
    border: 1px solid #333;
    background: #1b3a5c;
    color: #fff;
    cursor: pointer;
    font-size: 0.85rem;
  }
  .load-btn:hover {
    opacity: 0.9;
  }

  :global(body) {
    padding: 0;
  }
</style>
