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
      {
        id: "1.7",
        title: "1.7 Multilevel Routing",
        markdown: "07_multilevel_routing.md",
        yaml: "07_multilevel_routing.yml",
      },
      {
        id: "1.8",
        title: "1.8 Constants & State Persistence",
        markdown: "08_constants_and_scope.md",
        yaml: "08_constants_and_scope.yml",
      },
      {
        id: "1.9",
        title: "1.9 Interactive Narration (explain)",
        markdown: "09_interactive_narration.md",
        yaml: "09_interactive_narration.yml",
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
  {
    id: "ch3",
    title: "Chapter 3: Theming",
    dir: "ch3",
    lessons: [
      {
        id: "3.1",
        title: "3.1 Cloud Architecture Theme",
        markdown: "01_cloud_cards.md",
        yaml: "01_cloud_cards.yml",
      },
      {
        id: "3.2",
        title: "3.2 Datacenter Rack Units",
        markdown: "02_datacenter_rack.md",
        yaml: "02_datacenter_rack.yml",
      },
      {
        id: "3.3",
        title: "3.3 Cyberpunk Neon HUD",
        markdown: "03_cyberpunk_hud.md",
        yaml: "03_cyberpunk_hud.yml",
      },
      {
        id: "3.4",
        title: "3.4 Minimal Capsule Pills",
        markdown: "04_capsule_pills.md",
        yaml: "04_capsule_pills.yml",
      },
      {
        id: "3.5",
        title: "3.5 Layered & Composed Themes",
        markdown: "05_layered_theming.md",
        yaml: "05_layered_theming.yml",
      },
      {
        id: "3.6",
        title: "3.6 Remote Imports & Presets",
        markdown: "06_remote_imports.md",
        yaml: "06_remote_imports.yml",
      },
      {
        id: "3.7",
        title: "3.7 Remote Theme Packages",
        markdown: "07_remote_theme_imports.md",
        yaml: "07_remote_theme_imports.yml",
      },
    ],
  },
  {
    id: "ch4",
    title: "Chapter 4: Process Libraries (plibs)",
    dir: "ch4",
    lessons: [
      {
        id: "4.1",
        title: "4.1 Introduction to plibs",
        markdown: "01_intro_to_plibs.md",
        yaml: "01_intro_to_plibs.yml",
      },
      {
        id: "4.2",
        title: "4.2 Theme Libraries",
        markdown: "02_theme_libraries.md",
        yaml: "02_theme_libraries.yml",
      },
      {
        id: "4.3",
        title: "4.3 AWS Compute & Networking",
        markdown: "03_aws_compute_networking.md",
        yaml: "03_aws_compute_networking.yml",
      },
      {
        id: "4.4",
        title: "4.4 AWS Storage & Messaging",
        markdown: "04_aws_database_messaging.md",
        yaml: "04_aws_database_messaging.yml",
      },
      {
        id: "4.5",
        title: "4.5 AWS Cloud Architectures",
        markdown: "05_aws_full_architecture.md",
        yaml: "05_aws_full_architecture.yml",
      },
      {
        id: "4.6",
        title: "4.6 Authoring Custom plibs",
        markdown: "06_authoring_plibs.md",
        yaml: "06_authoring_plibs.yml",
      },
    ],
  },
  {
    id: "ch5",
    title: "Chapter 5: Layout & Architecture",
    dir: "ch5",
    lessons: [
      {
        id: "5.1",
        title: "5.1 Hierarchical Pipelines",
        markdown: "01_hierarchical_layouts.md",
        yaml: "01_hierarchical_layouts.yml",
      },
      {
        id: "5.2",
        title: "5.2 Groups & Architecture Tiers",
        markdown: "02_groups_and_tiers.md",
        yaml: "02_groups_and_tiers.yml",
      },
      {
        id: "5.3",
        title: "5.3 Grid & Circular Rings",
        markdown: "03_grid_and_circular.md",
        yaml: "03_grid_and_circular.yml",
      },
      {
        id: "5.4",
        title: "5.4 Manual Mode & Fine-Tuning",
        markdown: "04_manual_and_offsets.md",
        yaml: "04_manual_and_offsets.yml",
      },
    ],
  },
];

