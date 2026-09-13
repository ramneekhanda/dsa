// The chapter-wise tutorial shown in the "Learn" panel (`Help.svelte`).
// Two levels: a Chapter is a broad topic area, each with an ordered list of
// Lessons - a markdown file plus an optional runnable YAML snippet, both
// fetched from `static/tutorial/<chapter dir>/` at runtime (same pattern the
// old Examples menu used for `static/examples/`). Add a lesson by dropping
// its two files in the chapter's directory and adding one entry below; add a
// chapter by giving it a new `dir` and its own lessons array.
export interface TutorialLesson {
  id: string;
  title: string;
  markdown: string;
  yaml?: string;
}

export interface TutorialChapter {
  id: string;
  title: string;
  dir: string;
  lessons: TutorialLesson[];
}

export const tutorialChapters: TutorialChapter[] = [
  {
    id: "ch1",
    title: "Chapter 1: Fundamentals",
    dir: "ch1",
    lessons: [
      { id: "1.1", title: "1.1 Introduction", markdown: "01_introduction.md" },
      {
        id: "1.2",
        title: "1.2 Your First Graph",
        markdown: "02_first_graph.md",
        yaml: "02_first_graph.yml",
      },
      {
        id: "1.3",
        title: "1.3 Nodes Talking To Each Other",
        markdown: "03_messaging.md",
        yaml: "03_messaging.yml",
      },
      {
        id: "1.4",
        title: "1.4 Configurable Node Types",
        markdown: "04_params_and_icons.md",
        yaml: "04_params_and_icons.yml",
      },
      {
        id: "1.5",
        title: "1.5 State and Logging",
        markdown: "05_state_and_logging.md",
        yaml: "05_state_and_logging.yml",
      },
      {
        id: "1.6",
        title: "1.6 Putting It Together",
        markdown: "06_putting_it_together.md",
        yaml: "06_putting_it_together.yml",
      },
    ],
  },
  {
    id: "ch2",
    title: "Chapter 2: Custom Visuals",
    dir: "ch2",
    lessons: [
      {
        id: "2.1",
        title: "2.1 draw() Basics",
        markdown: "01_draw_basics.md",
        yaml: "01_draw_basics.yml",
      },
      {
        id: "2.2",
        title: "2.2 Shape Primitives",
        markdown: "02_shapes.md",
        yaml: "02_shapes.yml",
      },
      {
        id: "2.3",
        title: "2.3 Reusable Node Templates",
        markdown: "03_node_templates.md",
        yaml: "03_node_templates.yml",
      },
      {
        id: "2.4",
        title: "2.4 Parametrized Templates",
        markdown: "04_parametrized_templates.md",
        yaml: "04_parametrized_templates.yml",
      },
    ],
  },
];
