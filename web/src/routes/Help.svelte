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

  // Set of open/expanded chapter IDs. By default, all chapters are open.
  let openChapters: Record<string, boolean> = tutorialChapters.reduce((acc, ch) => {
    acc[ch.id] = true;
    return acc;
  }, {} as Record<string, boolean>);

  function toggleChapter(chapterId: string) {
    openChapters[chapterId] = !openChapters[chapterId];
    openChapters = { ...openChapters };
  }

  function selectLesson(chapter: TutorialChapter, lesson: TutorialLesson) {
    selectedChapter = chapter;
    selectedLesson = lesson;
    if (!openChapters[chapter.id]) {
      openChapters[chapter.id] = true;
      openChapters = { ...openChapters };
    }
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
      <div class="chapter-group">
        <button
          type="button"
          class="chapter-header"
          on:click={() => toggleChapter(chapter.id)}
          aria-expanded={openChapters[chapter.id] ? "true" : "false"}
        >
          <span class="chapter-chevron" class:collapsed={!openChapters[chapter.id]}>
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="6 9 12 15 18 9"></polyline>
            </svg>
          </span>
          <span class="chapter-title">{chapter.title}</span>
          <span class="lesson-count">{chapter.lessons.length}</span>
        </button>
        {#if openChapters[chapter.id]}
          <ul class="lesson-list">
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
        {/if}
      </div>
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
    flex: 0 0 280px;
    overflow-y: auto;
    border-right: 1px solid #e5e7eb;
    padding: 0.5rem;
    background: #fafafa;
  }
  .chapter-group {
    margin-bottom: 0.4rem;
  }
  .chapter-header {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 0.4rem;
    padding: 0.45rem 0.4rem;
    background: transparent;
    border: none;
    border-radius: 0.375rem;
    cursor: pointer;
    text-align: left;
    transition: background 0.15s ease;
    user-select: none;
  }
  .chapter-header:hover {
    background: #f0f2f5;
  }
  .chapter-chevron {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    color: #6b7280;
    transition: transform 0.2s ease;
    transform: rotate(0deg);
    flex-shrink: 0;
  }
  .chapter-chevron.collapsed {
    transform: rotate(-90deg);
  }
  .chapter-title {
    font-weight: 700;
    font-size: 0.78rem;
    text-transform: uppercase;
    letter-spacing: 0.03em;
    color: #374151;
    flex: 1;
    line-height: 1.3;
  }
  .lesson-count {
    font-size: 0.7rem;
    color: #6b7280;
    background: #e5e7eb;
    padding: 0.05rem 0.35rem;
    border-radius: 9999px;
    font-weight: 600;
    flex-shrink: 0;
  }
  .lesson-list {
    list-style: none;
    margin: 0.15rem 0 0.3rem 0;
    padding: 0 0 0 0.5rem;
  }
  .lesson {
    padding: 0.4rem 0.6rem;
    border-radius: 0.375rem;
    cursor: pointer;
    font-size: 0.85rem;
    color: #4b5563;
    transition: background 0.1s ease, color 0.1s ease;
  }
  .lesson:hover {
    background: #e5e7eb;
    color: #111827;
  }
  .lesson.active {
    background: #e0e7ff;
    color: #3730a3;
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
